# NEXT INSTANCE PROMPT - TP-MAC-02 Physics-First Development

Continue OpenPipeStress development from the active TP-MAC-02 physics-first context.

## Required Reading

Read only the compact active context first:

1. `AGENTS.md`
2. `execution/_Coordination/_COORDINATION.md`
3. `execution/_Coordination/NEXT_INSTANCE_STATE.md`
4. `plans/TP-MAC-02_PHYSICS_FIRST_APPLICATION_PLAN.md`
5. `docs/CONTRACT.md`
6. `docs/IP_AND_DATA_BOUNDARY.md`

Then read only the implementation files needed for the requested task.

Do not reload the full REV05 tranche/DAG/evidence history by default. Those files are retained for audit traceability, not as the normal starting context for TP-MAC-02 implementation.

## Active Objective

Build out the real engineering workflow around the invented preview model:

- product physics adapter first;
- computed mechanics results and diagnostics;
- local force/moment result metadata with explicit coordinate system, endpoint location, basis, and sign convention;
- design knowledge linked to actual result/diagnostic IDs;
- review-only agent proposals generated from computed context;
- desktop/Tauri polish only where it supports the physics workflow.

## Guardrails

- Use invented or cleared data only.
- Do not introduce protected standards content, owner data, private project data, allowables, SIF/flexibility tables, code criteria, or hidden engineering defaults.
- Do not claim compliance, certification, sealing, professional approval, release readiness, or production readiness.
- Do not allow agent proposals to mutate accepted model state.
- Return explicit diagnostics for unsupported or incomplete mechanics paths.
- Preserve `ResultItem.metadata` semantics when adding or consuming local force/moment result items.

## Current Gate

The live browser smoke gate passed on 2026-05-10. Local force/moment result
metadata has been promoted into the DEL-08-04 result export contract, and
preview mechanics results can now produce DEL-14-02 immutable analysis-run
records. Choose one bounded continuation only:

- surface the DEL-14-02 analysis-run record in the desktop report packet only as read-only reproducibility/audit context;
- endpoint-j force/moment result components only if report/UI/review workflows require both element ends.

## Closeout

When a task changes coordination state, update:

- `execution/_Coordination/NEXT_INSTANCE_STATE.md`
- `execution/_Coordination/_COORDINATION.md` only if the durable coordination posture changes

Keep active handoff concise. Archive bulky superseded phase material under `execution/_Coordination/_Archive/` instead of expanding the active root context.
