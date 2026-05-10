# NEXT INSTANCE STATE

## Active Phase

**Phase:** `Post TP-MAC-08`
**Last updated:** 2026-05-10
**Current posture:** TP-MAC-08 has been implemented and closed as the first
governed explicit mechanics-basis load-combination preview slice. The
product-preview baseline now includes per-load-case solves, qualified
non-default load-case result rows, and explicit user linear combination rows
while preserving the closed TP-MAC-07 midspan, TP-MAC-06 thermal mechanics,
TP-MAC-05 endpoint mechanics, TP-RUN-01 runtime, and TP-PER-01 persistence
baselines.

Historical REV05 evidence-closure and TP-MAC-01 desktop-hardening material is
retained for audit traceability only. Do not reload it unless a future task
explicitly asks for DAG/evidence/lifecycle reconciliation.

## Latest Closed Plan

**TP-MAC-08 Code-Neutral Load Combination Preview**
Plan: `plans/TP-MAC-08_CODE_NEUTRAL_LOAD_COMBINATION_PREVIEW_PLAN.md`
Purpose: Add explicit user-defined mechanics-basis load-combination preview
rows using the existing load-case algebra crate and no code defaults.

Implemented:

- independent solves for every `load_cases[]` record;
- legacy unqualified result IDs preserved for the first/default load case;
- qualified non-default load-case rows under
  `result:loadcase:{load_case_suffix}:{base_result_tail}`;
- explicit combination rows under
  `result:combination:{combination_suffix}:{base_result_tail}`;
- `basis_ref` and `source_result_refs` audit metadata;
- combination scalar algebra through `core/loads/load_case_algebra`;
- structured diagnostics for invalid combinations and skipped stress summaries;
- desktop result/report display of explicit combination basis and source refs.

## Completed Baseline

The TP-MAC physics-first product path now includes:

- `core/product_physics` with stable entrypoint
  `run_linear_static_preview(request)`;
- validation that blocks missing primitive loads, missing pipe orientations,
  invalid units, duplicate/empty IDs, and missing invented/cleared provenance;
- computed preview displacements, support reaction resultants, local
  force/moment endpoint components, endpoint stress component rows,
  open-formula stress summaries where supported, and explicit diagnostics for
  unsupported paths;
- explicit material thermal expansion input, uniform axial thermal equivalent
  loads, and thermal fixed-end correction for straight-pipe temperature-change
  preview loads;
- deterministic midspan force, moment, and stress rows recovered from
  interpolated endpoint resultants;
- explicit mechanics-basis user load-combination rows from matching scalar
  load-case result rows;
- `ResultItem.metadata` for local force/moment and stress components,
  coordinate system, endpoint location, recovery basis, and sign convention;
- `ResultItem.basis_ref` and combination `source_result_refs`;
- generated fallback fixture workflow:
  `npm run generate:product-preview-mechanics`;
- desktop result grouping, result selection, diagnostic selection,
  model/viewport context selection, review-only proposal targeting, endpoint
  force/moment pairing, and a mechanics gap ledger;
- read-only report-packet context with DEL-14-02 analysis-run audit hashes;
- schema/service/test coverage for result metadata, analysis-run records, and
  schema-shaped project persistence envelopes with run-history refs.

## Closed Tranches

**TP-MAC-02 Physics-First Application Plan**
Plan: `plans/TP-MAC-02_PHYSICS_FIRST_APPLICATION_PLAN.md`
Purpose: Introduced the bounded invented-preview physics adapter and connected
the desktop app to computed mechanics results.

**TP-MAC-03 Result Interpretation And Review Workflow**
Plan: `plans/TP-MAC-03_RESULT_INTERPRETATION_AND_REVIEW_WORKFLOW_PLAN.md`
Purpose: Made result rows and diagnostics selectable and reviewable with
read-only interpretation, linked knowledge, source run context, and
professional-boundary messaging.

**TP-MAC-04 Endpoint Result Consumer And End-J Recovery**
Plan: `plans/TP-MAC-04_ENDPOINT_RESULT_CONSUMER_AND_END_J_RECOVERY_PLAN.md`
Purpose: Added end-j local force/moment preview results from the existing
straight-pipe 12-DOF local force vector and gave the desktop result detail
panel an endpoint-pair consumer.

**TP-MAC-05 Endpoint Stress Component Results**
Plan: `plans/TP-MAC-05_ENDPOINT_STRESS_COMPONENT_RESULTS_PLAN.md`
Purpose: Exposed open-mechanics stress components at pipe end-i and end-j in
the product preview workflow while preserving the existing pipe-level stress
summary result.

**TP-MAC-06 Uniform Thermal Axial Preview**
Plan: `plans/TP-MAC-06_UNIFORM_THERMAL_AXIAL_PREVIEW_PLAN.md`
Purpose: Added explicit invented material thermal expansion input, uniform
axial thermal equivalent loads, and thermal fixed-end correction for
straight-pipe temperature-change preview loads.

**TP-MAC-07 Midspan Station Preview**
Plan: `plans/TP-MAC-07_MIDSPAN_STATION_PREVIEW_PLAN.md`
Purpose: Added deterministic midspan force, moment, and stress rows for solved
straight-pipe preview elements using interpolated endpoint resultants.

