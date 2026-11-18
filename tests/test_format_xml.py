import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from xml.parsers import expat

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "tools" / "format_xml.py"
sys.path.insert(0, str(SCRIPT.parent))
from format_xml import format_xml


class FormatXml(unittest.TestCase):
    def test_structural_indentation_and_idempotence(self):
        raw = b"<?xml version='1.0'?><model><input flag='>'/><steps><run/><run></run></steps></model>"
        result = (b"<?xml version='1.0'?>\n<model>\n  <input flag='>'/>\n"
                  b"  <steps>\n    <run/>\n    <run></run>\n  </steps>\n</model>\n")
        self.assertEqual(format_xml(raw), result)
        self.assertEqual(format_xml(result), result)

    def test_mixed_space_and_markup_content_stays_literal(self):
        preserved = [
            b"<p> before <em>text</em> after </p>",
            b'<keep xml:space="preserve"><inner xml:space="default"><a/><b/></inner></keep>',
            b"<preserved><![CDATA[<a> & x]]><node/></preserved>",
            b"<space>&#x20;<nested/></space>",
            b"<non_asci_space>\xc2\xa0<nested/></non_asci_space>",
            b'<body>\n  data &amp; values\n\n </body>',
        ]
        from_sequence = b"<root>" + b"".join(preserved) + b"</root>"
        pretty = b"<root>\n" + b"".join(b"  " + part + b"\n" for part in preserved) + b"</root>\n"
        self.assertEqual(format_xml(from_sequence), pretty)

    def test_comment_and_processing_instruction_payload_stays_literal(self):
        raw = b"<?xml version='1.0'?>\r\n<!-- keep <inner>\r\nbody  --><a><?pi abstract >?><b/></a>\r\n"
        expected = (b"<?xml version='1.0'?>\r\n<!-- keep <inner>\r\nbody  -->\r\n"
                    b"<a>\r\n  <?pi abstract >?>\r\n  <b/>\r\n</a>\r\n")
        self.assertEqual(format_xml(raw), expected)

    def test_reject_invalid_unbounded_and_dtd(self):
        with self.assertRaises(expat.ExpatError):
            format_xml(b"<foo><bar></foo>")
        with self.assertRaisesRegex(ValueError, "DOCTYPE"):
            format_xml(b'<!DOCTYPE root [<!ENTITY value "never resolve">]><root>&value;</root>')
        with self.assertRaisesRegex(ValueError, "source size"):
            format_xml(b"<a>" + b"a" * 1_048_576 + b"</a>")
        with self.assertRaisesRegex(ValueError, "depth"):
            format_xml(b"<a>" * 65 + b"</a>" * 65)

    def test_check_never_writes_and_explicit_write_preserves_mode(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "source.xml"
            old = b"<a><b/></a>"
            path.write_bytes(old)
            path.chmod(0o640)
            args = [sys.executable, "-B", SCRIPT]
            self.assertEqual(subprocess.run([*args, "--check", path], capture_output=True).returncode, 1)
            self.assertEqual(path.read_bytes(), old)
            self.assertEqual(subprocess.run([*args, "--write", path], capture_output=True).returncode, 0)
            self.assertEqual(subprocess.run([*args, "--check", path], capture_output=True).returncode, 0)
            self.assertEqual(path.stat().st_mode & 0o777, 0o640)
            self.assertEqual(path.read_bytes(), format_xml(old))

    def test_refuses_symlink(self):
        with tempfile.TemporaryDirectory() as temp:
            real = Path(temp) / "source.xml"
            real.write_text("<a><b/></a>")
            link = Path(temp) / "link.xml"
            link.symlink_to(real)
            run = subprocess.run([sys.executable, "-B", SCRIPT, "--write", link], capture_output=True)
            self.assertEqual(run.returncode, 1)
            self.assertEqual(real.read_text(), "<a><b/></a>")


if __name__ == "__main__":
    unittest.main()
