# Dependencies: DEL-01-03 Contributor certification workflow

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
- **Summary:** 5 ACTIVE rows: 3 ANCHOR, 2 EXECUTION.

| DependencyID | Class | Direction | Target | Status |
|---|---|---|---|---|
| DEP-001-03-001 | ANCHOR / IMPLEMENTS_NODE | UPSTREAM | PKG-01 | ACTIVE |
| DEP-001-03-002 | ANCHOR / TRACES_TO_REQUIREMENT | UPSTREAM | SOW-028 | ACTIVE |
| DEP-001-03-003 | ANCHOR / TRACES_TO_REQUIREMENT | UPSTREAM | SOW-048 | ACTIVE |
| DEP-001-03-004 | EXECUTION / CONSTRAINT | UPSTREAM | Human project authority/legal review | ACTIVE |
| DEP-001-03-005 | EXECUTION / HANDOVER | DOWNSTREAM | Future `CONTRIBUTING.md` section | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- 2026-04-30 - MODE=UPDATE; STRICTNESS=CONSERVATIVE; SCOPE=DEL-01-03; RUN_ROOT=`/Users/ryan/ai-env/projects/chirality-piping/execution`; DECOMPOSITION_PATH=`/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`.
- Source docs selected by DEFAULT/AUTO: anchor document `Datasheet.md`; execution documents `Procedure.md`, `Specification.md`, `Guidance.md`, `Datasheet.md`.
- Decomposition path was available and used for labels/traceability.
- Parent anchor check: PASS, exactly one ACTIVE `IMPLEMENTS_NODE` row.
- No rows were deleted. No prior extracted rows existed to retire.
- ID-format helper warning: local validator expects legacy three-digit package/deliverable IDs and reports `PKG-01`/`DEL-01-03` invalid even though these are the current SOFTWARE_DECOMP formats. Schema validation remains valid; human/tooling alignment may be needed.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 5
- RETIRED: 0
- Satisfaction status:
  - PENDING: 5
  - SATISFIED: 0
  - WAIVED: 0
  - TBD: 0
  - NOT_APPLICABLE: 0

## Consumer Handoff Notes
- CONSUMER_CONTEXT=NONE. No downstream estimating/aggregation-specific extension fields were added.
