# Dependencies: DEL-08-03 Warnings, assumptions, and provenance report section

## Coordination (human-owned)

- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION.

## Upstream (I need these before I can proceed) - human-owned declarations

- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations

- Dependencies coordinated externally by humans.

## Extracted Dependency Register

- **Status:** REFRESHED
- **Dependencies.csv:** `Dependencies.csv`
- **Schema Version:** v3.1
- **Active rows:** 12
- **Anchor rows:** 3
- **Execution rows:** 9
- **Upstream rows:** 11
- **Downstream rows:** 1

| DependencyID | Class | Direction | Type | Target | Satisfaction | Confidence |
|---|---|---|---|---|---|---|
| DEP-08-03-001 | ANCHOR | UPSTREAM | OTHER | SOW-024 Auditable calculation reports | SATISFIED | HIGH |
| DEP-08-03-002 | ANCHOR | UPSTREAM | OTHER | OBJ-007 Reproducible reports and exports objective | SATISFIED | HIGH |
| DEP-08-03-003 | ANCHOR | UPSTREAM | OTHER | OBJ-011 Professional responsibility objective | SATISFIED | HIGH |
| DEP-08-03-004 | EXECUTION | UPSTREAM | CONSTRAINT | Invariant catalog | SATISFIED | HIGH |
| DEP-08-03-005 | EXECUTION | UPSTREAM | CONSTRAINT | Architecture basis constraints | SATISFIED | HIGH |
| DEP-08-03-006 | EXECUTION | UPSTREAM | INTERFACE | DEL-08-01 Calculation report generator | PENDING | HIGH |
| DEP-08-03-007 | EXECUTION | UPSTREAM | HANDOVER | DEL-08-02 Audit manifest and model hash | PENDING | HIGH |
| DEP-08-03-008 | EXECUTION | UPSTREAM | INTERFACE | DEL-07-04 Missing-data warning and blocking UX | PENDING | MEDIUM |
| DEP-08-03-009 | EXECUTION | UPSTREAM | INTERFACE | DEL-07-07 Solve execution UX diagnostics | PENDING | MEDIUM |
| DEP-08-03-010 | EXECUTION | UPSTREAM | INTERFACE | DEL-06-03 Required-input completeness checker | PENDING | MEDIUM |
| DEP-08-03-011 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-08-05 Report protected-content linter | PENDING | HIGH |
| DEP-08-03-012 | EXECUTION | DOWNSTREAM | ENABLES | DEL-08-01 Calculation report generator | PENDING | HIGH |

## Run Notes

- `SCOPE`: DEL-08-03
- `RUN_ROOT`: repository root
- `DECOMPOSITION_PATH`: `docs/_Decomposition/SOFTWARE_DECOMP.md`
- `SOURCE_DOCS`: AUTO
- `ANCHOR_DOC`: `Specification.md` plus decomposition/register evidence
- `EXECUTION_DOC_ORDER`: `Procedure.md`, `Specification.md`, `Guidance.md`, `Datasheet.md`
- `MODE`: UPDATE
- `STRICTNESS`: CONSERVATIVE
- `CONSUMER_CONTEXT`: NONE
- Dependency rows are extracted from deliverable-local production documents plus decomposition/register references.
- Future implementation dependencies are marked `PENDING`; they do not block this setup/document-production run.
- The repository ID-format helper currently encodes older three-digit package/deliverable patterns, while this project uses `PKG-XX` and `DEL-XX-YY` per `docs/TYPES.md`; schema and enum validations are therefore treated as the deterministic dependency gates for this setup run.

## Run History

- 2026-04-30 - TASK+dependency-extract refreshed `Dependencies.csv` and `_DEPENDENCIES.md`; mode UPDATE; strictness CONSERVATIVE; decomposition path `docs/_Decomposition/SOFTWARE_DECOMP.md`; active rows 12; warnings: ID-format helper pattern mismatch noted.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 12 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 5 |
| PENDING | 7 |
| TBD | 0 |
| IN_PROGRESS | 0 |
| WAIVED | 0 |
| NOT_APPLICABLE | 0 |

## Downstream Handoff Notes

- Report generator work should consume the section requirements and fixture expectations without treating this setup artifact as implemented report code.
- Protected-content lint remains a future dependency; if unavailable, explicit human review must be recorded before public templates/examples are accepted.
- Exact report renderer API, audit manifest field names, provenance payload schema fields, and canonical professional notice wording remain `TBD`.

