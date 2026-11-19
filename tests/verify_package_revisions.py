"""Check deterministic revisions, immutable behavior, and native rejection gates."""
import argparse
import copy
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

from verify_loading_dock import DOCK_FIXTURE, oracle, score

ROOT = Path(__file__).resolve().parents[1]


def call(*args):
    return subprocess.run([str(a) for a in args], capture_output=True, timeout=30, check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--pack", required=True, type=Path)
    parser.add_argument("--engine-bin-dir", required=True, type=Path)
    args = parser.parse_args()
    bins = args.engine_bin_dir.resolve()
    with tempfile.TemporaryDirectory() as directory:
        work = Path(directory)
        for revision in sorted((ROOT / "packages").glob("*/versions/*")):
            original = revision.parent.parent
            archive = next(revision.glob("*.orcpkg"))
            source = revision / "source"
            outputs = [work / (original.name + str(i) + ".orcpkg") for i in range(2)]
            command = [sys.executable, "-B", ROOT / "tools/build_package.py", source]
            options = ["--pack", args.pack.resolve(), "--validator", bins / "validate-package"]
            for output in outputs:
                call(*command, output, *options)
                assert output.read_bytes() == archive.read_bytes()
            refused = subprocess.run([str(a) for a in [*command, outputs[0], *options]], capture_output=True, timeout=30)
            assert refused.returncode != 0 and outputs[0].read_bytes() == archive.read_bytes()
            bad = work / original.name
            shutil.copytree(source, bad)
            manifest = bad / "recipe.xml"
            manifest.write_bytes(manifest.read_bytes().rstrip(b"\n"))
            rejected = work / "rejected.orcpkg"
            result = subprocess.run([str(a) for a in [sys.executable, ROOT / "tools/build_package.py", bad, rejected, *options]], capture_output=True, timeout=30)
            assert result.returncode != 0 and not rejected.exists()
            for entry in json.loads((ROOT / "catalog.json").read_text())["entries"]:
                if entry["path"] != archive.relative_to(ROOT).as_posix():
                    continue
                inputs = ROOT / entry["inputs"]
                new = json.loads(call(bins / "solve-package", archive, inputs, entry["entrypoint"]).stdout)
                old = json.loads(call(bins / "solve-package", original / archive.name, inputs, entry["entrypoint"]).stdout)
                assert new["solution"] == old["solution"]
                assert new["content_identity"] != old["content_identity"]
        dock = ROOT / "packages/loading-dock-single-window/versions/1.0.1/loading-dock-single-window.orcpkg"
        closed = copy.deepcopy(DOCK_FIXTURE)
        closed["doors"][0]["available"] = False
        unavailable = copy.deepcopy(DOCK_FIXTURE)
        for door in unavailable["doors"]:
            door["capacity"] = 0
        for records in (DOCK_FIXTURE, closed, unavailable):
            inputs = work / "inputs.json"
            inputs.write_text(json.dumps(records))
            result = json.loads(call(bins / "solve-package", dock, inputs, "allocate").stdout)["solution"]
            actual = (-result["assigned"], result["minimized_edge_sum"])
            choices = [row["resource_id"] for row in result["assignments"]]
            assert actual == oracle(records, 1) == score(records, choices, 1)
    print("Two deterministic revisions; source parity, no-clobber, malformed-source rejection, unchanged solutions and three exact dock cases passed.")


if __name__ == "__main__":
    main()
