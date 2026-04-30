# Dependencies: DEL-06-02 Sandboxed unit-aware expression evaluator

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
| DEP-06-02-001 | ANCHOR | UPSTREAM | OTHER | SOW-045 | ACTIVE |
| DEP-06-02-002 | ANCHOR | UPSTREAM | OTHER | OBJ-005 | ACTIVE |
| DEP-06-02-003 | EXECUTION | UPSTREAM | CONSTRAINT | Invariant catalog | ACTIVE |
| DEP-06-02-004 | EXECUTION | UPSTREAM | CONSTRAINT | Architecture basis constraints | ACTIVE |
| DEP-06-02-005 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-02 | ACTIVE |
| DEP-06-02-006 | EXECUTION | UPSTREAM | INTERFACE | DEL-06-01 | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- **Run mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Scope:** DEL-06-02
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- **Source docs:** AUTO; scanned deliverable-local four-document kit, `_REFERENCES.md`, existing `_DEPENDENCIES.md`, and decomposition/register references named in the sealed brief.
- **Warnings:** None. Parent anchor is present exactly once for SOW-045. Interface rows are conservative proposals and retain `SatisfactionStatus=PENDING`.
- **2026-04-30:** TASK+dependency-extract updated `Dependencies.csv` v3.1 with conservative anchor and execution edges.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 6
- RETIRED: 0
- SatisfactionStatus:
  - SATISFIED: 4
  - PENDING: 2

## Consumer Handoff Notes
- No explicit `CONSUMER_CONTEXT` was supplied. Downstream consumers should treat DEL-02-02 and DEL-06-01 rows as conservative interface proposals, not scheduling commitments.
