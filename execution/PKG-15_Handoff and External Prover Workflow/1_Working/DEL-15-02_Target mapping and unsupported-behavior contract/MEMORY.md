# MEMORY - DEL-15-02 Target mapping and unsupported-behavior contract

## 2026-05-06 Implementation Notes

- Implemented `core/handoff/target_mapping/contract.py` as a provider-neutral
  target mapping contract builder.
- Added deterministic diagnostics for missing handoff context, unsupported
  target mapping categories, missing unit metadata, and untraceable
  unsupported or approximate behavior.
- Preserved model hash, units manifest, entity IDs, library/rule references,
  unresolved assumptions, warnings, privacy context, provenance, and
  professional-boundary metadata as explicit contract fields.
- Added `tests/test_target_mapping_contract.py` for focused contract checks.

## Boundary Decisions

- The implementation does not select a target product, implement a
  target-specific commercial parser, invoke an external solver, or create
  downstream validation/professional reliance records.
- Unsupported and approximate behavior remains explicit and reviewable; the
  module emits diagnostics rather than filling missing target values with
  hidden defaults.
- Unit-bearing mappings require explicit unit and dimension metadata.
