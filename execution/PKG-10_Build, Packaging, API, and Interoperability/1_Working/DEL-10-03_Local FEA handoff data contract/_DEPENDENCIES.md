# Dependencies: DEL-10-03 Local FEA handoff data contract

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
- **Summary:** 7 ACTIVE rows: 3 ANCHOR, 4 EXECUTION.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEP-10-03-001 | ANCHOR | UPSTREAM | OTHER | SOW-031 | ACTIVE |
| DEP-10-03-002 | ANCHOR | UPSTREAM | OTHER | SOW-049 | ACTIVE |
| DEP-10-03-003 | ANCHOR | UPSTREAM | OTHER | OBJ-009 | ACTIVE |
| DEP-10-03-004 | EXECUTION | UPSTREAM | CONSTRAINT | Invariant catalog | ACTIVE |
| DEP-10-03-005 | EXECUTION | UPSTREAM | CONSTRAINT | Architecture basis constraints | ACTIVE |
| DEP-10-03-006 | EXECUTION | UPSTREAM | INTERFACE | DEL-10-01 | ACTIVE |
| DEP-10-03-007 | EXECUTION | DOWNSTREAM | INTERFACE | DEL-10-02 | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- **Run mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Scope:** DEL-10-03
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- **Source docs:** AUTO; scanned deliverable-local four-document kit, `_REFERENCES.md`, existing `_DEPENDENCIES.md`, and decomposition/register references named in the sealed brief.
- **Warnings:** ID format helper mismatch: `tools/validation/validate_id_format.sh` expects three-digit IDs (`DEL-000-00`, `PKG-000`, `SOW-0000`), while SOFTWARE_DECOMP revision 0.4 and registers use `DEL-10-03`, `PKG-10`, and `SOW-031`/`SOW-049`. Schema and enum checks pass; canonical project IDs are preserved. Parent anchor is present exactly once for primary SOW-031. The DEL-10-02 downstream interface row is a conservative proposal and not a scheduling dependency.
- **2026-04-30:** TASK+dependency-extract updated `Dependencies.csv` v3.1 with conservative anchor and execution edges.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 7
- RETIRED: 0
- SatisfactionStatus:
  - SATISFIED: 6
  - PENDING: 1

## Consumer Handoff Notes
- No explicit `CONSUMER_CONTEXT` was supplied. Downstream consumers should treat the DEL-10-02 row as a conservative interface proposal, not as a committed schedule constraint.
