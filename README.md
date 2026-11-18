# Orchestrata recipes

Executable XML recipes and versioned packages under Apache-2.0. The checked catalog contains seven execution choices: four loose XML models and three entrypoints across two packages.

| Package | Entrypoint | Checked synthetic outcome |
| --- | --- | --- |
| `packages/maintenance-shift/maintenance-shift.orcpkg` | `cost` | Assign 10 work orders; maximize coverage, then minimize cents. |
| Same package | `delay` | Assign 10 work orders; maximize coverage, then minimize delay minutes. |
| `packages/loading-dock-single-window/loading-dock-single-window.orcpkg` | `allocate` | Assign three overlapping trailers to compatible doors in one supplied window. |

Maintenance is capacity allocation, not route planning. Loading-door allocation does not schedule arrivals; compatibility is supplied by the caller. See the [maintenance package usage](packages/maintenance-shift/README.md) for its input contract and source attribution.

## Build and check the runnable catalog

With [orchestrata-engine](https://github.com/ilstarno/orchestrata-engine) checked out beside this repository, run from the Recipes root:

```sh
ENGINE=../orchestrata-engine
cargo build --locked --manifest-path "$ENGINE/Cargo.toml" -p orchestrata-recipe-runtime -p orchestrata-recipe-package --bins
cargo build --locked --manifest-path "$ENGINE/examples/assignment-runner/Cargo.toml" --target-dir "$ENGINE/target"
python3 -B tools/validate_catalog.py --engine-bin-dir "$ENGINE/target/debug"
python3 -B -m unittest discover -s tests -v
python3 -B tests/verify_catalog.py --engine-bin-dir "$ENGINE/target/debug"
```

The catalog validator requires actual native execution against the checked synthetic inputs and expected results. It rejects descriptive XML, missing tools, unregistered recipe files, invalid packages and mismatched results. Catalog metadata and successful XML parsing alone do not establish executability. Use an Engine revision that provides all four required commands: `orchestrata-assignment-runner`, `solve-model`, `validate-package` and `solve-package`.

Run a package directly:

```sh
"$ENGINE/target/debug/solve-package" packages/loading-dock-single-window/loading-dock-single-window.orcpkg tests/fixtures/catalog/dock-inputs.json allocate
"$ENGINE/target/debug/solve-package" packages/maintenance-shift/maintenance-shift.orcpkg tests/fixtures/catalog/maintenance-inputs.json cost
```

The two `recipes/executable/` contracts run through `orchestrata-assignment-runner`; the two `recipes/mathematical/` models run through `solve-model`. `catalog.json` records their input and expected-result files. Package source XML is retained for deterministic rebuilds with the Engine authoring tool.

`recipes/benchmarks/new-york.xml`, `rome.xml` and `tirana.xml` configure workload generators. They are preserved but excluded from the executable catalog. Former standalone descriptive entries are no longer offered as runnable recipes; their previous paths remain recoverable from Git history. Engine embedded example APIs remain separate.

All acceptance inputs are synthetic.
