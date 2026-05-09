---
doc_id: DEL-07-08-MEMORY
doc_kind: task.memory
status: draft
created: 2026-05-09
deliverable_id: DEL-07-08
package_id: PKG-07
worker: DEL-07-08
tranche: DEV-001_REV05_TRANCHE_M
revision: 0.5
---

# DEL-07-08 Memory

Implemented a deterministic Python contract builder at
`core/gui/design_workspace/` for GUI-facing design-authoring and comparison
workspace state. The builder composes existing upstream records into
inspectable panel records for:

- design knowledge panel state;
- constraint and warning presentation;
- immutable state and analysis-run browser summaries;
- model-state and analysis-run comparison tables;
- descriptor-only graphical overlays;
- operation diff-review routing.

The implementation is intentionally contract-facing only. It does not start a
live GUI runtime, execute solver/prover work, mutate accepted model state,
silently fill missing data, or make professional/code-compliance claims.

Invented public fixtures were added in
`tests/test_design_authoring_comparison_workspace.py`. The test consumes the
existing warning UX, constraint validation, model-state comparison,
analysis-run comparison, operation validation preview, and operation audit
contracts.

Key boundary choices:

- unavailable upstream inputs become explicit workspace diagnostics and
  unavailable panel state;
- operation rows remain `not_applied` at the workspace layer;
- accepted operation labeling requires an upstream audit record with explicit
  user acceptance and no accepted-state mutation;
- overlays are descriptors backed by upstream comparison/preview refs only;
- tolerance/profile refs, hashes, provenance, diagnostics, warnings,
  assumptions, privacy classification, unmatched rows, and unresolved `TBD`
  markers are preserved where supplied.
