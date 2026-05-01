# MEMORY - DEL-05-01 Primitive Load Case Engine

## Implementation Notes

- 2026-05-01: Implemented the bounded primitive load case engine under
  `core/loads/primitive_loads`.
- The slice defines mechanics-only primitive categories for weight, pressure,
  thermal, imposed displacement, hydrotest, wind, seismic, and occasional
  loads.
- The implementation prepares deterministic nodal, element-uniform, and
  imposed-displacement contributions for later consumers.
- Missing targets, missing magnitudes, invalid target ranges, invalid
  dimensions, invalid directions, and unsupported category/target combinations
  are findings, not silent defaults.

## Boundaries Preserved

- No code-specific load combinations were added.
- No protected standards data, proprietary engineering values, public catalog
  defaults, rule-pack checks, stress recovery, GUI behavior, headless runner, or
  professional/code-compliance claims were introduced.
- Wind and seismic are represented only as user-supplied equivalent mechanics
  loads. Dynamic or code-procedure generation remains `TBD`.

## Remaining TBDs

- Canonical calculation unit basis and conversion constants.
- Final result-envelope integration and concrete application-service API.
- Load-case storage representation.
- Wind/seismic dynamic treatment and any future lawful procedure generators.
- Production tolerance policy.
