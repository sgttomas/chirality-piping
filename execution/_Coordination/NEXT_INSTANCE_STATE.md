# NEXT INSTANCE STATE

## Active Phase

**Phase:** `TP-MAC-03 Result Interpretation And Review Workflow` closed
**Last updated:** 2026-05-10
**Current posture:** TP-MAC-03 is implemented and closed for the current desktop preview workflow. Result rows and diagnostics can be selected, selected results and diagnostics derive read-only interpretation view models, affected refs select model/viewport context where possible, selected-target review narratives remain non-mutating, and a mechanics gap ledger records deferred/unsupported capabilities. The workspace is ready for a new governed plan.

The previous `_Coordination/` root was dominated by REV05 evidence-closure and TP-MAC-01 desktop-hardening material. That material is now historical context for this phase unless a future task explicitly asks for DAG/evidence/lifecycle reconciliation.

## Completed Baseline

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
- surfaced DEL-14-02 analysis-run reproducibility/audit context in the desktop
  report packet, including the run ID, read-only immutability policy,
  result-value hash count, result-envelope hash presence, and professional
  boundary summary.
- checked current report/UI/review/export consumers for endpoint-j demand; no
  active workflow requires both element ends yet, so endpoint-j force/moment
  recovery remains deferred until a concrete consumer needs it.
- added `core.product_preview.build_report_packet_preview()` to materialize a
  deterministic read-only report-packet context outside the desktop UI, with
  selected computed result refs, diagnostic refs, DEL-14-02 run context, hashes,
  proposal status, privacy boundary, and explicit non-report/non-export status.

## Closed Tranche

**Tranche:** `TP-MAC-03 Result Interpretation And Review Workflow`

**Plan:** `plans/TP-MAC-03_RESULT_INTERPRETATION_AND_REVIEW_WORKFLOW_PLAN.md`

**Purpose:** Move from "computed preview results exist" to "a user can inspect,
understand, and review result meaning."

**Closeout posture:** The TP-MAC-03 acceptance path is satisfied. Continue the
TP-MAC physics-first product path with a new governed plan. Do not start
endpoint-j force/moment recovery until the new plan defines a concrete consumer
for both element ends.

**Scope:**

- Add desktop result selection and detail review.
- Derive a `ResultInterpretation` view model from current
  `MechanicsResult.results[]`.
- Show result metadata: family, entity ref, value/unit, component, coordinate
  system, endpoint/location, recovery basis, sign convention, diagnostics,
  linked knowledge, source run/audit context, and professional boundary.
- Link selected result rows to model/viewport context using `entity_ref`.
- Extend review-only proposal/context flow so selected results can produce
  read-only review explanations.
- Add a mechanics gap ledger covering endpoint-j recovery, station recovery,
  pressure-to-frame load conversion, thermal behavior, support stiffness
  completeness, load combinations, and protected rule/code checks.
- Preserve all TP-MAC boundaries: invented/cleared data only, no hidden
  defaults, no acceptance mutation, no compliance/certification/sealing/
  approval/release/production claims.

**Deferred:**

- Endpoint-j force/moment recovery remains deferred until this interpretation
  workflow creates a concrete consumer for both element ends.
- Formal DEL-08 report-generator/report-section promotion remains deferred
  unless a governed calculation-report artifact is explicitly needed.

**Acceptance checks:**

- Clicking `result:force:pipe-P-120:axial` renders a result detail panel with
  `axial_force`, `element_local`, `end_i`, recovery basis, sign convention, and
  `pipe:P-120`.
- Selecting a result highlights/selects its model entity where possible.
- Review narrative references the selected computed result id and remains
  non-mutating.
- Mechanics gap ledger lists endpoint-j recovery as deferred/not implemented,
  not as a compliance failure.
- Existing report-packet and DEL-14-02 audit context remain intact.

## TP-MAC-03 Implementation Slice Completed

The first result-interpretation slice is present in the working tree:

- added desktop-only `SelectedReviewTarget`, `ResultInterpretation`, and
  `MechanicsGap` types;
- added `apps/desktop/src/features/results/resultInterpretation.ts` to derive
  result details from `MechanicsResult.results[]` without changing public
  solver/result contracts;
- selecting `result:force:pipe-P-120:axial` renders a result detail panel with
  family, `pipe:P-120`, value/unit, `axial_force`, `element_local`, `end_i`,
  recovery basis, sign convention, linked diagnostics/knowledge, DEL-14-02 run
  context, result-value hash status, envelope-hash status, and professional
  boundary;
- selected result rows resolve `entity_ref` to known model entities and update
  model tree/property/viewport context when possible;
- review-only proposal generation now uses the selected result or diagnostic
  review target and continues to keep acceptance disabled;
- diagnostics can be selected as review targets without mutating accepted model
  state;
- selected diagnostics now render a detail panel with diagnostic ID/code,
  severity, source, affected refs, linked computed results, linked knowledge,
  review-only explanation, and professional boundary;