**TP-MAC-08 Code-Neutral Load Combination Preview**
Plan: `plans/TP-MAC-08_CODE_NEUTRAL_LOAD_COMBINATION_PREVIEW_PLAN.md`
Purpose: Added explicit mechanics-basis user load-combination result rows over
solved preview load cases using existing load-case algebra.

**TP-RUN-01 Preview Runtime Spine**
Plan: `plans/TP-RUN-01_PREVIEW_RUNTIME_SPINE_PLAN.md`
Purpose: Unify desktop and headless preview runtime execution around
`core/product_physics` while preserving TP-MAC-05 mechanics behavior.

Delivered:

- Tauri preview mechanics command accepts optional preview model input and keeps
  the existing zero-argument fixture fallback.
- React preview service passes the loaded model into the Tauri solve path when
  available.
- Browser and Vitest fallback behavior remains fixture-backed.
- Headless runner has an in-memory structured preview bridge around
  `core/product_physics`.
- Runner results carry metadata, deterministic result refs, audit-manifest
  refs, privacy/professional-boundary flags, and hashes/checksums.
- Final CLI syntax, persistence, project containers, external execution, new
  mechanics outside governed TP-MAC plans, and release/professional claims
  remain out of scope.

**TP-PER-01 Project Persistence And Run History Spine**
Plan: `plans/TP-PER-01_PROJECT_PERSISTENCE_AND_RUN_HISTORY_PLAN.md`
Purpose: Added a schema-shaped persistence service for invented/cleared
project payloads, run-history refs, deterministic hashes, and round-trip
validation.

Delivered:

- `core/project_persistence` service helpers for building, validating, hashing,
  and canonical round-tripping persistence envelopes.
- Optional run-history support in `schemas/project_persistence.schema.yaml`
  for model-state refs, analysis-run refs or records, result-envelope refs,
  result refs, and hash manifests.
- Invented persisted-preview fixture backed by a canonical-model payload rather
  than directly wrapping the product-preview model.
- Deterministic SHA-256 hash coverage for project payloads, model payloads,
  hashless project envelopes, and optional embedded run-history records.
- Physical project container, desktop save/open UX, final CLI syntax,
  migrations, external storage, and professional acceptance remain out of
  scope.

## Selected Next Gate

No next governed tranche is selected after TP-MAC-08 closeout. Open a new
assessment or governed plan only when the next user request scopes one.

Latest prior assessment:
`execution/_Coordination/TP-MAC-08_NEXT_TRANCHE_ASSESSMENT.md`

## Deferred

Keep deferred unless a future governed plan explicitly scopes them:

- temperature-dependent material interpolation;
- code/rule combinations and protected combination criteria;
- code/rule thermal criteria, allowables, and protected checks;
- expansion-joint behavior;
- arbitrary station sweeps and exact internal force diagrams;
- shear force recovery;
- pressure-to-frame load conversion;
- equivalent/principal stress;
- support stiffness completeness beyond current preview restraints;
- governed calculation-report generation;
- protected rule/code checks and private criteria handling;
- final CLI binary, command syntax, and package-script contract;
- desktop save/open UX, physical project containers, migrations, external
  storage, and durable stored run history beyond schema-shaped envelopes;
- external execution, public transport, adapter formats, local FEA/prover
  invocation, and external result ingestion;
- release-readiness, production-readiness, compliance, certification, sealing,
  approval, or professional acceptance claims.

## Primary Files For Current Baseline

Read these first:

1. `plans/TP-MAC-08_CODE_NEUTRAL_LOAD_COMBINATION_PREVIEW_PLAN.md`
2. `plans/TP-MAC-07_MIDSPAN_STATION_PREVIEW_PLAN.md`
3. `plans/TP-MAC-06_UNIFORM_THERMAL_AXIAL_PREVIEW_PLAN.md`
4. `AGENTS.md`
5. `docs/CONTRACT.md`
6. `docs/IP_AND_DATA_BOUNDARY.md`
7. `plans/TP-MAC-05_ENDPOINT_STRESS_COMPONENT_RESULTS_PLAN.md`
8. `plans/TP-RUN-01_PREVIEW_RUNTIME_SPINE_PLAN.md`
9. `plans/TP-PER-01_PROJECT_PERSISTENCE_AND_RUN_HISTORY_PLAN.md`
10. `core/loads/load_case_algebra/src/lib.rs`
11. `core/product_physics/src/lib.rs`
12. `schemas/results.schema.yaml`
13. `fixtures/product_preview/invented_mechanics_result.json`
14. `fixtures/persistence/invented_persisted_preview_project.json`
15. `tests/product_preview/test_product_preview_service.py`
16. `tests/test_analysis_run_records.py`
17. `tests/test_results_schema.py`
18. `tests/test_project_persistence_service.py`

Only read REV05 tranche/DAG/evidence files if the current task is specifically
about governance traceability, evidence closure, dependency graph status, or
archival cleanup.

## Verification Baseline

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

The following commands passed after TP-PER-01:

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
