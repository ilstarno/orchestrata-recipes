# Scan locked dependencies

Run from the repository root on Linux x86-64 with Bash, curl and sha256sum:

```sh
bash scripts/scan-dependencies.sh
```

The script downloads the pinned Google OSV-Scanner release and checks its SHA-256, then downloads the public Rust advisory database. Matching runs offline against the checked-in lockfiles. An advisory or scanner failure returns a nonzero status.

The scan covers the Rust library and assignment runner lockfiles. It does not validate recipe contents or benchmark results.
