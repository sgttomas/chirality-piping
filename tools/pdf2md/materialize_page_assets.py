#!/usr/bin/env python3
"""
materialize_page_assets.py
Crop page-level prose-document assets and write anchored Markdown.

Usage:
    python3 materialize_page_assets.py --page-image page_0003.png --page-md page_0003.md \
      --asset-json page_0003_assets.json --assets-root _Sources --doc-stem MWK_1956 \
      --page 3 --output-md page_0003.anchored.md --manifest-output page_0003_assets_materialized.json

Inputs:
    --page-image       Rasterized page PNG
    --page-md          Clean per-page Markdown
    --asset-json       VLM-emitted page asset JSON from pdf2md-page-assets
    --assets-root      Public folder that will contain figures/, tables/, images/
    --doc-stem         Document stem used in stable asset IDs
    --page             1-indexed page number
    --output-md        Anchored per-page Markdown output
    --manifest-output  Per-page materialization manifest JSON
    --padding-ratio    Optional crop padding around bbox_norm (default: 0.05)

Outputs:
    Asset crops in assets-root/figures, assets-root/tables, assets-root/images;
    table CSVs under assets-root/tables; anchored per-page Markdown; and a
    per-page materialization manifest.

Example:
    python3 tools/pdf2md/materialize_page_assets.py \
      --page-image work/page_0003.png --page-md work/page_0003.md \
      --asset-json work/page_0003_assets.json --assets-root domain/_Sources \
      --doc-stem MWK_1956 --page 3 --output-md work/page_0003.anchored.md \
      --manifest-output work/page_0003_assets_materialized.json
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import re
import unicodedata
from io import StringIO
from pathlib import Path

try:
    from PIL import Image
except ImportError as exc:  # pragma: no cover - environment guard
    raise SystemExit("ERROR: Pillow is required for asset cropping") from exc


KIND_DIR = {
    "fig": "figures",
    "tbl": "tables",
    "img": "images",
}

KIND_ALIASES = {
    "fig": "fig", "figure": "fig", "figures": "fig", "diagram": "fig",
    "plot": "fig", "chart": "fig", "graph": "fig", "schematic": "fig",
    "tbl": "tbl", "table": "tbl", "tables": "tbl",
    "img": "img", "image": "img", "images": "img", "logo": "img",
    "photo": "img", "photograph": "img", "picture": "img",
}

CAPTION_KEYS = ("caption", "title", "label", "name")

ASSET_BLOCK_RE = re.compile(
    r"\n*<!-- PDF2MD-ASSETS:BEGIN page=\d+ -->.*?<!-- PDF2MD-ASSETS:END page=\d+ -->\n*",
    re.DOTALL,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Materialize PDF2MD page assets and anchored Markdown.")
    parser.add_argument("--page-image", required=True)
    parser.add_argument("--page-md", required=True)
    parser.add_argument("--asset-json", required=True)
    parser.add_argument("--assets-root", required=True)
    parser.add_argument("--doc-stem", required=True)
    parser.add_argument("--page", required=True, type=int)
    parser.add_argument("--output-md", required=True)
    parser.add_argument("--manifest-output", required=True)
    parser.add_argument("--padding-ratio", type=float, default=0.05)
    return parser.parse_args()


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def slugify(value: str, default: str = "untitled", max_len: int = 40) -> str:
    normalized = unicodedata.normalize("NFKD", value or "").encode("ascii", "ignore").decode("ascii")
    slug = re.sub(r"[^a-zA-Z0-9]+", "-", normalized).strip("-").lower()
    slug = re.sub(r"-{2,}", "-", slug)
    if not slug:
        slug = default
    return slug[:max_len].strip("-") or default


def bbox_from_asset(asset: dict) -> list[float] | None:
    raw = asset.get("bbox_norm")
    if isinstance(raw, dict):
        raw = [raw.get("x0"), raw.get("y0"), raw.get("x1"), raw.get("y1")]
    if not isinstance(raw, list) or len(raw) != 4:
        return None
    try:
        bbox = [float(item) for item in raw]
    except (TypeError, ValueError):
        return None
    return bbox


def padded_bbox_px(bbox_norm: list[float], width: int, height: int, padding_ratio: float) -> tuple[int, int, int, int] | None:
    x0, y0, x1, y1 = bbox_norm
    pad = max(0.0, padding_ratio)
    x0 = max(0.0, min(1.0, x0 - pad))
    y0 = max(0.0, min(1.0, y0 - pad))
    x1 = max(0.0, min(1.0, x1 + pad))
    y1 = max(0.0, min(1.0, y1 + pad))
    left = round(x0 * width)
    top = round(y0 * height)
    right = round(x1 * width)
    bottom = round(y1 * height)
    if left >= right or top >= bottom:
        return None
    return (left, top, right, bottom)


def normalize_csv_text(text: str) -> tuple[str, list[str]]:
    issues: list[str] = []
    stripped = (text or "").strip()
    if not stripped:
        return "", ["missing_csv_text"]

    try:
        rows = list(csv.reader(StringIO(stripped)))
    except csv.Error as exc:
        return stripped + "\n", [f"csv_parse_warning:{exc}"]

    output = StringIO()
    writer = csv.writer(output, lineterminator="\n")
    writer.writerows(rows)
    if not rows:
        issues.append("empty_csv")
    return output.getvalue(), issues


def load_assets(path: Path) -> list[dict]:
    data = json.loads(path.read_text(encoding="utf-8"))
    if isinstance(data, dict):
        assets = list(data.get("assets", []) or [])
        for sibling_key, default_kind in (("tables", "tbl"), ("figures", "fig"), ("images", "img")):
            sibling = data.get(sibling_key)
            if isinstance(sibling, list):
                for entry in sibling:
                    if isinstance(entry, dict) and not (entry.get("kind") or entry.get("type")):
                        merged = dict(entry)
                        merged["kind"] = default_kind
                        assets.append(merged)
                    elif isinstance(entry, dict):
                        assets.append(entry)
    elif isinstance(data, list):
        assets = data
    else:
        raise SystemExit(f"ERROR: unsupported asset JSON shape in {path}")
    if not isinstance(assets, list):
        raise SystemExit(f"ERROR: assets must be a list in {path}")
    return [asset for asset in assets if isinstance(asset, dict)]


def caption_from_asset(asset: dict) -> str:
    for key in CAPTION_KEYS:
        value = asset.get(key)
        if value:
            return str(value).strip()
    return ""


def normalize_kind(asset: dict) -> str:
    raw = asset.get("kind") or asset.get("type") or asset.get("subtype") or ""
    return KIND_ALIASES.get(str(raw).strip().lower(), "")


def strip_existing_asset_block(text: str) -> str:
    return ASSET_BLOCK_RE.sub("\n\n", text).rstrip()


def collapse_label_text(value: str) -> str:
    return re.sub(r"\s+", " ", str(value or "")).strip()


def escape_markdown_label(value: str) -> str:
    """Escape Markdown label delimiters while preserving readable caption text."""
    text = collapse_label_text(value)
    return (
        text.replace("\\", "\\\\")
        .replace("[", "\\[")
        .replace("]", "\\]")
        .replace("(", "\\(")
        .replace(")", "\\)")
    )


def escape_bold_text(value: str) -> str:
    """Escape caption text used inside bold spans in generated asset blocks."""
    text = escape_markdown_label(value)
    return text.replace("*", "\\*").replace("_", "\\_")


def build_reference(asset: dict) -> str:
    caption = collapse_label_text(asset.get("caption") or asset.get("title") or asset["asset_id"])
    label = escape_markdown_label(caption)
    bold_caption = escape_bold_text(caption)
    if asset["kind"] in {"fig", "img"}:
        if asset.get("png_path"):
            return f"![{label}]({asset['png_path']})"
        issues = ", ".join(asset.get("issues", [])) or "asset pending"
        return f"> **{asset['kind'].upper()} - {bold_caption}** - {issues}"

    links = []
    if asset.get("csv_path"):
        links.append(f"[CSV]({asset['csv_path']})")
    if asset.get("png_path"):
        links.append(f"[source crop]({asset['png_path']})")
    suffix = " and ".join(links) if links else "asset pending"
    return f"> **Table - {bold_caption}** - see {suffix}"


def main() -> int:
    args = parse_args()
    if args.page < 1:
        raise SystemExit("ERROR: --page must be >= 1")

    page_image = Path(args.page_image).resolve()
    page_md = Path(args.page_md).resolve()
    asset_json = Path(args.asset_json).resolve()
    assets_root = Path(args.assets_root).resolve()
    output_md = Path(args.output_md).resolve()
    manifest_output = Path(args.manifest_output).resolve()

    for path in (page_image, page_md, asset_json):
        if not path.is_file():
            raise SystemExit(f"ERROR: required input is missing: {path}")

    assets_root.mkdir(parents=True, exist_ok=True)
    for dirname in KIND_DIR.values():
        (assets_root / dirname).mkdir(parents=True, exist_ok=True)
    output_md.parent.mkdir(parents=True, exist_ok=True)
    manifest_output.parent.mkdir(parents=True, exist_ok=True)

    assets = load_assets(asset_json)
    counters = {kind: 0 for kind in KIND_DIR}
    materialized: list[dict] = []

    with Image.open(page_image) as image:
        width, height = image.size
        for index, asset in enumerate(assets, start=1):
            kind = normalize_kind(asset)
            if kind not in KIND_DIR:
                raw_kind = str(asset.get("kind") or asset.get("type") or "").strip()
                materialized.append({"source_index": index, "status": "skipped", "issues": [f"unknown_kind:{raw_kind!r}"]})
                continue

            counters[kind] += 1
            ordinal = asset.get("ordinal")
            try:
                ordinal_int = int(ordinal)
            except (TypeError, ValueError):
                ordinal_int = counters[kind]
            if ordinal_int < 1:
                ordinal_int = counters[kind]

            caption = caption_from_asset(asset)
            slug = slugify(str(asset.get("slug") or caption or "untitled"))
            asset_id = f"{args.doc_stem}_p{args.page:04d}_{kind}{ordinal_int:02d}"
            basename = f"{asset_id}_{slug}"
            target_dir = assets_root / KIND_DIR[kind]
            png_path = target_dir / f"{basename}.png"
            csv_path = target_dir / f"{basename}.csv" if kind == "tbl" else None

            issues: list[str] = []
            bbox_norm = bbox_from_asset(asset)
            bbox_px = None
            if bbox_norm is None:
                issues.append("missing_or_invalid_bbox_norm")
            else:
                bbox_px = padded_bbox_px(bbox_norm, width, height, args.padding_ratio)
                if bbox_px is None:
                    issues.append("invalid_bbox_geometry")
                else:
                    image.crop(bbox_px).save(png_path)

            if kind == "tbl":
                csv_text, csv_issues = normalize_csv_text(str(asset.get("csv_text") or asset.get("table_csv") or ""))
                issues.extend(csv_issues)
                if csv_text and csv_path is not None:
                    csv_path.write_text(csv_text, encoding="utf-8")

            record = {
                "asset_id": asset_id,
                "kind": kind,
                "ordinal": ordinal_int,
                "caption": caption,
                "slug": slug,
                "bbox_norm": bbox_norm,
                "bbox_px": list(bbox_px) if bbox_px else None,
                "png_path": f"{KIND_DIR[kind]}/{png_path.name}" if png_path.exists() else "",
                "csv_path": f"{KIND_DIR[kind]}/{csv_path.name}" if csv_path and csv_path.exists() else "",
                "status": "materialized" if not issues else "degraded",
                "issues": issues,
            }
            if png_path.exists():
                record["png_sha256"] = sha256(png_path)
            if csv_path and csv_path.exists():
                record["csv_sha256"] = sha256(csv_path)
            materialized.append(record)

    markdown = strip_existing_asset_block(page_md.read_text(encoding="utf-8"))
    references = [build_reference(asset) for asset in materialized if asset.get("status") in {"materialized", "degraded"}]
    if references:
        block = [
            f"<!-- PDF2MD-ASSETS:BEGIN page={args.page} -->",
            "",
            "#### Extracted Page Assets",
            "",
            *references,
            "",
            f"<!-- PDF2MD-ASSETS:END page={args.page} -->",
        ]
        markdown = markdown.rstrip() + "\n\n" + "\n\n".join(block) + "\n"
    else:
        markdown = markdown.rstrip() + "\n"
    output_md.write_text(markdown, encoding="utf-8")

    manifest = {
        "schema_version": "pdf2md-assets-materialized/v1",
        "doc_stem": args.doc_stem,
        "page": args.page,
        "page_image": str(page_image),
        "page_image_sha256": sha256(page_image),
        "source_asset_json": str(asset_json),
        "assets_root": str(assets_root),
        "anchored_markdown": str(output_md),
        "assets": materialized,
    }
    manifest_output.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(f"assets={len(materialized)} materialized_manifest={manifest_output}")
    print(f"anchored_markdown={output_md}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
