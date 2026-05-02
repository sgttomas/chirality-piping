---
doc_id: DEV-001-DISPATCH-DEL-07-01
doc_kind: coordination.dispatch_brief
status: sealed_brief_prepared
created: 2026-05-02
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-07-01
package_id: PKG-07
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-07-01 3D Viewport and Centerline Editor

**Dispatch status:** implementation committed as `4785806`; commit-backed
evidence promotion updated in working tree and awaiting commit.
**Coordination mode:** `FULL_GRAPH`
**Graph authority:** `execution/_DAG/DAG-001/DependencyEdges.csv`
**Implementation threshold:** upstream `COMMITTED` evidence

## Dispatch Decision

The human project authority authorized:

- a small coordination cleanup; and
- one next bounded DAG item: `DEL-07-01 - 3D viewport and centerline editor`.

This brief authorizes one bounded implementation pass for the first GUI
viewport/editor contract. Because the repository does not yet contain a Tauri,
React, Vite, or Three.js application scaffold, this implementation must not
invent a full app shell. It may define a schema-first viewport/editor contract,
add a bounded support module for viewport state, view primitives, selection,
and command-intent generation, add invented fixtures, add deterministic tests,
and update focused documentation. Actual Tauri/React/Three.js runtime
integration, package manifests, browser rendering, Playwright screenshots, and
production GUI styling remain downstream `TBD` work unless separately
authorized.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-07-01` |
| PackageID | `PKG-07` |
| Name | 3D viewport and centerline editor |
| Type | `UX_UI_SLICE` |
| Scope items | `SOW-020` |
| Objectives | `OBJ-006` |
| Context envelope | `L` |
| Context notes | GUI surface is broad but bounded to viewport/editor. |
| Deliverable path | `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| EdgeID | Upstream | Dependency type | Implementation-readiness satisfaction |
|---|---|---|---|
| `DAG-001-E0190` - `DAG-001-E0196` | Applicable `PKG-00` architecture basis | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DAG-001-E0478` | `DEL-02-01` Canonical domain model schema | `DOMAIN_MODEL` | `COMMITTED` evidence `7b256f3` |
| `DAG-001-E0479` | `DEL-02-02` Unit system and dimensional-analysis core contract | `UNIT_CONTRACT` | `COMMITTED` evidence `a458cba` |
| `DAG-001-E0480` | `DEL-02-05` Project persistence and round-trip serialization | `PERSISTENCE_CONTRACT` | `COMMITTED` evidence `742016e` |
| `DAG-001-E0481` | `DEL-03-02` Pipe section and component library schema | `DOMAIN_MODEL` | `COMMITTED` evidence `f0fdeac` |
| `DAG-001-E0482` | `DEL-03-03` Bend and elbow component model fields | `DOMAIN_MODEL` | `COMMITTED` evidence `7a84472` |
| `DAG-001-E0483` | `DEL-03-04` Branch connection component model fields | `DOMAIN_MODEL` | `COMMITTED` evidence `ae693b6` |
| `DAG-001-E0484` | `DEL-03-05` Rigid component models for valves, flanges, reducers, and specialty items | `DOMAIN_MODEL` | `COMMITTED` evidence `d8ee0db` |
| `DAG-001-E0485` | `DEL-03-06` Expansion joint component model | `DOMAIN_MODEL` | `COMMITTED` evidence `f15cbc6` |

Current implementation-readiness queue state:

- `DEL-07-01` is `UNBLOCKED`.
- `DEL-07-01` is recorded as `COMMITTED` evidence for `4785806` after
  post-commit evidence promotion.
- Candidate edges are excluded.

Downstream impact if later implemented and committed:

- `DEL-07-01` currently blocks `DEL-07-06` and `DEL-11-01` in the active
  implementation-readiness queue.

## Applicable Architecture Basis

Applicable basis IDs from `SCA-001`: `AB-00-01`, `AB-00-02`, `AB-00-03`,
`AB-00-05`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.

Resolved baseline to preserve: ADR traceability; module boundaries; Rust
core/application-service boundary; Tauri 2 desktop shell where GUI-facing;
TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D
viewport-facing; schema-first command/query/job/result envelopes; durable
project state separated from transient viewport, selection, camera, and
interaction state; diagnostics/result-envelope warning classes; API/plugin/
adapter no-bypass boundaries; and Cargo/Vitest/Playwright/validation/
protected-content gates as applicable.

Exact GUI dependency versions, frontend state library, physical project
container, final application-service transport, and runtime app shell remain
`TBD`.

## Explicit Write Scope

The bounded implementation write scope is limited to:

- `schemas/viewport_editor.schema.yaml`
- `core/gui/viewport_editor/`
- `fixtures/gui/invented/`
- `tests/test_viewport_editor_contract.py`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/MEMORY.md`
- `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_STATUS.md`
- `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/Dependencies.csv`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-07-01.md`
- `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch. In particular, do not create a
Tauri/React/Vite app shell, edit package manifests, edit aggregate `DAG-001`,
promote candidate edges, implement model tree/property inspector behavior,
implement material/component/rule-pack editors, implement solve execution UX,
implement results viewer behavior, add protected standards data, add
proprietary component/catalog values, or make professional/code-compliance
claims.

