# MEMORY - DEL-04-01 3D Frame Stiffness Kernel

## Decisions And Rulings

- 2026-05-01 - Human project authority authorized bounded implementation of
  `DEL-04-01` using
  `execution/_Coordination/DEV-001_DISPATCH_DEL-04-01.md`.
- 2026-05-01 - Implementation preserves the existing Rust crate path
  `core/solver/frame_kernel` and treats the dense solve interface as a
  temporary verification interface until the sparse numerical library is
  accepted.

## Implementation Summary

- `core/solver/frame_kernel` provides the first 3D frame kernel slice:
  two-node frame element stiffness, deterministic DOF ordering, local/global
  transform handling, dense assembly, boundary-condition reduction, and a
  temporary dense solve interface.
- The crate validates finite/positive numeric inputs, degenerate axes,
  repeated element node indices, invalid model node indices, repeated
  restrained DOFs, vector/matrix shape mismatches, and singular systems.
- Upstream unit/schema contracts remain responsible for unit compatibility and
  conversion. This deliverable does not define the final unit catalog,
  conversion constants, solver tolerances, sparse solver library, or canonical
  calculation unit basis.

## Evidence

- `cargo fmt --manifest-path core/solver/frame_kernel/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml` passed:
  11 tests, 0 failures.
- Test coverage includes local stiffness terms, transform behavior, assembly,
  boundary-condition reduction, dense solve, singular-system detection,
  repeated element node index rejection, invalid model node index rejection,
  repeated restraint rejection, degenerate orientation rejection, and
  non-finite input rejection.

## Open TBDs

- Accepted sparse numerical library remains `TBD`.
- Solver tolerance policy remains `TBD`.
- Canonical calculation unit basis and conversion constants remain `TBD`.
- Downstream solver adapter boundaries and result-envelope integration remain
  future work.

## Boundaries Preserved

- No protected standards text, protected tables, code-specific allowables,
  SIF/flexibility factors, proprietary catalog values, private project data, or
  professional/code-compliance claims were introduced.
- No lifecycle state transition, dependency-register edit, candidate-edge
  promotion, or blocker-queue refresh was performed.
