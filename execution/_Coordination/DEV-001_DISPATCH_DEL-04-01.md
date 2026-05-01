---
doc_id: DEV-001-DISPATCH-DEL-04-01
doc_kind: coordination.dispatch_brief
status: prepared_single_bounded_item
created: 2026-05-01
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-04-01
package_id: PKG-04
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-04-01

## Dispatch Decision

The human project authority authorized exactly one bounded DAG item:
`DEL-04-01 - 3D frame stiffness kernel`.

This dispatch brief is not broad fan-out. It does not authorize lifecycle
transitions, candidate-edge promotion, dependency-register edits, blocker queue
refresh, or work on any other deliverable.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-04-01` |
| PackageID | `PKG-04` |
| Name | 3D frame stiffness kernel |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope items | `SOW-005`, `SOW-035` |
| Objectives | `OBJ-003` |
| Context envelope | `L` |
| Context QA | `WATCH`; confirm scope and split if it expands |
| Deliverable path | `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel` |
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
| `DEL-02-01` | `DOMAIN_MODEL` | `COMMITTED` evidence `7b256f3` |
| `DEL-02-02` | `UNIT_CONTRACT` | `COMMITTED` evidence `a458cba` |
| `DEL-02-03` | `DOMAIN_MODEL` | `COMMITTED` evidence `abc1306` |

No `CANDIDATE` edge currently gates `DEL-04-01`.

## Applicable Architecture Basis

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module responsibilities and no-bypass dependency rules.
- `AB-00-03` - schema-first command/query/job/result-envelope service
  boundaries.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no
  certification/compliance claims.
- `AB-00-08` - layered validation, provenance, protected-content, and
  acceptance gates.

Resolved baseline to preserve: the solver kernel is Rust core code; GUI-facing
concerns remain out of scope; public schemas remain JSON Schema 2020-12 where
used; command/query/job envelopes and diagnostics boundaries must not be
bypassed; tests should use Cargo and project validation/protected-content gates
as applicable.

Remaining implementation-level TBDs are not resolved by this dispatch: exact
dependency versions, accepted sparse numerical library, canonical calculation
unit basis, conversion constants/tolerances, CI thresholds, and downstream
solver adapter boundaries.

## Applicable Contract Invariants

- `OPS-K-IP-1` through `OPS-K-IP-3` - no protected standards/proprietary data,
  provenance required where public data is introduced, suspected protected
  content quarantined.
- `OPS-K-DATA-1` and `OPS-K-DATA-2` - code-specific values are user-supplied or
  private; missing solve-required values are explicit findings, not defaults.
- `OPS-K-MECH-1` and `OPS-K-MECH-2` - the global model is a 3D
  centerline/frame mechanics solve; rule checks and professional approval are
  separate.
- `OPS-K-UNIT-1` - calculations, schemas, imports, exports, and solver
  boundaries are unit-aware and dimensionally checked.
- `OPS-K-SOLVER-1` - solver changes require deterministic verification tests.
- `OPS-K-AUTH-1` - no certification, sealing, approval, authentication, legal
  acceptance, or code-compliance claims for reliance.
- `OPS-K-AGENT-1` through `OPS-K-AGENT-4` - no invented engineering values,
  conflicts surfaced, sealed execution, draft until human accepted.

## Explicit Write Scope

Authorized product and evidence targets for this bounded item:

- `core/solver/frame_kernel/Cargo.toml`
- `core/solver/frame_kernel/README.md`
- `core/solver/frame_kernel/src/lib.rs`
- `core/solver/frame_kernel/.gitignore`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-04-01.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch unless the human project
authority issues a new explicit write-scope gate.

## Acceptance Criteria

This bounded item is acceptable for closeout when:

- The frame kernel provides a deterministic two-node 3D frame mechanics core
  with six degrees of freedom per node and documented DOF ordering.
- Local element stiffness, coordinate transforms, boundary-condition handling,
  global assembly, and a bounded solve interface are implemented or explicitly
  deferred with a reason.
- The implementation does not introduce protected standards text, protected
  tables, code-specific values, material allowables, SIF/flexibility data,
  private project data, or professional/code-compliance claims.
- Missing, invalid, non-finite, degenerate, singular, or unsupported inputs
  produce deterministic errors or diagnostics rather than silent defaults.
- Solver behavior is covered by deterministic tests for element stiffness,
  transform behavior, assembly, boundary-condition reduction, invalid inputs,
  and singular systems.
- Documentation identifies the kernel boundaries, dense/sparse solve status,
  unit obligations, and downstream replacement points without claiming
  engineering approval, legal acceptance, code compliance, or catalog
  completeness.
- No lifecycle state transition, dependency-register edit, candidate-edge
  change, or blocker-queue refresh occurs unless separately authorized.

## Sealed TASK Brief

```markdown
PURPOSE: Implement the sealed OpenPipeStress deliverable within explicit scope.
RequestedBy: WORKING_ITEMS

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel
DeliverablePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel
TaskProfile: solver-core

DeliverableID: DEL-04-01
PackageID: PKG-04

Tasks:
  - Implement or complete only the artifacts authorized for DEL-04-01.
  - Preserve all applicable contract invariants and architecture-basis constraints.
  - Produce or update deterministic Cargo tests and evidence appropriate to the frame kernel.
  - Record open issues for unresolved TBDs, assumptions, or cross-deliverable dependencies.

RuntimeOverrides:
  DecompositionPath: docs/_Decomposition/SOFTWARE_DECOMP.md
  DecompositionRevision: "0.4"
  DAGPath: execution/_DAG/DAG-001/DependencyEdges.csv
  DAGApprovalRecord: execution/_DAG/DAG-001/APPROVAL_RECORD.md
  CoordinationMode: FULL_GRAPH
  BlockerComputation: ENABLED_ACTIVE_EDGES_ONLY
  ScopeChangeID: SCA-001

CustomInstructions:
  - Read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, and existing frame-kernel files before editing.
  - Apply only applicable `AB-00-*` constraints; do not copy full `PKG-00` prose.
  - Treat protected standards/code data as out of scope.
  - Unknown engineering values remain `TBD`.
  - Do not claim certification, approval, sealing, or code compliance for reliance.
  - Do not edit files outside this sealed write scope unless explicitly authorized.
  - Do not recompute or mutate blocker queues unless explicitly assigned.

AllowedWriteTargets:
  - core/solver/frame_kernel/Cargo.toml
  - core/solver/frame_kernel/README.md
  - core/solver/frame_kernel/src/lib.rs
  - core/solver/frame_kernel/.gitignore
  - docs/SPEC.md
  - docs/TYPES.md
  - execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel/MEMORY.md
  - execution/_Coordination/DEV-001_DISPATCH_DEL-04-01.md
  - execution/_Coordination/NEXT_INSTANCE_STATE.md

ExpectedOutputs:
  - Implemented or completed frame-kernel artifacts within the authorized write scope.
  - Deterministic tests/evidence for stiffness, transform, assembly, boundary, invalid-input, and singular-system behavior.
  - Updated deliverable memory with decisions, open TBDs, and evidence used.
  - Open issue list for unresolved sparse-solver/library/unit-catalog decisions.

EXCLUSIONS:
  - No protected standards text, tables, examples, or proprietary engineering values.
  - No code-specific allowables, SIF/flexibility factors, material tables, or design-code checks.
  - No edits outside the sealed write scope.
  - No lifecycle state transition unless explicitly authorized by the human.
  - No local dependency-register edits unless explicitly assigned.
```
