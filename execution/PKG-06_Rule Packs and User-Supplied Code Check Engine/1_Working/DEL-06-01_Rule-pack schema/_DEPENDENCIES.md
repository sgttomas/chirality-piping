# Dependencies: DEL-06-01 Rule-pack schema

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
- **ACTIVE rows:** 11
- **RETIRED rows:** 0

| DependencyClass | Count |
|---|---:|
| ANCHOR | 3 |
| EXECUTION | 8 |

| Direction | Count |
|---|---:|
| UPSTREAM | 7 |
| DOWNSTREAM | 4 |

## Run Notes

- MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=NONE.
- RUN_ROOT=/Users/ryan/ai-env/projects/chirality-piping/execution.
- DECOMPOSITION_PATH=/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md.
- SOURCE_DOCS=AUTO: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, `_SEMANTIC_LENSING.md`.
- ANCHOR_DOC=AUTO resolved primarily to `_CONTEXT.md` and register/decomposition identifiers cited there.
- EXECUTION_DOC_ORDER=AUTO resolved to `Specification.md`, `Procedure.md`, `Datasheet.md`, `Guidance.md`.
- Parent anchor check: PASS. One ACTIVE `IMPLEMENTS_NODE` anchor found.
- Conservative extraction emitted direct anchors for SOW-016, SOW-042, and OBJ-005; architecture constraints from the sealed context; one contract constraint; and four proposed downstream handoffs within PKG-06.
- No protected rule content, proprietary allowables, copied standards formulas, standards text, or invented engineering values were extracted.

## Run History

- 2026-04-30: TASK+dependency-extract MODE=UPDATE STRICTNESS=CONSERVATIVE; decomposition present; ACTIVE rows=11; warnings=none.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 11 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 7 |
| PENDING | 4 |

## Consumer Handoff Notes

- No specialized consumer context was requested.
- Downstream consumers should treat DEL-06-02, DEL-06-03, DEL-06-04, and DEL-06-05 rows as conservative `PROPOSAL` edges until reviewed by reconciliation or human authority.

