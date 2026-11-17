use std::{env, error::Error, fs::File, io::Read, path::Path, process::ExitCode};

use orchestrata_recipes::assignment::{compile_capacity_json, load_assignment_recipe};
use orchestrata_solver::capacity_matching::{CapacityMatchProblem, match_capacity};

fn run() -> Result<(), Box<dyn Error>> {
    let paths = env::args_os().skip(1).collect::<Vec<_>>();
    let [xml, inputs] = paths.as_slice() else {
        return Err("Usage: orchestrata-assignment-runner <assignment.xml> <inputs.json>".into());
    };
    let spec = load_assignment_recipe(&read(Path::new(xml), 32 * 1024)?)?;
    let compiled = compile_capacity_json(&spec, &read(Path::new(inputs), 512 * 1024)?)?;
    let result = match_capacity(&CapacityMatchProblem {
        worker_capacities: compiled.worker_capacities,
        eligible_workers_by_job: compiled.eligible_workers_by_job,
    })?;
    println!(
        "{}",
        serde_json::json!({
            (spec.output_field()): result.workers_by_job,
            "assigned": result.assigned,
        })
    );
    Ok(())
}

fn read(path: &Path, bytes: u64) -> std::io::Result<String> {
    let mut result = String::new();
    File::open(path)?
        .take(bytes + 1)
        .read_to_string(&mut result)?;
    Ok(result)
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
