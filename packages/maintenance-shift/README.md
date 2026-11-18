# Maintenance shift / Turni i mirëmbajtjes

Allocate available technicians to eligible work orders within their capacity. `cost` maximizes coverage then minimizes cents; `delay` maximizes coverage then minimizes aggregate delay minutes. Both use the same typed named inputs and supported mathematical XML operators. This is capacity allocation, not route planning or appointment scheduling.

Caktoni teknikët e disponueshëm për punët e përshtatshme brenda kapacitetit të tyre. Modeli `cost` maksimizon mbulimin dhe pastaj minimizon koston në cent. Modeli `delay` maksimizon mbulimin dhe pastaj minimizon vonesën totale në minuta. Të dy modelet përdorin të njëjtat të dhëna hyrëse me tipe të përcaktuara.

Use the Engine's `solve-package` command with `maintenance-shift.orcpkg`, canonical JSON records and `cost` or `delay`. The [runnable twelve-order example](https://github.com/ilstarno/orchestrata-engine/tree/main/examples/maintenance-package) includes CSV and connector-style records, exact acceptance, expected totals, input types and result identities. Data normalization belongs outside this package.

With Engine and Recipes checked out as sibling directories, run from the Engine root:

```sh
cargo build --locked -p orchestrata-recipe-package --bins
target/debug/solve-package ../orchestrata-recipes/packages/maintenance-shift/maintenance-shift.orcpkg examples/maintenance-package/inputs.json cost
python3 crates/recipe-package/tools/pack.py ../orchestrata-recipes/packages/maintenance-shift/source /tmp/maintenance-shift.orcpkg --validator target/debug/validate-package
cmp /tmp/maintenance-shift.orcpkg ../orchestrata-recipes/packages/maintenance-shift/maintenance-shift.orcpkg
```

The output path must not exist. Authoring requires Python 3.10+, POSIX and filesystem hard-link support; it validates before atomic no-clobber publication. Edit source models/metadata and rebuild to update the package. Asset digests are refreshed in the packaged manifest without rewriting executable XML.

See [source attribution](PROVENANCE.md). Apache-2.0 terms are retained in English; localized metadata does not translate the license. The `free` tier and author declaration do not prove server authorization or publisher identity. Existing loose XML remains supported; descriptive catalog entries are not automatically executable packages.
