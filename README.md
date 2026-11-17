# Orchestrata recipes

Browse the [planning examples](recipes/README.md) for small, fictional scheduling problems. Each example includes XML metadata and a normalized input for an Orchestrata solver, plus an expected result for comparison.

Run the recipe tests or generate synthetic fleet stops from this checkout:

```sh
cargo test --workspace --all-targets
cargo run -p orchestrata-recipes --example fleet_benchmark -- recipes/benchmarks/new-york.xml --csv > new-york-stops.csv
```

The [fleet workloads](recipes/benchmarks/README.md) generate inputs only. They do not solve routes or estimate travel times. To solve a planning example, supply its `problem.json` to an installed Orchestrata CLI or running solve API.

For interview scheduling, interviewer absence recovery, and recruiting
coordinator handoffs, see [Hiring operations recipes](docs/hiring-operations.md).

For advisory DNS steering, backend allocation, and rollout gates, see
[Traffic planning recipes](docs/traffic-planning.md).

For payment exceptions and claims-work scheduling without transaction or
eligibility decisions, see [Financial operations recipes](docs/financial-operations.md).

## Run a recipe directly

The [executable field-service recipe](recipes/executable/README.md) reads an
XML assignment rule and named JSON inputs, then returns a plan. Other recipe
XML files in this repository describe examples that still run through their
checked-in JSON problems.
