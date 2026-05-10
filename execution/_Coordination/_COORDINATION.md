# Coordination Record

**Active phase:** `TP-MAC-02 Physics-First Application`
**Active objective:** TP-MAC-02 live browser smoke has passed, local force/moment result metadata is covered by DEL-08-04, and preview mechanics results can now produce DEL-14-02 immutable analysis-run records; future code work should continue the invented OpenPipeStress technical preview by selecting one bounded continuation if needed.
**Active plan:** `plans/TP-MAC-02_PHYSICS_FIRST_APPLICATION_PLAN.md`
**Latest state:** `execution/_Coordination/NEXT_INSTANCE_STATE.md`
**Historical archive:** `execution/_Coordination/_Archive/`

## Current Authority

The current development phase is no longer the `RT-1` evidence archive route or the `TP-MAC-01` desktop-hardening tranche. Those routes are historical context only.

The active authority for implementation is the physics-first plan:

- `plans/TP-MAC-02_PHYSICS_FIRST_APPLICATION_PLAN.md`
- `docs/CONTRACT.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `AGENTS.md`

Historical REV05/DAG coordination files remain available in this folder for audit traceability, but they are not the first context surface for TP-MAC-02 work. Do not reload the full historical coordination stack unless the next task explicitly asks for DAG/evidence/lifecycle reconciliation.

## Active Product Context

TP-MAC-02 has introduced a bounded product physics adapter:

- `core/product_physics/`
- `core/product_physics/src/validation.rs`
- stable entrypoint: `run_linear_static_preview(request)`
- repo-level fixture generation command: `npm run generate:product-preview-mechanics`
- Tauri command: `run_preview_mechanics`
- invented model fixture: `fixtures/product_preview/invented_preview_model.json`
- computed fallback result fixture: `fixtures/product_preview/invented_mechanics_result.json`
- browser smoke checklist: `apps/desktop/SMOKE.md`
- desktop app: `apps/desktop/`
- read-only report-packet consumer: `apps/desktop/src/features/report/ReportPanel.tsx`

The current computed result envelope includes:

- displacement magnitudes;
- support reaction resultants;
- local element force and moment components with metadata for component, coordinate system, endpoint location, recovery basis, and sign convention;
- open-formula stress summaries where the explicit preview inputs support them;
- explicit diagnostics for unsupported, incomplete, or review-required paths;
- professional-boundary and non-mutation fields.

The desktop flow now groups result rows by displacement, reaction, force, moment, and stress. Knowledge/proposal/report context is linked to computed result IDs, with `result:force:pipe-P-120:axial` as the current deterministic proposal target when available.

The formal DEL-08-04 result export surface now covers local force/moment
metadata in `schemas/results.schema.yaml` and validates required metadata in
`core/reporting/result_export`.

The DEL-14-02 analysis-run surface now includes `core/analysis_runs`, schema
support for individual `Result` references and `result_value` hashes, and a
`core/product_preview.build_analysis_run_preview()` service hook that binds
computed preview mechanics result IDs to immutable run records.

The adapter consumes invented preview data and existing mechanics crates:

- `core/solver/frame_kernel`
- `core/solver/straight_pipe`
- `core/solver/linear_supports`
- `core/loads/primitive_loads`
- `core/loads/stress_recovery`

## Boundaries

The active phase must preserve these constraints:

- invented or cleared public example data only;
- no protected standards tables, owner data, private project data, allowables, SIF/flexibility tables, compliance checks, certification, sealing, approval, release, or production-readiness claims;
- no autonomous accepted-model mutation from agent proposals;
- unsupported mechanics paths return explicit diagnostics instead of defaults;
- model validation blocks missing primitive loads, missing pipe orientation, invalid units, duplicate/empty IDs, missing invented/cleared provenance, and unsupported references;
- professional status remains `NOT_PROVIDED` / human review required.

## Archived Or Superseded Context

The TP-MAC-01 desktop assembly plan and sealed briefs have been moved to:

`execution/_Coordination/_Archive/TP-MAC-01_DESKTOP_ASSEMBLY_2026-05-10/`

The pre-compaction active coordination entrypoints were copied to:

`execution/_Coordination/_Archive/TP-MAC-02_SUPERSEDED_ACTIVE_STATE_2026-05-10/`

Older REV05 tranche, evidence, blocker, dependency, and DAG files remain in this directory as historical evidence. Treat them as audit records, not active implementation instructions for the physics-first phase.

## Next Gate

The live browser smoke gate has passed:

1. started `npm run dev --workspace apps/desktop`;
2. opened `http://127.0.0.1:5173/` in the in-app browser;
3. ran `apps/desktop/SMOKE.md`;
4. recorded result: PASS.

Any further work should continue from TP-MAC-02, not from the old desktop-hardening route:

1. surface the DEL-14-02 analysis-run record in the desktop report packet only as read-only reproducibility/audit context;
2. add endpoint-j force/moment result components only if the report/UI/review workflow needs both element ends;
3. only revisit macOS/Tauri polish when it supports the physics workflow.
