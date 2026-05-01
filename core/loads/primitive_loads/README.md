# Primitive Load Cases

This crate is the bounded implementation slice for `DEL-05-01`. It defines
code-neutral primitive load records and prepares deterministic mechanics
contributions for the current frame/support boundary.

## Scope

- Weight, pressure, thermal, imposed displacement, hydrotest, wind, seismic,
  and occasional primitive load categories.
- Explicit unit/dimension intent for load magnitudes.
- Deterministic findings for missing target references, invalid numeric input,
  invalid dimensions, and unsupported target/category combinations.
- Nodal force, element uniform load, and imposed-displacement contributions for
  later load-case algebra, stress recovery, GUI, and headless execution work.

## Boundary

This crate does not provide protected code load combinations, design-code load
categories, wind/seismic procedures, pressure stress formulas, material or
section defaults, catalog data, rule-pack checks, result envelopes, or
professional/code-compliance claims. Inputs are explicit mechanics quantities
that upstream schema, unit, provenance, and solver boundaries must already
govern.

## Verification

The unit tests cover every primitive category, invalid/missing data findings,
deterministic application behavior, and preservation of frame/support DOF
assumptions.
