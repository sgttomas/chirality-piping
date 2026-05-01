---
doc_id: DEV-001-DISPATCH-DEL-04-05
doc_kind: coordination.dispatch_brief
status: launched_single_bounded_item
created: 2026-05-01
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-04-05
package_id: PKG-04
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-04-05

## Dispatch Decision

The human project authority authorized implementing `DEL-04-005`. Project
registers identify the bounded item as `DEL-04-05 - Sparse solver performance
harness`; this brief uses the registered ID.

This dispatch is not broad fan-out. It does not authorize lifecycle
transitions, candidate-edge promotion, dependency-register edits, blocker queue
refresh, or work on any other deliverable.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-04-05` |
| PackageID | `PKG-04` |
| Name | Sparse solver performance harness |
| Type | `TEST_SUITE` |
| Scope items | `SOW-035` |
| Objectives | `OBJ-003`, `OBJ-008` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-05_Sparse solver performance harness` |
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
| `DEL-04-06` | `DIAGNOSTICS_CONTRACT` | `COMMITTED` evidence `fdb0252` |

## Applicable Architecture Basis

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module responsibilities and no-bypass dependency rules.
- `AB-00-03` - schema-first command/query/job/result-envelope service
  boundaries.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no
  certification/compliance claims.
- `AB-00-08` - layered validation, provenance, protected-content, and
  acceptance gates.

Resolved baseline to preserve: the harness is an observer/regression surface
around mechanics-solver behavior. It may record sparse-solver, conditioning,
and reproducibility evidence, but it must not alter solver behavior, bundle
protected/proprietary benchmark data, or claim engineering approval,
certification, sealing, or code compliance.

Remaining implementation-level TBDs are not resolved by this dispatch:
accepted sparse solver library, release timing/memory thresholds,
hardware-normalized methodology, practical model-size taxonomy, and final CI
gate policy.

## Explicit Write Scope

Authorized product and evidence targets for this bounded item:

- `core/solver/performance_harness/Cargo.toml`
- `core/solver/performance_harness/README.md`
- `core/solver/performance_harness/src/lib.rs`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-05_Sparse solver performance harness/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-04-05.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch.

## Acceptance Criteria

This bounded item is acceptable for closeout when:

- The harness remains separate from solver logic and does not modify production
  solver behavior.
- Invented/public fixtures can be run repeatedly through the current solve
  boundary with deterministic regression records.
- Records include fixture provenance, solver version, matrix/reduced-system
  metrics, residual/repeatability evidence, diagnostics, assumptions, and
  limitations.
- Unknown or suspected protected fixture provenance is rejected.
- Conditioning observations integrate with solver diagnostics without claiming
  final release threshold policy.
- Sparse solver selection, practical model-size bands, timing/memory thresholds,
  hardware normalization, and CI gates remain explicit `TBD`s.
- Tests cover repeatability, provenance rejection, invalid settings, residual
  calculation, and conditioning diagnostic behavior.
- No lifecycle state transition, dependency-register edit, candidate-edge
  change, or blocker-queue refresh occurs unless separately authorized.
