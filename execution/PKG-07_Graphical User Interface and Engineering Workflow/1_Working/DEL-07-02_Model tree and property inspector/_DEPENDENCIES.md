# Dependencies: DEL-07-02 Model tree and property inspector

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
- **ACTIVE rows:** 14
- **ANCHOR rows:** 3
- **EXECUTION rows:** 11

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEP-DEL-07-02-001 | ANCHOR | UPSTREAM | OTHER | SOW-020 | ACTIVE |
| DEP-DEL-07-02-002 | ANCHOR | UPSTREAM | OTHER | SOW-021 | ACTIVE |
| DEP-DEL-07-02-003 | ANCHOR | UPSTREAM | OTHER | OBJ-006 | ACTIVE |
| DEP-DEL-07-02-004 | EXECUTION | UPSTREAM | INTERFACE | DEL-07-01 | ACTIVE |
| DEP-DEL-07-02-005 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-01 | ACTIVE |
| DEP-DEL-07-02-006 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-02 | ACTIVE |
| DEP-DEL-07-02-007 | EXECUTION | UPSTREAM | INTERFACE | DEL-00-03 | ACTIVE |
| DEP-DEL-07-02-008 | EXECUTION | UPSTREAM | INTERFACE | DEL-00-05 | ACTIVE |
| DEP-DEL-07-02-009 | EXECUTION | UPSTREAM | INTERFACE | DEL-00-06 | ACTIVE |
| DEP-DEL-07-02-010 | EXECUTION | UPSTREAM | INTERFACE | DEL-03-01 | ACTIVE |
| DEP-DEL-07-02-011 | EXECUTION | UPSTREAM | INTERFACE | DEL-03-02 | ACTIVE |
| DEP-DEL-07-02-012 | EXECUTION | UPSTREAM | INTERFACE | DEL-06-01 | ACTIVE |
| DEP-DEL-07-02-013 | EXECUTION | DOWNSTREAM | ENABLES | DEL-07-03 | ACTIVE |
| DEP-DEL-07-02-014 | EXECUTION | DOWNSTREAM | ENABLES | DEL-07-04 | ACTIVE |

## Run Notes

- SCOPE: `DEL-07-02`
- RUN_ROOT: `/Users/ryan/ai-env/projects/chirality-piping/execution`
- DECOMPOSITION_PATH: `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- MODE: `UPDATE`
- STRICTNESS: `CONSERVATIVE`
- SOURCE_DOCS: `AUTO`; production documents plus `_CONTEXT.md` were used.
- No cross-deliverable synthesis was performed; target resolution used explicit decomposition/register names only.
- Proposed targets remain subject to human or REVIEW confirmation.

## Run History

- 2026-04-30 10:40 - TASK+dependency-extract setup run; MODE=UPDATE; STRICTNESS=CONSERVATIVE; schema validation PASS; enum validation PASS.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 14 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| PENDING | 14 |

## Consumer Handoff Notes

- Consumers should treat dependency targets marked `PROPOSAL` in `Notes` as routing hints, not accepted architecture decisions.
- No dependency row introduces implementation authorization.
