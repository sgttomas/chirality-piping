# Mechanics Benchmarks

This crate contains original mechanics verification fixtures for
`DEL-09-01 - Mechanics benchmark suite`.

The fixtures are public project content because their inputs, expected values,
and derivations are generated from elementary open mechanics within this
repository. They do not copy protected standards examples, commercial software
benchmarks, proprietary engineering values, or code-specific acceptance
criteria.

Numerical comparison values here are regression evidence for the current solver
mechanics. Release thresholds, final tolerance policy, CI gate policy, and
professional reliance remain `TBD` pending human approval.

## Fixture Families

| Family | Fixture IDs |
|---|---|
| Cantilever | `MECH-CANTILEVER-TIP-FORCE` |
| Frame | `MECH-PORTAL-SWAY-ORIGINAL` |
| Thermal growth | `MECH-FIXED-FIXED-THERMAL-AXIAL` |
| Imposed displacement | `MECH-IMPOSED-DISPLACEMENT-SPRING` |
| Stiffness transform | `MECH-INCLINED-MEMBER-TRANSFORM` |

Hand-calculation notes are in `validation/hand_calcs/mechanics/`.
