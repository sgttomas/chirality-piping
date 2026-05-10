# NEXT INSTANCE STATE

## Active Phase

**Phase:** `TP-MAC-02 Physics-First Application`
**Last updated:** 2026-05-10
**Current posture:** TP-MAC-02 live browser smoke has passed; result metadata is covered by DEL-08-04; preview mechanics results can now produce DEL-14-02 immutable analysis-run records. Continue physics-first product development by selecting one bounded continuation if needed.

The previous `_Coordination/` root was dominated by REV05 evidence-closure and TP-MAC-01 desktop-hardening material. That material is now historical context for this phase unless a future task explicitly asks for DAG/evidence/lifecycle reconciliation.

## Completed In Current Phase

The TP-MAC-02 implementation slice is present in the working tree:

- added `core/product_physics`;
- exposed `run_linear_static_preview(request)`;
- mapped invented preview nodes, straight pipes, supports, materials, and primitive loads into existing solver/load/stress crates;
- computed displacement magnitudes, support reaction resultants, and open-formula stress summaries where supported;
- returned diagnostics for unsupported or review-required paths;
- updated Tauri `run_preview_mechanics` to call the physics adapter;
- upgraded the invented preview fixture with explicit material, section, support, orientation, and primitive load inputs;
- updated the desktop result, diagnostics, knowledge, and review-only proposal flow to reference computed result/diagnostic IDs;
- added focused Rust, frontend, and product-preview tests;
- tightened `core/product_physics` validation so missing primitive loads and missing pipe orientation references block with explicit diagnostics instead of hidden mechanics defaults.
- added explicit unit validation and duplicate-ID diagnostics for active materials, nodes, pipes, supports, load cases, and primitive loads in `core/product_physics`.
- expanded the computed result envelope with local pipe force/moment component result IDs and connected desktop knowledge/proposal context to `result:force:pipe-P-120:axial`.
- added empty-ID and invented/cleared provenance validation in the physics adapter and preview service.
- improved under-restraint diagnostics to report restrained and missing global DOF classes.
- added service-level fixture parity checks for computed force/moment result IDs.
- grouped desktop result rows by displacement, reaction, force, moment, and stress.
- added structured result metadata for local force/moment component, coordinate system, endpoint location, recovery basis, and sign convention.
- promoted local force/moment result metadata into `schemas/results.schema.yaml` and
  `core/reporting/result_export` validation for DEL-08-04.
- added `core/analysis_runs` DEL-14-02 record support that builds immutable
  analysis-run records from computed preview mechanics results with result
  value hashes, result-envelope hashes, diagnostics, model-state refs,
  reproducibility notes, and professional-boundary fields.
- exposed `build_analysis_run_preview()` from `core/product_preview` so the
  preview service can bind computed result IDs to immutable run records.
- moved product-physics validation checks into `core/product_physics/src/validation.rs`.
- strengthened generated-vs-fallback fixture parity for force-result metadata.
- improved under-restraint diagnostics with support/node contribution details.
- current `ResultItem.metadata` covers component, coordinate system, endpoint location, recovery basis, and sign convention for local force/moment results.
- added a stable repo-level generation command for the computed fallback fixture:
  `npm run generate:product-preview-mechanics`.
- regenerated `fixtures/product_preview/invented_mechanics_result.json` from
  `core/product_physics/examples/preview_result.rs`.
- added stable desktop smoke selectors for the physics workflow and recorded
  the browser smoke checklist in `apps/desktop/SMOKE.md`.
- added a read-only desktop report-packet panel that consumes computed result
  IDs, diagnostics, proposal status, and professional-boundary status without
  exporting private data or claiming acceptance.

## Primary Files For Next Work

Read these first:

1. `AGENTS.md`
2. `docs/CONTRACT.md`
3. `docs/IP_AND_DATA_BOUNDARY.md`
4. `plans/TP-MAC-02_PHYSICS_FIRST_APPLICATION_PLAN.md`
5. `core/product_physics/src/lib.rs`
6. `core/product_physics/src/validation.rs`
7. `fixtures/product_preview/invented_preview_model.json`
8. `fixtures/product_preview/invented_mechanics_result.json`
9. `apps/desktop/src-tauri/src/lib.rs`
10. `apps/desktop/src/App.tsx`
11. `apps/desktop/src/features/results/ResultsPanel.tsx`
12. `apps/desktop/src/features/knowledge/KnowledgePanel.tsx`
13. `apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx`
14. `apps/desktop/src/services/previewService.ts`
15. `apps/desktop/src/types.ts`
16. `apps/desktop/src/features/report/ReportPanel.tsx`
17. `core/product_preview/service.py`
18. `core/analysis_runs/records.py`
19. `schemas/analysis_run.schema.json`
20. `tests/product_preview/test_product_preview_service.py`
21. `tests/test_analysis_run_records.py`
22. `apps/desktop/src/App.test.tsx`
23. `package.json`
24. `apps/desktop/SMOKE.md`

