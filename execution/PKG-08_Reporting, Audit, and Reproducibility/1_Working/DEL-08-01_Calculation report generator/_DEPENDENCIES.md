# Dependencies: DEL-08-01 Calculation report generator

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
| DEP-DEL-08-01-001 | ANCHOR | UPSTREAM | OTHER | SOW-024 | ACTIVE |
| DEP-DEL-08-01-002 | ANCHOR | UPSTREAM | OTHER | OBJ-007 | ACTIVE |
| DEP-DEL-08-01-003 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-01 | ACTIVE |
| DEP-DEL-08-01-004 | EXECUTION | UPSTREAM | INTERFACE | DEL-06-04 | ACTIVE |
| DEP-DEL-08-01-005 | EXECUTION | UPSTREAM | INTERFACE | DEL-08-02 | ACTIVE |
| DEP-DEL-08-01-006 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-08-03 | ACTIVE |
| DEP-DEL-08-01-007 | EXECUTION | DOWNSTREAM | ENABLES | DEL-08-05 | ACTIVE |

## Run Notes

- SCOPE: `DEL-08-01`
- RUN_ROOT: `/Users/ryan/ai-env/projects/chirality-piping/execution`
- DECOMPOSITION_PATH: `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- MODE: `UPDATE`
- STRICTNESS: `CONSERVATIVE`
- SOURCE_DOCS: `AUTO`; `_CONTEXT.md`, production documents, and decomposition/register references were used.
- No cross-deliverable synthesis was performed; target resolution used explicit decomposition/register names and source-grounded report requirements.
- Proposed targets remain subject to REVIEW, RECONCILIATION, or human confirmation.
- Tree anchor check: PASS; exactly one ACTIVE `IMPLEMENTS_NODE` row exists.

## Run History

- 2026-04-30 12:20 - TASK+dependency-extract setup run; MODE=UPDATE; STRICTNESS=CONSERVATIVE; schema validation PASS; enum validation PASS.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 7 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| PENDING | 7 |

## Consumer Handoff Notes
- Consumers should treat dependency targets marked `PROPOSAL` in `Notes` as routing hints until confirmed by REVIEW, RECONCILIATION, or the human project authority.
- No dependency row authorizes implementation beyond this setup/document-production session.
