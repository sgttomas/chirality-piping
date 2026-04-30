# Dependencies: DEL-03-07 Public/private library import provenance checker

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
- **Summary:** 5 ACTIVE rows: 4 ANCHOR, 1 EXECUTION.

| DependencyID | Class | Direction | Target | Status |
|---|---|---|---|---|
| DEL-03-07-DEP-001 | ANCHOR / IMPLEMENTS_NODE | UPSTREAM | SOW-019 | ACTIVE |
| DEL-03-07-DEP-002 | ANCHOR / TRACES_TO_REQUIREMENT | UPSTREAM | SOW-044 | ACTIVE |
| DEL-03-07-DEP-003 | ANCHOR / TRACES_TO_REQUIREMENT | UPSTREAM | OBJ-002 | ACTIVE |
| DEL-03-07-DEP-004 | ANCHOR / TRACES_TO_REQUIREMENT | UPSTREAM | OBJ-004 | ACTIVE |
| DEL-03-07-DEP-005 | EXECUTION / CONSTRAINT | UPSTREAM | Architecture basis AB-00-01/02/04/06/07/08 | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- 2026-04-30 - MODE=UPDATE; STRICTNESS=CONSERVATIVE; SCOPE=DEL-03-07.
- RUN_ROOT=/Users/ryan/ai-env/projects/chirality-piping/execution.
- DECOMPOSITION_PATH=/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md.
- SOURCE_DOCS=AUTO. Anchor source: `Datasheet.md` plus decomposition/register rows. Execution source order: `Procedure.md`, `Specification.md`, `Guidance.md`, `Datasheet.md`.
- [WARNING] No human-declared dependency list was provided; coordination remains externally managed.
- [WARNING] Legacy ID format validator rejects current two-digit IDs such as `DEL-03-07`, `PKG-03`, `SOW-019`, and `OBJ-002`; project IDs were preserved because they match the current decomposition/register basis.
- Parent anchor check: PASS (one ACTIVE IMPLEMENTS_NODE row).

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 5
- RETIRED: 0
- SatisfactionStatus: TBD=5

## Consumer Handoff Notes
- CONSUMER_CONTEXT=NONE. No downstream consumer-specific extension fields were emitted.
