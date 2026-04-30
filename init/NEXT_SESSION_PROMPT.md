---
doc_id: OPS-NEXT-SESSION-PROMPT
doc_kind: init.next_session_prompt
status: ready_for_orchestrator_full_software_setup_pkgs_01_12
updated: 2026-04-30
assignment: orchestrator_full_software_workflow_pkgs_01_12
approved_decomposition: docs/_Decomposition/SOFTWARE_DECOMP.md
approved_revision: "0.4"
scope: PKG-01 through PKG-12
exclude_scope: PKG-00
---

# NEXT SESSION PROMPT - ORCHESTRATOR Full SOFTWARE Workflow

Continue as `ORCHESTRATOR` for the OpenPipeStress SOFTWARE workflow.

Run the full SOFTWARE setup workflow for `PKG-01` through `PKG-12`, starting with the `four-documents` passes. This scope intentionally includes `PKG-02`; existing `PKG-02` four-document and semantic artifacts may be overwritten by the current workflow.

Do not rerun `PKG-00` unless the human explicitly requests it. Treat `PKG-00` as prior completed architecture-runway evidence, while still verifying its evidence before relying on it.

## First Action - Complete Required Reading

Before explaining a plan, dispatching a TASK, editing files, or summarizing state, complete this read list. The next session must explicitly say it has completed the reading task, then explain what it is going to do using that context.

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

After this read list is complete, report the governing workflow constraints and the intended execution plan before dispatching subagents.

## Required Workflow

Use the normal ORCHESTRATOR dispatch model from `AGENTS.md` and `agents/AGENT_ORCHESTRATOR.md`. ORCHESTRATOR must not author deliverable production content directly.

For each in-scope deliverable under `PKG-01` through `PKG-12`, dispatch bounded `TASK` work with explicit sealed scope:

1. `four-documents` with `RUN_PASSES=P1_P2`
2. `semantic-matrix-build`
3. `lens-register`
4. `four-documents` with `RUN_PASSES=P3_ONLY`
5. `dependency-extract`

After the setup workflow completes, run REVIEW, RECONCILIATION, and AUDIT over the produced evidence before any product implementation resumes.

## Guardrails

- No product implementation during this workflow.
- Use one deliverable per TASK dispatch.
- Every TASK dispatch must include `DeliverableID`, `PackageID`, register scope/objectives, applicable `docs/CONTRACT.md` invariants, acceptance criteria, and explicit write scope.
- Overwrite `PKG-02` artifacts as part of this current full SOFTWARE workflow.
- Do not use bulk scripts or ORCHESTRATOR-authored synthetic content as a replacement for TASK/subagent execution.
- Do not mark any deliverable `ISSUED`.
- Preserve protected-content, private-data, and no-certification boundaries.

## Completion Report

Report packages and deliverables processed, actual TASK/subagent dispatch evidence, files created or overwritten, validation/review/reconciliation/audit results, and any blockers or deviations.
