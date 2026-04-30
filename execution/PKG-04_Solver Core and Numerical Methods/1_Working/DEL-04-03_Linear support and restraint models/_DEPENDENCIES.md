# Dependencies: DEL-04-03 Linear support and restraint models

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
- **Summary:** 7 ACTIVE rows: 2 ANCHOR, 5 EXECUTION.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEP-04-03-001 | ANCHOR | UPSTREAM | OTHER | SOW-011 | ACTIVE |
| DEP-04-03-002 | ANCHOR | UPSTREAM | OTHER | OBJ-003 | ACTIVE |
| DEP-04-03-003 | EXECUTION | UPSTREAM | CONSTRAINT | Invariant catalog | ACTIVE |
| DEP-04-03-004 | EXECUTION | UPSTREAM | CONSTRAINT | Architecture basis constraints | ACTIVE |
| DEP-04-03-005 | EXECUTION | UPSTREAM | INTERFACE | DEL-04-01 | ACTIVE |
| DEP-04-03-006 | EXECUTION | DOWNSTREAM | INTERFACE | DEL-04-04 | ACTIVE |
| DEP-04-03-007 | EXECUTION | DOWNSTREAM | INTERFACE | DEL-04-06 | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- **Run mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Scope:** DEL-04-03
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- **Source docs:** AUTO; scanned deliverable-local four-document kit, `_REFERENCES.md`, existing `_DEPENDENCIES.md`, and decomposition/register references named in the sealed brief.
- **Warnings:** None. Parent anchor is present exactly once.
- **2026-04-30:** TASK+dependency-extract updated `Dependencies.csv` v3.1 with conservative anchor and execution edges.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 7
- RETIRED: 0
- SatisfactionStatus:
  - SATISFIED: 4
  - PENDING: 3

## Consumer Handoff Notes
- No explicit `CONSUMER_CONTEXT` was supplied. Downstream consumers should treat DEL-04-01, DEL-04-04, and DEL-04-06 edges as conservative information-flow/interface proposals, not scheduling dependencies.
