# Dependencies: DEL-03-02 Pipe section and component library schema

## Coordination (human-owned)
- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION.

## Upstream (I need these before I can proceed) - human-owned declarations
- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations
- Dependencies coordinated externally by humans.

## Extracted Dependency Register (populated by TASK+dependency-extract)
- **Status:** UPDATED
- **Dependencies.csv:** `Dependencies.csv`
- **Summary:** 5 ACTIVE extracted rows: 2 ANCHOR and 3 EXECUTION.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEL-03-02-DEP-001 | ANCHOR | UPSTREAM | OTHER | SOW-018 | ACTIVE |
| DEL-03-02-DEP-002 | ANCHOR | UPSTREAM | OTHER | OBJ-004 | ACTIVE |
| DEL-03-02-DEP-003 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-04 | ACTIVE |
| DEL-03-02-DEP-004 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-07 | ACTIVE |
| DEL-03-02-DEP-005 | EXECUTION | UPSTREAM | CONSTRAINT | CONTRACT | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- **Source docs:** AUTO; read local four-document kit, `_CONTEXT.md`, `_REFERENCES.md`, and existing `_DEPENDENCIES.md`.
- **Anchor doc:** AUTO; `_CONTEXT.md` plus register/decomposition rows.
- **Execution docs:** AUTO; `Specification.md`, `Procedure.md`, `Datasheet.md`, `Guidance.md`.
- **Warnings:** parent anchor count is 1 and decomposition path was available. `tools/validation/validate_id_format.sh` is legacy-format strict and rejects current SOFTWARE IDs `DEL-03-02`/`PKG-03` because it expects three-digit identifiers; IDs are preserved as assigned by `docs/TYPES.md` and SOFTWARE_DECOMP.
- 2026-04-30 - TASK+dependency-extract UPDATE CONSERVATIVE wrote Dependencies.csv v3.1 and refreshed this index.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE rows: 5
- RETIRED rows: 0
- Closure status: SATISFIED 2; PENDING 3; TBD 0.

## Consumer Handoff Notes
- No specialized consumer context was requested. Downstream aggregation/reconciliation may consume `Dependencies.csv` as a deliverable-local v3.1 register.
