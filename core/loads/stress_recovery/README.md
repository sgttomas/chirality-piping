# Stress Recovery

This crate is the bounded implementation slice for `DEL-05-03`. It recovers
code-neutral mechanics stresses from explicit element force resultants, section
properties, and optional pressure basis inputs.

## Scope

- Axial normal stress from supplied axial force and area.
- Bending normal stress components from supplied bending moments and section
  moduli.
- Torsional shear stress from supplied torque, torsion radius, and torsion
  constant.
- Thin-wall pressure membrane components from explicit pressure, radius, and
  wall thickness inputs.
- Deterministic findings for missing inputs, non-finite values, non-positive
  properties, incomplete status, and status-boundary violations.

## Boundary

This crate does not implement design-code stress equations, allowables, stress
indices, SIF/flexibility tables, rule-pack checks, protected standards content,
pipe tables, unit conversion constants, report rendering, GUI behavior, local
FEA handoff, or professional/code-compliance claims.

Inputs are already parsed mechanics quantities from governed solver, section,
load, unit, and status boundaries. Unknown conversion, result-envelope,
persistence, tolerance, rule-check, and benchmark-publication details remain
`TBD`.

## Verification

The unit tests cover axial, bending, torsion, pressure membrane components,
combined normal/shear summaries, missing inputs, invalid numeric values, and
analysis-status propagation without human approval or code-compliance claims.
