---
doc_id: OPS-NEXT-SESSION-PROMPT
doc_kind: init.next_session_prompt
status: ready_for_dev001_acceptance_gate_and_del0101_pilot
updated: 2026-04-30
assignment: dev001_acceptance_gate_then_del0101_pilot
approved_decomposition: docs/_Decomposition/SOFTWARE_DECOMP.md
approved_revision: "0.4"
scope: DEV-001 hardened control plane, human acceptance gate, DEL-01-01 pilot handoff
next_workflow: ORCHESTRATOR gate; WORKING_ITEMS only after human acceptance
exclude_scope: broad fan-out, candidate edge promotion, lifecycle transitions, product solver implementation
---

# NEXT SESSION PROMPT - DEV-001 Acceptance Gate And DEL-01-01 Pilot

Continue as `ORCHESTRATOR` for the OpenPipeStress SOFTWARE workflow in a fresh session.

The previous session hardened the DEV-001 dependency control plane and pushed it to `origin/main`.

Pushed commit:

- `24b8012 chore: harden dependency control plane`

## Current Ground Truth

Treat these as current unless contradicted by newer filesystem or git evidence:

- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision `0.4` remains the active decomposition basis.
- `DAG-001` remains the execution sequencing and blocker-computation authority.
- Non-`PKG-00` local `Dependencies.csv` files are synchronized mirrors/evidence from `DAG-001`, not independent sequencing authority.
- `PKG-00` remains architecture context only and does not receive local `Dependencies.csv` files.
- Architecture-basis rows targeting `PKG-00` remain preserved in non-`PKG-00` local mirrors as injected context evidence.
- `CANDIDATE` edges remain non-gating and require later `RECONCILIATION` plus `CHANGE` before promotion.
- DEV-001 hardening verification passed for targeted tests, aggregate DAG audit, local register schema checks, and local closure audit.
- The broad repo test suite still had unrelated publication-test failures; do not treat those as blockers to the dependency-control-plane hardening unless the next task enters publication tooling.

## Required Reading

Before acting, read:

1. `INIT.md`
2. `AGENTS.md`
3. `agents/AGENT_ORCHESTRATOR.md`
4. `agents/AGENT_WORKING_ITEMS.md`
5. `agents/AGENT_TASK.md`
6. `agents/AGENT_REVIEW.md`
7. `agents/AGENT_RECONCILIATION.md`
8. `agents/AGENT_CHANGE.md`
9. `docs/CONTRACT.md`
10. `docs/IP_AND_DATA_BOUNDARY.md`
11. `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`
12. `docs/_Decomposition/SOFTWARE_DECOMP.md`
13. `docs/_Registers/Deliverables.csv`
14. `execution/_DAG/DAG-001/APPROVAL_RECORD.md`
15. `execution/_DAG/DAG-001/DEV-001_Aggregate_DAG_Audit.md`
16. `execution/_DAG/DAG-001/evidence/dev001_aggregate_dag_audit.json`
17. `execution/_DAG/DAG-001/evidence/dev001_local_materialization_summary.json`
18. `execution/_Reconciliation/Reconciliation_Run_Summary_2026-04-30_DEV001_CONTROL_PLANE_HARDENING.md`
19. `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_POST_MATERIALIZATION_2026-04-30/RUN_SUMMARY.md`
20. `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`
21. `execution/_Coordination/DEV-001_PILOT_DISPATCH_DEL-01-01.md`
22. `plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md`

After reading, state that reading is complete, summarize the governing constraints, and check `git status --short`.

## First Gate

Do not launch `WORKING_ITEMS`, dispatch `TASK`, or edit `DEL-01-01` targets until the human project authority accepts the pushed DEV-001 hardening state.

If the human has not explicitly accepted the hardening state in the new session, ask for that acceptance directly and stop before dispatch.

If accepted, record the acceptance in the session closeout and proceed to the `DEL-01-01` pilot handoff.

## DEL-01-01 Pilot Rule

Use `execution/_Coordination/DEV-001_PILOT_DISPATCH_DEL-01-01.md` as the controlling pilot brief.

Pilot constraints:

- `ORCHESTRATOR` owns the dispatch gate.
- `WORKING_ITEMS` owns the actual deliverable work after the acceptance gate.
- Use one `WORKING_ITEMS` session for `DEL-01-01`.
- Dispatch at most one bounded `TASK` from that session.
- Do not do broad fan-out.
- Do not promote candidate edges.
- Do not recompute or alter the blocker queue unless explicitly assigned.
- Do not change lifecycle state unless explicitly authorized.

Authorized repo-level write targets for the `DEL-01-01` pilot:

- `docs/CONTRACT.md`
- `docs/DIRECTIVE.md`
- `governance/MAINTAINERS.md`

Deliverable path:

- `execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-01_Project governance baseline`

Applicable architecture basis:

- `AB-00-01`
- `AB-00-02`
- `AB-00-06`
- `AB-00-08`

## Guardrails

- No protected standards text, protected code data, proprietary engineering values, private project data, real secrets, or private libraries.
- No certification, sealing, approval, code-compliance, or professional-reliance claims.
- No lifecycle transition without explicit human authorization.
- No broad implementation work outside `DEL-01-01`.
- Keep candidate edges non-gating.
- Preserve `DAG-001` as sequencing authority and local `Dependencies.csv` files as mirrors/evidence.
- Route dependency ambiguity to `RECONCILIATION`.
- Route committed file-state changes through `CHANGE`.

## Expected Closeout

Closeout must include:

- whether human acceptance of DEV-001 hardening was granted;
- whether `DEL-01-01` pilot was launched or held;
- exact files changed, if any;
- tests/audits run and their results;
- any remaining human rulings or blockers.
