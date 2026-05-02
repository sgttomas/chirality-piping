# Axial Normal Stress Fixture

Fixture ID: `STRESS-AXIAL-NORMAL-ORIGINAL`

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
| Axial force | 120.0 | force |
| Area | 12.0 | area |

## Expected Value

Axial normal stress is the supplied axial force divided by supplied area.

`120.0 / 12.0 = 10.0`

| Result | Value | Dimension |
|---|---:|---|
| `axial_normal` | 10.0 | force_per_area |

## Boundary

This fixture verifies mechanics stress recovery only. It is not an allowable
comparison, code stress category, fatigue check, or professional approval.
