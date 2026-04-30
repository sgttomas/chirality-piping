# NEXT INSTANCE PROMPT - ORCHESTRATOR Full SOFTWARE Workflow

Run the full SOFTWARE setup workflow for `PKG-01` through `PKG-12`, starting with the `four-documents` passes. Include `PKG-02` in the run and allow the current workflow to overwrite existing `PKG-02` four-document and semantic artifacts.

Do not rerun `PKG-00` unless explicitly instructed. Treat `PKG-00` as prior completed architecture-runway evidence, subject to evidence verification before use.

## First Action - Complete Required Reading

The next session must complete this read list before explaining a plan, dispatching TASKs, editing files, or summarizing state. After reading, it must state that the read list is complete and explain what it is going to do from that context.

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

## Dispatch Rule

Use the normal ORCHESTRATOR/TASK workflow. ORCHESTRATOR scans, prepares sealed briefs, dispatches bounded TASK subagents, verifies outputs, and routes REVIEW/RECONCILIATION/AUDIT. ORCHESTRATOR does not author deliverable production content directly.

For each deliverable in `PKG-01` through `PKG-12`, run:

1. `four-documents` with `RUN_PASSES=P1_P2`
2. `semantic-matrix-build`
3. `lens-register`
4. `four-documents` with `RUN_PASSES=P3_ONLY`
5. `dependency-extract`

Every TASK dispatch must be one deliverable, sealed, and explicit about `DeliverableID`, `PackageID`, register scope/objectives, `docs/CONTRACT.md` invariants, acceptance criteria, and write scope.

## Hard Stops

- No product implementation.
- No bulk script in place of TASK/subagent execution.
- No ORCHESTRATOR-authored synthetic deliverable content.
- No lifecycle transition outside skill-owned safe updates.
- No `ISSUED` state changes.
- No protected standards/code data, proprietary engineering values, or certification/compliance claims.

## Review Gate

After the setup workflow, run REVIEW, RECONCILIATION, and AUDIT against the produced evidence. Do not resume implementation until the resulting evidence is accepted by the human.