Only read REV05 tranche/DAG/evidence files if the current task is specifically about governance traceability, evidence closure, dependency graph status, or archival cleanup.

## Current Verification Baseline

The following commands passed after the TP-MAC-02 first slice:

- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml`
- `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml`
- `cargo test --manifest-path core/solver/linear_supports/Cargo.toml`
- `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml`
- `cargo test --manifest-path core/loads/stress_recovery/Cargo.toml`
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run build --workspace apps/desktop`
- `npm test --workspace apps/desktop`
- `python3 -m pytest tests/product_preview`
- `git diff --check`

Additional check after the explicit-input validation increment:

- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo fmt --manifest-path core/product_physics/Cargo.toml`
- `git diff --check`

Additional check after the explicit-unit and duplicate-ID validation increment:

- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo fmt --manifest-path core/product_physics/Cargo.toml`

Additional check after the force-component result envelope and UI linkage increment:

- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo fmt --manifest-path core/product_physics/Cargo.toml`
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- `python3 -m pytest tests/product_preview`
- `git diff --check`

Additional check after the remaining TP-MAC-02 hardening tasks:

- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo fmt --manifest-path core/product_physics/Cargo.toml`
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- `python3 -m pytest tests/product_preview`

Additional check after result-metadata, validation-module, fixture-parity, and diagnostics cleanup:

- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo fmt --manifest-path core/product_physics/Cargo.toml`
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- `python3 -m pytest tests/product_preview`

Additional check after adding the generated fallback-fixture workflow:

- `npm run generate:product-preview-mechanics`
- `cargo fmt --manifest-path core/product_physics/Cargo.toml`
- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `python3 -m pytest tests/product_preview`
- `npm test --workspace apps/desktop`

Additional check after adding stable desktop smoke selectors/checklist:

- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`

Additional check after adding the report-packet consumer:

- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`

Additional check after browser-capable handoff:

- `npm run dev --workspace apps/desktop`
- opened `http://127.0.0.1:5173/` in the in-app browser
- executed `apps/desktop/SMOKE.md`
- result: PASS

Live browser smoke passed on 2026-05-10. The rendered desktop workflow showed the preview shell/header, solve status, computed result groups, axial force result row, diagnostics, knowledge context, report packet context, generated review proposal, disabled accept control, and technical-preview boundary text. The accept control was verified by its actual `disabled` attribute; the footer boundary text uses readiness/compliance terms only in explicit `no ... claim` language.

Additional check after promoting force/moment result metadata into the DEL-08-04 export contract:

- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `python3 -m pytest tests/product_preview`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- `cargo fmt --manifest-path core/reporting/result_export/Cargo.toml`
- `cargo test --manifest-path core/reporting/result_export/Cargo.toml`
- `python3 tests/test_results_schema.py`

Additional check after adding DEL-14-02 preview analysis-run records:

- `python3 -m pytest tests/test_analysis_run_records.py tests/product_preview/test_product_preview_service.py`
- `python3 tests/test_analysis_run_schema.py`

## Active Boundaries

- No protected standards/code data, proprietary values, private project data, real secrets, private libraries, allowables, SIF/flexibility tables, or compliance criteria.
- No software claim of certification, sealing, professional approval, release readiness, production readiness, or code compliance.
- Agent proposals remain review-only and non-mutating.
- Mechanics inputs must be explicit; no hidden defaults.
- Unsupported solver, load, stress, rule, or professional paths must emit diagnostics.

## Coordination Archive

Moved out of the active root:

- `execution/_Coordination/_Archive/TP-MAC-01_DESKTOP_ASSEMBLY_2026-05-10/`

Copied pre-compaction active state:

- `execution/_Coordination/_Archive/TP-MAC-02_SUPERSEDED_ACTIVE_STATE_2026-05-10/`

Historical REV05 files still in `_Coordination/` are retained as audit records.

## Browser Smoke Result

The live desktop smoke checklist has been run and passed. If future UI changes touch this workflow, rerun `apps/desktop/SMOKE.md`; if a checklist item fails, fix only that bounded workflow issue and rerun the affected desktop checks.

## Recommended Next Work

Select one bounded continuation only if needed, for example:

- surface the DEL-14-02 analysis-run record in the desktop report packet only
  as read-only reproducibility/audit context.
- add endpoint-j force/moment result components only if the report/UI/review
  workflow needs both element ends.
