use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use super::{AssignmentError, AssignmentRecipeSpec, CompiledCapacity, named_fields::NamedFields};

const MAX_BYTES: usize = 512 * 1024;
const MAX_RECORDS: usize = 200;
const MAX_NAME_BYTES: usize = 128;

/// Compile named inputs exactly as declared by this recipe. No fields are inferred.
pub fn compile_capacity_json(
    spec: &AssignmentRecipeSpec,
    source: &str,
) -> Result<CompiledCapacity, AssignmentError> {
    if source.len() > MAX_BYTES {
        return Err(AssignmentError::InputsTooLarge);
    }
    let NamedFields(values) =
        serde_json::from_str(source).map_err(|_| AssignmentError::InvalidInputs)?;
    if values.keys().any(|key| {
        ![
            &spec.jobs,
            &spec.workers,
            &spec.capacities,
            &spec.eligibility,
        ]
        .contains(&key)
    }) {
        return Err(AssignmentError::UnexpectedFields);
    }

    let jobs = name_count(&values, &spec.jobs)?;
    let workers = name_count(&values, &spec.workers)?;
    if jobs + workers > MAX_RECORDS {
        return Err(AssignmentError::TooManyRecords);
    }
    let capacities = array(&values, &spec.capacities)?;
    if capacities.len() != workers {
        return Err(field_error(&spec.capacities));
    }
    let worker_capacities = capacities
        .iter()
        .map(|value| {
            bounded_index(value, MAX_RECORDS + 1).ok_or_else(|| field_error(&spec.capacities))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let pairs = array(&values, &spec.eligibility)?;
    if pairs.len() > 2 * jobs * workers || pairs.len() % 2 != 0 {
        return Err(field_error(&spec.eligibility));
    }
    let mut eligible_workers_by_job = vec![Vec::new(); jobs];
    let mut seen_pairs = vec![false; jobs * workers];
    for pair in pairs.as_chunks::<2>().0 {
        let job = bounded_index(&pair[0], jobs).ok_or_else(|| field_error(&spec.eligibility))?;
        let worker =
            bounded_index(&pair[1], workers).ok_or_else(|| field_error(&spec.eligibility))?;
        if std::mem::replace(&mut seen_pairs[job * workers + worker], true) {
            return Err(field_error(&spec.eligibility));
        }
        eligible_workers_by_job[job].push(worker);
    }
    Ok(CompiledCapacity {
        worker_capacities,
        eligible_workers_by_job,
    })
}

fn name_count(values: &BTreeMap<String, Value>, field: &str) -> Result<usize, AssignmentError> {
    let names = array(values, field)?;
    if names.len() > MAX_RECORDS {
        return Err(field_error(field));
    }
    let mut unique = BTreeSet::new();
    for value in names {
        let name = value.as_str().ok_or_else(|| field_error(field))?;
        if name.is_empty()
            || name.len() > MAX_NAME_BYTES
            || name.trim() != name
            || !unique.insert(name)
        {
            return Err(field_error(field));
        }
    }
    Ok(names.len())
}

fn array<'a>(
    values: &'a BTreeMap<String, Value>,
    field: &str,
) -> Result<&'a Vec<Value>, AssignmentError> {
    values
        .get(field)
        .ok_or_else(|| AssignmentError::MissingField(field.to_owned()))?
        .as_array()
        .ok_or_else(|| field_error(field))
}

fn field_error(field: &str) -> AssignmentError {
    AssignmentError::InvalidField(field.to_owned())
}

fn bounded_index(value: &Value, limit: usize) -> Option<usize> {
    let index = usize::try_from(value.as_u64()?).ok()?;
    (index < limit).then_some(index)
}
