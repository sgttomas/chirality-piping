# MEMORY - DEL-04-02 Straight Pipe Element

## Decisions And Rulings

- 2026-05-01 - Human project authority authorized proceeding with
  `DEL-04-02` after the `DEL-04-05` evidence refresh.
- 2026-05-01 - This slice consumes committed `DEL-04-01` frame-kernel evidence,
  committed `DEL-03-08` section-property calculator evidence, committed
  `DEL-02-02` unit-contract evidence, and the newly committed `DEL-04-05`
  performance-harness evidence as regression context.

## Implementation Summary

- Added `core/solver/straight_pipe`, a Rust mechanics crate with a path
  dependency on `core/solver/frame_kernel`.
- Added explicit straight-pipe section-property integration into the frame
  element boundary.
- Added weight-hook records for explicit mass-per-length and gravity inputs.
- Added local force recovery from element or global-model displacement vectors.

## Evidence

- `cargo fmt --manifest-path core/solver/straight_pipe/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml` passed:
  6 tests, 0 failures.
- Upstream/regression Rust checks passed:
  `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml` and
  `cargo test --manifest-path core/solver/performance_harness/Cargo.toml`.

## Open TBDs

- Canonical calculation unit basis and conversion constants remain `TBD`.
- Primitive load-case application of weight remains future `PKG-05` work.
- Downstream stress recovery remains future `DEL-05-03` work.
- Result-envelope/application-service integration remains future work.

## Boundaries Preserved

- No protected standards text, protected tables, code-specific allowables,
  SIF/flexibility factors, proprietary data, private project data, or
  professional/code-compliance claims were introduced.
- No lifecycle state, dependency register, candidate edge, or blocker queue was
  changed by the product implementation.
