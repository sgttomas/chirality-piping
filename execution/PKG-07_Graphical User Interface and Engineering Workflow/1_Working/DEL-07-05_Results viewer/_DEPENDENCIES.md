# Dependencies: DEL-07-05 Results viewer

## Coordination (human-owned)
- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION. Extracted rows below are conservative information-flow evidence and proposals, not a full schedule graph.

## Upstream (I need these before I can proceed) - human-owned declarations
- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations
- Dependencies coordinated externally by humans.

## Extracted Dependency Register (populated by TASK+dependency-extract)
- **Status:** UPDATED
- **Dependencies.csv:** `Dependencies.csv`
- **Schema:** v3.1
- **Summary:** 9 ACTIVE rows: 3 ANCHOR, 6 EXECUTION.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEP-07-05-001 | ANCHOR | UPSTREAM | OTHER | SOW-023 | ACTIVE |
| DEP-07-05-002 | ANCHOR | UPSTREAM | OTHER | OBJ-006 | ACTIVE |
| DEP-07-05-003 | ANCHOR | UPSTREAM | OTHER | OBJ-007 | ACTIVE |
| DEP-07-05-004 | EXECUTION | UPSTREAM | CONSTRAINT | Invariant catalog | ACTIVE |
| DEP-07-05-005 | EXECUTION | UPSTREAM | CONSTRAINT | Technical specification | ACTIVE |
| DEP-07-05-006 | EXECUTION | UPSTREAM | INTERFACE | DEL-00-06 | ACTIVE |
| DEP-07-05-007 | EXECUTION | UPSTREAM | INTERFACE | DEL-05-03 | ACTIVE |
| DEP-07-05-008 | EXECUTION | UPSTREAM | INTERFACE | DEL-06-03 | ACTIVE |
| DEP-07-05-009 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-08-04 | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- **Run mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Scope:** DEL-07-05
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- **Source docs:** AUTO; scanned deliverable-local four-document kit, `_REFERENCES.md`, existing `_DEPENDENCIES.md`, and decomposition/register references named in the sealed brief.
- **Warnings:** None. Parent anchor is present exactly once for primary SOW-023. Execution edges to DEL-05-03, DEL-06-03, and DEL-08-04 are conservative implementation-interface proposals, not human-declared blockers.
- **2026-04-30:** TASK+dependency-extract updated `Dependencies.csv` v3.1 with conservative anchor and execution edges.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 9
- RETIRED: 0
- SatisfactionStatus:
  - SATISFIED: 6
  - PENDING: 3

## Consumer Handoff Notes
- No explicit `CONSUMER_CONTEXT` was supplied.
- Downstream consumers should treat DEL-05-03, DEL-06-03, and DEL-08-04 rows as interface proposals for implementation planning, not as scheduling decisions.
- No protected standards data, code-specific thresholds, proprietary values, or professional-approval claims are represented by these dependencies.
