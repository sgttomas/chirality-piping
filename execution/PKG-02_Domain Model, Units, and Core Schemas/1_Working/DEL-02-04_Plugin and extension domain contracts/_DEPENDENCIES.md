# Dependencies: DEL-02-04 Plugin and extension domain contracts

## Coordination (human-owned)
- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION.

## Upstream (I need these before I can proceed) - human-owned declarations
- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations
- Dependencies coordinated externally by humans.

## Extracted Dependency Register (populated by TASK+dependency-extract)
- **Status:** UPDATED
- **Dependencies.csv:** `Dependencies.csv` v3.1
- **Summary:** 5 ACTIVE rows: 2 ANCHOR, 3 EXECUTION. Parent anchor count: 1. Trace anchors: 1. Execution constraints: 3.

| DependencyID | Class | Direction | Type | Target | Status | Satisfaction |
|---|---|---|---|---|---|---|
| DEL-02-04-DEP-001 | ANCHOR / IMPLEMENTS_NODE | UPSTREAM | OTHER | SOW-038 - Extensible architecture without governance bypass | ACTIVE | SATISFIED |
| DEL-02-04-DEP-002 | ANCHOR / TRACES_TO_REQUIREMENT | UPSTREAM | OTHER | OBJ-009 - Enable interoperability and extensibility while preserving governance boundaries | ACTIVE | SATISFIED |
| DEL-02-04-DEP-003 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-07 - API boundary and adapter contract map architecture basis | ACTIVE | PENDING |
| DEL-02-04-DEP-004 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-02 - Layer and module responsibility architecture basis | ACTIVE | PENDING |
| DEL-02-04-DEP-005 | EXECUTION | UPSTREAM | CONSTRAINT | CONTRACT - OpenPipeStress invariant catalog | ACTIVE | PENDING |

## Run Notes and History (populated by TASK+dependency-extract)
- 2026-04-30T09:46:44-0600 - MODE=UPDATE; STRICTNESS=CONSERVATIVE; SCOPE=DEL-02-04; RUN_ROOT=`/Users/ryan/ai-env/projects/chirality-piping/execution`; DECOMPOSITION_PATH=`/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`.
- SOURCE_DOCS=AUTO. Anchor source selected from `_CONTEXT.md`, register rows, and decomposition rows. Execution sources selected from `Specification.md`, `Procedure.md`, `_CONTEXT.md`, `docs/_Decomposition/SOFTWARE_DECOMP.md`, and `docs/CONTRACT.md`.
- Existing human-owned declarations were preserved unchanged. No declared dependency rows were present in `Dependencies.csv` because the file did not exist before this run.
- Conservative extraction emitted only explicit anchors/constraints supported by DEL-02-04 context, registers, decomposition, contract invariants, and production documents.
- No cross-deliverable execution prerequisites were emitted because the local sources do not explicitly require another deliverable before this setup artifact can proceed.
- Integrity warnings: none. Parent anchor count is exactly one. Decomposition path was available.
- QA note: dependency schema validation passed and enum validation passed. `tools/validation/validate_id_format.sh` rejected current project IDs such as `DEL-02-04` because that helper still expects legacy three-digit package/deliverable formats (`DEL-000-00`, `PKG-000`); the accepted SOFTWARE_DECOMP/register IDs use two-digit package/deliverable form.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE rows: 5
- RETIRED rows: 0
- By class: ANCHOR=2; EXECUTION=3
- By satisfaction: SATISFIED=2; PENDING=3; TBD=0; IN_PROGRESS=0; WAIVED=0; NOT_APPLICABLE=0

## Consumer Handoff Notes
- CONSUMER_CONTEXT=NONE. Downstream aggregation/reconciliation may consume `Dependencies.csv`; no estimating-specific extension columns were added.
