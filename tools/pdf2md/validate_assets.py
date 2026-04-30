#!/usr/bin/env python3
"""
validate_assets.py
Validate PDF2MD asset references against the document asset manifest and disk.

Usage:
    python3 validate_assets.py --markdown MWK_1956.md --manifest MWK_1956_assets_manifest.json --assets-root _Sources

Inputs:
    --markdown     assembled Markdown file
    --manifest     document-level asset manifest from aggregate_asset_manifest.py
    --assets-root  folder containing figures/, tables/, and images/

Outputs:
    PASS/FAIL summary on stdout. Exit 0 when all referenced and manifest-declared
    assets resolve; exit 1 for orphan/widow/missing asset findings; exit 2 for
    input/setup errors.
    Recognizes inline Markdown links/images, reference-style link definitions,
    and HTML <img src="..."> references.

Example:
    python3 tools/pdf2md/validate_assets.py \
      --markdown domain/_Sources/MWK_1956.md \
      --manifest domain/_Sources/MWK_1956_assets_manifest.json \
      --assets-root domain/_Sources
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path


INLINE_DEST_RE = re.compile(r"(?<!\\)\]\(([^)]+)\)")
REFERENCE_DEF_RE = re.compile(r"^\s*\[[^\]]+\]:\s*(\S+)", re.MULTILINE)
HTML_IMG_SRC_RE = re.compile(r"<img\b[^>]*\bsrc=[\"']([^\"']+)[\"']", re.IGNORECASE)
ASSET_PREFIXES = ("figures/", "tables/", "images/")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate PDF2MD asset references.")
    parser.add_argument("--markdown", required=True)
    parser.add_argument("--manifest", required=True)
    parser.add_argument("--assets-root", required=True)
    return parser.parse_args()


def extract_asset_links(markdown: str) -> set[str]:
    links = set()
    for pattern in (INLINE_DEST_RE, REFERENCE_DEF_RE, HTML_IMG_SRC_RE):
        for match in pattern.finditer(markdown):
            target = match.group(1).strip()
            target = target.strip("<>")
            target = target.split("#", 1)[0].split("?", 1)[0]
            if target.startswith("./"):
                target = target[2:]
            if target.startswith(ASSET_PREFIXES):
                links.add(target)
    return links


def manifest_paths(manifest: dict) -> set[str]:
    paths = set()
    for asset in manifest.get("assets", []):
        if not isinstance(asset, dict):
            continue
        for key in ("png_path", "csv_path"):
            value = str(asset.get(key) or "").strip()
            if value:
                paths.add(value)
    return paths


def disk_asset_paths(root: Path) -> set[str]:
    paths = set()
    for dirname in ASSET_PREFIXES:
        directory = root / dirname.rstrip("/")
        if not directory.is_dir():
            continue
        for path in directory.iterdir():
            if path.is_file():
                paths.add(f"{dirname}{path.name}")
    return paths


def main() -> int:
    args = parse_args()
    markdown_path = Path(args.markdown).resolve()
    manifest_path = Path(args.manifest).resolve()
    assets_root = Path(args.assets_root).resolve()

    for path in (markdown_path, manifest_path):
        if not path.is_file():
            print(f"ERROR: missing input file: {path}", file=sys.stderr)
            return 2
    if not assets_root.is_dir():
        print(f"ERROR: assets root is not a directory: {assets_root}", file=sys.stderr)
        return 2

    markdown_links = extract_asset_links(markdown_path.read_text(encoding="utf-8"))
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    declared_paths = manifest_paths(manifest)
    disk_paths = disk_asset_paths(assets_root)

    missing_on_disk = sorted(path for path in markdown_links | declared_paths if path not in disk_paths)
    orphan_links = sorted(path for path in markdown_links if path not in declared_paths)
    widow_manifest_paths = sorted(path for path in declared_paths if path not in markdown_links)
    unmanifested_disk_paths = sorted(path for path in disk_paths if path not in declared_paths)

    print(f"markdown_links={len(markdown_links)}")
    print(f"manifest_paths={len(declared_paths)}")
    print(f"disk_paths={len(disk_paths)}")

    findings = {
        "missing_on_disk": missing_on_disk,
        "orphan_links": orphan_links,
        "widow_manifest_paths": widow_manifest_paths,
        "unmanifested_disk_paths": unmanifested_disk_paths,
    }
    for name, values in findings.items():
        if values:
            print(f"{name}={len(values)}")
            for value in values[:50]:
                print(f"  - {value}")
            if len(values) > 50:
                print(f"  ... {len(values) - 50} more")

    if any(findings.values()):
        print("asset_validation=FAIL")
        return 1
    print("asset_validation=PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
