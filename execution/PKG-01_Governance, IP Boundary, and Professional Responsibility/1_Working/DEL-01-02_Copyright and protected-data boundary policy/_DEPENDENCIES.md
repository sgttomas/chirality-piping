# Dependencies: DEL-01-02 Copyright and protected-data boundary policy

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
- **Schema Version:** v3.1
- **Summary:** 8 ACTIVE rows: 4 ANCHOR, 4 EXECUTION.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEL-01-02-DEP-001 | ANCHOR | UPSTREAM | OTHER | PKG-01 | ACTIVE |
| DEL-01-02-DEP-002 | ANCHOR | UPSTREAM | OTHER | SOW-003 | ACTIVE |
| DEL-01-02-DEP-003 | ANCHOR | UPSTREAM | OTHER | SOW-028 | ACTIVE |
| DEL-01-02-DEP-004 | ANCHOR | UPSTREAM | OTHER | OBJ-002 | ACTIVE |
| DEL-01-02-DEP-005 | EXECUTION | UPSTREAM | PREREQUISITE | docs/CONTRACT.md | ACTIVE |
| DEL-01-02-DEP-006 | EXECUTION | UPSTREAM | PREREQUISITE | docs/IP_AND_DATA_BOUNDARY.md | ACTIVE |
| DEL-01-02-DEP-007 | EXECUTION | DOWNSTREAM | HANDOVER | docs/IP_AND_DATA_BOUNDARY.md | ACTIVE |
| DEL-01-02-DEP-008 | EXECUTION | DOWNSTREAM | HANDOVER | Contribution review checklist | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- **Run:** 2026-04-30 TASK+dependency-extract
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Scope:** DEL-01-02
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- **Source docs:** AUTO; selected Datasheet.md as anchor-supporting source, then Specification.md, Guidance.md, Procedure.md as execution sources.
- **Warnings:** none. Parent anchor count = 1. Decomposition available. Final checklist path and reviewer role are unresolved `TBD` values, not extraction failures.
- **History:** 2026-04-30 - MODE=UPDATE, STRICTNESS=CONSERVATIVE, ACTIVE anchors=4, ACTIVE execution edges=4.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 8
- RETIRED: 0
- SatisfactionStatus:
  - NOT_APPLICABLE: 6
  - PENDING: 2

## Consumer Handoff Notes
- CONSUMER_CONTEXT=NONE. Downstream aggregation should treat repo-level policy/checklist handoffs as proposed future artifacts requiring human acceptance, not completed repository edits.
