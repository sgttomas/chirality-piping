# MEMORY - DEL-04-06 Solver Diagnostics And Singularity Detection

## Decisions And Rulings

- 2026-05-01 - Human project authority authorized proceeding with
  `DEL-04-06` as the recommended continuity item after `DEL-04-01`.
- 2026-05-01 - Candidate edge `DAG-001-E0622` remains non-gating; nonlinear
  support warning classes requiring `DEL-04-04` are not finalized in this
  slice.

## Implementation Summary

- Added `core/solver/diagnostics`, a Rust mechanics-diagnostics crate with a
  path dependency on `core/solver/frame_kernel`.
- The crate maps `FrameKernelError` variants into deterministic diagnostic
  records with code, severity, source, message, and optional affected
  reference.
- Added conditioning-ratio classification, nonconvergence diagnostics, and
  explicit sparse-solver/tolerance-policy `TBD` warning diagnostics.

## Evidence

- `cargo fmt --manifest-path core/solver/diagnostics/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/solver/diagnostics/Cargo.toml` passed:
  10 tests, 0 failures.

## Open TBDs

- Accepted sparse numerical library remains `TBD`.
- Release-quality solver tolerance thresholds remain `TBD`.
- Nonlinear-support warning classes remain future work pending `DEL-04-04`.
- Final result-envelope and application-service integration remains future
  work.

## Boundaries Preserved

- No protected standards text, protected tables, code-specific allowables,
  SIF/flexibility factors, proprietary catalog values, private project data, or
  professional/code-compliance claims were introduced.
- No lifecycle state transition, dependency-register edit, candidate-edge
  promotion, or blocker-queue refresh was performed.
