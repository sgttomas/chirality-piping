# Dependencies: DEL-04-05 Sparse solver performance harness

## Coordination (human-owned)
- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION.

## Upstream (I need these before I can proceed) - human-owned declarations
- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations
- Dependencies coordinated externally by humans.

## Extracted Dependency Register

- **Status:** UPDATED
- **Dependencies.csv:** `Dependencies.csv`
- **Register schema:** v3.1
- **ACTIVE rows:** 5
- **RETIRED rows:** 0

| DependencyID | Class | Direction | Type | Target | Evidence |
|---|---|---|---|---|---|
| DEP-DEL-04-05-001 | ANCHOR | UPSTREAM | OTHER | SOW-035 | `_CONTEXT.md#Scope Coverage`; `docs/_Decomposition/SOFTWARE_DECOMP.md#SOW-035` |
| DEP-DEL-04-05-002 | ANCHOR | UPSTREAM | OTHER | OBJ-003 | `_CONTEXT.md#Objective Support`; `docs/_Decomposition/SOFTWARE_DECOMP.md#OBJ-003` |
| DEP-DEL-04-05-003 | ANCHOR | UPSTREAM | OTHER | OBJ-008 | `_CONTEXT.md#Objective Support`; `docs/_Decomposition/SOFTWARE_DECOMP.md#OBJ-008` |
| DEP-DEL-04-05-004 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-01 | `Procedure.md#Prerequisites` |
| DEP-DEL-04-05-005 | EXECUTION | UPSTREAM | INTERFACE | DEL-04-06 | `Specification.md#Requirements`; `Procedure.md#Verification` |

## Run Notes

- SCOPE: DEL-04-05.
- RUN_ROOT: `/Users/ryan/ai-env/projects/chirality-piping/execution`.
- DECOMPOSITION_PATH: `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`.
- MODE: UPDATE.
- STRICTNESS: CONSERVATIVE.
- SOURCE_DOCS: AUTO; scanned `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, and `_DEPENDENCIES.md`.
- ANCHOR_DOC: AUTO; `_CONTEXT.md` and decomposition/register rows supplied explicit scope and objective anchors.
- EXECUTION_DOC_ORDER: AUTO; `Procedure.md`, `Specification.md`, and `Guidance.md` provided execution-edge signals.
- Target resolution was conservative. No target was invented from structural adjacency alone.
- Parent anchor check: PASS; one ACTIVE `IMPLEMENTS_NODE` anchor is present.
- Protected/proprietary benchmark data: none introduced.

## Run History

- 2026-04-30T10:15:00-06:00 - TASK+dependency-extract UPDATE / CONSERVATIVE. Wrote v3.1 register with 5 ACTIVE rows and 0 RETIRED rows. Decomposition path available. Warnings: none.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 5 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| PENDING | 2 |
| NOT_APPLICABLE | 3 |

## Downstream Handoff Notes

- Consumer context was `NONE`; no specialized downstream handoff notes were requested.
