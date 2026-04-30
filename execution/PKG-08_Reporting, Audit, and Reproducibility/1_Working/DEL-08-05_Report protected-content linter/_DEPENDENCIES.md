# Dependencies: DEL-08-05 Report protected-content linter

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
- **ACTIVE rows:** 8
- **ANCHOR rows:** 3
- **EXECUTION rows:** 5

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEP-DEL-08-05-001 | ANCHOR | UPSTREAM | OTHER | SOW-043 | ACTIVE |
| DEP-DEL-08-05-002 | ANCHOR | UPSTREAM | OTHER | OBJ-002 | ACTIVE |
| DEP-DEL-08-05-003 | ANCHOR | UPSTREAM | OTHER | OBJ-007 | ACTIVE |
| DEP-DEL-08-05-004 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-01-02 | ACTIVE |
| DEP-DEL-08-05-005 | EXECUTION | UPSTREAM | INTERFACE | DEL-08-01 | ACTIVE |
| DEP-DEL-08-05-006 | EXECUTION | UPSTREAM | INTERFACE | DEL-08-03 | ACTIVE |
| DEP-DEL-08-05-007 | EXECUTION | UPSTREAM | INTERFACE | DEL-06-04 | ACTIVE |
| DEP-DEL-08-05-008 | EXECUTION | DOWNSTREAM | ENABLES | DEL-10-04 | ACTIVE |

## Run Notes

- SCOPE: `DEL-08-05`
- RUN_ROOT: `/Users/ryan/ai-env/projects/chirality-piping/execution`
- DECOMPOSITION_PATH: `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- MODE: `UPDATE`
- STRICTNESS: `CONSERVATIVE`
- SOURCE_DOCS: `AUTO`; `_CONTEXT.md`, production documents, and decomposition/register references were used.
- No cross-deliverable synthesis was performed; target resolution used explicit decomposition/register names and source-grounded report-linter requirements.
- Proposed targets remain subject to REVIEW, RECONCILIATION, or human confirmation.
- Tree anchor check: PASS; exactly one ACTIVE `IMPLEMENTS_NODE` row exists.
- Protected-content boundary: no protected standards text, copied standards table content, proprietary formula, private rule-pack content, linter source, CI workflow, test fixture, report template, or repo-level artifact was introduced by this setup run.
- ID format validator warning: `tools/validation/validate_id_format.sh` currently expects older PROJECT-style three-digit IDs and rejects SOFTWARE-style IDs such as `DEL-08-05` and `PKG-08`; schema and enum validation remain PASS.

## Run History

- 2026-04-30 13:20 - TASK+dependency-extract setup run completed after interrupted turn; MODE=UPDATE; STRICTNESS=CONSERVATIVE; schema validation PASS; enum validation PASS.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 8 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| PENDING | 8 |

## Consumer Handoff Notes
- Consumers should treat dependency targets marked `PROPOSAL` in `Notes` as routing hints until confirmed by REVIEW, RECONCILIATION, or the human project authority.
- No dependency row authorizes implementation beyond this setup/document-production session.
- The linter is a heuristic-plus-review control only and must not be treated as sole legal clearance.
