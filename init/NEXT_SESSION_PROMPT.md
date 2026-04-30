---
doc_id: OPS-NEXT-SESSION-PROMPT
doc_kind: init.next_session_prompt
status: ready_for_orchestrator_downstream_initialization_tranche_setup
updated: 2026-04-30
assignment: orchestrator_downstream_four_doc_initialization_tranche_setup
approved_decomposition: docs/_Decomposition/SOFTWARE_DECOMP.md
approved_revision: "0.4"
active_plan: plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md
scope_change_snapshot: execution/_ScopeChange/SCA-001_2026-04-30_0045/
---

# NEXT SESSION PROMPT - ORCHESTRATOR Downstream Four-Document Initialization

Continue as `ORCHESTRATOR` for the OpenPipeStress SOFTWARE workflow.

This is a normal ORCHESTRATOR control-loop run after `SCA-001` Gate 5. It is not SCA intake, not a scope-change amendment run, and not a CHANGE/file-state run unless the human explicitly redirects.

The active workflow plan remains:

`plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md`

However, the live filesystem scan found that downstream production four-document kits do not yet exist outside `PKG-00`. Therefore, interpret the plan's downstream "refresh" language as **initialization/creation of missing downstream four-document kits** for `PKG-01` through `PKG-12`, using the SCA-001-injected `_CONTEXT.md` files as the dispatch basis.

Do not implement product code. Do not compute blocked/unblocked states while the Full DAG is deferred. Do not edit downstream production documents until the human approves a concrete tranche/batch.

---

## Prerequisite Reading

Read in this order before acting:

1. `INIT.md`
2. `AGENTS.md`
3. `agents/AGENT_ORCHESTRATOR.md`
4. `agents/AGENT_TASK.md`
5. `agents/AGENT_DELIVERABLE_TASK.md`
6. `agents/AGENT_REVIEW.md`
7. `agents/AGENT_RECONCILIATION.md`
8. `agents/AGENT_AUDIT_DECOMP.md`
9. `docs/README.md`
10. `docs/DIRECTIVE.md`
11. `docs/CONTRACT.md`
12. `docs/TYPES.md`
13. `docs/SPEC.md`
14. `docs/IP_AND_DATA_BOUNDARY.md`
15. `docs/VALIDATION_STRATEGY.md`
16. `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`
17. `docs/_Decomposition/SOFTWARE_DECOMP.md`
18. `docs/_Registers/ScopeLedger.csv`
19. `docs/_Registers/Deliverables.csv`
20. `docs/_Registers/ContextBudgetQA.csv`
21. `execution/_Coordination/_COORDINATION.md`
22. `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`
23. `execution/_Coordination/NEXT_INSTANCE_STATE.md`
24. `execution/_ScopeChange/SCA-001_2026-04-30_0045/RUN_SUMMARY.md`
25. `execution/_ScopeChange/SCA-001_2026-04-30_0045/Handoff_State.md`
26. `plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md`

Do not use `docs/INIT.md`; it is retired.

---

## Current Facts

- Current decomposition: `docs/_Decomposition/SOFTWARE_DECOMP.md`
- Current revision: `0.4`
- SCA amendment ID: `SCA-001`
- SCA-001 status: Gate 5 executed
- Packages: `13`
- Deliverables: `73`
- Scope items: `63`
- Downstream contexts with SCA-001 injection: `65 / 65`
- Downstream contexts at accepted revision `0.4`: `65 / 65`
- `PKG-00` contexts remain at accepted revision `0.3`: `8 / 8`
- `PKG-00` lifecycle state: `8 / 8 SEMANTIC_READY`
- `PKG-01` through `PKG-12` lifecycle state: `65 / 65 OPEN`
- Four-document production files in `execution/`: `32`
- Four-document production files under `PKG-00`: `32 / 32`
- Downstream four-document kits outside `PKG-00`: `0 / 65 complete`, `0 / 65 partial`, `65 / 65 missing all four files`
- Coordination representation: Full DAG intended
- DAG authoring: deferred
- Blocker computation: disabled until a human-approved acyclic DAG exists
- `PKG-00` is not `ISSUED`
- SCA-001 did not edit downstream `Datasheet.md`, `Specification.md`, `Guidance.md`, or `Procedure.md`

---

## Authorized Work

Use `AGENT_ORCHESTRATOR.md` as the active role.

Authorized before human approval:

1. Read and analyze source files.
2. Confirm filesystem state and SCA-001 propagation coverage.
3. Confirm that downstream four-document kits are missing outside `PKG-00`.
4. Produce a concrete tranche proposal for downstream four-document kit initialization.
5. Recommend the first batch, defaulting to the plan's conservative order starting with `PKG-02` unless evidence suggests a better first batch.
6. Draft sealed TASK brief structure for the proposed first batch.
7. Identify which REVIEW, RECONCILIATION, AUDIT_*, and CHANGE handoffs are required.

