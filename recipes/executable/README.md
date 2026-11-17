# Assign field-service jobs from XML

The [assignment recipe](field-service/recipe.xml) names the inputs, declares
maximum coverage and names the result field. Supply a list of jobs, workers,
their job-count limits and eligible job-worker pairs. The engine assigns as
many jobs as possible without placing any job outside that eligibility list
or exceeding a worker's limit.

Run from the repository root with stable Rust:

```sh
cargo run --locked --manifest-path examples/assignment-runner/Cargo.toml -- \
  recipes/executable/field-service/recipe.xml \
  recipes/executable/field-service/inputs.json
```

The [sample inputs](field-service/inputs.json) give this result:

```json
{"assigned":3,"assigned-technicians":[2,0,1]}
```

The entries are worker indices in the order you supplied the jobs. Supply
[field-service inputs with a staffing gap](field-service/shortage.json)
instead to see one unassigned job:

```json
{"assigned":2,"assigned-technicians":[2,0,null]}
```

## Supply your own inputs

- Use one JSON object with exactly the four input fields named in the XML.
- Provide a separate ordered list of unique job and worker names. Use at most
  200 jobs and workers combined.
- Provide a job-count limit from 0 to 200 for each worker, in worker order.
- List eligible pairs as consecutive zero-based indices, job first and worker
  second. Use an empty array when no pair is eligible.

A malformed recipe or input returns an error, not a plan. An unassigned job
means it did not fit the supplied eligibility and capacity limits; it does
not state why the business rules produced those inputs. Version 1 does not
infer qualifications or availability. The runner does not compare
preferences, schedule visits over time or plan routes. Other XML examples describe their individual use
cases but do not run through this assignment format.

## Match jobs to skills and available workers

The [version 2 recipe](field-service-skills/recipe.xml) takes two lists of
records rather than supplied eligibility pairs. Each job has an `id`, a
`name` and a `required_skill`. Each worker has an `id`, a `name`, a list of
`skills`, a true/false `available` value and a whole-number `capacity`
from 0 to 200. Keep IDs unique within each list. Use up to 64 lowercase
ASCII letters, digits or hyphens per ID. Display names can repeat. Use up
to 200 records combined and up to 20 skills per worker. Names and skills
must be nonempty and at most 128 bytes with no surrounding spaces.

Version 2 adds an edge only when the available worker has capacity and
the required skill. Use `match="exact"` to match the text as written or
`match="ascii-case-insensitive"` to ignore case in A–Z only. Neither mode
translates skills, changes accents, or treats different Unicode spellings
as equal. Missing or malformed fields fail rather than supplying defaults.

Use the same runner with
`recipes/executable/field-service-skills/recipe.xml` and
`recipes/executable/field-service-skills/inputs.json`. The example returns
`{"assigned":2,"assigned-technicians":[0,1]}`. The worker indices refer to the
original worker order, even when names repeat. Make either available worker
unavailable and one job is left unassigned. Version 1 remains available
for callers that already provide an indexed eligibility list.
