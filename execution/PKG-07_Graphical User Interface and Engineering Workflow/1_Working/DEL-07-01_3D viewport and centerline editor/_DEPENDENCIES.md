# Dependencies: DEL-07-01 3D viewport and centerline editor

## Coordination (human-owned)
- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION. Dependency DAG/blocker computation remains disabled unless the human later approves a tracked DAG.

## Upstream (I need these before I can proceed) - human-owned declarations
- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations
- Dependencies coordinated externally by humans.

## Extracted Dependency Register

- **Status:** REFRESHED
- **Dependencies.csv:** `Dependencies.csv`
- **Register schema:** v3.1
- **ACTIVE rows:** 9
- **RETIRED rows:** 0

| DependencyID | Class | Direction | Type | TargetType | Target | Satisfaction | Evidence |
|---|---|---|---|---|---|---|---|
| DEP-007-01-001 | ANCHOR | UPSTREAM | OTHER | WBS_NODE | DEL-07-01 3D viewport and centerline editor | SATISFIED | `_CONTEXT.md` Package Reference |
| DEP-007-01-002 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | SOW-020 3D centerline modeler scope | SATISFIED | `_CONTEXT.md` Scope Coverage |
| DEP-007-01-003 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | OBJ-006 Visible GUI workflow objective | SATISFIED | `_CONTEXT.md` Objective Support |
| DEP-007-01-004 | EXECUTION | UPSTREAM | CONSTRAINT | DOCUMENT | OPS-PRD GUI requirements | SATISFIED | `Specification.md` Requirements |
| DEP-007-01-005 | EXECUTION | UPSTREAM | CONSTRAINT | DOCUMENT | OPS-CONTRACT Invariant catalog | SATISFIED | `Specification.md` Requirements |
| DEP-007-01-006 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-00-05 GUI state and interaction architecture | SATISFIED | `_CONTEXT.md` Architecture Basis Injection |
| DEP-007-01-007 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-00-03 Application service command-query-job model | SATISFIED | `_CONTEXT.md` Architecture Basis Injection |
| DEP-007-01-008 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-00-06 Diagnostics warning and result-envelope contract | SATISFIED | `Specification.md` Requirements |
| DEP-007-01-009 | EXECUTION | DOWNSTREAM | INTERFACE | DELIVERABLE | DEL-07-02 Model tree and property inspector | PENDING | `Datasheet.md` Construction |

## Run Notes

- **Skill:** TASK+dependency-extract
- **Generated:** 2026-04-30
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** NONE
- **Decomposition path:** `docs/_Decomposition/SOFTWARE_DECOMP.md`
- **Source docs scanned:** `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`
- **Warnings:** None
- **Parent anchor check:** PASS. One ACTIVE `IMPLEMENTS_NODE` row exists.
- **Protected data boundary:** No protected standards text, tables, copied formulas, code-specific values, proprietary component data, or commercial examples were introduced.
- **Professional boundary:** Dependencies preserve decision-support language only; no certification, sealing, approval, authentication, or code-compliance claim is made.
- **TBD handling:** Exact GUI dependency versions and component/state-management libraries remain `TBD` by design.

## Run History

| Date | Mode | Strictness | Decomposition | ACTIVE rows | Warnings |
|---|---|---|---|---:|---|
| 2026-04-30 | UPDATE | CONSERVATIVE | `docs/_Decomposition/SOFTWARE_DECOMP.md` | 9 | None |

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 9 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 8 |
| PENDING | 1 |

| DependencyClass | Count |
|---|---:|
| ANCHOR | 3 |
| EXECUTION | 6 |

## Downstream Handoff Notes

- `DEL-07-02` has a downstream interface to consume stable selection and model-entity identity from the future viewport/editor slice.
- This dependency is not a schedule blocker in the current `NOT_TRACKED` coordination mode.
