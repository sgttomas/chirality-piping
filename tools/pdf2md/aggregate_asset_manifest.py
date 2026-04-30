#!/usr/bin/env python3
"""
aggregate_asset_manifest.py
Merge per-page PDF2MD asset materialization manifests into one document manifest.

Usage:
    python3 aggregate_asset_manifest.py WORK_DIR OUTPUT_JSON [--doc-stem MWK_1956]

Inputs:
    WORK_DIR      PDF2MD work directory containing page_NNNN_assets_materialized.json files
    OUTPUT_JSON   document-level asset manifest path
    --doc-stem    optional expected document stem

Outputs:
    OUTPUT_JSON containing all page asset records in page/order sequence.

Example:
    python3 tools/pdf2md/aggregate_asset_manifest.py \
      ./MWK_1956_pdf2md_work ./_Sources/MWK_1956_assets_manifest.json --doc-stem MWK_1956
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path


PAGE_MANIFEST_RE = re.compile(r"^page_(\d{4})_assets_materialized\.json$")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Aggregate PDF2MD per-page asset manifests.")
    parser.add_argument("work_dir")
    parser.add_argument("output_json")
    parser.add_argument("--doc-stem")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    work_dir = Path(args.work_dir).resolve()
    output_json = Path(args.output_json).resolve()
    if not work_dir.is_dir():
        raise SystemExit(f"ERROR: work_dir is not a directory: {work_dir}")

    manifests: list[tuple[int, Path]] = []
    for path in work_dir.glob("page_*_assets_materialized.json"):
        match = PAGE_MANIFEST_RE.match(path.name)
        if match:
            manifests.append((int(match.group(1)), path))
    manifests.sort()

    pages = []
    assets = []
    issues = []
    doc_stems = set()
    for page_num, path in manifests:
        data = json.loads(path.read_text(encoding="utf-8"))
        doc_stem = data.get("doc_stem")
        if doc_stem:
            doc_stems.add(str(doc_stem))
        if args.doc_stem and doc_stem and doc_stem != args.doc_stem:
            issues.append(f"{path.name}: doc_stem {doc_stem!r} != expected {args.doc_stem!r}")
        page_assets = data.get("assets", [])
        if not isinstance(page_assets, list):
            issues.append(f"{path.name}: assets is not a list")
            page_assets = []
        pages.append(
            {
                "page": page_num,
                "manifest_path": str(path),
                "asset_count": len(page_assets),
                "anchored_markdown": data.get("anchored_markdown", ""),
            }
        )
        for asset in page_assets:
            if isinstance(asset, dict):
                record = dict(asset)
                record["page"] = page_num
                assets.append(record)

    output = {
        "schema_version": "pdf2md-assets-document/v1",
        "doc_stem": args.doc_stem or (sorted(doc_stems)[0] if len(doc_stems) == 1 else ""),
        "work_dir": str(work_dir),
        "page_manifest_count": len(manifests),
        "asset_count": len(assets),
        "pages": pages,
        "assets": assets,
        "issues": issues,
    }
    output_json.parent.mkdir(parents=True, exist_ok=True)
    output_json.write_text(json.dumps(output, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(f"page_manifests={len(manifests)} assets={len(assets)} output={output_json}")
    if issues:
        print(f"issues={len(issues)}")
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
