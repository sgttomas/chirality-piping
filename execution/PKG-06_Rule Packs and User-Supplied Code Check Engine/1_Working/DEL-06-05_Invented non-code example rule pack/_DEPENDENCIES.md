# Dependencies: DEL-06-05 Invented non-code example rule pack

## Coordination (human-owned)

- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION.

## Upstream (I need these before I can proceed) - human-owned declarations

- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations

- Dependencies coordinated externally by humans.

## Extracted Dependency Register

- **Status:** REFRESHED
- **Dependencies.csv:** `Dependencies.csv`
- **Register schema:** v3.1
- **ACTIVE rows:** 7
- **RETIRED rows:** 0
- **Anchor rows:** 4
- **Execution rows:** 3

| DependencyID | Class | Direction | Type | Target | Satisfaction | Notes |
|---|---|---|---|---|---|---|
| DEP-006-05-001 | ANCHOR | UPSTREAM | OTHER | DEL-06-05 WBS node | SATISFIED | Parent anchor |
| DEP-006-05-002 | ANCHOR | UPSTREAM | OTHER | SOW-016 | SATISFIED | Scope trace |
| DEP-006-05-003 | ANCHOR | UPSTREAM | OTHER | OBJ-005 | SATISFIED | Objective trace |
| DEP-006-05-004 | ANCHOR | UPSTREAM | OTHER | OBJ-011 | SATISFIED | Objective trace |
| DEP-006-05-005 | EXECUTION | UPSTREAM | CONSTRAINT | OPS-CONTRACT | SATISFIED | Invented/no-protected-content constraint |
| DEP-006-05-006 | EXECUTION | UPSTREAM | CONSTRAINT | OPS-TYPES | SATISFIED | Professional-boundary constraint |
| DEP-006-05-007 | EXECUTION | UPSTREAM | INTERFACE | DEL-06-01 Rule-pack schema | PENDING | Future implementation interface and not a setup blocker |

## Run Notes

- `SCOPE`: DEL-06-05.
- `RUN_ROOT`: repository root.
- `DECOMPOSITION_PATH`: `docs/_Decomposition/SOFTWARE_DECOMP.md`.
- `SOURCE_DOCS`: AUTO.
- `ANCHOR_DOC`: `_CONTEXT.md`.
- `EXECUTION_DOC_ORDER`: `Specification.md`, `Procedure.md`, `Datasheet.md`, `Guidance.md`.
- `MODE`: UPDATE.
- `STRICTNESS`: CONSERVATIVE.
- `CONSUMER_CONTEXT`: NONE.
- Parent anchor count: 1.
- No hierarchy discovery was performed.
- No cross-deliverable scheduling edges were created.
- `DEP-006-05-007` is retained as a future implementation interface because the setup documents explicitly defer schema-specific payload finalization to `DEL-06-01`.
- `[WARNING] ID_FORMAT_HELPER_MISMATCH`: `tools/validation/validate_id_format.sh` expects legacy three-digit `PKG`, `DEL`, and four-digit `SOW` formats, while current `docs/TYPES.md`, decomposition, and registers use `PKG-06`, `DEL-06-05`, and `SOW-016`. `Dependencies.csv` preserves the authoritative current IDs.

## Run History

- 2026-04-30 - TASK+dependency-extract refreshed `Dependencies.csv` and `_DEPENDENCIES.md`; mode UPDATE; strictness CONSERVATIVE; active rows 7; retired rows 0; warning `ID_FORMAT_HELPER_MISMATCH`.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 7 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 6 |
| PENDING | 1 |
| TBD | 0 |
| IN_PROGRESS | 0 |
| WAIVED | 0 |
| NOT_APPLICABLE | 0 |

## Downstream Handoff Notes

- The dependency register is deliverable-local evidence for aggregation or reconciliation.
- The only open dependency is a future implementation interface to `DEL-06-01`; it does not block this setup/document-production deliverable.
- Public example implementation remains outside this session's write scope.
- ID-format helper remediation should be handled outside this deliverable because the helper conflicts with project-wide identifier formats.
