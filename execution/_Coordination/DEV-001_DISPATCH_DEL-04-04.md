---
doc_id: DEV-001-DISPATCH-DEL-04-04
doc_kind: coordination.dispatch_brief
status: launched_single_bounded_item
created: 2026-05-01
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-04-04
package_id: PKG-04
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-04-04

## Dispatch Decision

The human project authority authorized one bounded DAG-ready item:

- `DEL-04-04 - Nonlinear support active-set solver`

The scope is deliberately constrained to a nonlinear support active-set
foundation: active/inactive state modeling, gap/lift-off/one-way/friction
status decisions, deterministic iteration convergence reporting, and tests.
This does not authorize broad solver integration, sparse-solver selection,
load-case work, lifecycle transitions, dependency-register edits, candidate-edge
promotion, or work on any other deliverable.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-04-04` |
| PackageID | `PKG-04` |
| Name | Nonlinear support active-set solver |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope items | `SOW-012` |
| Objectives | `OBJ-003` |
| Context envelope | `L` |
| Deliverable path | `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver` |
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
| `DEL-04-03` | `SOLVER_PREDECESSOR` | `COMMITTED` evidence `d227a27` |
| `DEL-04-01` | `SOLVER_PREDECESSOR` | `COMMITTED` evidence `1506cc0` |
| `DEL-04-06` | `DIAGNOSTICS_CONTRACT` | `COMMITTED` evidence `fdb0252` |

Current implementation-readiness queue state:

- `DEL-04-04` is `UNBLOCKED`.
- `DEL-04-04` has `MISSING_EVIDENCE` for its own implementation.
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

Resolved baseline to preserve: nonlinear support behavior extends the linear
support mechanics boundary and reports convergence/nonconvergence through the
existing diagnostics contract. It remains mechanics-only and code-neutral.

Remaining implementation-level TBDs are not resolved by this dispatch:
canonical calculation unit basis, conversion constants, final support coordinate
convention, rigid-restraint numerical method, constraint-elimination or penalty
strategy, sparse-solver integration, production residual/tolerance policy, and
final result-envelope integration.

## Explicit Write Scope

Authorized product and evidence targets for this bounded implementation:

- `core/solver/nonlinear_supports/.gitignore`
- `core/solver/nonlinear_supports/Cargo.toml`
- `core/solver/nonlinear_supports/README.md`
- `core/solver/nonlinear_supports/src/lib.rs`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-04-04.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch. In particular, do not edit
`DAG-001`, candidate edges, deliverable-local `Dependencies.csv`,
`DEV-001_IMPLEMENTATION_EVIDENCE.csv`, or `DEV-001_BLOCKER_QUEUE.*` during this
implementation unless separately authorized.

## Acceptance Criteria

This bounded item is acceptable for implementation closeout when:

- The nonlinear support module defines explicit mechanics-only inputs for
  one-way restraints, gap/lift-off checks, and friction-limited supports without
  adding protected standards data, public catalog defaults, or rule-pack checks.
- The module consumes the linear-support DOF concepts or otherwise preserves
  the `DEL-04-03` six-DOF boundary without changing frame-kernel behavior.
- Active-set evaluation produces deterministic active/inactive/sliding/sticking
  state transitions from supplied trial displacement/reaction data.
- Iteration state records active-set changes, residuals, convergence,
  nonconvergence, and iteration limits without claiming release-quality
  tolerance policy.
- Nonconvergence is reported through the existing diagnostics contract.
- Tests cover one-way activation/deactivation, gap/lift-off behavior,
  friction stick/slip classification, convergence and nonconvergence reporting,
  invalid numeric inputs, and deterministic iteration state.
- No lifecycle transition, dependency-register edit, candidate-edge change,
  blocker-queue refresh, protected standards data, proprietary engineering
  value, or professional/code-compliance claim occurs unless separately
  authorized.
