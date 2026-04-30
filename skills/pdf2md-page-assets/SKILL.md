---
name: pdf2md-page-assets
description: Identify visible figures, tables, and meaningful images on one rasterized prose PDF page and emit bbox-normalized asset records for deterministic materialization.
compatibility: Chirality TASK; invoked by PDF2MD asset-enabled mode after per-page Markdown cleanup.
metadata:
  chirality-skill-version: "1"
  chirality-task-profile: NONE
---

# SKILL - pdf2md-page-assets

## Purpose

Identify extractable assets on one rasterized prose-document PDF page:

- `fig` - captioned figures, diagrams, plots, charts, plates, or illustrations
- `tbl` - tabular data that should exist as an auditable CSV artifact
- `img` - meaningful non-figure images such as logos, covers, photographs, or uncaptioned visual objects

The skill reads one page image plus that page's cleaned Markdown context and writes one JSON file. It does not crop images, write CSV files, rewrite Markdown links, assemble manifests, or validate filesystem references. Those are deterministic PDF2MD tool responsibilities.

## Suitable agent shells

- `TASK` in generic shell mode, spawned by the `PDF2MD` orchestrator

Not the right fit for:

- whole-PDF conversion
- cross-page asset stitching
- deterministic filename assignment
- cropping, CSV file creation, Markdown rewriting, or manifest aggregation

## Inputs

Required runtime overrides:

- `IMAGE_PATH` - absolute path to the page PNG
- `PAGE_MD_PATH` - absolute path to the cleaned per-page Markdown
- `OUTPUT_PATH` - absolute path for the asset JSON
- `DOC_STEM` - document stem used by downstream deterministic naming tools
- `PAGE_NUM` - 1-indexed page number
- `TOTAL_PAGES` - total pages in the document

Optional runtime overrides:

- `ASSET_POLICY` - policy label; default `prose-document-assets-v1`

## Read boundary

Reads are limited to exactly two files:

- `{IMAGE_PATH}`
- `{PAGE_MD_PATH}`

Do not read neighbouring pages, manifests, sibling Markdown files, or previously materialized assets. This preserves page-level parallelism.

## Write boundary

Writes are limited to exactly one file:

- `{OUTPUT_PATH}`

The output filename is supplied by the orchestrator and must not be derived or changed by the skill.

## Tool usage

- No deterministic tools.
- The `allowed-tools` frontmatter field is intentionally omitted.
- The agent uses native multimodal reading for `IMAGE_PATH`, native text reading for `PAGE_MD_PATH`, and native writing for `OUTPUT_PATH`.

Disallowed behavior:

- Do not crop images.
- Do not write PNG or CSV files.
- Do not rewrite page Markdown.
- Do not assign final filenames.
- Do not widen read or write scope beyond the declared page files.

## Method

### Step 1 - Validate inputs

1. Confirm `IMAGE_PATH` exists and has a `.png` extension.
2. Confirm `PAGE_MD_PATH` exists and has a `.md` extension.
3. Confirm `OUTPUT_PATH` parent exists.
4. If required inputs are missing, write a valid JSON file with `run_status: "FAILED_INPUTS"` and an empty `assets` list.

### Step 2 - Inspect the page

1. Read `PAGE_MD_PATH` first to understand whether captions or inline tables already appear in text.
2. Read `IMAGE_PATH` and inspect the page visually.
3. Identify every visible asset that should remain auditable outside plain Markdown.
4. Preserve document reading order within each page.

### Step 3 - Emit JSON

Write only JSON, with no Markdown fences or commentary.

**REQUIRED EXACT VALUES — do not substitute synonyms or restructure.** The downstream materializer relies on these exact field names, types, and string literals:

