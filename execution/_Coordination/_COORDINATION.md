# Coordination Record

**Representation:** Full graph
**Dependency tracking mode:** FULL_GRAPH
**External schedule / coordination artifact:** N/A
**Semantic context maturity threshold:** SEMANTIC_READY
**DAG status:** APPROVED_FOR_DEVELOPMENT_COORDINATION_BASIS
**Accepted DAG:** `execution/_DAG/DAG-001/`
**DAG approval record:** `execution/_DAG/DAG-001/APPROVAL_RECORD.md`
**Blocker computation:** ENABLED from approved `ACTIVE` DAG edges only; `CANDIDATE` edges excluded; DEV-001 uses implementation-readiness semantics
**Default blocker satisfaction threshold:** `COMMITTED` implementation evidence
**Implementation evidence register:** `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`
**Active graph authority:** aggregate `execution/_DAG/DAG-001/DependencyEdges.csv`
**Accepted decomposition basis:** `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.5` accepted for SCA-002 downstream refresh planning
**Downstream refresh plan:** `plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`
**Current DAG status relative to revision 0.5:** `DAG-001` is historical revision `0.4` coordination evidence and stale for new dispatch until refreshed/accepted
**Local dependency registers:** synchronized mirrors/evidence materialized from `DAG-001`; not sequencing authority
**Quarantined reference corpus:** `docs/_ScopeChange/chirality-app-docs/` is read-only perspective material; not implementation scope, runtime architecture, UI requirement, dependency authority, or dispatch authority under SCA-002
**Pilot state:** `DEL-01-01` pilot completed; pattern accepted for bounded one-item execution, not broad fan-out
**Last bounded item:** `DEL-03-07 - Public/private library import provenance checker`
**Root next-session prompt posture:** Stable bootstrap; delegate current objective discovery to coordination state and latest human gate
**Next-instance prompt posture:** Stable protocol; derive current objective from mutable coordination state, `DAG-001`, and the latest human gate

## Human Rulings

