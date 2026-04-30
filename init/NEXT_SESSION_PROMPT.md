---
doc_id: OPS-NEXT-SESSION-PROMPT
doc_kind: init.next_session_prompt
status: ready_for_clean_session_pkg12_finish_and_review
updated: 2026-04-30
assignment: orchestrator_finish_pkg12_then_review_reconcile_audit
approved_decomposition: docs/_Decomposition/SOFTWARE_DECOMP.md
approved_revision: "0.4"
scope: finish PKG-12, then REVIEW/RECONCILIATION/AUDIT
exclude_scope: PKG-00
---

# NEXT SESSION PROMPT - Finish PKG-12 and Post-Setup Checks

Continue as `ORCHESTRATOR` for the OpenPipeStress SOFTWARE workflow in a fresh session.

The prior session completed and package-validated `PKG-01` through `PKG-11` for the setup workflow. Do not restart those packages unless validation in this new session finds a concrete defect that requires remediation.

`PKG-12` was started in the prior session but encountered agent-thread-limit/background-agent behavior. Treat the current `PKG-12` state carefully as described below.

## First Action - Required Reading

Before dispatching any TASK, editing files, or summarizing a plan, read:

1. `INIT.md`
2. `AGENTS.md`
3. `agents/AGENT_ORCHESTRATOR.md`
4. `agents/AGENT_TASK.md`
5. `agents/AGENT_REVIEW.md`
6. `agents/AGENT_RECONCILIATION.md`
7. `agents/AGENT_AUDIT_DEP_CLOSURE.md`
8. `docs/CONTRACT.md`
9. `docs/_Decomposition/SOFTWARE_DECOMP.md`
10. `docs/_Registers/Deliverables.csv`
11. `docs/_Registers/ScopeLedger.csv`
12. `docs/_Registers/ContextBudgetQA.csv`
13. `skills/four-documents/SKILL.md`
14. `skills/semantic-matrix-build/SKILL.md`
15. `skills/lens-register/SKILL.md`
16. `skills/dependency-extract/SKILL.md`

After this read list is complete, explicitly report that reading is complete, summarize the governing constraints, and state the execution plan.

## Current Known State

Completed and package-validated in the prior session:

- `PKG-01` through `PKG-07`
- `PKG-08`
- `PKG-09`
- `PKG-10`
- `PKG-11`

`PKG-00` remains out of scope. Use it only as prior architecture-runway evidence where the decomposition says to inject `AB-00-*` constraints.

Known tool/process issue:

- `tools/validation/validate_id_format.sh` still expects legacy three-digit IDs and rejects current OpenPipeStress IDs such as `PKG-12` and `DEL-12-01`. Treat this as a tooling mismatch unless the canonical registers change.

## PKG-12 Handoff State

The prior session deleted the four-document artifacts from all five `PKG-12` deliverable folders because they were created in error:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`

Then only `DEL-12-01` was visibly relaunched. It recreated the four documents and reached `SEMANTIC_READY` on disk with fresh run records. The visible worker was shut down after the filesystem showed completion.

Current expected `PKG-12` state at handoff:

- `DEL-12-01_Local-first storage and private data paths`
  - Has recreated four documents.
  - Has `Dependencies.csv`, `_DEPENDENCIES.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `_STATUS.md`.
  - Has fresh run records for the five required phases.
  - Needs verification in the new session before accepting as complete.
- `DEL-12-02_Private data redaction and export controls`
  - Has stale non-four-document artifacts from the failed/background run.
  - Four documents should still be absent.
  - Treat all existing setup artifacts as stale and overwrite them by rerunning the full TASK workflow.
- `DEL-12-03_Telemetry off-by-default design`
  - Has stale non-four-document artifacts from the failed/background run.
  - Four documents should still be absent.
  - Treat all existing setup artifacts as stale and overwrite them by rerunning the full TASK workflow.
- `DEL-12-04_Secret and private-library handling`
  - Has stale non-four-document artifacts and stale run records from the failed/background run.
  - Four documents should still be absent.
  - Treat all existing setup artifacts as stale and overwrite them by rerunning the full TASK workflow.
- `DEL-12-05_Security threat model`
  - Has stale non-four-document artifacts and stale run records from the failed/background run.
  - Four documents should still be absent.
  - Treat all existing setup artifacts as stale and overwrite them by rerunning the full TASK workflow.

Before dispatching `PKG-12`, inspect the package folders and confirm this state. If a file state differs, report it and adapt conservatively.

## Required PKG-12 Workflow

For each `PKG-12` deliverable, dispatch bounded `TASK` work with explicit sealed scope and explicit write scope. Existing artifacts inside the assigned deliverable folder may be overwritten. Do not edit repo-level product artifacts.

Required sequence per deliverable:

1. `four-documents` with `RUN_PASSES=P1_P2`
2. `semantic-matrix-build`
3. `lens-register`
4. `four-documents` with `RUN_PASSES=P3_ONLY`
5. `dependency-extract`

Recommended start:

1. Verify `DEL-12-01` artifacts and run `validate_dependencies_schema.py` on its `Dependencies.csv`.
2. If `DEL-12-01` passes, do not rerun it.
3. Relaunch `DEL-12-02` through `DEL-12-05` as clean TASK dispatches, allowing overwrite inside each deliverable folder.
4. If any spawn call reports thread-limit or failed launch, stop dispatching immediately and inspect filesystem state before continuing.

## PKG-12 Guardrails

- No product implementation during this workflow.
- Do not mark any deliverable `ISSUED`.
- Preserve local-first design; no cloud operation unless separately approved.
- Telemetry default is no telemetry. Any telemetry design requires explicit human approval and must be opt-in.
- Do not create real secrets, real private libraries, real private project data, or protected standards content.
- Do not claim legal sufficiency, engineering certification, code compliance, approval, seal, or professional reliance.
- Keep all edits inside the assigned deliverable folder unless the human explicitly expands scope.

## After PKG-12

When `PKG-12` is complete:

1. Run package-level validation for `PKG-12`:
   - all five `_STATUS.md` files show `Current State: SEMANTIC_READY`;
   - every `Dependencies.csv` passes `tools/validation/validate_dependencies_schema.py`;
   - scan for `MatrixError: [1-9]`, `MATRIX_ERROR`, `run-status: PENDING`, `run-status: FAILED`, `FAILED_INPUTS`, `OPS-K_MISSING`, and `Current State: ISSUED`.
2. Run REVIEW over the setup evidence.
3. Run RECONCILIATION over cross-package dependencies, stale assumptions, terminology conflicts, and `PROPOSAL` dependency rows.
4. Run AUDIT dependency-closure checks.
5. Report package/deliverable completion, validation results, review/reconciliation/audit findings, and remaining human rulings.

Do not resume product implementation until REVIEW, RECONCILIATION, and AUDIT are complete.
