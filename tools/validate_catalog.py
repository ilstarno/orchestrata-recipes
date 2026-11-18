"""List catalog entries only after native execution matches their checked examples."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path, PurePosixPath
import subprocess
import sys

COMMANDS = {
    "assignment": "orchestrata-assignment-runner",
    "model": "solve-model",
    "package": "solve-package",
}
ENTRY_KEYS = {"id", "path", "kind", "inputs", "expected", "entrypoint"}


def read_json(path):
    if path.stat().st_size > 1024 * 1024:
        raise ValueError(f"JSON too large: {path.name}")

    def unique(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError(f"Duplicate JSON key: {key}")
            result[key] = value
        return result

    return json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=unique)


def local_file(root, value):
    if not isinstance(value, str) or not value or "\\" in value:
        raise ValueError("Invalid catalog path")
    relative = PurePosixPath(value)
    if relative.is_absolute() or any(p in ("", ".", "..") for p in value.split("/")):
        raise ValueError("Catalog path must be relative and canonical")
    path = root
    for part in relative.parts:
        path = path / part
        if path.is_symlink():
            raise ValueError("Catalog paths must not contain symlinks")
    if not path.is_file():
        raise ValueError(f"Missing catalog file: {value}")
    return path


def check_entry(root, entry):
    if not isinstance(entry, dict) or set(entry) != ENTRY_KEYS:
        raise ValueError("Invalid entry fields")
    identifier, kind, mode = entry["id"], entry["kind"], entry["entrypoint"]
    if not isinstance(identifier, str) or not identifier:
        raise ValueError("Entry needs an ID")
    if not isinstance(kind, str) or kind not in COMMANDS:
        raise ValueError("Unsupported execution kind")
    for key in ("path", "inputs", "expected"):
        local_file(root, entry[key])
    for key in ("inputs", "expected"):
        if not entry[key].startswith("tests/fixtures/catalog/"):
            raise ValueError("Examples must be in the checked fixture directory")
    if kind == "package":
        if (
            not entry["path"].startswith("packages/")
            or not entry["path"].endswith(".orcpkg")
            or not isinstance(mode, str)
            or not mode
        ):
            raise ValueError("Package needs an explicit entrypoint")
    elif (
        not entry["path"].startswith("recipes/")
        or not entry["path"].endswith(".xml")
        or mode is not None
    ):
        raise ValueError("Loose XML cannot declare a package entrypoint")


def inventory(root):
    document = read_json(root / "catalog.json")
    if (
        not isinstance(document, dict)
        or set(document) != {"version", "entries", "generators"}
        or type(document["version"]) is not int
        or document["version"] != 1
    ):
        raise ValueError("Unsupported catalog document")
    entries, generators = document["entries"], document["generators"]
    if not isinstance(entries, list) or not 1 <= len(entries) <= 32:
        raise ValueError("Catalog needs 1..32 executable entries")
    if (
        not isinstance(generators, list)
        or len(generators) > 32
        or not all(isinstance(p, str) for p in generators)
        or len(set(generators)) != len(generators)
    ):
        raise ValueError("Invalid generator inventory")
    ids, executions, loose, packages = set(), set(), set(), set()
    for entry in entries:
        check_entry(root, entry)
        if entry["id"] in ids:
            raise ValueError("Duplicate entry ID")
        ids.add(entry["id"])
        execution = (entry["path"], entry["entrypoint"])
        if execution in executions:
            raise ValueError("Duplicate execution")
        executions.add(execution)
        (packages if entry["kind"] == "package" else loose).add(entry["path"])
    for path in generators:
        if not path.startswith("recipes/benchmarks/") or not path.endswith(".xml"):
            raise ValueError("Generators must remain outside executable discovery")
        local_file(root, path)
    xml = {
        str(p.relative_to(root))
        for p in (root / "recipes").rglob("*")
        if p.suffix.lower() == ".xml"
    }
    archives = {
        str(p.relative_to(root))
        for p in (root / "packages").rglob("*")
        if p.suffix.lower() == ".orcpkg"
    }
    if (
        loose & set(generators)
        or xml != loose | set(generators)
        or archives != packages
    ):
        raise ValueError("Unregistered XML/package or overlapping generator entry")
    return entries


def invoke(binary, *args):
    result = subprocess.run(
        [str(binary), *map(str, args)], capture_output=True, timeout=30
    )
    if len(result.stdout) > 1024 * 1024 or len(result.stderr) > 1024 * 1024:
        raise ValueError("Runtime output limit exceeded")
    return result


def validate(root, engine_bins):
    entries = inventory(root)
    names = {*COMMANDS.values(), "validate-package"}
    binaries = {name: engine_bins / name for name in names}
    for name, path in binaries.items():
        if not path.is_file() or not os.access(path, os.X_OK):
            raise ValueError(f"Required native runtime unavailable: {name}")
    descriptor = local_file(root, "tests/fixtures/catalog/descriptive.xml")
    inputs = local_file(root, "tests/fixtures/catalog/assignment-inputs.json")
    for name in ("orchestrata-assignment-runner", "solve-model"):
        rejected = invoke(binaries[name], descriptor, inputs)
        if rejected.returncode == 0 or rejected.stdout:
            raise ValueError(f"Runtime accepted descriptive XML: {name}")
    verified = []
    for entry in entries:
        model = local_file(root, entry["path"])
        if entry["kind"] == "package":
            checked = invoke(binaries["validate-package"], model)
            if checked.returncode:
                raise ValueError(f"Package validation failed: {entry['id']}")
        args = [model, local_file(root, entry["inputs"])]
        if entry["entrypoint"] is not None:
            args.append(entry["entrypoint"])
        result = invoke(binaries[COMMANDS[entry["kind"]]], *args)
        expected = read_json(local_file(root, entry["expected"]))
        if result.returncode:
            raise ValueError(f"Native execution failed: {entry['id']}")
        actual = json.loads(result.stdout)
        if json.dumps(actual, sort_keys=True) != json.dumps(expected, sort_keys=True):
            raise ValueError(f"Native execution mismatch: {entry['id']}")
        verified.append({key: entry[key] for key in ("id", "path", "kind", "entrypoint")})
    return verified


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--engine-bin-dir", type=Path, required=True)
    args = parser.parse_args()
    try:
        entries = validate(Path(__file__).resolve().parents[1], args.engine_bin_dir.resolve())
    except (ValueError, OSError, subprocess.TimeoutExpired) as error:
        print(f"Catalog rejected: {error}", file=sys.stderr)
        return 1
    print(json.dumps({"verified_entries": entries}, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
