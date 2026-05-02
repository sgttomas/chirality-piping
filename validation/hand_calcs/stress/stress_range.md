# Mechanics Stress Range Fixture

Fixture ID: `STRESS-RANGE-MECHANICS-ORIGINAL`

## Provenance

- Source: OpenPipeStress original stress recovery benchmark.
- License basis: project-original-public-content.
- Contributor: OpenPipeStress agentic development workflow.
- Certification: generated from elementary open mechanics; not copied from
  protected standards, code formulas, commercial software examples, or
  proprietary data.

## Inputs

The fixture compares two invented mechanics states using the same section
properties. Pressure inputs are omitted for this mechanics range case.

| Quantity | State A | State B | Dimension |
|---|---:|---:|---|
| Axial force | 60.0 | 180.0 | force |
| Bending moment about local y | -20.0 | 80.0 | force_length |
| Bending moment about local z | 10.0 | 10.0 | force_length |
| Torsional moment | 20.0 | 60.0 | force_length |
| Area | 12.0 | 12.0 | area |
| Section modulus about local y | 25.0 | 25.0 | length_cubed |
| Section modulus about local z | 15.0 | 15.0 | length_cubed |
| Torsion radius | 2.0 | 2.0 | length |
| Torsion constant | 80.0 | 80.0 | length_fourth |

## Expected Values

Stress range is the absolute difference between recovered mechanics components.

Axial range:

`abs((180.0 / 12.0) - (60.0 / 12.0)) = 10.0`

Bending-y range:

`abs((80.0 / 25.0) - (-20.0 / 25.0)) = 4.0`

Torsional shear range:

`abs((60.0 * 2.0 / 80.0) - (20.0 * 2.0 / 80.0)) = 1.0`

| Result | Value | Dimension |
|---|---:|---|
| `axial_normal_range` | 10.0 | force_per_area |
| `bending_normal_y_range` | 4.0 | force_per_area |
| `torsional_shear_range` | 1.0 | force_per_area |

## Boundary

This is a mechanics-only range comparison. It is not a fatigue assessment,
allowable comparison, design-code check, or professional approval.
