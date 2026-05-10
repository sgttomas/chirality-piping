---
doc_id: DEV-001-REV05-MACOS-DESKTOP-PRODUCT-ASSEMBLY-TRANCHE-PLAN
doc_kind: coordination.product_assembly_tranche_plan
status: sealed_briefs_prepared_dispatch_not_authorized
created: 2026-05-10
requested_by: human_project_authority
source_state: execution/_Coordination/NEXT_INSTANCE_STATE.md
graph_authority: execution/_DAG/DAG-002
rt1_archive_route: paused_by_human_instruction
sealed_briefs: prepared_2026-05-10
implementation_dispatch: not_authorized_not_performed
worker_launch: not_authorized_not_performed
graph_mutation: not_authorized_not_performed
candidate_promotion: not_authorized_not_performed
dependency_mirror_refresh: not_authorized_not_performed
lifecycle_evidence_change: not_authorized_not_performed
release_readiness_claim: not_claimed
production_readiness_claim: not_claimed
professional_acceptance_claim: not_claimed
---

# DEV-001 Rev 0.5 macOS Desktop Product-Assembly Tranche Plan

## Human Direction

The human project authority paused the `RT-1` evidence archive route and
approved preparation of a PKG/DEL-driven product-assembly implementation
tranche for a macOS desktop OpenPipeStress technical preview.

Target product shape:

- Tauri 2 desktop shell for macOS;
- React/Vite GUI;
- Three.js 3D centerline pipe rendering;
- invented-data piping model;
- model tree and property inspector;
- design-knowledge panel;
- bounded agentic proposal interface;
- schema-backed persistence;
- mechanics/diagnostics execution path;
- no protected standards data;
- no professional approval, certification, sealing, code-compliance, or
  production-readiness claim.

This artifact began as a tranche plan only. On 2026-05-10, sealed
implementation briefs were prepared for `TP-MAC-01-A` through `TP-MAC-01-F`.
That preparation does not dispatch implementation, launch workers, create a
desktop app, mutate `DAG-002`, promote candidates, refresh dependency mirrors,
or change lifecycle/evidence state.

## Product Milestone

Milestone ID: `TP-MAC-01`

Name: macOS desktop technical preview vertical slice

Goal: make the committed PKG/DEL evidence usable through one runnable desktop
workflow rather than continuing archive/release-planning recursion.

The preview should let a reviewer open a local desktop application, load one
invented piping design, inspect it in 3D and structured panels, run a bounded
mechanics/diagnostics path, view result/diagnostic summaries, and review a
non-mutating agentic model-operation proposal.

## Tranche Boundary

### In Scope

| Area | In-scope behavior |
|---|---|
| Desktop shell | Create or extend a Tauri 2 macOS-capable desktop app scaffold under a dedicated app path. |
| GUI | Build the first usable React/Vite engineering workspace, not a landing page. |
| 3D rendering | Render an invented centerline piping model with nodes, pipe segments, supports, and simple component markers in Three.js. |
| Model navigation | Show model tree, selected entity details, and read-only property inspection. |
| Design knowledge | Show user-supplied design-knowledge records and constraint/warning messages from invented data. |
| Persistence | Load/save the invented preview model through schema-backed local JSON files or an equivalent local package placeholder. |
| Mechanics/diagnostics | Execute a bounded mechanics/diagnostics path using existing committed solver/diagnostic evidence where possible; unsupported pieces must be explicit diagnostics, not silent defaults. |
| Agentic interface | Provide a bounded proposal panel that can create a structured proposed model operation, validate/diff it, and require user acceptance before any mutation. Initial implementation may keep accepted mutation disabled if persistence mutation is not ready. |
| Tests | Add focused tests for app build, schema fixture validity, model-to-render data mapping, proposal validation, and no protected/professional-claim text. |

### Out Of Scope

- Product release, signing, notarization, publishing, release matrix, or
  production-readiness claim.
- Professional engineering acceptance, certification, sealing,
  code-compliance, or project-specific reliance.
- ASME or other protected standards text, tables, examples, formulas,
  allowables, SIF/flexibility tables, owner standards, private rule packs, or
  proprietary project data.
- External solver/prover execution, commercial parser behavior, or external
  result ingestion.