Do not create or edit downstream `Datasheet.md`, `Specification.md`, `Guidance.md`, or `Procedure.md` until the human approves the concrete tranche/batch.

Authorized after explicit human batch approval:

1. Dispatch or execute one bounded TASK per approved deliverable.
2. Limit write scope to the approved deliverable folder.
3. Create only the authorized deliverable's four-document kit and required run/evidence records.
4. Run REVIEW on each deliverable or approved batch.
5. Run RECONCILIATION before expanding cross-package interface changes.
6. Run bounded audits after meaningful batches and final closeout.
7. Hand off accepted file state to CHANGE if needed.

---

## Required Architecture Basis

Use the SCA-001 architecture basis in `SOFTWARE_DECOMP.md` revision `0.4`:

- `AB-00-01`: ADR and decision-record discipline.
- `AB-00-02`: layer/module boundaries and no-bypass dependency rules.
- `AB-00-03`: command/query/job/result-envelope boundaries.
- `AB-00-04`: deterministic versioned persistence and canonical JSON/JCS-compatible hash basis.
- `AB-00-05`: GUI durable/transient state split, service-command mutation route, and scoped undo/redo.
- `AB-00-06`: diagnostics/result-envelope fields, warning classes, and no certification/compliance claims.
- `AB-00-07`: internal/public API, adapter, plugin, provenance, and privacy boundaries.
- `AB-00-08`: layered test and acceptance gates.

Resolved baseline choices:

- Rust core/application services.
- Tauri 2 desktop shell.
- TypeScript/React/Vite GUI.
- Three.js viewport where 3D viewport-facing.
- JSON Schema 2020-12 contracts.
- Schema-first command/query/job/result envelopes.
- Canonical JSON/JCS-compatible hash basis where JSON payloads are hashed.
- Cargo, Vitest, Playwright, validation, and protected-content/provenance test gates as applicable.

Remaining implementation-level TBDs:

- exact dependency versions;
- solver numerical library;
- rule expression grammar/library;
- public API transport;
- import/export format list;
- CI provider and coverage/performance thresholds;
- physical project package/container and migration framework.

Do not over-resolve these TBDs unless a sealed brief or explicit human ruling authorizes it.

---

## Required Workflow

1. Confirm current filesystem state:
   - 13 package folders;
   - 73 deliverable folders;
   - 63 scope-ledger rows;
   - 73 deliverables-register rows;
   - 73 context-budget rows;
   - 65 downstream `_CONTEXT.md` files with SCA-001 architecture-basis injection;
   - lifecycle distribution remains 65 `OPEN`, 8 `SEMANTIC_READY`;
   - four-document kit files exist only in `PKG-00`;
   - no downstream four-document kits exist yet.

2. Implement the ORCHESTRATOR tranche setup:
   - scan and report downstream four-document kit absence;
   - identify missing kit files for downstream deliverables;
   - report DAG/blocker computation as disabled;
   - propose a concrete first initialization tranche.

3. Use the plan's conservative batch order unless the scan proves a different first batch is safer:
   - `PKG-02`
   - `PKG-06`
   - `PKG-10`
   - `PKG-07`
   - `PKG-04` and `PKG-05`
   - `PKG-08`
   - `PKG-09`
   - `PKG-12`
   - `PKG-01`, `PKG-03`, and `PKG-11` as needed.

4. Produce the proposed first-batch TASK brief template:
   - one `DeliverableID`;
   - parent `PackageID`;
   - `ScopePath`;
   - scope items and objectives from `_Registers/Deliverables.csv`;
   - applicable invariants from `docs/CONTRACT.md`;
   - acceptance criteria from `_CONTEXT.md` or the sealed brief;
   - applicable `AB-00-*` basis IDs;
   - resolved SCA-001 baseline;
   - remaining implementation-level TBDs;
   - explicit deliverable-local write scope.

5. Stop and ask for human approval of the first concrete batch before creating downstream production documents.

---

## Guardrails

- Do not mark `PKG-00` as `ISSUED`.
- Do not compute blockers while the DAG is deferred.
- Do not create protected standards/code data or proprietary engineering values.
- Do not represent generated software results as certified, approved, sealed, or code-compliant for reliance.
- Do not copy full `PKG-00` prose into downstream artifacts.
- Do not bypass the Type 2 execution rule from `AGENTS.md`.
- Do not bulk-create multiple deliverables without sealed per-deliverable scope and human-approved batching.
- Treat `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` as evidence aids only, not independent engineering authority.

---

## Completion Report

Report concisely:

- package/deliverable/register counts;
- lifecycle distribution;
- SCA-001 context propagation coverage;
- confirmation that downstream four-document kits are absent outside `PKG-00`;
- proposed first initialization tranche;
- TASK brief shape for the first tranche;
- required REVIEW/RECONCILIATION/AUDIT/CHANGE handoffs;
- explicit confirmation that no downstream production documents were created or edited before batch approval.
