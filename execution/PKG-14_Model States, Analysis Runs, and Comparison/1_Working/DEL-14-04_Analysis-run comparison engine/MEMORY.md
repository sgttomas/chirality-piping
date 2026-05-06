# DEL-14-04 Memory

## Implementation Notes

- Added a narrow diagnostic analysis-run comparison module at `core/comparison/analysis_run/engine.py`.
- The engine accepts structured analysis-run records, result envelopes, mapping records, optional tolerance profile records, caller-supplied unit conversion factors, and optional settings maps.
- Result deltas preserve raw magnitude deltas separately from unit-normalized deltas and tolerance-profile classification.
- Unit normalization only occurs when dimensions match, units are explicit, and required conversion factors are supplied by the caller.
- Diagnostics are emitted for unresolved mappings, missing result data, incompatible dimensions, missing units, unsupported conversion paths, unsupported result categories, and carried run diagnostics.
- Output professional-boundary flags remain diagnostic/review oriented and do not make external validation or professional determinations.

## Verification Notes

- `python3 tests/test_analysis_run_comparison.py`
- `python3 tests/test_analysis_run_schema.py`
- `python3 tests/test_results_schema.py`
- `python3 tests/test_comparison_contracts.py`
- `python3 tests/test_units_schema.py`
- `python3 tests/test_model_state_schema.py`
- `git diff --check`
- Scoped protected/private/prohibited-claim scan over `core/comparison/analysis_run` and `tests/test_analysis_run_comparison.py`
