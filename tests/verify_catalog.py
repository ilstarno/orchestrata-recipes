"""Exercise catalog admission with real native runtimes and conservative failures."""
import argparse
import importlib.util
from pathlib import Path
import shutil
import tempfile

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("validate_catalog", ROOT / "tools/validate_catalog.py")
catalog = importlib.util.module_from_spec(spec)
spec.loader.exec_module(catalog)


def rejected(root, binaries):
    try:
        catalog.validate(root, binaries)
    except ValueError:
        return
    raise AssertionError("Invalid catalog admitted")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--engine-bin-dir", type=Path, required=True)
    args = parser.parse_args()
    bins = args.engine_bin_dir.resolve()
    assert len(catalog.validate(ROOT, bins)) == 7
    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory) / "catalog"
        shutil.copytree(ROOT, root, ignore=shutil.ignore_patterns(".git", "__pycache__"))
        rejected(root, root / "missing-runtime")
        model = root / "recipes/mathematical/maintenance-cost.xml"
        original = model.read_bytes()
        model.write_bytes((root / "tests/fixtures/catalog/descriptive.xml").read_bytes())
        rejected(root, bins)
        model.write_bytes(original)
        package = root / "packages/loading-dock-single-window/loading-dock-single-window.orcpkg"
        original = package.read_bytes()
        package.write_bytes(b"not a validated package")
        rejected(root, bins)
        package.write_bytes(original)
        expected = root / "tests/fixtures/catalog/loading-dock-single-window-expected.json"
        expected.write_text("{}")
        rejected(root, bins)
    print("PASS: seven native executions, descriptive XML rejection, missing runtime, corrupt package and incorrect result rejection.")


if __name__ == "__main__":
    main()
