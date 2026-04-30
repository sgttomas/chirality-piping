# pdf2md-page-assets - QA Checks

Minimum checks for a valid run:

1. `IMAGE_PATH` exists and has a `.png` extension, or the skill returns `FAILED_INPUTS`.
2. `PAGE_MD_PATH` exists and has a `.md` extension, or the skill returns `FAILED_INPUTS`.
3. `OUTPUT_PATH` parent directory exists before write.
4. `OUTPUT_PATH` exists after the run and is non-empty.
5. `OUTPUT_PATH` parses with `json.loads`.
6. No files other than `IMAGE_PATH` and `PAGE_MD_PATH` were read.
7. No files other than `OUTPUT_PATH` were written.

## JSON shape checks

The output JSON must include:

| Field | Requirement |
|---|---|
| `schema_version` | `pdf2md-page-assets/v1` |
| `run_status` | `SUCCESS`, `NO_ASSETS`, `FAILED`, or `FAILED_INPUTS` |
| `doc_stem` | Echo of runtime override |
| `page` | Echo of `PAGE_NUM` |
| `total_pages` | Echo of `TOTAL_PAGES` |
| `asset_policy` | Echo of `ASSET_POLICY` or default |
| `assets` | List, possibly empty |
| `issues` | List, possibly empty |

## Asset row checks

Each item in `assets` must satisfy:

| Field | Requirement |
|---|---|
| `kind` | Exactly `fig`, `tbl`, or `img` |
| `ordinal` | Positive integer within `(page, kind)` reading order |
| `caption` | Visible caption or concise visual description |
| `slug` | Short ASCII-friendly advisory slug, or blank if uncertain |
| `bbox_norm` | Four numbers `[x0, y0, x1, y1]`, top-left origin, each in `[0,1]`, with `x0 < x1` and `y0 < y1` |
| `confidence` | `high`, `medium`, or `low` |
| `csv_text` | Required for `tbl` when legible; otherwise blank plus an issue |

## Failure posture

| Failure mode | Required output |
|---|---|
| Missing input file | `run_status: "FAILED_INPUTS"`, empty `assets`, issue naming the missing input |
| No extractable assets | `run_status: "NO_ASSETS"`, empty `assets`, no invented placeholders |
| Unreadable page image | `run_status: "FAILED"`, empty `assets`, issue explaining the problem |
| Partial table uncertainty | `run_status: "SUCCESS"` with table issue such as `table_unreadable` or `possible_continuation` |

## Orchestrator-side checks

These checks belong to `PDF2MD`, not this skill:

- Running `materialize_page_assets.py` on every page asset JSON.
- Confirming crop PNGs exist for all non-skipped figure/image/table records.
- Confirming table CSV files exist for table records with CSV text.
- Aggregating the document asset manifest.
- Running `validate_assets.py` against the final assembled Markdown.
- Treating unresolved asset references as degraded output requiring human acknowledgment before downstream use.