- 2026-04-30 - Use a Full DAG coordination representation for the OpenPipeStress SOFTWARE workflow.
- 2026-04-30 - Superseded by `DAG-001` approval: the prior hold on authoring or inferring dependency edges applied before the governed DAG-authoring pass.
- 2026-04-30 - Use `SEMANTIC_READY` as the development-phase readiness threshold before downstream workflow proceeds.
- 2026-04-30 - Treat implementation-phase dependency planning as a separate future DAG over the implementation plan, not the current decomposition scaffold.
- 2026-04-30 - Add `PKG-00 - Software Architecture Runway` as the first workflow gate before `PKG-01` through `PKG-12`.
- 2026-04-30 - Approve `DAG-001` active acyclic edge set as the OpenPipeStress SOFTWARE development coordination basis.
- 2026-04-30 - Retain all `DAG-001` candidate edges as non-gating candidates pending later `RECONCILIATION`; do not promote candidate edges through this approval.
- 2026-04-30 - Select `DEL-01-01` as the first DEV-001 pilot dispatch candidate.
- 2026-04-30 - ORCHESTRATOR resolves the `DEL-01-01` write-scope gate and owns the control plane; `WORKING_ITEMS` is the persona for actual deliverable work.
- 2026-04-30 - Authorize repo-level write targets for the `DEL-01-01` pilot: `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, and `governance/MAINTAINERS.md`.
- 2026-04-30 - Enable blocker queue computation from the approved active DAG only.
- 2026-04-30 - Treat `PKG-00` as prerequisite architecture context processed through `SCOPE_CHANGE` and injected into downstream deliverable contexts; it may be left out of implementation dependency graphs and does not require deliverable-local `Dependencies.csv` files.
- 2026-04-30 - Accept the pushed DEV-001 hardening state in-session and authorize the `DEL-01-01` pilot handoff.
- 2026-04-30 - Complete the `DEL-01-01` pilot as commit `7650cf6 docs: tighten maintainer governance gates`; next execution should not broaden beyond one gated tranche without human review of the pilot behavior.
- 2026-04-30 - Accept the completed `DEL-01-01` pilot pattern and authorize exactly one next bounded DAG item: `DEL-02-01 - Canonical domain model schema`; broad fan-out remains prohibited.
- 2026-04-30 - Complete `DEL-02-01 - Canonical domain model schema` as commit `7b256f3 schema: tighten canonical domain model contract`, with handoff correction commit `8f57f85 docs: record del-02-01 commit handoff`.
- 2026-04-30 - Treat `NEXT_INSTANCE_PROMPT.md` as stable control-loop protocol. Agents derive the current objective from `NEXT_INSTANCE_STATE.md`, `_COORDINATION.md`, accepted `DAG-001` artifacts, the blocker queue when explicitly current, and the latest human approval gate instead of hard-coded next-deliverable language.
- 2026-04-30 - Treat `init/NEXT_SESSION_PROMPT.md` as a stable bootstrap entrypoint. It should direct fresh sessions into the coordination protocol and mutable handoff state, not encode the next deliverable objective.
- 2026-05-01 - Replace DEV-001 blocker queue semantics with an implementation-readiness view: `FromDeliverableID` is the downstream consumer blocked by `TargetDeliverableID`; only `COMMITTED` implementation evidence satisfies non-architecture upstreams; `SEMANTIC_READY` remains context readiness only; `PKG-00` `ARCHITECTURE_BASIS` edges are satisfied by the accepted architecture baseline; `CANDIDATE` edges remain excluded.
- 2026-05-03 - Accept corrected SCA-002 revision `0.5` for downstream refresh planning. `ORCHESTRATOR` may run planning from `plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`; `DAG-001`, blocker queues, dispatch briefs, lifecycle states, dependency mirrors, and implementation evidence remain stale relative to revision `0.5` until refreshed through later gates.
- 2026-05-03 - Classify `docs/_ScopeChange/chirality-app-docs/` as a quarantined reference corpus for SCA-002. It may inform governance perspective, but no Chirality app/harness implementation, UI/runtime work, SDK/provider integration, DAG edge, Type 2 dispatch, package-local context, lifecycle transition, or dependency-register change is authorized from that corpus without a later explicit scope change or architecture decision.

## Operating Rules

- Report lifecycle state and approved DAG coordination facts.
- Semantic readiness answers: is the task context prepared for bounded dispatch/review?
- Implementation readiness answers: can this item safely consume committed upstream artifacts?
- DEV-001 dispatch uses implementation readiness after the 2026-05-01 blocker-queue replacement.
- Compute implementation blocked/unblocked states only from approved `ACTIVE` DAG edges, the DEV-001 implementation evidence register, the `COMMITTED` evidence threshold, and satisfied `PKG-00` architecture-basis edges unless a later human ruling changes the rule.
- Do not use `CANDIDATE` edges for blocker queues, wave placement, readiness claims, schedule, staffing, or priority.
- `PKG-00` has reached the selected architecture readiness threshold; downstream product-development work still requires one sealed deliverable scope and explicit write targets.
- Do not require local dependency registers for `PKG-00`; use the scope-change/context-injection record as the evidence surface for `PKG-00` architecture basis.
- Treat a DEV-001 implementation projection that excludes `PKG-00` nodes and `ARCHITECTURE_BASIS` edges as a derived coordination view only; it does not replace `DAG-001` or remove `AB-00-*` brief injection.
- Treat non-`PKG-00` deliverable-local `Dependencies.csv` files as synchronized mirrors/evidence, not as independent sequencing authority. Resolve dependency ambiguity through `RECONCILIATION`; apply any committed file-state repairs through `CHANGE`.
- Keep dependency proposals labeled `PROPOSAL` until accepted by the human project authority.
- Keep `NEXT_INSTANCE_PROMPT.md` objective-neutral. Put mutable item state and immediate handoff facts in `NEXT_INSTANCE_STATE.md`; put durable coordination rulings here.
- Keep `init/NEXT_SESSION_PROMPT.md` objective-neutral. Use it only as the fresh-session bootstrap into the coordination control loop.
- Keep `NEXT_INSTANCE_STATE.md` in rotating handoff form: archive the previous latest completed task in the compact history table, then summarize only the just-completed task as the latest state.
- After SCA-002 acceptance, do not use `DAG-001`, old blocker queues, or old dispatch briefs for new Type 2 dispatch until a revision `0.5` refresh is planned, accepted, and materialized through the appropriate workflow gates.
- Treat the Chirality app docs corpus as quarantined reference material. Route any proposed promotion into OpenPipeStress through `SCOPE_CHANGE` or a human-approved architecture decision before development execution.
