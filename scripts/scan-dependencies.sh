#!/usr/bin/env bash
set -euo pipefail

test -s "Cargo.lock"
test -s "examples/assignment-runner/Cargo.lock"
scan_dir="$(mktemp -d)"
trap 'rm -rf -- "$scan_dir"' EXIT
curl --fail --silent --show-error --location --retry 2 --max-time 120 \
  --header 'Accept: application/octet-stream' \
  https://api.github.com/repos/google/osv-scanner/releases/assets/726165286 \
  --output "$scan_dir/osv-scanner"
printf '%s  %s\n' fb7dab19a8bd9bbe5c3189f09d4b7ee1820c9553a23bac202648a56efd2f6ee8 \
  "$scan_dir/osv-scanner" | sha256sum --check
chmod 0755 "$scan_dir/osv-scanner"
export OSV_SCANNER_LOCAL_DB_CACHE_DIRECTORY="$scan_dir/cache"
mkdir -p "$OSV_SCANNER_LOCAL_DB_CACHE_DIRECTORY/osv-scalibr/crates.io"
curl --fail --silent --show-error --location --retry 2 --max-time 120 \
  https://storage.googleapis.com/osv-vulnerabilities/crates.io/all.zip \
  --output "$OSV_SCANNER_LOCAL_DB_CACHE_DIRECTORY/osv-scalibr/crates.io/all.zip"
"$scan_dir/osv-scanner" scan source --offline --lockfile="Cargo.lock" --lockfile="examples/assignment-runner/Cargo.lock"
