# Coordination Record

**Active phase:** `TP-MAC-03 Result Interpretation And Review Workflow` closed
**Active objective:** TP-MAC-03 moved the desktop workflow from computed preview results to selectable result/diagnostic interpretation and review-only context. The tranche is implemented and closed; the workspace is ready for a new governed plan.
**Active plan:** `plans/TP-MAC-03_RESULT_INTERPRETATION_AND_REVIEW_WORKFLOW_PLAN.md`
**Latest state:** `execution/_Coordination/NEXT_INSTANCE_STATE.md`
**Historical archive:** `execution/_Coordination/_Archive/`

## Current Authority

The current development phase is no longer the `RT-1` evidence archive route or the `TP-MAC-01` desktop-hardening tranche. Those routes are historical context only.

The latest closed implementation authority is the TP-MAC-03 result
interpretation plan, with TP-MAC-02 as the completed baseline:

- `plans/TP-MAC-03_RESULT_INTERPRETATION_AND_REVIEW_WORKFLOW_PLAN.md`
- `plans/TP-MAC-02_PHYSICS_FIRST_APPLICATION_PLAN.md`
- `docs/CONTRACT.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `AGENTS.md`

Historical REV05/DAG coordination files remain available in this folder for audit traceability, but they are not the first context surface for TP-MAC-03 work. Do not reload the full historical coordination stack unless the next task explicitly asks for DAG/evidence/lifecycle reconciliation.

## Active Product Context

TP-MAC-02 introduced a bounded product physics adapter:

- `core/product_physics/`
- `core/product_physics/src/validation.rs`
- stable entrypoint: `run_linear_static_preview(request)`
- repo-level fixture generation command: `npm run generate:product-preview-mechanics`
- Tauri command: `run_preview_mechanics`
- invented model fixture: `fixtures/product_preview/invented_preview_model.json`
- computed fallback result fixture: `fixtures/product_preview/invented_mechanics_result.json`
- browser smoke checklist: `apps/desktop/SMOKE.md`
- desktop app: `apps/desktop/`
- read-only report-packet consumer with DEL-14-02 audit context: `apps/desktop/src/features/report/ReportPanel.tsx`
- read-only report-packet materialization service: `core/product_preview/service.py`

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
computed preview mechanics result IDs to immutable run records. The desktop
report packet also displays the current run ID, read-only immutability policy,
result-value hash count, result-envelope hash presence, and professional
boundary summary without export, acceptance, or compliance claims.

The current report/UI/review/export consumers do not require endpoint-j local
force/moment results. Endpoint-j recovery remains deferred until a concrete
consumer needs both element ends. The current externalized preview report
packet is deliberately a read-only context packet, not a rendered calculation
report, result export payload, external handoff payload, or professional
acceptance record.

TP-MAC-03 built the interpretation layer around that baseline:

- desktop result selection and detail review are implemented for current
  preview results;
- a desktop-only `ResultInterpretation` view model is derived from
  `MechanicsResult.results[]`;
- linked result metadata, diagnostics, knowledge, source run/audit context, and
  professional-boundary display are surfaced in the result detail panel;
- selected result rows resolve `entity_ref` to known model entities in the tree
  and viewport where possible;
- review-only explanations can target selected results or diagnostics;
- selected diagnostics display affected refs, linked computed results, linked
  knowledge, and review-only explanation context;
- selected diagnostics can resolve affected model refs into model tree/property/
  viewport selection, as shown by `HIGH_DISPLACEMENT_REVIEW` selecting
  `node:N-140`;
- a mechanics gap ledger records deferred capabilities such as endpoint-j
  recovery, station recovery, pressure-to-frame load conversion, thermal
  behavior, support stiffness completeness, load combinations, and protected
  rule/code checks.

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

The first TP-MAC-03 live browser smoke has passed:

1. started `npm run dev --workspace apps/desktop -- --host 127.0.0.1`;
2. opened `http://127.0.0.1:5173/` in the in-app browser;
3. ran the TP-MAC-03 result-interpretation checks from `apps/desktop/SMOKE.md`;
4. recorded result: PASS for axial result selection, detail metadata,
   `pipe:P-120` model/viewport context selection, selected-result review-only
   proposal narrative, disabled accept control, DEL-14-02 audit context, and
   endpoint-j recovery listed as a deferred mechanics gap.

The TP-MAC-03 diagnostic-interpretation live browser smoke has also passed:

1. selected `diagnostic-HIGH_DISPLACEMENT_REVIEW`;
2. confirmed diagnostic detail context for
   `diagnostic:physics:high-displacement-review`, `result:disp:node-N-140`,
   and `node:N-140`;
3. confirmed model/viewport context selects `node:N-140` / Terminal tie-in;
4. generated a selected-diagnostic review-only proposal narrative;
5. verified the accept control remains disabled.

TP-MAC-03 is closed. Any further implementation should start from a new
governed plan, not from open-ended TP-MAC-03 continuation:

1. prefer a small endpoint/station recovery consumer plan that defines how
   users will inspect both element ends before adding endpoint-j solver output;
2. keep endpoint-j recovery deferred until that consumer exists;
3. keep report-generator/report-section promotion deferred unless the new plan
   explicitly needs a governed calculation-report artifact;
4. only revisit UI polish where it directly supports the next physics workflow.
