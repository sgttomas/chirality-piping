# Dependencies: DEL-04-06 Solver diagnostics and singularity detection

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
- **Summary:** 8 ACTIVE rows: 5 ANCHOR, 3 EXECUTION.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEP-04-06-001 | ANCHOR | UPSTREAM | OTHER | SOW-053 | ACTIVE |
| DEP-04-06-002 | ANCHOR | UPSTREAM | OTHER | SOW-035 | ACTIVE |
| DEP-04-06-003 | ANCHOR | UPSTREAM | OTHER | OBJ-003 | ACTIVE |
| DEP-04-06-004 | ANCHOR | UPSTREAM | OTHER | OBJ-008 | ACTIVE |
| DEP-04-06-005 | ANCHOR | UPSTREAM | OTHER | OBJ-012 | ACTIVE |
| DEP-04-06-006 | EXECUTION | UPSTREAM | CONSTRAINT | Invariant catalog | ACTIVE |
| DEP-04-06-007 | EXECUTION | UPSTREAM | CONSTRAINT | Architecture basis constraints | ACTIVE |
| DEP-04-06-008 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-10-05 | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- **Run mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Scope:** DEL-04-06
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- **Source docs:** AUTO; scanned deliverable-local four-document kit, `_REFERENCES.md`, existing `_DEPENDENCIES.md`, and decomposition/register references named in the sealed brief.
- **Warnings:** None. Parent anchor is present exactly once for primary SOW-053; SOW-035 is retained as a trace anchor because it is also listed in the sealed DEL-04-06 scope.
- **2026-04-30:** TASK+dependency-extract updated `Dependencies.csv` v3.1 with conservative anchor and execution edges.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 8
- RETIRED: 0
- SatisfactionStatus:
  - SATISFIED: 7
  - PENDING: 1

## Consumer Handoff Notes
- No explicit `CONSUMER_CONTEXT` was supplied. Downstream consumers should treat the DEL-10-05 edge as a conservative proposal for structured diagnostic handoff, not as a scheduling dependency.
