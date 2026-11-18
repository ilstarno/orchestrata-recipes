"""Indent human-authored XML without reserializing tags or text."""
from __future__ import annotations

import argparse
import os
import re
import stat
import tempfile
from dataclasses import dataclass, field
from pathlib import Path
from xml.parsers import expat

MAX_BYTES = 1_048_576
MAX_DEPTH = 64
INDENT = "  "
TOKEN = re.compile(
    r"<!--.*?-->|<!\[CDATA\[.*?\]\]>|<\?.*?\?>|</?[^<>\x22\x27]+?"
    r"(?:\x22[^\x22]*\x22|\x27[^\x27]*\x27|[^<>\x22\x27])*?>|[^<]+",
    re.DOTALL,
)
XML_SPACE = re.compile(r"""\sxml:space\s*=\s*(['"])(.*?)\1""", re.DOTALL)


@dataclass
class Element:
    opening: str
    closing: str = ""
    literal: bool = False
    content: list[Element | str] = field(default_factory=list)

    def original(self) -> str:
        return self.opening + "".join(
            part.original() if isinstance(part, Element) else part for part in self.content
        ) + self.closing


def reject_doctype(*_args: object) -> None:
    raise ValueError("DOCTYPE is not allowed")


def validated_xml(source: bytes) -> str:
    if len(source) > MAX_BYTES:
        raise ValueError(f"maximum source size is {MAX_BYTES} bytes")
    if source.startswith(b"\xef\xbb\xbf"):
        raise ValueError("UTF-8 byte-order marks are not accepted")
    value = source.decode("utf-8")
    parser = expat.ParserCreate(encoding="UTF-8")
    parser.StartDoctypeDeclHandler = reject_doctype
    parser.ExternalEntityRefHandler = lambda *_args: 0
    parser.Parse(source, True)
    return value


def lexical_tree(source: str) -> Element:
    document = Element("")
    stack = [document]
    cursor = 0
    while cursor < len(source):
        match = TOKEN.match(source, cursor)
        if match is None:
            raise ValueError("unsupported XML token")
        token = match.group()
        cursor = match.end()
        parent = stack[-1]
        if token.startswith("</"):
            stack.pop().closing = token
        elif token.startswith("<") and not token.startswith(("<?", "<!")):
            # Keep inherited xml:space subtrees byte-identical; this
            # conservative policy also preserves an explicit nested reset.
            node = Element(token, literal=parent.literal or bool(XML_SPACE.search(token)))
            parent.content.append(node)
            if not token.endswith("/>"):
                if len(stack) > MAX_DEPTH:
                    raise ValueError(f"maximum element depth is {MAX_DEPTH}")
                stack.append(node)
        else:
            parent.content.append(token)
    return document


def significant(part: Element | str) -> bool:
    if isinstance(part, Element):
        return False
    if part.startswith(("<!--", "<?")):
        return False
    # Only XML whitespace is ignorable. Non-ASCII whitespace and character
    # references may be data, and any CDATA is preserved as-is.
    return bool(part.strip(" \r\n\t"))


def layout(element: Element, depth: int, line_end: str) -> str:
    if element.literal or any(significant(part) for part in element.content):
        return element.original()
    children = [
        part for part in element.content
        if isinstance(part, Element) or part.startswith(("<!--", "<?"))
    ]
    if not children:
        return element.original()
    level = depth + bool(element.opening)
    inner = line_end.join(
        INDENT * level + (layout(part, level, line_end) if isinstance(part, Element) else part)
        for part in children
    )
    if not element.opening:
        return inner
    return element.opening + line_end + inner + line_end + INDENT * depth + element.closing


def format_xml(source: bytes) -> bytes:
    source_text = validated_xml(source)
    newline = "\r\n" if b"\r\n" in source and b"\n" not in source.replace(b"\r\n", b"") else "\n"
    rendered = (layout(lexical_tree(source_text), 0, newline) + newline).encode("utf-8")
    validated_xml(rendered)
    return rendered


def replace_file(path: Path, content: bytes, mode: int) -> None:
    with tempfile.NamedTemporaryFile(dir=path.parent, prefix=".xml-format-", delete=False) as stream:
        target = Path(stream.name)
        try:
            stream.write(content)
            stream.flush()
            os.fchmod(stream.fileno(), mode)
            stream.close()
            os.replace(target, path)
        finally:
            target.unlink(missing_ok=True)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("files", metavar="SOURCE.xml", nargs="+", type=Path)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--write", action="store_true")
    args = parser.parse_args()
    failed = False
    for path in args.files:
        try:
            info = path.lstat()
            if not stat.S_ISREG(info.st_mode) or path.suffix != ".xml":
                raise ValueError("expected a regular .xml source file")
            with path.open("rb") as stream:
                original = stream.read(MAX_BYTES + 1)
            rendered = format_xml(original)
            if rendered != original:
                if args.write:
                    replace_file(path, rendered, stat.S_IMODE(info.st_mode))
                else:
                    failed = True
                    print(f"{path}: needs XML indentation")
        except (ValueError, OSError, expat.ExpatError) as error:
            failed = True
            print(f"{path}: {error}")
    return int(failed)


if __name__ == "__main__":
    raise SystemExit(main())
