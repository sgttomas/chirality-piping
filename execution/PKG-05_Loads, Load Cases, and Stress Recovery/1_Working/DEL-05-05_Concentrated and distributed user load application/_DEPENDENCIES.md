# Dependencies: DEL-05-05 Concentrated and distributed user load application

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
- **Summary:** 10 ACTIVE rows: 4 ANCHOR, 6 EXECUTION.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEP-05-05-001 | ANCHOR | UPSTREAM | OTHER | SOW-052 | ACTIVE |
| DEP-05-05-002 | ANCHOR | UPSTREAM | OTHER | SOW-013 | ACTIVE |
| DEP-05-05-003 | ANCHOR | UPSTREAM | OTHER | OBJ-003 | ACTIVE |
| DEP-05-05-004 | ANCHOR | UPSTREAM | OTHER | OBJ-012 | ACTIVE |
| DEP-05-05-005 | EXECUTION | UPSTREAM | CONSTRAINT | Invariant catalog | ACTIVE |
| DEP-05-05-006 | EXECUTION | UPSTREAM | CONSTRAINT | Architecture basis constraints | ACTIVE |
| DEP-05-05-007 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 | ACTIVE |
| DEP-05-05-008 | EXECUTION | UPSTREAM | INTERFACE | DEL-04-01 | ACTIVE |
| DEP-05-05-009 | EXECUTION | UPSTREAM | INTERFACE | DEL-05-01 | ACTIVE |
| DEP-05-05-010 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-05-03 | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- **Run mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Scope:** DEL-05-05
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- **Source docs:** AUTO; scanned deliverable-local four-document kit, `_REFERENCES.md`, existing `_DEPENDENCIES.md`, and decomposition/register references named in the sealed brief.
- **Warnings:** None. Parent anchor is present exactly once for primary SOW-052; SOW-013 is retained as a trace anchor because it is also listed in the sealed DEL-05-05 scope.
- **2026-04-30:** TASK+dependency-extract updated `Dependencies.csv` v3.1 with conservative anchor and execution edges.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 10
- RETIRED: 0
- SatisfactionStatus:
  - SATISFIED: 6
  - PENDING: 4

## Consumer Handoff Notes
- No explicit `CONSUMER_CONTEXT` was supplied. Downstream consumers should treat DEL-02-02, DEL-04-01, DEL-05-01, and DEL-05-03 execution edges as conservative setup proposals until future implementation briefs seal exact interfaces.
