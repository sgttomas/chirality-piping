# MECH-INCLINED-MEMBER-TRANSFORM

## Purpose

Stiffness-transform benchmark for an inclined member with a 45-degree projection
in the global `XY` plane.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, or proprietary
  data.

## Invented Inputs

The member starts at `(0.0, 0.0, 0.0)` and ends at `(1.0, 1.0, 0.0)`.

## Expected Values

The local `x` axis is the normalized node-to-node vector:

```text
local_x = [1.0, 1.0, 0.0] / sqrt(2.0)
        = [0.7071067811865475, 0.7071067811865475, 0.0]
```

The transformed global stiffness matrix must remain symmetric.

Tolerance policy: `TBD`.
