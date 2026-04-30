# Dependencies: DEL-04-04 Nonlinear support active-set solver

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
- **ACTIVE rows:** 8
- **ANCHOR rows:** 2
- **EXECUTION rows:** 6

| DependencyID | Class / Type | Direction | Target | Status |
|---|---|---|---|---|
| DEL-04-04-DEP-001 | ANCHOR / IMPLEMENTS_NODE | UPSTREAM | SOW-012 | ACTIVE |
| DEL-04-04-DEP-002 | ANCHOR / TRACES_TO_REQUIREMENT | UPSTREAM | OBJ-003 | ACTIVE |
| DEL-04-04-DEP-003 | EXECUTION / CONSTRAINT | UPSTREAM | AB-00-01/02/03/06/08 | ACTIVE |
| DEL-04-04-DEP-004 | EXECUTION / CONSTRAINT | UPSTREAM | CONTRACT | ACTIVE |
| DEL-04-04-DEP-005 | EXECUTION / INTERFACE | UPSTREAM | DEL-04-03 | ACTIVE |
| DEL-04-04-DEP-006 | EXECUTION / INTERFACE | UPSTREAM | DEL-04-01 | ACTIVE |
| DEL-04-04-DEP-007 | EXECUTION / INTERFACE | UPSTREAM | DEL-04-06 | ACTIVE |
| DEL-04-04-DEP-008 | EXECUTION / ENABLES | DOWNSTREAM | DEL-04-05 | ACTIVE |

## Run Notes

- SCOPE: DEL-04-04
- RUN_ROOT: `/Users/ryan/ai-env/projects/chirality-piping/execution`
- DECOMPOSITION_PATH: `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- MODE: UPDATE
- STRICTNESS: CONSERVATIVE
- Source documents scanned: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_CONTEXT.md`, `_REFERENCES.md`, and decomposition/register references.
- Conservative posture used: exact implementation interfaces, convergence tolerances, friction defaults, and solver numerical library choices remain `TBD`.
- Warnings: none.

## Run History

- 2026-04-30 10:15 - TASK+dependency-extract UPDATE/CONSERVATIVE produced 8 ACTIVE rows; schema validation run locally.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 8 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| PENDING | 8 |

## Consumer Handoff Notes

- Downstream consumers should treat dependency rows as setup-level evidence, not implementation contracts.
- Rows with `Notes` beginning `PROPOSAL` require later confirmation before use as hard engineering or scheduling gates.
