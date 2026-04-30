# Dependencies: DEL-05-03 Fundamental stress recovery module

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
| DEP-DEL-05-03-001 | ANCHOR | UPSTREAM | OTHER | SOW-015 | ACTIVE |
| DEP-DEL-05-03-002 | ANCHOR | UPSTREAM | OTHER | OBJ-003 | ACTIVE |
| DEP-DEL-05-03-003 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-02 | ACTIVE |
| DEP-DEL-05-03-004 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-03-08 | ACTIVE |
| DEP-DEL-05-03-005 | EXECUTION | UPSTREAM | INTERFACE | DEL-05-01 | ACTIVE |
| DEP-DEL-05-03-006 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-06-01 | ACTIVE |
| DEP-DEL-05-03-007 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-09-02 | ACTIVE |

## Run Notes

- SCOPE: `DEL-05-03`
- RUN_ROOT: `/Users/ryan/ai-env/projects/chirality-piping/execution`
- DECOMPOSITION_PATH: `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- MODE: `UPDATE`
- STRICTNESS: `CONSERVATIVE`
- SOURCE_DOCS: `AUTO`; production documents plus `_CONTEXT.md` were used.
- No cross-deliverable synthesis was performed; target resolution used explicit decomposition/register names only.
- Proposed targets remain subject to human or REVIEW confirmation.

## Run History

- 2026-04-30 10:23 - TASK+dependency-extract setup run; MODE=UPDATE; STRICTNESS=CONSERVATIVE; schema validation PASS; enum validation PASS.

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
- No dependency row introduces implementation authorization.
