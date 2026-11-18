import importlib.util
import json
from pathlib import Path
import shutil
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("validate_catalog", ROOT / "tools/validate_catalog.py")
catalog = importlib.util.module_from_spec(spec)
spec.loader.exec_module(catalog)


class Catalog(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name) / "catalog"
        shutil.copytree(ROOT, self.root, ignore=shutil.ignore_patterns(".git", "__pycache__"))

    def mutate(self, fn):
        path = self.root / "catalog.json"
        data = json.loads(path.read_text())
        fn(data)
        path.write_text(json.dumps(data))

    def test_inventory_has_seven_executions_and_two_packages(self):
        entries = catalog.inventory(self.root)
        self.assertEqual(len(entries), 7)
        self.assertEqual(len({e["path"] for e in entries if e["kind"] == "package"}), 2)

    def test_no_runtime_means_no_accepted_catalog(self):
        with self.assertRaisesRegex(ValueError, "runtime unavailable"):
            catalog.validate(self.root, self.root / "missing")

    def test_reintroduced_descriptor_is_not_discovered_as_runnable(self):
        shutil.copyfile(self.root / "tests/fixtures/catalog/descriptive.xml", self.root / "recipes/description.xml")
        with self.assertRaisesRegex(ValueError, "Unregistered"):
            catalog.inventory(self.root)

    def test_duplicate_or_unknown_execution_is_rejected(self):
        self.mutate(lambda d: d["entries"].append(d["entries"][0]))
        with self.assertRaises(ValueError):
            catalog.inventory(self.root)

    def test_generator_cannot_be_silently_promoted(self):
        self.mutate(lambda d: d["entries"][0].update(path=d["generators"][0]))
        with self.assertRaises(ValueError):
            catalog.inventory(self.root)

    def test_metadata_does_not_substitute_for_execution(self):
        self.mutate(lambda d: d["entries"][0].update(validated=True))
        with self.assertRaises(ValueError):
            catalog.inventory(self.root)

    def test_unsafe_paths_and_symlinks_are_rejected(self):
        for value in ("../catalog.json", "/etc/passwd", "recipes//x.xml", "recipes/./x.xml"):
            with self.assertRaises(ValueError):
                catalog.local_file(self.root, value)
        path = self.root / "alias.json"
        path.symlink_to(self.root / "catalog.json")
        with self.assertRaises(ValueError):
            catalog.local_file(self.root, "alias.json")

    def test_duplicate_json_keys_are_rejected(self):
        path = self.root / "duplicate.json"
        path.write_text('{"version":1,"version":1}')
        with self.assertRaises(ValueError):
            catalog.read_json(path)
