# Solver Diagnostics

This crate is the bounded implementation slice for `DEL-04-06`. It provides code-neutral solver diagnostic records for frame-kernel failures, conditioning warnings, nonconvergence, invalid restraints, and explicit sparse-solver `TBD` state.

## Scope

- Deterministic diagnostic codes, severities, sources, and solver status values.
- Mapping from `open_pipe_stress_frame_kernel::FrameKernelError` into diagnostic records.
- Basic finite/nonnegative condition-ratio classification for small verification paths.
- Nonconvergence diagnostics for iterative methods without implementing nonlinear support behavior.
- Explicit `TBD` diagnostics for sparse-solver adapter selection and tolerance policy.

## Boundary

This crate reports mechanics-solver diagnostics only. It does not solve models, select the sparse numerical library, set final tolerance policy, perform rule-pack checks, encode protected standards data, or make professional/code-compliance claims.

## Verification

The unit tests cover singular-system mapping, invalid restraint mapping, invalid model topology, non-finite condition estimates, ill-conditioning warnings, failed conditioning, nonconvergence, completed convergence, and sparse-solver `TBD` diagnostics.
