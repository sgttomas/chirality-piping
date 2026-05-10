# NEXT INSTANCE PROMPT - Post TP-MAC-03 Planning

Continue OpenPipeStress development after closing TP-MAC-03 result
interpretation and review workflow implementation.

This file is the handoff entry point. Use it to orient first, then read the
linked plan and supporting documents below.

## Required Reading

After this file, read only the compact active context in this order:

1. `plans/TP-MAC-03_RESULT_INTERPRETATION_AND_REVIEW_WORKFLOW_PLAN.md`
2. `AGENTS.md`
3. `docs/CONTRACT.md`
4. `docs/IP_AND_DATA_BOUNDARY.md`
5. `execution/_Coordination/NEXT_INSTANCE_STATE.md`
6. `execution/_Coordination/_COORDINATION.md`
7. `plans/TP-MAC-02_PHYSICS_FIRST_APPLICATION_PLAN.md`

Then read only the implementation files needed for the requested task.

Do not reload the full REV05 tranche/DAG/evidence history by default. Those
files are retained for audit traceability, not as the normal starting context
for the next TP-MAC plan.

## Active Objective

TP-MAC-03 is closed. The next objective is to create a new governed plan before
adding more mechanics output.

The recommended next plan is an endpoint/station recovery consumer tranche:

- define how users inspect both element ends and station-level results;
- identify the desktop/report consumer before adding endpoint-j solver output;
- preserve the existing result/diagnostic interpretation workflow as the
  consumer surface;
- keep endpoint-j and station recovery deferred until the new plan explicitly
  scopes them.

## Guardrails

- Use invented or cleared data only.
- Do not introduce protected standards content, owner data, private project
  data, allowables, SIF/flexibility tables, code criteria, or hidden
  engineering defaults.
- Do not claim compliance, certification, sealing, professional approval,
  release readiness, or production readiness.
- Do not allow agent proposals or review explanations to mutate accepted model
  state.
- Return explicit diagnostics or gap-ledger entries for unsupported or
  incomplete mechanics paths.
- Preserve `ResultItem.metadata` semantics when adding or consuming local
  force/moment result items.

## Current Gate

TP-MAC-02 live browser smoke passed on 2026-05-10 and forms the implementation
baseline. Local force/moment result metadata is covered by DEL-08-04, preview
mechanics results can produce DEL-14-02 immutable analysis-run records, and the
desktop report packet surfaces read-only DEL-14-02 audit context.

TP-MAC-03 is implemented and closed in
`plans/TP-MAC-03_RESULT_INTERPRETATION_AND_REVIEW_WORKFLOW_PLAN.md`.

The desktop interpretation workflow is implemented: result rows can be selected,
`ResultInterpretation` details are derived from
`MechanicsResult.results[]`, selected result rows update model/viewport context
through `entity_ref`, selected-result review narratives remain non-mutating,
diagnostics can be selected with affected-ref/linked-result detail, selected
diagnostics update model/viewport context where possible, selected-diagnostic
review narratives remain non-mutating, and the mechanics gap ledger lists
endpoint-j recovery as deferred.

Endpoint-j force/moment recovery is important but remains deferred until the
next plan creates a concrete consumer for both element ends.

## Closeout

When a task changes coordination state, update:

- `execution/_Coordination/NEXT_INSTANCE_STATE.md`
- `execution/_Coordination/_COORDINATION.md` only if the durable coordination
  posture changes

Keep active handoff concise. Archive bulky superseded phase material under
`execution/_Coordination/_Archive/` instead of expanding the active root
context.
