# MEMORY - DEL-04-04 Nonlinear Support Active-Set Solver

## 2026-05-01 bounded implementation

Human project authority approved one deliberately scoped `DEL-04-04` item after
`DEL-04-03` implementation evidence was committed.

Implemented:

- `core/solver/nonlinear_supports`, a Rust mechanics-boundary crate for
  nonlinear support active-set decisions.
- Explicit behavior models for one-way supports, gaps, lift-off/contact, and
  friction-limited supports.
- Trial-state classification into active, inactive, sticking, and sliding
  states.
- Active-set iteration records with changed supports, residual norm,
  convergence flag, and diagnostics sourced through `core/solver/diagnostics`.
- Tests for one-way activation, gap closure, lift-off, friction stick/slip,
  convergence, nonconvergence, and invalid numeric inputs.

Boundaries preserved:

- No global nonlinear matrix assembly or production solve loop integration.
- No sparse-solver selection, production tolerance policy, load-case algebra,
  rule-pack checks, protected standards data, public support/catalog defaults,
  lifecycle transition, dependency-register edit, or professional/code-
  compliance claim.

Open items:

- Canonical calculation unit basis, conversion constants, final support
  coordinate convention, rigid-restraint numerical method, constraint-
  elimination or penalty strategy, sparse-solver integration, production
  residual/tolerance policy, and final result-envelope integration remain
  `TBD`.
