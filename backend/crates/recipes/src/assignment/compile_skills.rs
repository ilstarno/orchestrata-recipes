use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;

use super::{
    AssignmentError, CompiledCapacity, skill_job::SkillJob, skill_match::SkillMatch,
    skill_named_inputs::SkillNamedInputs, skill_worker::SkillWorker,
};
use crate::constants::MAX_IDENTIFIER_BYTES;

const MAX_RECORDS: usize = 200;
const MAX_TEXT_BYTES: usize = 128;
const MAX_SKILLS: usize = 20;

pub(super) fn compile_skills(
    jobs_field: &str,
    workers_field: &str,
    mode: SkillMatch,
    source: &str,
) -> Result<CompiledCapacity, AssignmentError> {
    let SkillNamedInputs(input) =
        serde_json::from_str(source).map_err(|_| AssignmentError::InvalidInputs)?;
    if input
        .keys()
        .any(|key| key != jobs_field && key != workers_field)
    {
        return Err(AssignmentError::UnexpectedFields);
    }
    let jobs: Vec<SkillJob> = read_field(&input, jobs_field)?;
    let workers: Vec<SkillWorker> = read_field(&input, workers_field)?;
    if jobs.len() + workers.len() > MAX_RECORDS {
        return Err(AssignmentError::TooManyRecords);
    }

    let mut seen = BTreeSet::new();
    for job in &jobs {
        if !identifier(&job.id)
            || !seen.insert(&job.id)
            || !text(&job.name)
            || !text(&job.required_skill)
        {
            return Err(AssignmentError::InvalidField(jobs_field.to_owned()));
        }
    }

    let mut worker_capacities = Vec::with_capacity(workers.len());
    let mut workers_for_skill = BTreeMap::<_, Vec<usize>>::new();
    seen.clear();
    for (index, worker) in workers.iter().enumerate() {
        if !identifier(&worker.id)
            || !seen.insert(&worker.id)
            || !text(&worker.name)
            || worker.skills.len() > MAX_SKILLS
            || worker.capacity > MAX_RECORDS as u64
        {
            return Err(AssignmentError::InvalidField(workers_field.to_owned()));
        }
        let mut skills = BTreeSet::new();
        for skill in &worker.skills {
            if !text(skill) || !skills.insert(mode.key(skill)) {
                return Err(AssignmentError::InvalidField(workers_field.to_owned()));
            }
        }
        // Preserve workers with no capacity as indexed records, but don't emit their edges.
        worker_capacities.push(if worker.available {
            worker.capacity as usize
        } else {
            0
        });
        if worker.available && worker.capacity > 0 {
            for skill in skills {
                workers_for_skill.entry(skill).or_default().push(index);
            }
        }
    }
    let eligible_workers_by_job = jobs
        .iter()
        .map(|job| {
            workers_for_skill
                .get(&mode.key(&job.required_skill))
                .cloned()
                .unwrap_or_default()
        })
        .collect();

    Ok(CompiledCapacity {
        worker_capacities,
        eligible_workers_by_job,
    })
}

fn read_field<'de, T: Deserialize<'de>>(
    input: &BTreeMap<String, &'de serde_json::value::RawValue>,
    field: &str,
) -> Result<T, AssignmentError> {
    let raw = input
        .get(field)
        .ok_or_else(|| AssignmentError::MissingField(field.to_owned()))?;
    serde_json::from_str(raw.get()).map_err(|_| AssignmentError::InvalidField(field.to_owned()))
}

fn identifier(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= MAX_IDENTIFIER_BYTES
        && id
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

fn text(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_TEXT_BYTES
        && value.trim() == value
        && !value.chars().any(char::is_control)
}
