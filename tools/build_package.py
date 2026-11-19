"""Build a canonical recipe revision using the Engine packer and native validator."""
import argparse
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import zipfile

from format_xml import format_xml


def build(source, output, packer, validator):
    source, output = Path(source).resolve(), Path(output).absolute()
    with tempfile.TemporaryDirectory(dir=output.parent, prefix=".recipe-build-") as directory:
        work = Path(directory)
        normalized = work / "normalized.orcpkg"
        subprocess.run([sys.executable, str(packer), str(source), str(normalized),
                        "--validator", str(validator)], check=True, timeout=30, capture_output=True)
        candidate = work / "candidate.orcpkg"
        with zipfile.ZipFile(normalized) as archive, zipfile.ZipFile(candidate, "w") as result:
            for info in archive.infolist():
                content = archive.read(info.filename)
                if info.filename == "recipe.xml":
                    content = format_xml(content)
                if content != (source / info.filename).read_bytes():
                    raise ValueError(f"Source differs from canonical package member: {info.filename}")
                if info.filename == "recipe.xml" or info.filename.startswith("models/"):
                    if format_xml(content) != content:
                        raise ValueError(f"Source needs canonical formatting: {info.filename}")
                result.writestr(info, content)
        subprocess.run([str(validator), str(candidate)], check=True, timeout=30, capture_output=True)
        os.link(candidate, output)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--pack", required=True, type=Path)
    parser.add_argument("--validator", required=True, type=Path)
    args = parser.parse_args()
    build(args.source, args.output, args.pack.resolve(), args.validator.resolve())


if __name__ == "__main__":
    main()
