# Dependencies: DEL-11-02 Developer guide for solver and rule packs

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
- **ACTIVE rows:** 16
- **RETIRED rows:** 0

| DependencyClass | Count |
|---|---:|
| ANCHOR | 3 |
| EXECUTION | 13 |

| Direction | Count |
|---|---:|
| UPSTREAM | 11 |
| DOWNSTREAM | 5 |

## Run Notes

- MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=NONE.
- RUN_ROOT=/Users/ryan/ai-env/projects/chirality-piping/execution.
- DECOMPOSITION_PATH=/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md.
- SOURCE_DOCS=AUTO: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, `_SEMANTIC_LENSING.md`.
- ANCHOR_DOC=AUTO resolved primarily to `_CONTEXT.md` and register/decomposition identifiers cited there.
- EXECUTION_DOC_ORDER=AUTO resolved to `Specification.md`, `Guidance.md`, `Procedure.md`, `Datasheet.md`.
- Parent anchor check: PASS. One ACTIVE `IMPLEMENTS_NODE` anchor found.
- Conservative extraction emitted direct anchors for SOW-033, OBJ-001, and OBJ-002; architecture constraints from the sealed context; two governance/validation document constraints; and five proposed downstream handoffs to solver, rule-pack, diagnostics, and contributor-onboarding work.
- No protected standards text, protected tables, protected examples, copied formulas, proprietary commercial data, private rule packs, private project models, or invented engineering values were extracted.

## Run History

- 2026-04-30: TASK+dependency-extract MODE=UPDATE STRICTNESS=CONSERVATIVE; decomposition present; ACTIVE rows=16; warnings=none.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 16 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 11 |
| PENDING | 5 |

## Consumer Handoff Notes

- No specialized consumer context was requested.
- Downstream consumers should treat DEL-04-01, DEL-04-06, DEL-06-01, DEL-06-02, and DEL-11-05 rows as conservative `PROPOSAL` edges until reviewed by reconciliation or human authority.
