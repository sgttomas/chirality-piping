# NEXT INSTANCE PROMPT - TP-PER-01 Closeout Baseline

Continue OpenPipeStress development from the implemented TP-PER-01 project
persistence spine, the implemented TP-RUN-01 preview runtime spine, and the
closed TP-MAC-05 mechanics baseline.

This file is the handoff entry point. Use it to orient first, then read the
compact active context below.

## Required Reading

After this file, read only the compact active context in this order:

1. `plans/TP-PER-01_PROJECT_PERSISTENCE_AND_RUN_HISTORY_PLAN.md`
2. `plans/TP-RUN-01_PREVIEW_RUNTIME_SPINE_PLAN.md`
3. `plans/TP-MAC-05_ENDPOINT_STRESS_COMPONENT_RESULTS_PLAN.md`
4. `execution/_Coordination/NEXT_INSTANCE_STATE.md`
5. `execution/_Coordination/_COORDINATION.md`
6. `AGENTS.md`
7. `docs/CONTRACT.md`
8. `docs/IP_AND_DATA_BOUNDARY.md`

Then read only the implementation files needed for the requested task.

Do not reload the full REV05 tranche/DAG/evidence history by default. Those
files are retained for audit traceability, not as the normal starting context
for current product-preview work.

## Active Objective

Preserve the implemented schema-shaped persistence and preview runtime
baselines unless a new governed tranche explicitly changes them:

- `core/project_persistence` builds, validates, hashes, and round-trips
  `openpipestress.project_persistence` envelopes;
- persistence envelopes may carry run-history refs or records for model states,
  analysis runs, result envelopes, result refs, and hash manifests;
- the invented persisted-preview fixture uses a canonical-model payload and
  carries TP-RUN-01 result-envelope and endpoint-stress refs;
- the Tauri preview mechanics command accepts optional preview model input and
  keeps the zero-argument invented fixture fallback;
- the loaded React preview model is passed into the desktop runtime solve when
  Tauri is available;
- the in-memory headless runner bridge accepts structured runner metadata plus
  `LinearStaticPreviewRequest`;
- preserve TP-MAC-05 result IDs and mechanics behavior.

Do not implement desktop save/open UX, final CLI syntax, physical project
containers, migrations, external storage, external execution, new mechanics,
release claims, or professional acceptance workflows without a new governed
plan.

## Current Baseline

TP-PER-01 is implemented and closed in
`plans/TP-PER-01_PROJECT_PERSISTENCE_AND_RUN_HISTORY_PLAN.md`.

The persistence baseline now includes:

- schema-shaped project persistence service helpers;
- deterministic canonical JSON and SHA-256 hash manifests;
- validation diagnostics for missing provenance, private-data boundary,
  professional-boundary, run-history refs, and hash mismatches;
- optional schema support for model-state refs/records, analysis-run
  refs/records, result-envelope refs, result refs, and hash manifests;
- invented persisted-preview fixture with canonical model payload and
  TP-RUN-01 endpoint stress result refs.

TP-RUN-01 remains the closed runtime baseline. TP-MAC-05 remains the closed
mechanics baseline.

## Guardrails

- Use invented or cleared data only.
- Do not introduce protected standards content, owner data, private project
  data, allowables, SIF/flexibility tables, code criteria, or hidden
  engineering defaults.
- Do not claim compliance, certification, sealing, professional approval,
  release readiness, production readiness, or durable storage readiness.
- Do not allow agent proposals or review explanations to mutate accepted model
  state.
- Return explicit diagnostics or gap-ledger entries for unsupported or
  incomplete mechanics/persistence paths.
- Preserve existing result IDs unless a future governed plan explicitly adds a
  new ID pattern.

## Deferred

Keep deferred unless a new governed plan explicitly scopes them:

- desktop save/open UX;
- physical project containers and package formats;
- migrations and external storage;
- final CLI syntax and package-script contract;
- intermediate station recovery;
- shear force recovery;
- pressure-to-frame load conversion;
- equivalent/principal stress;
- protected rule/code checks and private criteria handling;
- governed calculation-report generation;
- professional acceptance workflows.

## Closeout State

TP-PER-01 verification passed on 2026-05-10:

- `python3 tests/test_persistence_schema.py`
- `python3 -m pytest tests/test_project_persistence_service.py`
- `python3 tests/test_model_state_schema.py`
- `python3 tests/test_analysis_run_schema.py`
- `python3 tests/test_analysis_run_records.py`
- `python3 tests/test_headless_runner_contract.py`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_results_schema.py`
- `python3 tests/test_results_schema.py`
- `python3 -m py_compile core/project_persistence/service.py core/project_persistence/__init__.py`
- `git diff --check`

No successor implementation tranche is open in this prompt; open a governed
plan before adding desktop save/open UX, final CLI syntax, physical project
containers, migrations, external storage, external execution, new mechanics,
release/professional claims, or professional acceptance workflows.
