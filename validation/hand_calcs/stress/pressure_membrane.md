# Pressure Membrane Stress Fixture

Fixture ID: `STRESS-PRESSURE-MEMBRANE-ORIGINAL`

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
| Pressure | 100.0 | force_per_area |
| Membrane radius | 3.0 | length |
| Wall thickness | 0.5 | length |

## Expected Values

The current stress-recovery mechanics boundary computes thin-wall pressure
membrane components from explicit pressure basis inputs.

Hoop component:

`100.0 * 3.0 / 0.5 = 600.0`

Longitudinal component:

`600.0 / 2.0 = 300.0`

| Result | Value | Dimension |
|---|---:|---|
| `pressure_hoop` | 600.0 | force_per_area |
| `pressure_longitudinal` | 300.0 | force_per_area |

## Boundary

This fixture does not provide pressure design criteria, code equations,
allowables, or professional approval.
