"""Verify migrated loading-dock descriptor through the released native package tools."""
from __future__ import annotations

import argparse
import copy
import itertools
import json
import shutil
import subprocess
import sys
import tempfile
from collections import Counter
from pathlib import Path


# Port the historical simultaneous loading-door example, not customer data.
DOCK_FIXTURE = {'trailers': [{'id': 'refrigerated-inbound'},
              {'id': 'parcel-outbound'},
              {'id': 'palletized-transfer'}],
 'doors': [{'id': 'door-4', 'capacity': 1, 'available': True},
           {'id': 'door-7', 'capacity': 1, 'available': True},
           {'id': 'door-9', 'capacity': 1, 'available': True}],
 'compatibility': [{'trailer_id': 'refrigerated-inbound',
                    'door_id': 'door-4',
                    'avoidance_penalty': 0},
                   {'trailer_id': 'refrigerated-inbound',
                    'door_id': 'door-7',
                    'avoidance_penalty': 0},
                   {'trailer_id': 'parcel-outbound',
                    'door_id': 'door-7',
                    'avoidance_penalty': 0},
                   {'trailer_id': 'parcel-outbound',
                    'door_id': 'door-9',
                    'avoidance_penalty': 5},
                   {'trailer_id': 'palletized-transfer',
                    'door_id': 'door-4',
                    'avoidance_penalty': 0},
                   {'trailer_id': 'palletized-transfer',
                    'door_id': 'door-9',
                    'avoidance_penalty': 0}]}


def call(*argv, check=True):
    return subprocess.run(argv, capture_output=True, timeout=30, check=check)


def score(document, choices, max_per_door):
    trailers = [row["id"] for row in document["trailers"]]
    doors = {row["id"]: row for row in document["doors"]}
    penalties = {(row["trailer_id"], row["door_id"]): row["avoidance_penalty"]
                 for row in document["compatibility"]}
    used = Counter(choice for choice in choices if choice is not None)
    if any(not doors[v]["available"] or n > min(doors[v]["capacity"], max_per_door)
           for v, n in used.items()):
        return None
    decisions = [(trailer, door) for trailer, door in zip(trailers, choices) if door is not None]
    if any(pair not in penalties for pair in decisions):
        return None
    return (-len(decisions), sum(penalties[pair] for pair in decisions))


def oracle(document, max_per_door):
    options = [None] + [row["id"] for row in document["doors"]]
    scores = (score(document, choices, max_per_door)
              for choices in itertools.product(options, repeat=len(document["trailers"])))
    return min(candidate for candidate in scores if candidate is not None)


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    for arg in ("--pack", "--validate", "--solve"):
        cli.add_argument(arg, required=True, type=Path)
    args = cli.parse_args()
    root = Path(__file__).resolve().parents[1]
    candidate = root / "packages" / "loading-dock-single-window"
    # Only authored structural XML is linted; legal terms are copied byte-for-byte.
    call(sys.executable, "-B", root / "tools" / "format_xml.py", "--check",
         candidate / "source" / "recipe.xml", candidate / "source" / "models" / "door-allocation.xml")
    package = candidate / "loading-dock-single-window.orcpkg"
    baseline = DOCK_FIXTURE
    with tempfile.TemporaryDirectory() as temp:
        work = Path(temp)
        record_file = work / "records.json"

        def solve(path, records, expected, max_per_door=1):
            record_file.write_text(json.dumps(records))
            answer = json.loads(call(args.solve, path, record_file, "allocate").stdout)["solution"]
            actual = (-answer["assigned"], answer["minimized_edge_sum"])
            assert actual == oracle(records, max_per_door) == expected, (actual, expected)
            assert [r["item_id"] for r in answer["assignments"]] == [r["id"] for r in records["trailers"]]
            choices = [r["resource_id"] for r in answer["assignments"]]
            assert score(records, choices, max_per_door) == actual
            return choices

        rebuilt = work / "rebuilt.orcpkg"
        call(sys.executable, args.pack, candidate / "source", rebuilt, "--validator", args.validate)
        assert package.read_bytes() == rebuilt.read_bytes()
        assert solve(package, baseline, (-3, 0)) == ["door-4", "door-7", "door-9"]

        closed = copy.deepcopy(baseline)
        closed["doors"][0]["available"] = False
        for door in closed["doors"]:
            door["capacity"] = 2
        assert solve(package, closed, (-2, 0)).count(None) == 1

        amended = work / "amended"
        shutil.copytree(candidate / "source", amended)
        model = amended / "models/door-allocation.xml"
        original = model.read_text()
        mutated = original.replace('value="1"', 'value="2"', 1)
        assert mutated != original
        model.write_text(mutated)
        amended_package = work / "amended.orcpkg"
        call(sys.executable, args.pack, amended, amended_package, "--validator", args.validate)

        # Counterfactual XML only: a larger per-resource capacity after closure
        # permits all 3 in this abstract facility. Not a claim about real doors.
        changed = solve(amended_package, closed, (-3, 0), max_per_door=2)
        assert changed == ["door-7", "door-7", "door-9"]
        assert call(args.validate, package).stdout != call(args.validate, amended_package).stdout

        for mutation in (
            lambda data: data["doors"][0].update(capacity="1"),
            lambda data: data["compatibility"][0].update(door_id="unknown-door"),
            lambda data: data["compatibility"][0].update(avoidance_penalty=-1),
            lambda data: data["compatibility"][0].update(avoidance_penalty=True),
            lambda data: data["compatibility"][0].update(avoidance_penality=0),
        ):
            bad = copy.deepcopy(baseline)
            mutation(bad)
            record_file.write_text(json.dumps(bad))
            result = call(args.solve, package, record_file, "allocate", check=False)
            assert result.returncode != 0 and not result.stdout
    print("Deterministic package; 3 exact scenarios and 5 malformed-input rejections.")
    print("Historical point-in-time fixture reproduced; changing a capacity rule only in XML changed the solve.")


if __name__ == "__main__":
    main()
