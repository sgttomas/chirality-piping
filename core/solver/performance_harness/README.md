# Solver Performance Harness

This crate is the bounded implementation slice for `DEL-04-05`. It provides a deterministic regression harness around the current frame-kernel solve boundary so sparse-solver performance and conditioning evidence can be recorded without changing solver logic.

## Scope

- Invented frame-chain benchmark fixtures with explicit public provenance posture.
- Repeat-run regression records for the same fixture, solver version, and harness settings.
- Matrix size, nonzero-count, residual, repeatability, and condition-ratio observations.
- Integration with solver diagnostics for conditioning classification and unresolved sparse-solver/tolerance-policy `TBD` states.

## Boundary

This crate does not select the final sparse numerical library, set release-quality timing thresholds, alter the frame kernel, define code-specific checks, encode protected standards examples, or make professional/code-compliance claims.

The current solve path uses the temporary dense verification interface from `core/solver/frame_kernel`. The harness preserves a stable observer boundary so a future sparse adapter can be measured behind the same record shape.

## Verification

The unit tests cover deterministic repeat-run records, provenance rejection, invalid settings, nonzero-count metrics, conditioning diagnostics, and residual calculation.
