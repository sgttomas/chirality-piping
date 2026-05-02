# Bending Normal Stress Fixture

Fixture ID: `STRESS-BENDING-NORMAL-ORIGINAL`

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
| Bending moment about local y | 50.0 | force_length |
| Section modulus about local y | 25.0 | length_cubed |
| Bending moment about local z | -30.0 | force_length |
| Section modulus about local z | 15.0 | length_cubed |

## Expected Values

Bending normal stress components are supplied bending moments divided by the
corresponding supplied section moduli.

`50.0 / 25.0 = 2.0`

`-30.0 / 15.0 = -2.0`

| Result | Value | Dimension |
|---|---:|---|
| `bending_normal_y` | 2.0 | force_per_area |
| `bending_normal_z` | -2.0 | force_per_area |

## Boundary

The sign convention follows the current stress-recovery API inputs. This
fixture does not encode design-code stress categories, stress indices,
allowables, or professional approval.
