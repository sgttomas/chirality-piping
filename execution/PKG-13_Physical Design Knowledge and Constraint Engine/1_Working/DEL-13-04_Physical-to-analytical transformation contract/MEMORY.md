# MEMORY - DEL-13-04 Physical-to-analytical transformation contract

## Implementation Notes

- Added a provider-neutral Python transform contract at `core/model_transform/physical_to_analytical/contract.py`.
- The transform accepts schema-shaped physical model mappings and derives an analytical `analytical_solver_model` view without mutating the physical source model.
- Deterministic behavior is enforced by stable record ordering, stable diagnostic ordering, and stable traceability ordering.
- Missing solve-required fields, unresolved references, missing or `TBD` unit metadata, unsupported element/support representations, and unresolved coordinate/DOF data emit diagnostics instead of inferred engineering defaults.
- Output stays within the project centerline/frame mechanics boundary. Unsupported physical-only records are omitted with traceable diagnostics.
- No protected standards data, owner rules, proprietary catalog values, private project data, external prover workflow, GUI/runtime behavior, or professional authority claims were added.

## Verification Notes

- Added `tests/test_physical_to_analytical_transform.py`.
- Focused cases cover deterministic output, source model preservation, traceability, missing-unit diagnostics, unsupported physical-record diagnostics, dependent-record omission after source omission, and prohibited authority-claim text scanning.
