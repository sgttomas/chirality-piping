# Dependencies: DEL-04-02 Straight pipe element

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
- **ACTIVE rows:** 6
- **ANCHOR rows:** 2
- **EXECUTION rows:** 4

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEP-DEL-04-02-001 | ANCHOR | UPSTREAM | OTHER | SOW-006 | ACTIVE |
| DEP-DEL-04-02-002 | ANCHOR | UPSTREAM | OTHER | OBJ-003 | ACTIVE |
| DEP-DEL-04-02-003 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-01 | ACTIVE |
| DEP-DEL-04-02-004 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-03-08 | ACTIVE |
| DEP-DEL-04-02-005 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-05-03 | ACTIVE |
| DEP-DEL-04-02-006 | EXECUTION | UPSTREAM | INTERFACE | DEL-05-01 | ACTIVE |

## Run Notes

- SCOPE: `DEL-04-02`
- RUN_ROOT: `/Users/ryan/ai-env/projects/chirality-piping/execution`
- DECOMPOSITION_PATH: `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- MODE: `UPDATE`
- STRICTNESS: `CONSERVATIVE`
- SOURCE_DOCS: `AUTO`; production documents plus `_CONTEXT.md` were used.
- No cross-deliverable synthesis was performed; target resolution used explicit decomposition/register names only.
- Proposed targets remain subject to human or REVIEW confirmation.

## Run History

- 2026-04-30 10:15 - TASK+dependency-extract setup run; MODE=UPDATE; STRICTNESS=CONSERVATIVE; schema validation PASS; enum validation PASS.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 6 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| PENDING | 6 |

## Consumer Handoff Notes

- Consumers should treat dependency targets marked `PROPOSAL` in `Notes` as routing hints, not accepted architecture decisions.
- No dependency row introduces implementation authorization.
