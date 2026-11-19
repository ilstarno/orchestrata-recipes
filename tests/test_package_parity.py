"""Keep canonical revision sources and immutable published archives aligned."""
import hashlib
from pathlib import Path
import sys
import unittest
import xml.etree.ElementTree as ET
import zipfile

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))
from format_xml import format_xml


class PackageParity(unittest.TestCase):
    def test_revision_members_match_canonical_sources_and_inventory(self):
        revisions = sorted((ROOT / "packages").glob("*/versions/*"))
        self.assertEqual(len(revisions), 2)
        for revision in revisions:
            source = revision / "source"
            manifest = ET.fromstring((source / "recipe.xml").read_bytes())
            self.assertEqual(manifest.get("version"), revision.name)
            expected = {"recipe.xml"} | {a.get("path") for a in manifest.find("assets")}
            self.assertEqual(expected, {p.relative_to(source).as_posix() for p in source.rglob("*") if p.is_file()})
            with zipfile.ZipFile(next(revision.glob("*.orcpkg"))) as archive:
                self.assertEqual(len(archive.namelist()), len(expected))
                self.assertEqual(set(archive.namelist()), expected)
                for path in sorted(expected):
                    content = (source / path).read_bytes()
                    self.assertEqual(archive.read(path), content, path)
                    if path == "recipe.xml" or path.startswith("models/"):
                        self.assertEqual(content, format_xml(content), path)
                for asset in manifest.find("assets"):
                    content = archive.read(asset.get("path"))
                    self.assertEqual(int(asset.get("bytes")), len(content))
                    self.assertEqual(asset.get("sha256"), hashlib.sha256(content).hexdigest())

    def test_models_preserve_prior_mathematical_structure(self):
        for revision in (ROOT / "packages").glob("*/versions/*"):
            original = revision.parent.parent / "source"
            for model in (revision / "source/models").glob("*.xml"):
                before = ET.canonicalize((original / "models" / model.name).read_text(), strip_text=True)
                after = ET.canonicalize(model.read_text(), strip_text=True)
                self.assertEqual(before, after)
            self.assertEqual((original / "legal/terms.xml").read_bytes(),
                             (revision / "source/legal/terms.xml").read_bytes())

    def test_original_archives_remain_immutable(self):
        self.assertEqual(hashlib.sha256((ROOT / 'packages/loading-dock-single-window/loading-dock-single-window.orcpkg').read_bytes()).hexdigest(),
                         '2be89a11c2f61b52e82352339ccd673c2799b437a3e28ebcbc94e10ad8839254')
        self.assertEqual(hashlib.sha256((ROOT / 'packages/maintenance-shift/maintenance-shift.orcpkg').read_bytes()).hexdigest(),
                         '8cde0508bdf5bbe14fa5ebc78a4ffe4a216d6de40f4fe0cf2d306a00fff53165')
