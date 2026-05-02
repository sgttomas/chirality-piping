# Stress Recovery Benchmarks

This crate contains original stress recovery verification fixtures for
`DEL-09-02 - Stress recovery benchmark suite`.

The fixtures are public project content because their inputs, expected values,
and derivations are generated from elementary open mechanics within this
repository. They do not copy protected standards examples, code formulas,
commercial software benchmarks, proprietary engineering values, allowables,
SIF/flexibility factors, or code-specific acceptance criteria.

Numerical comparison values here are regression evidence for current
code-neutral stress recovery behavior. Release thresholds, final tolerance
policy, CI gate policy, stress range acceptance, fatigue criteria, and
professional reliance remain `TBD` pending human approval.

## Fixture Families

| Family | Fixture IDs |
|---|---|
| Axial normal stress | `STRESS-AXIAL-NORMAL-ORIGINAL` |
| Bending normal stress | `STRESS-BENDING-NORMAL-ORIGINAL` |
| Torsional shear stress | `STRESS-TORSIONAL-SHEAR-ORIGINAL` |
| Pressure membrane stress | `STRESS-PRESSURE-MEMBRANE-ORIGINAL` |
| Stress range | `STRESS-RANGE-MECHANICS-ORIGINAL` |

Hand-calculation notes are in `validation/hand_calcs/stress/`.
