use std::{path::Path, process::Command};

use serde_json::{Value, json};

fn invoke(recipe: &str, inputs: &str) -> std::process::Output {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Command::new(env!("CARGO_BIN_EXE_orchestrata-assignment-runner"))
        .arg(root.join(recipe))
        .arg(root.join(inputs))
        .output()
        .unwrap()
}

#[test]
fn executable_recipe_and_supplied_inputs_run_through_rust_engine() {
    let sample = invoke(
        "recipes/executable/field-service/recipe.xml",
        "recipes/executable/field-service/inputs.json",
    );
    assert!(
        sample.status.success(),
        "{}",
        String::from_utf8_lossy(&sample.stderr)
    );
    let actual: Value = serde_json::from_slice(&sample.stdout).unwrap();
    assert_eq!(actual, json!({"assigned":3,"assigned-technicians":[2,0,1]}));
}

#[test]
fn a_job_with_no_eligible_worker_stays_unassigned() {
    let sample = invoke(
        "recipes/executable/field-service/recipe.xml",
        "recipes/executable/field-service/shortage.json",
    );
    assert!(sample.status.success());
    let actual: Value = serde_json::from_slice(&sample.stdout).unwrap();
    assert_eq!(
        actual,
        json!({"assigned":2,"assigned-technicians":[2,0,null]})
    );
}

#[test]
fn legacy_metadata_and_nonconforming_inputs_never_return_a_plan() {
    for (recipe, input) in [
        (
            "recipes/field-service.xml",
            "recipes/executable/field-service/inputs.json",
        ),
        (
            "recipes/executable/field-service/recipe.xml",
            "recipes/examples/field-service/problem.json",
        ),
    ] {
        let rejected = invoke(recipe, input);
        assert!(!rejected.status.success());
        assert!(rejected.stdout.is_empty());
        assert!(!rejected.stderr.is_empty());
    }
}

#[test]
fn skills_runner_rejects_positional_records_before_solving() {
    use std::{
        env, fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = env::temp_dir().join(format!(
        "assignment-named-records-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir(&dir).unwrap();
    let path = dir.join("input.json");

    let recipe = "recipes/executable/field-service-skills/recipe.xml";
    let valid: Value = serde_json::from_str(include_str!(
        "../../../recipes/executable/field-service-skills/inputs.json"
    ))
    .unwrap();
    let accepted = invoke(
        recipe,
        "recipes/executable/field-service-skills/inputs.json",
    );
    assert!(accepted.status.success());
    assert_eq!(
        serde_json::from_slice::<Value>(&accepted.stdout).unwrap(),
        json!({"assigned":2,"assigned-technicians":[0,1]})
    );

    let mut outputs = Vec::new();
    for (field, positional) in [
        ("jobs", json!(["door-west", "Repair door", "Repair"])),
        (
            "workers",
            json!(["tech-1", "Technician", ["Repair"], true, 1]),
        ),
    ] {
        let mut invalid = valid.clone();
        invalid[field][0] = positional;
        fs::write(&path, invalid.to_string()).unwrap();
        outputs.push((field, invoke(recipe, path.to_str().unwrap())));
    }
    fs::remove_dir_all(dir).unwrap();

    for (field, output) in outputs {
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert_eq!(
            String::from_utf8(output.stderr).unwrap().trim(),
            format!("Check the values in {field}.")
        );
    }
}
