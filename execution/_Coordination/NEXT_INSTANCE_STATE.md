# NEXT INSTANCE STATE

## Active Phase

**Phase:** `TP-PER-01 Project Persistence And Run History Spine` implemented
**Last updated:** 2026-05-10
**Current posture:** TP-PER-01 has been implemented from the closed TP-MAC-05
mechanics and TP-RUN-01 runtime baselines. The next instance should preserve
the TP-MAC-05/TP-RUN-01/TP-PER-01 product-preview baseline unless a new
governed tranche is opened.

Historical REV05 evidence-closure and TP-MAC-01 desktop-hardening material is
retained for audit traceability only. Do not reload it unless a future task
explicitly asks for DAG/evidence/lifecycle reconciliation.

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
- `ResultItem.metadata` for local force/moment and stress components,
  coordinate system, endpoint location, recovery basis, and sign convention;
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
  mechanics, and release/professional claims remain out of scope.

**TP-PER-01 Project Persistence And Run History Spine**
Plan: `plans/TP-PER-01_PROJECT_PERSISTENCE_AND_RUN_HISTORY_PLAN.md`
Purpose: Added a schema-shaped persistence service for invented/cleared project
payloads, run-history refs, deterministic hashes, and round-trip validation.

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

## Active Plan

No successor implementation tranche is open in this compact state. Open a new
governed plan before adding desktop save/open UX, final CLI syntax, physical
project containers, migrations, external execution, new mechanics, or
professional acceptance workflows.

## Deferred

Keep deferred unless a new governed plan explicitly scopes them:

- intermediate station recovery;
- shear force recovery;
- pressure-to-frame load conversion;
- equivalent/principal stress;
- thermal behavior;
- support stiffness completeness beyond current preview restraints;
- load-case algebra and user combinations;
- governed calculation-report generation;
- protected rule/code checks and private criteria handling.
- final CLI binary, command syntax, and package-script contract;
- desktop save/open UX, physical project containers, migrations, external
  storage, and durable stored run history beyond schema-shaped envelopes;
- external execution, public transport, adapter formats, local FEA/prover
  invocation, and external result ingestion;
- release-readiness, production-readiness, compliance, certification, sealing,
  approval, or professional acceptance claims.

## Primary Files For Current Baseline

Read these first:

1. `AGENTS.md`
2. `docs/CONTRACT.md`
3. `docs/IP_AND_DATA_BOUNDARY.md`
4. `plans/TP-RUN-01_PREVIEW_RUNTIME_SPINE_PLAN.md`
5. `plans/TP-PER-01_PROJECT_PERSISTENCE_AND_RUN_HISTORY_PLAN.md`
6. `plans/TP-MAC-05_ENDPOINT_STRESS_COMPONENT_RESULTS_PLAN.md`
7. `core/product_physics/src/lib.rs`
8. `core/runner/headless/src/lib.rs`
9. `core/project_persistence/service.py`
10. `schemas/project_persistence.schema.yaml`
11. `fixtures/persistence/invented_persisted_preview_project.json`
12. `fixtures/product_preview/invented_preview_model.json`
13. `fixtures/product_preview/invented_mechanics_result.json`
14. `schemas/headless_runner.schema.yaml`
15. `schemas/results.schema.yaml`
16. `tests/test_project_persistence_service.py`
17. `tests/test_persistence_schema.py`
18. `tests/test_headless_runner_contract.py`
19. `tests/product_preview/test_product_preview_service.py`
20. `tests/test_analysis_run_records.py`

Only read REV05 tranche/DAG/evidence files if the current task is specifically
about governance traceability, evidence closure, dependency graph status, or
archival cleanup.

## Verification Baseline

The following commands passed after TP-MAC-05:

- `npm run generate:product-preview-mechanics`
- `cargo fmt --manifest-path core/product_physics/Cargo.toml`
- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo test --manifest-path core/loads/stress_recovery/Cargo.toml`
- `cargo test --manifest-path validation/benchmarks/stress/Cargo.toml`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py tests/test_results_schema.py`
- `python3 tests/test_results_schema.py`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- browser smoke against `http://127.0.0.1:5173/` using `apps/desktop/SMOKE.md`
- `git diff --check`

`npm run build --workspace apps/desktop` may report the existing Vite
chunk-size warning; that warning is not specific to TP-MAC-05.

The following commands passed after TP-RUN-01:

- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo test --manifest-path core/runner/headless/Cargo.toml`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py tests/test_results_schema.py`
- `python3 tests/test_headless_runner_contract.py`
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- browser smoke against `http://127.0.0.1:5173/` using `apps/desktop/SMOKE.md`
- `git diff --check`

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
