# Dependencies: DEL-09-02 Stress recovery benchmark suite

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
- **Schema:** v3.1
- **ACTIVE rows:** 7
- **ANCHOR rows:** 2
- **EXECUTION rows:** 5

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEP-DEL-09-02-001 | ANCHOR | UPSTREAM | OTHER | SOW-026 | ACTIVE |
| DEP-DEL-09-02-002 | ANCHOR | UPSTREAM | OTHER | OBJ-008 | ACTIVE |
| DEP-DEL-09-02-003 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-05-03 | ACTIVE |
| DEP-DEL-09-02-004 | EXECUTION | UPSTREAM | INTERFACE | DEL-05-02 | ACTIVE |
| DEP-DEL-09-02-005 | EXECUTION | UPSTREAM | CONSTRAINT | OPS-K-IP-1 | ACTIVE |
| DEP-DEL-09-02-006 | EXECUTION | UPSTREAM | CONSTRAINT | OPS-K-UNIT-1 | ACTIVE |
| DEP-DEL-09-02-007 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-09-05 | ACTIVE |

## Run Notes

- SCOPE: `DEL-09-02`
- RUN_ROOT: `/Users/ryan/ai-env/projects/chirality-piping/execution`
- DECOMPOSITION_PATH: `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- MODE: `UPDATE`
- STRICTNESS: `CONSERVATIVE`
- SOURCE_DOCS: `AUTO`; production documents plus `_CONTEXT.md` were used.
- No cross-deliverable synthesis was performed; target resolution used explicit decomposition/register names only.
- Proposed targets remain subject to human or REVIEW confirmation.
- No dependency row authorizes benchmark source-file implementation or final numerical tolerances.

## Run History

- 2026-04-30 10:50 - TASK+dependency-extract setup run; MODE=UPDATE; STRICTNESS=CONSERVATIVE; schema validation PASS; enum validation PASS.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 7 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| PENDING | 7 |

## Consumer Handoff Notes

- Consumers should treat dependency targets marked `PROPOSAL` in `Notes` as routing hints, not accepted architecture decisions.
- Dependency rows preserve information-flow and constraint evidence only; they do not create implementation authorization.
