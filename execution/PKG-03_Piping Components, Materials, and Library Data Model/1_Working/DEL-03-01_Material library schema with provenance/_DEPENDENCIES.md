# Dependencies: DEL-03-01 Material library schema with provenance

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
- **Schema version:** v3.1
- **ACTIVE rows:** 7
- **RETIRED rows:** 0

| DependencyClass | Count |
|---|---:|
| ANCHOR | 2 |
| EXECUTION | 5 |

| Direction | Count |
|---|---:|
| UPSTREAM | 5 |
| DOWNSTREAM | 2 |

## Run Notes

- MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=NONE.
- RUN_ROOT=/Users/ryan/ai-env/projects/chirality-piping/execution.
- DECOMPOSITION_PATH=/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md.
- SOURCE_DOCS=AUTO: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, `_SEMANTIC_LENSING.md`.
- ANCHOR_DOC=AUTO resolved primarily to `_CONTEXT.md` and register/decomposition identifiers cited there.
- EXECUTION_DOC_ORDER=AUTO resolved to `Specification.md`, `Procedure.md`, `Datasheet.md`, `Guidance.md`.
- Parent anchor check: PASS. One ACTIVE `IMPLEMENTS_NODE` anchor found.
- Conservative extraction emitted direct anchors for SOW-017 and OBJ-004, architecture constraints from the sealed brief/context, and two proposed downstream handoffs supported by package/decomposition relationships plus local document evidence.
- No protected material allowable tables, proprietary values, or invented engineering values were extracted.

## Run History

- 2026-04-30: TASK+dependency-extract MODE=UPDATE STRICTNESS=CONSERVATIVE; decomposition present; ACTIVE rows=7; warnings=none.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 7 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 5 |
| PENDING | 2 |

## Consumer Handoff Notes

- No specialized consumer context was requested.
- Downstream consumers should treat DEL-03-07 and DEL-03-08 rows as conservative `PROPOSAL` edges until reviewed by reconciliation or human authority.

