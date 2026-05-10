# NEXT INSTANCE STATE

## Active Phase

**Phase:** `TP-MAC-02 Physics-First Application`
**Last updated:** 2026-05-10
**Current posture:** continue physics-first product development from the implemented first slice.

The previous `_Coordination/` root was dominated by REV05 evidence-closure and TP-MAC-01 desktop-hardening material. That material is now historical context for this phase unless a future task explicitly asks for DAG/evidence/lifecycle reconciliation.

## Completed In Current Phase

The first TP-MAC-02 implementation slice is present in the working tree:

- added `core/product_physics`;
- exposed `run_linear_static_preview(request)`;
- mapped invented preview nodes, straight pipes, supports, materials, and primitive loads into existing solver/load/stress crates;
- computed displacement magnitudes, support reaction resultants, and open-formula stress summaries where supported;
- returned diagnostics for unsupported or review-required paths;
- updated Tauri `run_preview_mechanics` to call the physics adapter;
- upgraded the invented preview fixture with explicit material, section, support, orientation, and primitive load inputs;
- updated the desktop result, diagnostics, knowledge, and review-only proposal flow to reference computed result/diagnostic IDs;
- added focused Rust, frontend, and product-preview tests.

## Primary Files For Next Work

Read these first:

1. `AGENTS.md`
2. `docs/CONTRACT.md`
3. `docs/IP_AND_DATA_BOUNDARY.md`
4. `plans/TP-MAC-02_PHYSICS_FIRST_APPLICATION_PLAN.md`
5. `core/product_physics/src/lib.rs`
6. `fixtures/product_preview/invented_preview_model.json`
7. `apps/desktop/src-tauri/src/lib.rs`
8. `apps/desktop/src/App.tsx`
9. `apps/desktop/src/features/knowledge/KnowledgePanel.tsx`
10. `apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx`
11. `apps/desktop/src/services/previewService.ts`
12. `tests/product_preview/test_product_preview_service.py`
13. `apps/desktop/src/App.test.tsx`

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

Browser smoke testing was not completed because the required Browser plugin control callable was not exposed in the session.

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

## Recommended Next Work

The next task should be one bounded TP-MAC-02 physics increment, for example:

- improve support classification and under-restraint diagnostics;
- expand result envelope shape for element force components;
- add service-level parity tests between Tauri output and fallback fixtures;
- add model validation diagnostics for missing explicit mechanics inputs;
- improve the UI display of computed summaries and diagnostic references.
