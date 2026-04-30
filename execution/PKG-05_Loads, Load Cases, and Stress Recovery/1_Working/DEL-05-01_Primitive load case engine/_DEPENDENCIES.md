# Dependencies: DEL-05-01 Primitive load case engine

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
- **Schema:** v3.1
- **Summary:** 6 ACTIVE rows: 2 ANCHOR, 4 EXECUTION.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEP-05-01-001 | ANCHOR | UPSTREAM | OTHER | SOW-013 | ACTIVE |
| DEP-05-01-002 | ANCHOR | UPSTREAM | OTHER | OBJ-003 | ACTIVE |
| DEP-05-01-003 | EXECUTION | UPSTREAM | CONSTRAINT | Invariant catalog | ACTIVE |
| DEP-05-01-004 | EXECUTION | UPSTREAM | CONSTRAINT | Architecture basis constraints | ACTIVE |
| DEP-05-01-005 | EXECUTION | DOWNSTREAM | INTERFACE | DEL-05-02 | ACTIVE |
| DEP-05-01-006 | EXECUTION | DOWNSTREAM | INTERFACE | DEL-05-03 | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- **Run mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Scope:** DEL-05-01
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- **Source docs:** AUTO; scanned deliverable-local four-document kit, `_REFERENCES.md`, existing `_DEPENDENCIES.md`, and decomposition/register references named in the sealed brief.
- **Warnings:** ID format helper mismatch: `tools/validation/validate_id_format.sh` expects three-digit IDs (`DEL-000-00`, `PKG-000`), while SOFTWARE_DECOMP revision 0.4 and registers use `DEL-05-01` / `PKG-05`. Schema and enum checks pass; canonical project IDs are preserved. Parent anchor is present exactly once for primary SOW-013. Downstream interface rows are conservative proposals and not scheduling dependencies.
- **2026-04-30:** TASK+dependency-extract updated `Dependencies.csv` v3.1 with conservative anchor and execution edges.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 6
- RETIRED: 0
- SatisfactionStatus:
  - SATISFIED: 4
  - PENDING: 2

## Consumer Handoff Notes
- No explicit `CONSUMER_CONTEXT` was supplied. Downstream consumers should treat DEL-05-02 and DEL-05-03 rows as conservative interface proposals, not as committed schedule constraints.
