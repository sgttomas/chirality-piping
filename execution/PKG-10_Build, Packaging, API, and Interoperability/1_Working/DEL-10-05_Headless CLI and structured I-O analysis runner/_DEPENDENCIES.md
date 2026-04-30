# Dependencies: DEL-10-05 Headless CLI and structured I/O analysis runner

## Coordination (human-owned)
- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION.

## Upstream (I need these before I can proceed) - human-owned declarations
- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations
- Dependencies coordinated externally by humans.

## Extracted Dependency Register
- **Status:** UPDATED
- **Dependencies.csv:** `Dependencies.csv`
- **Register Schema:** v3.1
- **ACTIVE rows:** 14
- **ANCHOR rows:** 6
- **EXECUTION rows:** 8

| DependencyID | Class | Direction | Target | Type | Status |
|---|---|---|---|---|---|
| DEP-10-05-001 | ANCHOR | UPSTREAM | PKG-10 | OTHER | ACTIVE |
| DEP-10-05-002 | ANCHOR | UPSTREAM | SOW-054 | OTHER | ACTIVE |
| DEP-10-05-003 | ANCHOR | UPSTREAM | SOW-032 | OTHER | ACTIVE |
| DEP-10-05-004 | ANCHOR | UPSTREAM | OBJ-008 | OTHER | ACTIVE |
| DEP-10-05-005 | ANCHOR | UPSTREAM | OBJ-009 | OTHER | ACTIVE |
| DEP-10-05-006 | ANCHOR | UPSTREAM | OBJ-012 | OTHER | ACTIVE |
| DEP-10-05-007 | EXECUTION | UPSTREAM | DEL-00-03 | INTERFACE | ACTIVE |
| DEP-10-05-008 | EXECUTION | UPSTREAM | DEL-00-06 | INTERFACE | ACTIVE |
| DEP-10-05-009 | EXECUTION | UPSTREAM | DEL-00-08 | CONSTRAINT | ACTIVE |
| DEP-10-05-010 | EXECUTION | UPSTREAM | DEL-08-04 | INTERFACE | ACTIVE |
| DEP-10-05-011 | EXECUTION | UPSTREAM | DEL-10-04 | CONSTRAINT | ACTIVE |
| DEP-10-05-012 | EXECUTION | UPSTREAM | DEL-02-01 | INTERFACE | ACTIVE |
| DEP-10-05-013 | EXECUTION | UPSTREAM | DEL-02-02 | INTERFACE | ACTIVE |
| DEP-10-05-014 | EXECUTION | UPSTREAM | DEL-04-06 | HANDOVER | ACTIVE |

## Run Notes
- **TaskSkill:** dependency-extract
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** NONE
- **Source Docs:** AUTO; scanned `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, and decomposition/register pointers from `_CONTEXT.md`.
- **Anchor Doc:** `_CONTEXT.md`
- **Execution Docs:** `Specification.md`, `Procedure.md`, `Datasheet.md`, `Guidance.md`
- **Decomposition Path:** `docs/_Decomposition/SOFTWARE_DECOMP.md`
- **Warnings:** none
- **Notes:** Dependencies are information-flow and constraint records only. They do not create schedule precedence or authorize edits outside DEL-10-05. Targets with implementation details still marked TBD in the source documents remain pending.

## Run History
- 2026-04-30 11:05 MDT: TASK+dependency-extract MODE=UPDATE STRICTNESS=CONSERVATIVE; decomposition present; ACTIVE rows=14; warnings=none; schema validation passed.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 14 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| NOT_APPLICABLE | 6 |
| PENDING | 8 |

## Downstream Handoff Notes
- Downstream aggregation/reconciliation should treat DEL-00-03, DEL-00-06, DEL-00-08, DEL-08-04, DEL-10-04, DEL-02-01, DEL-02-02, and DEL-04-06 rows as setup-stage execution dependencies requiring review before implementation planning.
- Exact CLI commands, schema fields, public transport, CI provider, release matrix, and package/container choices remain TBD and should not be inferred from this dependency register.