- Full rule-pack code-check workflow.
- Full GUI feature completion beyond the first vertical slice.
- Promotion of `docs/_ScopeChange/chirality-app-docs/` into implementation
  scope.

## PKG/DEL Mapping

This tranche intentionally consumes existing committed deliverable evidence and
adds an integration surface. It should not re-open every DEL as independent
scope. The implementation briefs should cite these DELs as governing product
contracts.

| Package | Deliverables used | Role in vertical slice |
|---|---|---|
| `PKG-07` GUI workflow | `DEL-07-01`, `DEL-07-02`, `DEL-07-04`, `DEL-07-05`, `DEL-07-06`, `DEL-07-07`, `DEL-07-08` | Primary desktop GUI: viewport, model tree/property inspector, warnings, results, accessibility baseline, solve execution UX, design-authoring/comparison workspace. |
| `PKG-10` build/API/interoperability | `DEL-10-04`, `DEL-10-05` | Desktop app scaffold/build route and headless/structured execution bridge. |
| `PKG-02` domain model/units/persistence | `DEL-02-01`, `DEL-02-02`, `DEL-02-03`, `DEL-02-05` | Schema-backed model, units, analysis-status boundaries, and local persistence. |
| `PKG-13` design knowledge/constraints | `DEL-13-01`, `DEL-13-02`, `DEL-13-03`, `DEL-13-04` | Design knowledge panel, constraint/warning messages, and physical-to-analytical transformation boundary. |
| `PKG-04` solver core | `DEL-04-01`, `DEL-04-02`, `DEL-04-03`, `DEL-04-06` | Mechanics/diagnostics path for the invented centerline model. |
| `PKG-05` loads/stress/status | `DEL-05-01`, `DEL-05-03`, `DEL-05-04`, `DEL-05-05` | Primitive/user load handling, fundamental stress/result summary, and explicit status semantics. |
| `PKG-16` model operations/agent proposals | `DEL-16-01`, `DEL-16-02`, `DEL-16-03`, `DEL-16-04` | Structured model operation proposal, validation/diff preview, user-acceptance boundary, and agent rationale guardrails. |

Adjacent dependency to consume without new write scope unless separately
authorized:

- `PKG-03` component/section/material schemas for invented pipe dimensions and
  component metadata. The first preview may use minimal invented records from
  existing fixtures and schemas; protected catalogs remain out of scope.

## Proposed Implementation Units

The tranche should be split into bounded implementation briefs with disjoint
write scopes.

| Unit | Primary DEL basis | Proposed write scope | Output |
|---|---|---|---|
| `TP-MAC-01-A` Desktop app scaffold | `DEL-10-04`, `DEL-07-06` | `apps/desktop/`, `apps/desktop/src-tauri/`, root package/workspace files only if required | Tauri 2 + React/Vite app scaffold that can run locally on macOS, with build/dev scripts and a restrained engineering workspace shell. |
| `TP-MAC-01-B` 3D model workspace | `DEL-07-01`, `DEL-07-02`, `DEL-07-08` | `apps/desktop/src/`, `apps/desktop/src/features/model-workspace/`, `fixtures/product_preview/` | Three.js centerline renderer, model tree, property inspector, design workspace layout, and invented model fixture. |
| `TP-MAC-01-C` Application service bridge | `DEL-02-01`, `DEL-02-02`, `DEL-02-05`, `DEL-10-05` | `core/application_service/` or `core/product_preview/`, `apps/desktop/src-tauri/src/`, `tests/product_preview/` | Local command/query bridge for loading/saving preview model, validation, and structured run request/result envelopes. |
| `TP-MAC-01-D` Design knowledge and diagnostics | `DEL-13-01`, `DEL-13-02`, `DEL-13-03`, `DEL-04-06`, `DEL-07-04` | `core/product_preview/`, `apps/desktop/src/features/knowledge/`, `tests/product_preview/` | Design-knowledge panel, constraint/warning feed, and explicit missing/unsupported diagnostics. |
| `TP-MAC-01-E` Mechanics execution slice | `DEL-04-01`, `DEL-04-02`, `DEL-04-03`, `DEL-05-01`, `DEL-05-03`, `DEL-05-04`, `DEL-05-05`, `DEL-07-05`, `DEL-07-07` | `core/product_preview/`, `apps/desktop/src/features/solve/`, `tests/product_preview/` | Bounded invented-data mechanics/diagnostics execution and result summary shown in GUI. Unsupported physics must be surfaced as diagnostics. |
| `TP-MAC-01-F` Agentic proposal panel | `DEL-16-01`, `DEL-16-02`, `DEL-16-03`, `DEL-16-04`, `DEL-07-08` | `core/product_preview/`, `apps/desktop/src/features/agent-proposals/`, `tests/product_preview/` | Non-autonomous proposal UI: proposed operation, rationale, validation/diff preview, and explicit user acceptance boundary. |

