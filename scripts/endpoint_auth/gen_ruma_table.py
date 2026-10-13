"""Extract per-endpoint authentication from a checkout of ruma.

Reads every ``metadata!`` block under ``<ruma>/crates`` and writes a JSON map
of ``"METHOD /path" -> [scheme, source file]``. Path placeholders are
normalised to ``{}``.
"""

import argparse
import collections
import glob
import json
import re
from pathlib import Path

PATH_RE = re.compile(
    r'"(/_matrix[^"]*|/\.well-known[^"]*|/_synapse[^"]*|/_conduwuit[^"]*)"'
)
BLOCK_RE = re.compile(r"metadata!\s*\{(.*?)\n\s*\};?\s*\n", re.S)


def extract(ruma):
    """Return ``({(method, path): (scheme, file)}, scheme_counts)``."""
    rows = {}
    counts = collections.Counter()
    pattern = str(Path(ruma) / "crates/*/src/**/*.rs")
    for name in glob.glob(pattern, recursive=True):
        text = Path(name).read_text(encoding="utf-8")
        for match in BLOCK_RE.finditer(text):
            block = match.group(1)
            auth = re.search(r"authentication:\s*(\w+)", block)
            method = re.search(r"method:\s*(\w+)", block)
            if not (auth and method):
                continue
            for path in PATH_RE.findall(block):
                key = (method.group(1), re.sub(r"\{[^}]*\}", "{}", path))
                rows[key] = (auth.group(1), name.split("crates/")[1])
                counts[auth.group(1)] += 1
    return rows, counts


def main():
    """Command-line entry point."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ruma", default="ruwuma_rev", help="ruma checkout")
    parser.add_argument("--out", default="ruma_auth.json")
    args = parser.parse_args()
    rows, counts = extract(args.ruma)
    print(len(rows), counts)
    data = {f"{k[0]} {k[1]}": v for k, v in rows.items()}
    with open(args.out, "w", encoding="utf-8") as out:
        json.dump(data, out, indent=0)


if __name__ == "__main__":
    main()
