# Nonlinear Supports

This crate is the bounded implementation slice for `DEL-04-04`. It models the
active-set decision boundary for one-way supports, gaps, lift-off/contact, and
friction-limited supports.

The crate consumes trial displacement and reaction facts from a mechanics solve
iteration and classifies nonlinear support states as active, inactive, sticking,
or sliding. It records changed support states, residuals, convergence, and
nonconvergence diagnostics. It does not assemble or solve the global nonlinear
system, select sparse solvers, define release tolerance policy, perform load
case algebra, run rule-pack checks, or make professional/code-compliance
claims.

## Boundary

- Support behavior is explicit model data; no support/catalog defaults are
  supplied.
- Sign conventions are explicit through activation and gap-direction enums
  rather than hidden assumptions.
- Friction behavior uses user/model supplied coefficient and normal reaction
  values; no engineering defaults are invented.
- Nonconvergence is surfaced through the solver diagnostics contract.

## Tests

The unit tests cover one-way activation, gap opening/closing, lift-off behavior,
friction sticking/sliding, invalid numeric inputs, active-set convergence, and
iteration-limit nonconvergence diagnostics.
