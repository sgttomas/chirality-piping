---
doc_id: DEV-001-REV05-SEALED-BRIEF-TP-MAC-01-E
doc_kind: coordination.sealed_product_assembly_brief
status: sealed_brief_prepared_dispatch_not_authorized
created: 2026-05-10
prepared_by: ORCHESTRATOR
tranche: TP-MAC-01
unit_id: TP-MAC-01-E
unit_name: Mechanics execution slice
primary_deliverables: DEL-04-01, DEL-04-02, DEL-04-03, DEL-05-01, DEL-05-03, DEL-05-04, DEL-05-05, DEL-07-05, DEL-07-07
worker_launch: not_authorized
implementation_dispatch: not_authorized
source_plan: execution/_Coordination/DEV-001_REV05_MACOS_DESKTOP_PRODUCT_ASSEMBLY_TRANCHE_PLAN.md
---

# Sealed Brief - TP-MAC-01-E Mechanics Execution Slice

## Dispatch Boundary

This brief is prepared for later bounded implementation only. It does not
authorize worker launch, lifecycle/evidence changes, dependency refresh,
`DAG-002` mutation, external solver/prover execution, rule-pack compliance
checks, protected standards data, or professional/code-compliance claims.

## Product Slice

Connect the invented desktop model to a bounded mechanics/diagnostics execution
path and display deterministic result/status summaries in the GUI. Unsupported
physics or incomplete data must produce explicit diagnostics.

## PKG/DEL Basis

| Package | Deliverable | Use in this brief |
|---|---|---|
| `PKG-04` | `DEL-04-01` 3D frame stiffness kernel | Mechanics assembly basis where currently usable. |
| `PKG-04` | `DEL-04-02` Straight pipe element | Pipe element behavior for invented centerline spans. |
| `PKG-04` | `DEL-04-03` Linear support and restraint models | Simple support/restraint inputs for preview. |
| `PKG-04` | `DEL-04-06` Solver diagnostics and singularity detection | Deterministic diagnostics and failure states. |
| `PKG-05` | `DEL-05-01` Primitive load case engine | Invented primitive load request shape. |
| `PKG-05` | `DEL-05-03` Fundamental stress recovery module | Mechanics-only stress/result summary where supported. |
| `PKG-05` | `DEL-05-04` Analysis status semantics | Distinguish mechanics solved, rule checked, and professional acceptance. |
| `PKG-05` | `DEL-05-05` Concentrated and distributed user load application | Invented user-load examples. |
| `PKG-07` | `DEL-07-05`, `DEL-07-07` | Results viewer and solve execution UX. |

Applicable architecture basis: `AB-00-03`, `AB-00-06`, `AB-00-07`,
`AB-00-08`.

## Allowed Write Scope

- `core/product_preview/`
- `apps/desktop/src/features/solve/`
- `apps/desktop/src/features/results/`
- `tests/product_preview/`
- invented mechanics fixtures under `fixtures/product_preview/`

Do not edit existing solver crates except through narrow public integration
adapters. If solver crates require changes, stop and request a separate sealed
scope because that changes deliverable implementation surfaces.

## Required Output

- Preview mechanics execution adapter for invented centerline model data.
- Deterministic result envelope with displacement/reaction/stress summary if
  supported by existing committed modules.
- Explicit diagnostics for unsupported calculations, singular/incomplete
  states, missing data, or unavailable integration paths.
- GUI solve action with progress/status and result/diagnostic presentation.

## Acceptance Criteria

- The desktop preview can run a bounded mechanics/diagnostics action.
- Result/status output is deterministic for the invented fixture.
- Mechanics result, rule-check status, and professional acceptance status are
  separate and visible.
- Unsupported paths do not throw opaque errors or invent engineering defaults.
- No code-specific combinations, allowables, SIF/flexibility factors, or
  protected formulas are introduced.

## Required Verification

```text
python3 -m pytest tests/product_preview tests/test_analysis_status_schema.py tests/test_results_schema.py tests/test_solve_execution_ux.py tests/test_results_viewer_contract.py
cargo test --manifest-path core/solver/frame_kernel/Cargo.toml
cargo test --manifest-path core/solver/straight_pipe/Cargo.toml
cargo test --manifest-path core/solver/linear_supports/Cargo.toml
cargo test --manifest-path core/solver/diagnostics/Cargo.toml
npm run build --workspace apps/desktop
git diff --check
```

## Stop Conditions

Stop before adding standards-derived values, rule-pack compliance checks,
external solver/prover execution, hidden defaults, or professional/release
claims.
