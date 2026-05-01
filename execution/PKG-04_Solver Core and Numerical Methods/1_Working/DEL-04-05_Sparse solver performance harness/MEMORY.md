# MEMORY - DEL-04-05 Sparse Solver Performance Harness

## Decisions And Rulings

- 2026-05-01 - Human project authority authorized proceeding with
  `DEL-04-05` as a bounded implementation item. The user typed
  `DEL-04-005`; project registers use `DEL-04-05`.
- 2026-05-01 - This slice consumes committed `DEL-04-01` frame-kernel evidence
  and committed `DEL-04-06` solver-diagnostics evidence.

## Implementation Summary

- Added `core/solver/performance_harness`, a Rust observer/regression crate
  with path dependencies on `core/solver/frame_kernel` and
  `core/solver/diagnostics`.
- Added an invented cantilever-chain fixture generator with explicit public
  provenance posture and rejection of unknown/protected fixture statuses.
- Added repeat-run harness records for matrix size, reduced DOFs, nonzero
  counts, force nonzero counts, repeatability delta, residual, condition-ratio
  estimate, diagnostics, assumptions, limitations, and provenance notes.
- The harness reports sparse-solver and tolerance-policy `TBD` diagnostics
  rather than choosing a sparse library or release threshold policy.

## Evidence

- `cargo fmt --manifest-path core/solver/performance_harness/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/solver/performance_harness/Cargo.toml`
  passed: 6 tests, 0 failures.
- Upstream Rust checks passed:
  `cargo test --manifest-path core/solver/diagnostics/Cargo.toml` and
  `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml`.
- `git diff --check` passed.
- Focused protected-content/prohibited-claim scan found only boundary/negative
  statements, no bundled protected data or positive compliance claims.

## Open TBDs

- Accepted sparse numerical library remains `TBD`.
- Release timing, memory, practical-size bands, conditioning, and CI threshold
  policies remain `TBD`.
- Hardware-normalized performance methodology remains `TBD`.
- Future sparse-adapter integration remains downstream work.

## Boundaries Preserved

- No protected standards text, protected tables, code-specific allowables,
  SIF/flexibility factors, proprietary benchmark models, private project data,
  or professional/code-compliance claims were introduced.
- No solver logic, lifecycle state, dependency register, candidate edge, or
  blocker queue was changed.
