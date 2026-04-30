# Dependencies: DEL-05-04 Analysis status semantics

## Coordination (human-owned)
- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION.

## Upstream (I need these before I can proceed) - human-owned declarations
- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations
- Dependencies coordinated externally by humans.

## Extracted Dependency Register (populated by TASK+dependency-extract)
- **Status:** UPDATED
- **Dependencies.csv:** `Dependencies.csv`
- **Schema:** v3.1
- **Summary:** 8 ACTIVE rows: 3 ANCHOR, 5 EXECUTION.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEP-05-04-001 | ANCHOR | UPSTREAM | OTHER | SOW-047 | ACTIVE |
| DEP-05-04-002 | ANCHOR | UPSTREAM | OTHER | OBJ-005 | ACTIVE |
| DEP-05-04-003 | ANCHOR | UPSTREAM | OTHER | OBJ-011 | ACTIVE |
| DEP-05-04-004 | EXECUTION | UPSTREAM | CONSTRAINT | docs/CONTRACT.md | ACTIVE |
| DEP-05-04-005 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08 | ACTIVE |
| DEP-05-04-006 | EXECUTION | UPSTREAM | CONSTRAINT | docs/architecture/analysis_status_semantics.md | ACTIVE |
| DEP-05-04-007 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-06-03 | ACTIVE |
| DEP-05-04-008 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-08-03 | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- 2026-04-30 - TASK+dependency-extract SCOPE=DEL-05-04 RUN_ROOT=/Users/ryan/ai-env/projects/chirality-piping/execution DECOMPOSITION_PATH=/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md MODE=UPDATE STRICTNESS=CONSERVATIVE.
- Source docs scanned: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, and decomposition/register slices.
- Anchor doc: `Specification.md` plus `docs/_Registers/ScopeLedger.csv` / `docs/_Decomposition/SOFTWARE_DECOMP.md` for validation.
- Execution docs: `Specification.md`, `Guidance.md`, `Procedure.md`, `docs/CONTRACT.md`, `docs/architecture/analysis_status_semantics.md`.
- Warnings: none. Parent anchor count is 1.
- Conservative extraction: downstream handoffs to DEL-06-03 and DEL-08-03 are marked `PROPOSAL` with `Confidence=MEDIUM`.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 8
- RETIRED: 0
- Satisfaction status:
  - SATISFIED: 6
  - PENDING: 2

## Consumer Handoff Notes
- CONSUMER_CONTEXT=NONE. Downstream consumers should treat proposed handoffs as dependency evidence for later aggregation/reconciliation, not as a human-approved DAG.