## Acceptance Criteria

The tranche is acceptable only if all applicable items pass:

- `apps/desktop` starts as a local desktop/web dev target and renders the
  primary workspace without a blank canvas.
- The preview opens an invented model fixture with no protected standards data
  and no private project data.
- The 3D view renders centerline pipe geometry, nodes, supports, and selected
  entity highlighting from structured model data.
- The model tree and property inspector reflect the same selected entity as
  the 3D view.
- The design-knowledge panel shows provenance-aware design knowledge and
  constraint/warning messages.
- The solve/diagnostics action returns a deterministic result envelope or an
  explicit unsupported/missing-data diagnostic.
- Result/status UI distinguishes mechanics result, rule-check status, and
  professional acceptance status.
- The agentic proposal panel cannot silently mutate accepted model state.
- Proposed operations include rationale, affected entity IDs, validation
  result, diff preview, and professional-boundary wording.
- Tests cover fixture validity, core service bridge behavior, UI state mapping
  where practical, and boundary text scans.
- No artifact claims production readiness, code compliance, certification,
  sealing, professional approval, external validation acceptance, or release
  readiness.

## Verification Plan

Expected verification after implementation:

```text
npm install
npm run dev --workspace apps/desktop
npm run build --workspace apps/desktop
npm test --workspace apps/desktop
python3 -m pytest tests/product_preview tests/test_model_schema.py tests/test_design_knowledge_schema.py tests/test_constraint_validation.py tests/test_agent_rationale_boundary.py
cargo test --manifest-path core/solver/frame_kernel/Cargo.toml
cargo test --manifest-path core/solver/straight_pipe/Cargo.toml
cargo test --manifest-path core/solver/linear_supports/Cargo.toml
cargo test --manifest-path core/solver/diagnostics/Cargo.toml
git diff --check
```

The exact commands may change after the app scaffold exists. If dependencies
must be installed from the network, that remains an execution-time approval
matter under the normal sandbox rules.

For GUI work, the final implementation review should include browser or
Playwright screenshots of the running workspace, including a nonblank Three.js
canvas.

## Dispatch Recommendation

Sealed implementation briefs have been prepared for `TP-MAC-01-A` through
`TP-MAC-01-F`. Worker launch remains a separate human gate.

Briefs:

- `execution/_Coordination/DEV-001_REV05_SEALED_BRIEF_TP-MAC-01-A.md`
- `execution/_Coordination/DEV-001_REV05_SEALED_BRIEF_TP-MAC-01-B.md`
- `execution/_Coordination/DEV-001_REV05_SEALED_BRIEF_TP-MAC-01-C.md`
- `execution/_Coordination/DEV-001_REV05_SEALED_BRIEF_TP-MAC-01-D.md`
- `execution/_Coordination/DEV-001_REV05_SEALED_BRIEF_TP-MAC-01-E.md`
- `execution/_Coordination/DEV-001_REV05_SEALED_BRIEF_TP-MAC-01-F.md`

The sealed briefs should:

- bind each unit to the DELs listed above;
- preserve disjoint write scopes;
- require invented data only;
- require no professional/release claims;
- include SCA-001 architecture basis IDs for service boundary, GUI state,
  diagnostics, API boundary, and layered tests;
- treat existing DEV-001 implementation evidence as reusable contract/support
  code, not as proof of full product runtime.

## Recommended Next Gate

```text
APPROVE: launch bounded implementation workers for TP-MAC-01-A through
TP-MAC-01-F from the sealed product-assembly briefs. Workers must keep the
brief write scopes disjoint or explicitly coordinated, preserve invented-data
and professional-boundary guardrails, and return implementation summaries and
verification results. Do not mutate DAG-002, promote candidate edges, refresh
dependency mirrors, change lifecycle/evidence state, create a release, or use
protected standards data.
```
