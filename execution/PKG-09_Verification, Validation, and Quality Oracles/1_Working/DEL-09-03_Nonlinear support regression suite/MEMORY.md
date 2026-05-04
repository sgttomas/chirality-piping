# MEMORY - DEL-09-03 Nonlinear support regression suite

## Scope

Implemented a bounded nonlinear support regression suite for DEV-001 revision
0.5 Tranche A using invented, non-project fixtures only.

## Evidence

- Added `validation/benchmarks/nonlinear/` as an executable Rust benchmark crate.
- Covered active-set one-way support, gap closure, lift-off, friction
  stick/slide behavior, and unresolved non-convergence at the iteration limit.
- Added `tests/test_nonlinear_support_regression.py` as the focused repo-level
  regression entry point for the nonlinear benchmark crate.

## Validation

- `cargo test --quiet` in `validation/benchmarks/nonlinear`: passed, 5 tests.
- `python3 -m pytest -q tests/test_nonlinear_support_regression.py`: passed, 2 tests.
- Existing `cargo test --quiet` in `core/solver/nonlinear_supports`: passed, 8 tests.
- Existing `cargo test --quiet` in `core/solver/diagnostics`: passed, 10 tests.
- `git diff --check`: passed.
- Protected-content, private-data, credential, and professional-claim scans over the
  DEL-09-03 changed files: passed.

## Remaining TBDs

- Production release tolerance policy remains `TBD`.
- Release thresholds for nonlinear support regression evidence remain `TBD`.
- External validation claims remain `TBD`; this suite is software verification
  evidence only.
