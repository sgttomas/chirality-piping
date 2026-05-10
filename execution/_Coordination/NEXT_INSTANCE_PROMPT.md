# NEXT INSTANCE PROMPT - Post TP-MAC-08

Continue OpenPipeStress development from the closed TP-MAC-08 code-neutral
load-combination preview baseline, with the closed TP-MAC-07 midspan station
preview, TP-MAC-06 uniform axial thermal, TP-PER-01 project persistence,
TP-RUN-01 preview runtime, and TP-MAC-05 endpoint stress-component mechanics
baselines preserved.

This file is the handoff entry point. Use it to orient first, then read the
compact active context below.

## Required Reading

After this file, read only the compact active context in this order:

1. `plans/TP-MAC-08_CODE_NEUTRAL_LOAD_COMBINATION_PREVIEW_PLAN.md`
2. `plans/TP-MAC-07_MIDSPAN_STATION_PREVIEW_PLAN.md`
3. `plans/TP-MAC-06_UNIFORM_THERMAL_AXIAL_PREVIEW_PLAN.md`
4. `plans/TP-PER-01_PROJECT_PERSISTENCE_AND_RUN_HISTORY_PLAN.md`
5. `plans/TP-RUN-01_PREVIEW_RUNTIME_SPINE_PLAN.md`
6. `plans/TP-MAC-05_ENDPOINT_STRESS_COMPONENT_RESULTS_PLAN.md`
7. `execution/_Coordination/NEXT_INSTANCE_STATE.md`
8. `execution/_Coordination/_COORDINATION.md`
9. `execution/_Coordination/TP-MAC-08_NEXT_TRANCHE_ASSESSMENT.md`
10. `AGENTS.md`
11. `docs/CONTRACT.md`
12. `docs/IP_AND_DATA_BOUNDARY.md`

Then read only the implementation files needed for the next governed task.

Do not reload the full REV05 tranche/DAG/evidence history by default. Those
files are retained for audit traceability, not as the normal starting context
for current product-preview work.

## Current Objective

TP-MAC-08 is implemented. The next instance should preserve the explicit
mechanics-basis load-combination preview baseline and open a new governed plan
only if asked or if an explicitly provided sealed task scopes one. Preserve
the TP-MAC-08, TP-MAC-07, and TP-MAC-06 baselines:

- solve each preview load case independently;
- preserve legacy unqualified result IDs for the first/default load case;
- emit qualified non-default load-case rows using
  `result:loadcase:{load_case_suffix}:{base_result_tail}`;
- emit explicit combination rows using
  `result:combination:{combination_suffix}:{base_result_tail}`;
- combine only matching scalar rows using explicit user factors through
  `core/loads/load_case_algebra`;
- carry `basis_ref` and `source_result_refs` on combination rows;
- skip `open_formula_stress_summary` combination rows with diagnostics;
- emit deterministic `midspan` force, moment, and stress result rows for solved
  straight-pipe preview elements;
- compute midspan resultants by linear interpolation from TP-MAC-06 corrected
  endpoint local resultants;
- label midspan metadata with `location: "midspan"` and
  `basis: "interpolated_from_endpoint_resultants"`;
- keep endpoint-pair display endpoint-only;
- preserve explicit thermal expansion input, thermal equivalent loads, and
  thermal fixed-end correction from TP-MAC-06;
- preserve existing endpoint result IDs and TP-MAC-05 endpoint mechanics
  behavior except for value changes caused by explicit thermal input;
- keep arbitrary station sweeps, exact internal diagrams, shear recovery,
  code/rule combinations, and broader thermal behavior deferred unless a
  future governed plan scopes them.

Preserve the implemented schema-shaped persistence and preview runtime
baselines unless a future governed plan explicitly requires a narrow update:

- `core/project_persistence` builds, validates, hashes, and round-trips
  `openpipestress.project_persistence` envelopes;
- persistence envelopes may carry run-history refs or records for model states,
  analysis runs, result envelopes, result refs, and hash manifests;
- the Tauri preview mechanics command accepts optional preview model input and
  keeps the zero-argument invented fixture fallback;
- the loaded React preview model is passed into the desktop runtime solve when
  Tauri is available;
