# Dependencies: DEL-02-01 Canonical domain model schema

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
- **Summary:** 2 ACTIVE rows: 2 ANCHOR, 0 EXECUTION, 0 RETIRED.

| DependencyID | Class | AnchorType | Direction | TargetType | TargetRefID | TargetName | Status |
|---|---|---|---|---|---|---|---|
| DEP-02-01-001 | ANCHOR | IMPLEMENTS_NODE | UPSTREAM | WBS_NODE | SOW-041 | Machine-readable schema scope item | ACTIVE |
| DEP-02-01-002 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | REQUIREMENT | OBJ-001 | Open auditable piping stress analysis platform objective | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- 2026-04-30: TASK+dependency-extract ran with `SCOPE=DEL-02-01`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `RUN_ROOT=/Users/ryan/ai-env/projects/chirality-piping/execution`, and `DECOMPOSITION_PATH=/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`.
- Run record: `_run_records/TASK_RUN_2026-04-30_0941_004.md`.
- Source document selection used `AUTO`: anchor evidence from `_CONTEXT.md`; execution scan considered `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, and `_DEPENDENCIES.md`.
- Decomposition validation used `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4, `docs/_Registers/Deliverables.csv`, and `docs/_Registers/ScopeLedger.csv`.
- Conservative extraction emitted anchor rows only. No execution rows were emitted because no concrete deliverable-to-deliverable prerequisite, handoff, interface, or constraint was explicitly stated in the deliverable-local sources.
- No `[WARNING] FLOATING_NODE`: one ACTIVE `IMPLEMENTS_NODE` anchor is present.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE rows: 2.
- RETIRED rows: 0.
- SatisfactionStatus counts: `NOT_APPLICABLE=2`.
- Parent anchor integrity: PASS, exactly one ACTIVE `DependencyClass=ANCHOR` and `AnchorType=IMPLEMENTS_NODE` row.

## Consumer Handoff Notes
- `CONSUMER_CONTEXT=NONE`; no consumer-specific extension columns were added.