## Applicable Invariants

- `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`
- `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`
- `OPS-K-AUTH-1`
- `OPS-K-MECH-1`, `OPS-K-MECH-2`
- `OPS-K-UNIT-1`
- `OPS-K-PRIV-1`
- `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`, `OPS-K-AGENT-4`

## Acceptance Criteria

This bounded item is acceptable for implementation closeout when:

- `schemas/viewport_editor.schema.yaml` exists, remains strict JSON syntax
  parseable by Python `json`, and is traceable to `DEL-07-01`, `PKG-07`,
  `SOW-020`, and `OBJ-006`.
- The contract defines viewport/editor status, camera/transient state, selected
  entities, view primitives for nodes, pipe runs, bend arcs, branches, and
  simple component symbols, command-intent records, diagnostics, provenance,
  and professional-boundary fields.
- Durable model edits are represented as application-service command intents;
  the viewport contract does not directly mutate persisted project payloads.
- Transient camera, hover, selection, snapping, drag, and interaction state are
  clearly separated from durable project state.
- Coordinates and editable quantities are unit-aware and dimension-tagged, and
  missing or incompatible values produce diagnostics rather than silent
  defaults.
- The implementation does not bundle protected standards content, copied
  examples, component dimension tables, SIF/flexibility values, allowables,
  proprietary catalog values, private project data, private rule packs, private
  library data, or real secrets.
- The support module, if implemented, provides bounded in-memory viewport state
  and deterministic command-intent generation for creating nodes, connecting
  pipe runs, inserting bend symbols, inserting simple component symbols, and
  selecting entities. It must not render Three.js scenes, access host
  resources, parse project files, run GUI/CLI/API/adapter workflows, or claim
  engineering approval.
- Invented fixtures use synthetic non-engineering IDs and geometry only.
- Focused documentation updates in `docs/SPEC.md` and `docs/TYPES.md` describe
  only the viewport/editor boundary and keep full app shell/runtime integration
  as downstream or `TBD`.
- Deliverable `MEMORY.md` records the work, source basis, verification, and
  remaining open decisions.
- Verification includes `python3 tests/test_viewport_editor_contract.py`,
  `cargo fmt --manifest-path core/gui/viewport_editor/Cargo.toml -- --check`,
  `cargo test --manifest-path core/gui/viewport_editor/Cargo.toml`, dependency
  schema validation for the local mirror, aggregate DAG schema/audit checks,
  blocker queue rebuild after evidence promotion, `git diff --check`, and
  focused protected-content/private-secret/prohibited-claim scans.

## Stop Conditions

Stop and escalate if implementation requires:

- a frontend package/application scaffold;
- network or package downloads;
- real engineering examples or protected/proprietary component data;
- direct edits to adjacent GUI deliverables;
- aggregate `DAG-001` changes or candidate-edge promotion;
- public claims of certification, sealing, professional approval, or code
  compliance.

## Implementation Summary

Implemented from this sealed brief in the working tree on 2026-05-02:

- `schemas/viewport_editor.schema.yaml`
- `core/gui/viewport_editor/`
- `core/gui/viewport_editor/Cargo.lock`
- `fixtures/gui/invented/viewport_editor_session.json`
- `tests/test_viewport_editor_contract.py`
- focused `docs/SPEC.md` and `docs/TYPES.md` updates
- deliverable `MEMORY.md`, lifecycle display state, local dependency mirror,
  implementation evidence, blocker queue, and next-instance handoff state

The implementation remains contract-first because no frontend application
scaffold exists in the repository. The implementation/closeout patch was
committed as `4785806 schema: add viewport editor contract`; evidence-promotion
metadata is updated in the working tree and awaits a separate commit.

## Verification Summary

- `python3 tests/test_viewport_editor_contract.py` passed.
- `cargo fmt --manifest-path core/gui/viewport_editor/Cargo.toml -- --check`
  passed after formatting.
- `cargo test --manifest-path core/gui/viewport_editor/Cargo.toml` passed with
  6 unit tests.
- `python3 tests/test_model_schema.py` passed.
- `python3 tests/test_component_section_schema.py` passed.
- Local dependency schema validation passed for `DEL-07-01`.
- DEV-001 blocker queue rebuild passed with 68 unblocked / 5 blocked after
  `DEL-07-01` was recorded as `COMMITTED` evidence for `4785806`.