- the in-memory headless runner bridge accepts structured runner metadata plus
  `LinearStaticPreviewRequest`.

Do not implement desktop save/open UX, final CLI syntax, physical project
containers, migrations, external storage, external execution, arbitrary
station sweeps, pressure-to-frame conversion, shear recovery,
equivalent/principal stress, code/rule combinations, protected rule/code
checks, release claims, or professional acceptance workflows without a future
governed plan.

## Current Baseline

TP-MAC-08 is implemented and closed in
`plans/TP-MAC-08_CODE_NEUTRAL_LOAD_COMBINATION_PREVIEW_PLAN.md`.

TP-MAC-07 is implemented and closed in
`plans/TP-MAC-07_MIDSPAN_STATION_PREVIEW_PLAN.md`.

TP-MAC-06 is implemented and closed in
`plans/TP-MAC-06_UNIFORM_THERMAL_AXIAL_PREVIEW_PLAN.md`.

TP-PER-01 is implemented and closed in
`plans/TP-PER-01_PROJECT_PERSISTENCE_AND_RUN_HISTORY_PLAN.md`.

TP-RUN-01 remains the closed runtime baseline. TP-MAC-05 remains the closed
endpoint stress-component mechanics baseline.

The active product-preview baseline now includes:

- schema-shaped project persistence service helpers;
- deterministic canonical JSON and SHA-256 hash manifests;
- validation diagnostics for missing provenance, private-data boundary,
  professional-boundary, run-history refs, and hash mismatches;
- optional schema support for model-state refs/records, analysis-run
  refs/records, result-envelope refs, result refs, and hash manifests;
- invented persisted-preview fixture with canonical model payload and
  TP-RUN-01 endpoint stress, TP-MAC-07 midspan, and TP-MAC-08 combination
  result refs;
- endpoint force/moment and endpoint stress result rows from TP-MAC-05;
- explicit invented thermal input, uniform axial thermal equivalent loads, and
  thermal fixed-end correction from TP-MAC-06;
- deterministic midspan station force, moment, and stress rows from TP-MAC-07;
- explicit mechanics-basis user load-combination rows from TP-MAC-08.

## Guardrails

- Use invented or cleared data only.
- Do not introduce protected standards content, owner data, private project
  data, allowables, SIF/flexibility tables, code criteria, hidden engineering
  defaults, or private thermal property values.
- Do not claim compliance, certification, sealing, professional approval,
  release readiness, production readiness, durable storage readiness, or
  professional acceptance.
- Do not allow agent proposals or review explanations to mutate accepted model
  state.
- Return explicit diagnostics or gap-ledger entries for unsupported or
  incomplete mechanics/persistence paths.
- Preserve existing result IDs unless a future governed plan explicitly adds a
  new ID pattern.
- Thermal loads must not silently fall through the existing uniform
  force-per-length load path.

## Deferred

Keep deferred unless a future governed plan explicitly scopes them:

- temperature-dependent material interpolation;
- code/rule combinations and protected combination criteria;
- code/rule thermal criteria, allowables, and protected checks;
- expansion-joint behavior;
- desktop save/open UX;
- physical project containers and package formats;
- migrations and external storage;
- final CLI syntax and package-script contract;
- arbitrary station sweeps and exact internal force diagrams;
- shear force recovery;
- pressure-to-frame load conversion;
- equivalent/principal stress;
- protected rule/code checks and private criteria handling;
- governed calculation-report generation;
- professional acceptance workflows.

## Closing State

TP-MAC-08 was implemented and closed on 2026-05-10.

Final TP-MAC-08 verification:

- `cargo test --manifest-path core/loads/load_case_algebra/Cargo.toml`
- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `npm run generate:product-preview-mechanics`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py tests/test_results_schema.py tests/test_project_persistence_service.py`
- `python3 tests/test_results_schema.py`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- desktop browser smoke at `http://127.0.0.1:5173/`
- `git diff --check`

The desktop build no longer emits the prior Vite chunk-size warning after lazy
fixture loading and vendor chunk splitting.

TP-PER-01 closeout verification passed on 2026-05-10:

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
