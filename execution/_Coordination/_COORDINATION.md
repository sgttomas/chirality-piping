# Coordination Record

**Active phase:** `TP-MAC-02 Physics-First Application`
**Active objective:** make the invented OpenPipeStress technical preview run a real bounded linear-static mechanics loop, then let diagnostics, design knowledge, and review-only agent proposals consume computed result context.
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
- stable entrypoint: `run_linear_static_preview(request)`
- Tauri command: `run_preview_mechanics`
- invented model fixture: `fixtures/product_preview/invented_preview_model.json`
- computed fallback result fixture: `fixtures/product_preview/invented_mechanics_result.json`
- desktop app: `apps/desktop/`

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
- professional status remains `NOT_PROVIDED` / human review required.

## Archived Or Superseded Context

The TP-MAC-01 desktop assembly plan and sealed briefs have been moved to:

`execution/_Coordination/_Archive/TP-MAC-01_DESKTOP_ASSEMBLY_2026-05-10/`

The pre-compaction active coordination entrypoints were copied to:

`execution/_Coordination/_Archive/TP-MAC-02_SUPERSEDED_ACTIVE_STATE_2026-05-10/`

Older REV05 tranche, evidence, blocker, dependency, and DAG files remain in this directory as historical evidence. Treat them as audit records, not active implementation instructions for the physics-first phase.

## Next Gate

Default next work should continue from TP-MAC-02, not from the old desktop-hardening route:

1. strengthen `core/product_physics` model coverage and diagnostics;
2. expand computed mechanics result surfaces only where explicit data exists;
3. connect frontend knowledge/proposal panels to actual result and diagnostic IDs;
4. add focused tests around physics adapter and service/UI behavior;
5. only revisit macOS/Tauri polish when it supports the physics workflow.