- Top-level field names are exactly: `schema_version`, `run_status`, `doc_stem`, `page` (NOT `page_num`), `total_pages`, `asset_policy`, `assets`, `issues`.
- `schema_version` MUST be the literal string `"pdf2md-page-assets/v1"`.
- `run_status` MUST be one of the four uppercase literals: `"SUCCESS"`, `"NO_ASSETS"`, `"FAILED"`, `"FAILED_INPUTS"`. Do NOT use `"ok"`, `"success"`, `"done"`, etc.
- `assets` is a flat array. Do NOT add a sibling `tables: [...]` array; tables are entries inside `assets` with `kind: "tbl"`.
- Per-asset `kind` MUST be one of the three 3-letter literals: `"fig"`, `"tbl"`, or `"img"`. Do NOT use `"figure"`, `"image"`, `"table"`, `"diagram"`, `"plot"`, `"chart"`, `"logo"`, `"photo"`, etc.
- Per-asset `bbox_norm` MUST be a 4-element JSON array `[x0, y0, x1, y1]`. Do NOT emit a JSON object with named keys like `{"x0": ..., "y0": ..., "x1": ..., "y1": ...}`.
- Per-asset captions go in the `caption` field (NOT `label`, `title`, or `name`).
- `bbox_norm` should generously include the full asset AND its visible caption AND a small margin (~3-5% of page on each side). It is far better to include an extra few percent of whitespace than to clip the figure or caption text.

Example:

```json
{
  "schema_version": "pdf2md-page-assets/v1",
  "run_status": "SUCCESS",
  "doc_stem": "MWK_1956",
  "page": 3,
  "total_pages": 386,
  "asset_policy": "prose-document-assets-v1",
  "assets": [
    {
      "kind": "fig",
      "ordinal": 1,
      "caption": "Yield stress-strain curve of copper in compression",
      "slug": "yield-stress-strain-copper",
      "bbox_norm": [0.11, 0.18, 0.88, 0.55],
      "confidence": "medium",
      "notes": "Caption visible below plot"
    },
    {
      "kind": "tbl",
      "ordinal": 1,
      "caption": "Creep strength ratios",
      "slug": "creep-strength-ratios",
      "bbox_norm": [0.08, 0.22, 0.91, 0.64],
      "csv_text": "Material,Ratio\\nCarbon steel,1.0\\n",
      "confidence": "medium",
      "notes": "Simple two-column table"
    }
  ],
  "issues": []
}
```

## Output rules

- `kind` must be exactly `fig`, `tbl`, or `img`.
- `ordinal` is 1-indexed within `(page, kind)` in reading order.
- `caption` should use the visible caption when present; otherwise use a concise visual description.
- `slug` is advisory only. Downstream tools normalize it and append it to a stable ID.
- `bbox_norm` is `[x0, y0, x1, y1]` in normalized page coordinates, top-left origin, values between 0 and 1. It MUST be a 4-element JSON array, NOT a dict.
- `bbox_norm` MUST include the full asset, the visible caption, AND a small margin of whitespace (~3-5% of page width/height) around both. Generous bboxes are preferred — clipped figures or truncated captions are a defect; including a little extra surrounding whitespace is fine.
- `csv_text` is required for `tbl` when table text is legible. Use RFC-4180-style CSV with a header row when possible.
- If no assets are visible, return `run_status: "NO_ASSETS"` with an empty `assets` list.
- If an asset is visible but cannot be safely transcribed to CSV, include the table record with `csv_text: ""` and an issue such as `table_unreadable`.

## Non-negotiable constraints

- Single page in, one JSON out.
- No cross-page context.
- No final filename assignment.
- No asset materialization.
- No invented assets or table values.
- Every emitted asset must be visible on the page image.
- The JSON must parse with standard `json.loads`.

## QA expectations

- `OUTPUT_PATH` exists and is non-empty.
- JSON parses successfully.
- `run_status` is one of `SUCCESS`, `NO_ASSETS`, `FAILED`, or `FAILED_INPUTS`.
- Every asset has valid `kind`, `ordinal`, `caption`, `bbox_norm`, and `confidence`.
- Table assets include `csv_text` or an explicit issue explaining why CSV text is absent.
