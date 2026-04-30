# Dependencies: DEL-02-05 Project persistence and round-trip serialization

## Coordination (human-owned)
- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION.

## Upstream (I need these before I can proceed) - human-owned declarations
- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations
- Dependencies coordinated externally by humans.

## Extracted Dependency Register (populated by TASK+dependency-extract)
- **Status:** REFRESHED
- **Dependencies.csv:** `Dependencies.csv`
- **Register schema:** v3.1
- **Summary:** 8 ACTIVE extracted rows: 4 ANCHOR, 4 EXECUTION.

| DependencyID | Class | Direction | Type | Target | Status | Confidence |
|---|---|---|---|---|---|---|
| DEL-02-05-ANCHOR-001 | ANCHOR / IMPLEMENTS_NODE | UPSTREAM | OTHER | SOW-050 - Create/open/save/versioned project persistence | ACTIVE | HIGH |
| DEL-02-05-ANCHOR-002 | ANCHOR / TRACES_TO_REQUIREMENT | UPSTREAM | OTHER | SOW-041 - Machine-readable schemas | ACTIVE | HIGH |
| DEL-02-05-ANCHOR-003 | ANCHOR / TRACES_TO_REQUIREMENT | UPSTREAM | OTHER | OBJ-001 - Open auditable platform objective | ACTIVE | HIGH |
| DEL-02-05-ANCHOR-004 | ANCHOR / TRACES_TO_REQUIREMENT | UPSTREAM | OTHER | OBJ-012 - Deterministic reproducible model data flow objective | ACTIVE | HIGH |
| DEL-02-05-EXEC-001 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-01 - Canonical domain model schema | ACTIVE | HIGH |
| DEL-02-05-EXEC-002 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-04 - Software Architecture Runway persistence/hash basis | ACTIVE | HIGH |
| DEL-02-05-EXEC-003 | EXECUTION | UPSTREAM | CONSTRAINT | OI-011 - Physical container and migration details | ACTIVE | HIGH |
| DEL-02-05-EXEC-004 | EXECUTION | DOWNSTREAM | HANDOVER | DOWNSTREAM-REPORT-AUDIT - Reports and audit manifests | ACTIVE | MEDIUM |

## Run Notes and History (populated by TASK+dependency-extract)
- **Run timestamp:** 2026-04-30T09:46:29-0600
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Scope:** DEL-02-05
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- **Source docs:** AUTO; scanned deliverable-local `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_CONTEXT.md`, `_REFERENCES.md`, existing `_DEPENDENCIES.md`, and the declared decomposition path.
- **Anchor doc:** `_CONTEXT.md` plus register/decomposition evidence.
- **Execution doc order:** `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`.
- **Defaults used:** `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`, `CONSUMER_CONTEXT=NONE`.
- **Warnings:** none. Parent anchor count is 1; decomposition path was available; no ambiguous parent anchor found.
- **Run history:** 2026-04-30T09:46:29-0600 - TASK+dependency-extract refreshed `Dependencies.csv` v3.1 in UPDATE/CONSERVATIVE mode; ACTIVE counts ANCHOR=4, EXECUTION=4; warnings=none.

## Lifecycle Summary (populated by TASK+dependency-extract)
- **ACTIVE rows:** 8
- **RETIRED rows:** 0
- **DependencyClass counts:** ANCHOR=4; EXECUTION=4
- **SatisfactionStatus counts:** SATISFIED=4; PENDING=4
- **RequiredMaturity:** SEMANTIC_READY for all ACTIVE rows
- **ProposedMaturity:** SEMANTIC_READY for all ACTIVE rows

## Consumer Handoff Notes
- `CONSUMER_CONTEXT=NONE`; no consumer-specific extension columns were added.
- Downstream aggregation should preserve unresolved targets as `UNKNOWN`/`TBD` unless a later human or architecture ruling identifies exact deliverable IDs.
