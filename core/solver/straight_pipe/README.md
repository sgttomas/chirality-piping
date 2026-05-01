# Straight Pipe Element

This crate is the bounded implementation slice for `DEL-04-02`. It adapts explicit straight-pipe section and mass properties into the existing 3D frame kernel, exposes weight hooks for later load-case work, and recovers local mechanical element forces.

## Scope

- Straight two-node pipe element construction.
- Explicit section-property integration through caller-supplied area, second moments, and torsion constant.
- Explicit mass-per-length and weight-per-length hooks.
- Local element displacement and force recovery using the frame-kernel stiffness and orientation conventions.

## Boundary

This crate does not provide pipe dimension tables, material defaults, code-specific values, stress checks, load combinations, or professional/code-compliance claims. Inputs are mechanics quantities that upstream schemas, unit contracts, provenance checks, and section-property calculations must already govern.

## Verification

The unit tests cover section-property validation, frame-kernel stiffness integration, weight hooks, axial local force recovery, transverse bending recovery, and displacement-length validation.