- selecting `diagnostic:physics:high-displacement-review` selects
  `node:N-140` / Terminal tie-in through affected refs and links back to
  `result:disp:node-N-140`;
- the mechanics gap ledger lists endpoint-j recovery, station recovery,
  pressure-to-frame load conversion, thermal behavior, support stiffness
  completeness, load combinations, and protected rule/code checks as deferred,
  not implemented, or requiring private inputs.

## Recommended Next Plan

Create a new governed plan for an endpoint/station recovery consumer before
adding new solver output. The next plan should decide how users will inspect
both element ends and station-level results in the desktop/report workflow, then
scope endpoint-j or station recovery against that concrete consumer.

Keep deferred unless explicitly scoped by the new plan:

- endpoint-j local force/moment recovery;
- intermediate station recovery;
- governed calculation-report generation;
- protected rule/code checks and private criteria handling.

## Primary Files For Next Work

Read these first:

1. `AGENTS.md`
2. `docs/CONTRACT.md`
3. `docs/IP_AND_DATA_BOUNDARY.md`
4. `plans/TP-MAC-03_RESULT_INTERPRETATION_AND_REVIEW_WORKFLOW_PLAN.md`
5. `plans/TP-MAC-02_PHYSICS_FIRST_APPLICATION_PLAN.md`
6. `core/product_physics/src/lib.rs`
7. `core/product_physics/src/validation.rs`
8. `fixtures/product_preview/invented_preview_model.json`
9. `fixtures/product_preview/invented_mechanics_result.json`
10. `apps/desktop/src-tauri/src/lib.rs`
11. `apps/desktop/src/App.tsx`
12. `apps/desktop/src/features/results/ResultsPanel.tsx`
13. `apps/desktop/src/features/knowledge/KnowledgePanel.tsx`
14. `apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx`
15. `apps/desktop/src/services/previewService.ts`
16. `apps/desktop/src/types.ts`
17. `apps/desktop/src/features/report/ReportPanel.tsx`
18. `core/product_preview/service.py`
19. `core/analysis_runs/records.py`
20. `schemas/analysis_run.schema.json`
21. `tests/product_preview/test_product_preview_service.py`
22. `tests/test_analysis_run_records.py`
23. `apps/desktop/src/App.test.tsx`
24. `package.json`
25. `apps/desktop/SMOKE.md`

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

Additional check after surfacing DEL-14-02 analysis-run context in the desktop
report packet:

- `python3 -m pytest tests/test_analysis_run_records.py tests/product_preview/test_product_preview_service.py`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- `git diff --check`
- `npm run dev --workspace apps/desktop`
- opened `http://127.0.0.1:5173/` in the in-app browser
- smoke result: PASS for the computed results, diagnostics, knowledge,
  DEL-14-02 report-packet audit context, review-only proposal, disabled accept
  control, and professional-boundary footer

Additional check after adding the read-only report-packet materialization path:

- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- `git diff --check`

Additional check after adding the TP-MAC-03 result-interpretation slice:

- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py`
- `git diff --check`
- `npm run dev --workspace apps/desktop -- --host 127.0.0.1`
- opened `http://127.0.0.1:5173/` in the in-app browser
- smoke result: PASS for axial result selection, result detail metadata,
  `pipe:P-120` model/viewport context selection, selected-result review-only
  proposal narrative, disabled accept control, DEL-14-02 audit context, and
  endpoint-j recovery listed as a deferred mechanics gap

Additional check after adding the TP-MAC-03 diagnostic-interpretation slice:

- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py`
- `git diff --check`
- `npm run dev --workspace apps/desktop -- --host 127.0.0.1`
- opened `http://127.0.0.1:5173/` in the in-app browser
- smoke result: PASS for `HIGH_DISPLACEMENT_REVIEW` diagnostic selection,
  linked `result:disp:node-N-140` value context, `node:N-140` model/viewport
  context selection, selected-diagnostic review-only proposal narrative, and
  disabled accept control

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

The live desktop smoke checklist has been run and passed after surfacing the
DEL-14-02 report-packet context. The rendered workflow showed the preview
shell/header, solve status, computed result groups, axial force result row,
diagnostics, knowledge context, report packet context including
`DEL-14-02; run:preview-linear-static-001`, result-value hash count,
`result_envelope` hash scope, generated review proposal, disabled accept
control, and technical-preview boundary text. If future UI changes touch this
workflow, rerun `apps/desktop/SMOKE.md`; if a checklist item fails, fix only
that bounded workflow issue and rerun the affected desktop checks.

## Recommended Next Work

Create a new governed plan before adding more mechanics output.

Recommended next plan: endpoint/station recovery consumer. Define how users
will inspect both element ends and station-level results in the desktop/report
workflow, then scope endpoint-j or station recovery against that concrete
consumer.
