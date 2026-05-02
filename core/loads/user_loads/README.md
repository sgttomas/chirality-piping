# User Load Application

This crate is the bounded implementation slice for `DEL-05-05`. It prepares
explicit concentrated forces, concentrated moments, and distributed user loads
for the current frame-solver boundary.

## Scope

- Concentrated force inputs applied to node translational degrees of freedom.
- Concentrated moment inputs applied to node rotational degrees of freedom.
- Uniform distributed user-load inputs applied to element translational
  directions over an explicit normalized span.
- Deterministic findings for missing targets, invalid dimensions, unsupported
  target/load combinations, non-finite inputs, invalid spans, and non-positive
  element lengths where a length is supplied.
- Traceable contribution records and result-recovery hooks for later stress
  recovery, reporting, export, GUI, and headless execution work.

## Boundary

This crate does not provide design-code load combinations, public default
factors, wind/seismic procedures, material or section defaults, protected
standards content, proprietary project data, rule-pack checks, result-envelope
integration, or professional/code-compliance claims. Inputs are explicit
mechanics quantities that upstream schema, unit, provenance, and solver
boundaries must already govern.

## Verification

The unit tests cover concentrated force, concentrated moment, uniform
distributed load behavior, invalid/missing input findings, dimension checks,
deterministic contribution ordering, result-recovery hook traces, and
preservation of the no-default/no-compliance boundary.
