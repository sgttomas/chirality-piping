# MEMORY - DEL-04-03 Linear Support And Restraint Models

## Decisions And Rulings

- 2026-05-01: Human project authority authorized bounded implementation after
  the sealed dispatch brief was prepared.
- Implementation stays mechanics-only and linear-only. Nonlinear gap,
  lift-off, one-way, friction, and active-set behavior remain assigned to
  `DEL-04-04`.
- Support quantities retain dimension intent but do not resolve canonical
  calculation units, conversion constants, coordinate-frame policy, rigid
  restraint numerical method, sparse-solver integration, or final
  result-envelope integration.

## Implementation Notes

- Added `core/solver/linear_supports`, a Rust crate for anchors, guides, line
  stops, vertical supports, linear springs, and imposed displacement boundary
  data.
- The crate maps support records to frame-kernel global DOF indices and records
  deterministic findings for missing or invalid solve-required support data.
- No protected support defaults, catalog values, code-specific checks,
  dependency-register edits, lifecycle transitions, or blocker-queue refreshes
  were introduced.

## Verification

- `cargo fmt --manifest-path core/solver/linear_supports/Cargo.toml`
- `cargo test --manifest-path core/solver/linear_supports/Cargo.toml` passed
  8 tests.
- Adjacent solver checks passed:
  `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml`,
  `cargo test --manifest-path core/solver/diagnostics/Cargo.toml`, and
  `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml`.
- `git diff --check` passed.
- Focused protected-content/prohibited-claim scan found only boundary/negative
  statements and no bundled protected data or positive compliance claims.

## Open Items

- Canonical calculation unit basis and conversion constants remain `TBD`.
- Support coordinate-frame convention remains `TBD`; the current crate maps
  explicit node DOFs rather than arbitrary vector directions.
- Rigid-restraint numerical method and constraint-elimination or penalty
  strategy remain `TBD`.
- Sparse-solver and final result-envelope integration remain downstream work.
