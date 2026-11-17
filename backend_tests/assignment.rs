use orchestrata_recipes::assignment::{
    AssignmentError, compile_capacity_json, load_assignment_recipe,
};
use serde_json::json;

const RECIPE: &str = include_str!("../recipes/executable/field-service/recipe.xml");
const INPUTS: &str = include_str!("../recipes/executable/field-service/inputs.json");

#[test]
fn declared_fields_compile_in_original_order() {
    let spec = load_assignment_recipe(RECIPE).unwrap();
    assert_eq!(spec.output_field(), "assigned-technicians");
    let compiled = compile_capacity_json(&spec, INPUTS).unwrap();
    assert_eq!(compiled.worker_capacities, [1, 1, 1]);
    assert_eq!(
        compiled.eligible_workers_by_job,
        [vec![0, 2], vec![0, 1], vec![1, 2]]
    );
    let alternate = RECIPE.replace("job-names", "jobs");
    let spec = load_assignment_recipe(&alternate).unwrap();
    assert_eq!(
        compile_capacity_json(&spec, &INPUTS.replace("job-names", "jobs")).unwrap(),
        compiled
    );
    assert!(compile_capacity_json(&spec, INPUTS).is_err());
}

#[test]
fn no_rule_is_inferred_from_metadata_prose_or_unknown_xml() {
    for xml in [
        include_str!("../recipes/field-service.xml").to_owned(),
        RECIPE.replace("maximum-coverage", "minimum-cost"),
        RECIPE.replace("version=\"1\">", "version=\"2\">"),
        RECIPE.replace("<jobs ", "<jobs extra=\"x\" "),
        RECIPE.replace("<jobs ", "<jobs xmlns=\"urn:x\" "),
        RECIPE.replace(
            "<jobs ",
            "<xi:include href=\"file:///not-opened\" xmlns:xi=\"urn:x\"/><jobs ",
        ),
        RECIPE.replace("<jobs ", "<?process ignore?><jobs "),
        RECIPE.replace(
            "<assignment-recipe",
            "<!DOCTYPE assignment-recipe><assignment-recipe",
        ),
        RECIPE.replace("assigned-technicians", "assigned"),
        RECIPE.replace("technician-capacities", "technician-names"),
    ] {
        assert_eq!(
            load_assignment_recipe(&xml),
            Err(AssignmentError::InvalidRecipe)
        );
    }
    assert_eq!(
        load_assignment_recipe(&" ".repeat(32 * 1024 + 1)),
        Err(AssignmentError::RecipeTooLarge)
    );
}

#[test]
fn malformed_duplicate_wrong_type_and_out_of_range_inputs_are_rejected() {
    let spec = load_assignment_recipe(RECIPE).unwrap();
    for inputs in [
        "[]".to_owned(),
        INPUTS.replace("\"job-names\":", "\"job-names\": [], \"\\u006aob-names\":"),
        INPUTS.replace("\"eligible-pairs\"", "\"unrecognized\""),
        INPUTS.replace("0, 0, 0, 2", "-1, 0, 0, 2"),
        INPUTS.replace("0, 0, 0, 2", "0, 3, 0, 2"),
        INPUTS.replace("0, 0, 0, 2", "0, 0, 0, 0"),
        INPUTS.replace("0, 0, 0, 2", "0, 0, 0"),
        INPUTS.replace("0, 0, 0, 2", "0, 0, 0, 1.0"),
        INPUTS.replace("[1, 1, 1]", "[1, 1]"),
        INPUTS.replace("[1, 1, 1]", "[1, 1, 201]"),
        INPUTS.replace("\"Sam\"", "\"Alex\""),
        INPUTS.replace("\"Sam\"", "\" \""),
    ] {
        assert!(compile_capacity_json(&spec, &inputs).is_err(), "{inputs}");
    }
    assert_eq!(
        compile_capacity_json(&spec, "{}"),
        Err(AssignmentError::MissingField("job-names".to_owned()))
    );
    assert_eq!(
        compile_capacity_json(&spec, &" ".repeat(512 * 1024 + 1)),
        Err(AssignmentError::InputsTooLarge)
    );
}

#[test]
fn combined_record_limit_admits_a_dense_200_record_sheet() {
    let spec = load_assignment_recipe(RECIPE).unwrap();
    let jobs = (0..100).map(|i| format!("job-{i}")).collect::<Vec<_>>();
    let workers = (0..100).map(|i| format!("worker-{i}")).collect::<Vec<_>>();
    let edges = (0..100)
        .flat_map(|j| (0..100).flat_map(move |w| [j, w]))
        .collect::<Vec<_>>();
    let input = json!({"job-names":jobs,"technician-names":workers,"technician-capacities":vec![1;100],"eligible-pairs":edges});
    let result = compile_capacity_json(&spec, &input.to_string()).unwrap();
    assert!(
        result
            .eligible_workers_by_job
            .iter()
            .all(|row| row.len() == 100)
    );
    let mut excessive = input;
    excessive["job-names"]
        .as_array_mut()
        .unwrap()
        .push(json!("job-101"));
    assert_eq!(
        compile_capacity_json(&spec, &excessive.to_string()),
        Err(AssignmentError::TooManyRecords)
    );
}

#[test]
fn explicit_no_workers_is_valid_and_never_filled_in() {
    let spec = load_assignment_recipe(RECIPE).unwrap();
    let input = json!({"job-names":["job-1"],"technician-names":[],"technician-capacities":[],"eligible-pairs":[]});
    let compiled = compile_capacity_json(&spec, &input.to_string()).unwrap();
    assert!(compiled.worker_capacities.is_empty());
    assert_eq!(compiled.eligible_workers_by_job, [Vec::<usize>::new()]);
}
