---
doc_id: DEV-001-DISPATCH-DEL-04-06
doc_kind: coordination.dispatch_brief
status: launched_single_bounded_item
created: 2026-05-01
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-04-06
package_id: PKG-04
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-04-06

## Dispatch Decision

The human project authority authorized proceeding with the recommended next
bounded DAG item: `DEL-04-06 - Solver diagnostics and singularity detection`.

This dispatch is not broad fan-out. It does not authorize lifecycle
transitions, candidate-edge promotion, dependency-register edits, blocker queue
refresh, or work on any other deliverable.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-04-06` |
| PackageID | `PKG-04` |
| Name | Solver diagnostics and singularity detection |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope items | `SOW-053`, `SOW-035` |
| Objectives | `OBJ-003`, `OBJ-008`, `OBJ-012` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-06_Solver diagnostics and singularity detection` |
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
| `DEL-02-03` | `DIAGNOSTICS_CONTRACT` | `COMMITTED` evidence `abc1306` |
| `DEL-02-02` | `UNIT_CONTRACT` | `COMMITTED` evidence `a458cba` |

Candidate edge `DAG-001-E0622` from `DEL-04-06` to `DEL-04-04` remains
non-gating. This slice must not finalize nonlinear-support warning classes that
require future `DEL-04-04` implementation evidence.

## Applicable Architecture Basis

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module responsibilities and no-bypass dependency rules.
- `AB-00-03` - schema-first command/query/job/result-envelope service
  boundaries.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no
  certification/compliance claims.
- `AB-00-08` - layered validation, provenance, protected-content, and
  acceptance gates.

Resolved baseline to preserve: diagnostics are mechanics-solver findings only;
they may feed result envelopes and UI/report surfaces later but must not become
rule-pack checks, professional approvals, code-compliance claims, or hidden
defaults.

Remaining implementation-level TBDs are not resolved by this dispatch:
accepted sparse solver library, release-quality tolerance thresholds,
nonlinear-support warning classes, and final result-envelope integration.

## Explicit Write Scope

Authorized product and evidence targets for this bounded item:

- `core/solver/diagnostics/Cargo.toml`
- `core/solver/diagnostics/.gitignore`
- `core/solver/diagnostics/README.md`
- `core/solver/diagnostics/src/lib.rs`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-06_Solver diagnostics and singularity detection/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-04-06.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch.

## Acceptance Criteria

This bounded item is acceptable for closeout when:

- Solver diagnostics map frame-kernel singularity, invalid restraint, invalid
  topology, invalid numeric input, and shape errors into deterministic
  diagnostic records.
- Conditioning and nonconvergence diagnostics are represented without claiming
  final release tolerance policy.
- Sparse solver selection and tolerance policy remain explicit `TBD` warning
  diagnostics rather than hidden assumptions.
- Tests cover singular systems, invalid restraints, invalid topology,
  conditioning warning/failure behavior, nonconvergence, and sparse-solver
  `TBD` handling.
- Documentation identifies the diagnostics boundaries without claiming
  engineering approval, legal acceptance, code compliance, or production sparse
  performance.
- No lifecycle state transition, dependency-register edit, candidate-edge
  change, or blocker-queue refresh occurs unless separately authorized.
