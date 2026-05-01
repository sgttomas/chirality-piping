---
doc_id: DEV-001-DISPATCH-DEL-04-03
doc_kind: coordination.dispatch_brief
status: prepared_not_launched
created: 2026-05-01
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-04-03
package_id: PKG-04
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-04-03

## Dispatch Decision

The human project authority authorized preparation of a sealed dispatch brief
for `DEL-04-03 - Linear support and restraint models`.

This brief preparation is not implementation and is not broad fan-out. It does
not authorize lifecycle transitions, candidate-edge promotion,
dependency-register edits, blocker-queue refresh, or work on any other
deliverable. Implementation may begin only after a separate human gate accepts
this brief and authorizes the bounded item.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-04-03` |
| PackageID | `PKG-04` |
| Name | Linear support and restraint models |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope items | `SOW-011` |
| Objectives | `OBJ-003` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-03_Linear support and restraint models` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| Target | Dependency type | Implementation-readiness satisfaction |
|---|---|---|
| `DEL-00-01` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-02` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-03` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-06` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-08` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-04-01` | `SOLVER_PREDECESSOR` | `COMMITTED` evidence `1506cc0` |
| `DEL-02-01` | `DOMAIN_MODEL` | `COMMITTED` evidence `7b256f3` |
| `DEL-02-02` | `UNIT_CONTRACT` | `COMMITTED` evidence `a458cba` |

Current implementation-readiness queue state:

- `DEL-04-03` is `UNBLOCKED`.
- `DEL-04-03` has `MISSING_EVIDENCE` for its own implementation.
- Candidate edges are excluded.

## Applicable Architecture Basis

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module responsibilities and no-bypass dependency rules.
- `AB-00-03` - schema-first command/query/job/result-envelope service
  boundaries.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no
  certification/compliance claims.
- `AB-00-08` - layered validation, provenance, protected-content, and
  acceptance gates.

Resolved baseline to preserve: linear support and restraint models are
mechanics-boundary inputs to the 3D frame solver. They must use explicit
unit-checked user/model data, expose missing or invalid data as diagnostics, and
remain separate from nonlinear active-set support behavior, rule-pack checks,
load-case algebra, and professional/code-compliance claims.

Remaining implementation-level TBDs are not resolved by this dispatch:
canonical calculation unit basis, conversion constants, coordinate-frame
convention for support directions, rigid-restraint numerical method,
constraint-elimination or penalty strategy, sparse-solver integration, and
final result-envelope integration.

## Explicit Write Scope

Authorized product and evidence targets for the future bounded implementation:

- `core/solver/linear_supports/.gitignore`
- `core/solver/linear_supports/Cargo.toml`
- `core/solver/linear_supports/README.md`
- `core/solver/linear_supports/src/lib.rs`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-03_Linear support and restraint models/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-04-03.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch. In particular, do not edit
`DAG-001`, candidate edges, deliverable-local `Dependencies.csv`, or
`DEV-001_BLOCKER_QUEUE.*` during implementation unless separately authorized.

## Acceptance Criteria

This bounded item is acceptable for implementation closeout when:

- The linear support module stays mechanics-only and does not implement
  nonlinear gap, lift-off, one-way, friction, or active-set behavior assigned to
  `DEL-04-04`.
- The model covers anchors, guides, line stops, vertical supports, springs, and
  imposed displacement boundary data as explicit support-family records or
  equivalent typed mechanics inputs.
- Support data maps to the six frame-kernel node degrees of freedom without
  changing frame-kernel behavior outside the approved write scope.
- Required stiffness, direction, target, and imposed-displacement data are
  explicit; missing solve-required data produces deterministic findings rather
  than silent defaults.
- Unit-bearing support quantities retain dimension intent for translational
  stiffness, rotational stiffness, displacement, and rotation inputs without
  choosing unresolved conversion constants.
- Tests cover support family validation, missing-data diagnostics, degree-of-
  freedom mapping, imposed displacement representation, and deterministic
  mechanics boundary preparation.
- No protected standards data, proprietary values, public support/catalog
  defaults, stress code checks, lifecycle transition, dependency-register edit,
  candidate-edge change, or blocker-queue refresh occurs unless separately
  authorized.
