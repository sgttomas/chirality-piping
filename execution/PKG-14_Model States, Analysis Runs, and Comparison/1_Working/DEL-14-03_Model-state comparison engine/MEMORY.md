# MEMORY - DEL-14-03 Model-state comparison engine

## 2026-05-05 Implementation Notes

- Added `core/comparison/model_state/engine.py` as a narrow, provider-neutral
  Python module for deterministic model-state entity comparison.
- Inputs are immutable model-state records or wrappers with explicit entity
  lists. The engine does not mutate accepted model states and does not compare
  analysis-run results.
- Stable identifiers are the primary matching basis. DEL-14-05-style entity
  mapping records are consumed for explicit counterparts where stable IDs do
  not match.
- Output classifications cover `added`, `removed`, `changed`, `unchanged`,
  `mapped_changed`, `mapped_unchanged`, and `unresolved`.
- Unit-bearing fields listed in comparison settings produce blocking
  diagnostics when changed without unit and dimension metadata, or when units
  or dimensions are incompatible without a governed normalization contract.
- Metadata preservation covers notes, external references, unresolved
  assumptions, warnings, hashes, provenance, and professional-boundary fields.
- No protected standards/code data, private project data, real secrets,
  commercial-prover ingestion, GUI/runtime behavior, analysis-run result
  deltas, or professional/code-compliance claims were added.
