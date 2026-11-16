use std::collections::BTreeMap;

use orchestrata_recipes::{FieldValue, PayloadError, load_recipe};

const EXAMPLE: &str = include_str!("../recipes/field-service.xml");

fn example_input() -> BTreeMap<String, FieldValue> {
    BTreeMap::from([
        (
            "job-names".to_owned(),
            FieldValue::StringList(
                ["Lift repair", "Safety inspection", "Preventive maintenance"]
                    .map(str::to_owned)
                    .to_vec(),
            ),
        ),
        (
            "technician-names".to_owned(),
            FieldValue::StringList(["Alex", "Sam", "Noor"].map(str::to_owned).to_vec()),
        ),
        (
            "eligible-pairs".to_owned(),
            FieldValue::IntegerList(vec![0, 0, 0, 2, 1, 0, 1, 1, 2, 1, 2, 2]),
        ),
    ])
}

#[test]
fn checks_the_named_field_service_input_and_result_shapes() {
    let recipe = load_recipe(EXAMPLE).unwrap();
    let mut inputs = example_input();
    assert_eq!(recipe.validate_inputs(&inputs), Ok(()));
    inputs.insert(
        "preferred-lift-technician".to_owned(),
        FieldValue::Integer(0),
    );
    assert_eq!(recipe.validate_inputs(&inputs), Ok(()));

    let output = BTreeMap::from([(
        "assigned-technicians".to_owned(),
        FieldValue::IntegerList(vec![0, 1, 2]),
    )]);
    assert_eq!(recipe.validate_outputs(&output), Ok(()));
    assert_eq!(
        recipe.validate_inputs(&output),
        Err(PayloadError::UnknownField(
            "assigned-technicians".to_owned()
        ))
    );
}

#[test]
fn limits_dynamic_values_and_reports_missing_or_mismatched_fields() {
    let recipe = load_recipe(EXAMPLE).unwrap();
    let mut inputs = example_input();
    inputs.remove("job-names");
    assert_eq!(
        recipe.validate_inputs(&inputs),
        Err(PayloadError::MissingField("job-names".to_owned()))
    );
    inputs.insert("job-names".to_owned(), FieldValue::Boolean(true));
    assert_eq!(
        recipe.validate_inputs(&inputs),
        Err(PayloadError::WrongType("job-names".to_owned()))
    );
    inputs.insert(
        "job-names".to_owned(),
        FieldValue::StringList(vec!["ab".repeat(2_049)]),
    );
    assert_eq!(recipe.validate_inputs(&inputs), Err(PayloadError::TooLarge));

    inputs.insert(
        "job-names".to_owned(),
        FieldValue::StringList(vec!["job".repeat(1_000); 50]),
    );
    assert_eq!(recipe.validate_inputs(&inputs), Err(PayloadError::TooLarge));
}
