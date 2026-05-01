---
doc_id: DEV-001-DISPATCH-DEL-04-02
doc_kind: coordination.dispatch_brief
status: launched_single_bounded_item
created: 2026-05-01
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-04-02
package_id: PKG-04
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-04-02

## Dispatch Decision

The human project authority authorized `DEL-04-02 - Straight pipe element` as
the next product DAG item after the `DEL-04-05` implementation-evidence refresh.

This dispatch is not broad fan-out. It does not authorize lifecycle
transitions, candidate-edge promotion, dependency-register edits, blocker queue
refresh, or work on any other deliverable.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-04-02` |
| PackageID | `PKG-04` |
| Name | Straight pipe element |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope items | `SOW-006` |
| Objectives | `OBJ-003` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-02_Straight pipe element` |
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
| `DEL-03-08` | `DOMAIN_MODEL` | `COMMITTED` evidence `9712e98` |
| `DEL-02-02` | `UNIT_CONTRACT` | `COMMITTED` evidence `a458cba` |

## Applicable Architecture Basis

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module responsibilities and no-bypass dependency rules.
- `AB-00-03` - schema-first command/query/job/result-envelope service
  boundaries.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no
  certification/compliance claims.
- `AB-00-08` - layered validation, provenance, protected-content, and
  acceptance gates.

Resolved baseline to preserve: the straight-pipe element is a mechanics-solver
component around explicit governed inputs. It must not provide public pipe
tables, material defaults, protected/code-specific values, stress checks,
load combinations, or human/professional approval claims.

Remaining implementation-level TBDs are not resolved by this dispatch:
canonical calculation unit basis, conversion constants, primitive load-case
application, stress recovery, and final result-envelope integration.

## Explicit Write Scope

Authorized product and evidence targets for this bounded item:

- `core/solver/straight_pipe/.gitignore`
- `core/solver/straight_pipe/Cargo.toml`
- `core/solver/straight_pipe/README.md`
- `core/solver/straight_pipe/src/lib.rs`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-02_Straight pipe element/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-04-02.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch.

## Acceptance Criteria

This bounded item is acceptable for closeout when:

- The straight-pipe element remains separate from global assembly and does not
  modify frame-kernel solver behavior.
- Explicit section properties adapt into the frame-kernel element boundary.
- Missing mass-per-length for weight hooks produces an explicit finding rather
  than a silent default.
- Local element force recovery is deterministic and mechanics-only.
- Tests cover section-property validation, frame-kernel integration,
  weight-hook behavior, displacement validation, and local force recovery.
- No protected standards data, proprietary values, public defaults, stress
  code checks, lifecycle transition, dependency-register edit, candidate-edge
  change, or blocker-queue refresh occurs unless separately authorized.
