# Dependencies: DEL-03-06 Expansion joint component model

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
- **Summary:** 3 ACTIVE rows: 2 ANCHOR, 1 EXECUTION.

| DependencyID | Class | Direction | Target | Status |
|---|---|---|---|---|
| DEL-03-06-DEP-001 | ANCHOR / IMPLEMENTS_NODE | UPSTREAM | SOW-010 | ACTIVE |
| DEL-03-06-DEP-002 | ANCHOR / TRACES_TO_REQUIREMENT | UPSTREAM | OBJ-004 | ACTIVE |
| DEL-03-06-DEP-003 | EXECUTION / CONSTRAINT | UPSTREAM | Architecture basis AB-00-01/02/04/06/07/08 | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- 2026-04-30 - MODE=UPDATE; STRICTNESS=CONSERVATIVE; SCOPE=DEL-03-06.
- RUN_ROOT=/Users/ryan/ai-env/projects/chirality-piping/execution.
- DECOMPOSITION_PATH=/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md.
- SOURCE_DOCS=AUTO. Anchor source: `Datasheet.md` plus decomposition/register rows. Execution source order: `Procedure.md`, `Specification.md`, `Guidance.md`, `Datasheet.md`.
- [WARNING] No human-declared dependency list was provided; coordination remains externally managed.
- Parent anchor check: PASS (one ACTIVE IMPLEMENTS_NODE row).

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 3
- RETIRED: 0
- SatisfactionStatus: TBD=3

## Consumer Handoff Notes
- CONSUMER_CONTEXT=NONE. No downstream consumer-specific extension fields were emitted.
