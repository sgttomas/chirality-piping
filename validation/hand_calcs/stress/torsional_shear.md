# Torsional Shear Stress Fixture

Fixture ID: `STRESS-TORSIONAL-SHEAR-ORIGINAL`

## Provenance

- Source: OpenPipeStress original stress recovery benchmark.
- License basis: project-original-public-content.
- Contributor: OpenPipeStress agentic development workflow.
- Certification: generated from elementary open mechanics; not copied from
  protected standards, code formulas, commercial software examples, or
  proprietary data.

## Inputs

| Quantity | Value | Dimension |
|---|---:|---|
| Torsional moment | 40.0 | force_length |
| Torsion radius | 2.0 | length |
| Torsion constant | 80.0 | length_fourth |

## Expected Value

Torsional shear stress is supplied torque times supplied radius divided by
supplied torsion constant.

`40.0 * 2.0 / 80.0 = 1.0`

| Result | Value | Dimension |
|---|---:|---|
| `torsional_shear` | 1.0 | force_per_area |

## Boundary

This fixture verifies a mechanics shear stress component only. It is not a code
allowable comparison, fatigue criterion, or professional approval.
