"""Generate the spec-derived endpoint authentication table.

Reads the matrix-spec OpenAPI files (``data/api``) and writes a Rust table of
``(method, path, AuthScheme)`` used by slipstream's spec cross-check test.
Endpoints with no ``security`` block (``NONE_DECLARED``) or a scheme this script cannot map
(``UNKNOWN``) are never defaulted: each must be resolved in ``spec_overrides.json``
(with a reason) or excluded there, otherwise the script exits non-zero.
"""

import argparse
import glob
import json
import os
import re
import sys
from collections import Counter
from pathlib import Path

import yaml

API_DIRS = ("client-server", "server-server", "application-service")
DEFAULT_ROOT = Path(__file__).resolve().parents[3] / "spec/data/api"
BEARER = {"accessTokenBearer", "accessTokenQuery"}
APPSERVICE = {"appserviceAccessTokenBearer", "appserviceAccessTokenQuery"}


def scheme(sec):
    """Map an OpenAPI ``security`` value to an ``AuthScheme`` name."""
    if sec is None:
        return None
    if sec == []:
        return "None"
    names = set()
    optional = False
    for entry in sec:
        if not entry:
            optional = True
        names |= set(entry.keys())
    if "signedRequest" in names:
        return "ServerSignatures"
    if names & APPSERVICE and not names & BEARER:
        return "AppserviceToken"
    if names & BEARER:
        return "AccessTokenOptional" if optional else "AccessToken"
    return "UNKNOWN:" + ",".join(sorted(names))


def base_path(spec):
    """Return the ``basePath`` declared by an OpenAPI file, or ``""``."""
    base = ""
    for server in spec.get("servers", []):
        found = server.get("variables", {}).get("basePath", {}).get("default")
        if found:
            base = found
    return base


def collect(root):
    """Return ``{(METHOD, path): (scheme, file)}`` for every operation."""
    rows = {}
    for directory in API_DIRS:
        for name in sorted(glob.glob(f"{root}/{directory}/*.yaml")):
            with open(name, encoding="utf-8") as handle:
                spec = yaml.safe_load(handle)
            if not isinstance(spec, dict) or "paths" not in spec:
                continue
            base = base_path(spec)
            top = spec.get("security")
            for path, ops in spec["paths"].items():
                for method, op in ops.items():
                    if method not in ("get", "post", "put", "delete"):
                        continue
                    found = scheme(op.get("security", top))
                    full = re.sub(r"\{[^}]*\}", "{}", base + path)
                    rows[(method.upper(), full)] = (
                        found or "NONE_DECLARED",
                        os.path.basename(name),
                    )
    return rows


def load_overrides(path):
    """Load the reviewed overrides: resolved, excluded and excluded prefixes."""
    data = json.loads(Path(path).read_text(encoding="utf-8"))
    resolved = {(o["method"], o["path"]): o["scheme"] for o in data["resolved"]}
    excluded = {(o["method"], o["path"]) for o in data["excluded"]}
    prefixes = tuple(o["prefix"] for o in data["excluded_prefixes"])
    return resolved, excluded, prefixes


def resolve(rows, overrides):
    """Apply the overrides; return ``(final, unresolved)``."""
    resolved, excluded, prefixes = overrides
    final = {}
    unresolved = []
    for key, (found, name) in sorted(rows.items(), key=lambda kv: (kv[0][1], kv[0][0])):
        if key in excluded or key[1].startswith(prefixes):
            continue
        if found.startswith(("UNKNOWN", "NONE_DECLARED")):
            if key not in resolved:
                unresolved.append((key, found, name))
                continue
            found = resolved[key]
        final[key] = found
    return final, unresolved


def render(final):
    """Render the Rust source for the table."""
    out = [
        "// @generated from matrix-org/matrix-spec `data/api` (client-server and server-server).",
        "// Used only by the cross-check test; placeholders are `{}`.",
        "use super::AuthScheme;",
        "",
        "pub(super) const SPEC_AUTH: &[(&str, &str, AuthScheme)] = &[",
    ]
    for (method, path), found in final.items():
        out.append(f'\t("{method}", "{path}", AuthScheme::{found}),')
    out.append("];")
    return "\n".join(out) + "\n"


def main():
    """Command-line entry point."""
    here = Path(__file__).resolve().parent
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", default=str(DEFAULT_ROOT), help="data/api")
    parser.add_argument("--overrides", default=str(here / "spec_overrides.json"))
    parser.add_argument("--out", default=str(here.parents[1] / "src/endpoint_auth_spec.rs"))
    args = parser.parse_args()
    rows = collect(args.root)
    if not rows:
        parser.error(f"No API operations found under {args.root!r}")
    final, unresolved = resolve(rows, load_overrides(args.overrides))
    print(len(final), "endpoints", Counter(final.values()), file=sys.stderr)
    if unresolved:
        for (method, path), found, name in unresolved:
            print(f"UNRESOLVED {method} {path} ({found}, {name})", file=sys.stderr)
        sys.exit(1)
    Path(args.out).write_text(render(final), encoding="utf-8")


if __name__ == "__main__":
    main()
