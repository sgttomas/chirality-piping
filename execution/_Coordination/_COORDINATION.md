# Coordination Record

**Representation:** Full graph
**Dependency tracking mode:** FULL_GRAPH
**External schedule / coordination artifact:** N/A
**Semantic context maturity threshold:** SEMANTIC_READY
**DAG status:** `DAG-002` unapproved revision `0.5` proposal snapshot refreshed from targeted review and edge-disposition review
**Accepted DAG:** None for revision `0.5`; `execution/_DAG/DAG-001/` remains historical revision `0.4` evidence
**DAG approval record:** None for revision `0.5`; historical `DAG-001` approval remains at `execution/_DAG/DAG-001/APPROVAL_RECORD.md`
**Blocker computation:** HELD for revision `0.5` until explicit human graph approval; `CANDIDATE` edges excluded; DEV-001 uses implementation-readiness semantics only after graph approval
**Default blocker satisfaction threshold:** `COMMITTED` implementation evidence
**Implementation evidence register:** `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`
**Implementation evidence status projection:** `execution/_Coordination/DEV-001_REV05_IMPLEMENTATION_EVIDENCE_STATUS.csv`
**Active graph authority:** None approved for revision `0.5`; latest aggregate proposal is `execution/_DAG/DAG-002/DependencyEdges.csv`
**Accepted decomposition basis:** `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.5` accepted for SCA-002 downstream refresh planning
**Downstream refresh plan:** `plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`
**Current DAG status relative to revision 0.5:** `DAG-002` is a current unapproved proposal; `DAG-001` is historical revision `0.4` evidence and must not drive new revision `0.5` dispatch
**Current SCA-002 proposal surface:** `execution/_DAG/DAG-002/`, `execution/_Coordination/SCA-002_REV05_TARGETED_REVIEW_DEL-01-04_DEL-02-01.md`, `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md`, and `execution/_Coordination/SCA-002_DAG-002_PROPOSAL_PLAN.md`
**Current lifecycle snapshot:** `execution/_Coordination/REV05_LIFECYCLE_STATE_SNAPSHOT.csv`
**Current blocker queue:** `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md` / `.csv` hold-state queue; not readiness computation; not refreshed after the latest proposal-only DAG edge update
**Local dependency registers:** historical synchronized mirrors/evidence materialized from `DAG-001`; not sequencing authority; current status projection at `execution/_Coordination/DEV-001_REV05_DEPENDENCY_REGISTER_STATUS.csv`
**Quarantined reference corpus:** `docs/_ScopeChange/chirality-app-docs/` is read-only perspective material; not implementation scope, runtime architecture, UI requirement, dependency authority, or dispatch authority under SCA-002
**Pilot state:** `DEL-01-01` pilot completed; pattern accepted for bounded one-item execution, not broad fan-out
**Last bounded item:** `SCA-002 DAG-002 targeted review, edge disposition, and unapproved proposal refresh`
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
- 2026-05-03 - Authorize the SCA-002 `DAG-002` proposal-plan handoff closeout. The next fresh agent may create an unapproved revision `0.5` `DAG-002` proposal snapshot under `execution/_DAG/DAG-002/` using `execution/_Coordination/SCA-002_DAG-002_PROPOSAL_PLAN.md`, while preserving `DAG-001` as historical revision `0.4` evidence. This does not approve a graph, approve any active edge set, regenerate blockers, update lifecycle state, update implementation evidence, refresh dependency mirrors, dispatch Type 2 work, run `PREPARATION`, or promote Chirality corpus material.
- 2026-05-03 - Request current-state alignment for lifecycle, evidence, blocker queue, DAG, and dependency registers. The implemented alignment creates `execution/_DAG/DAG-002/` as an unapproved revision `0.5` proposal, replaces `DEV-001_BLOCKER_QUEUE.*` with a hold-state queue, and adds revision `0.5` lifecycle/evidence/dependency status projections. This does not approve a graph, approve any active edge set, compute implementation readiness, refresh deliverable-local dependency mirrors, dispatch Type 2 work, run `PREPARATION`, or promote Chirality corpus material.
- 2026-05-03 - Authorize targeted `REVIEW` for `DEL-01-04` and `DEL-02-01`, bounded `DAG-002` graph-authoring review for `DAG2-RD-001` through `DAG2-RD-016` and inherited candidate rows, and proposal-only update/validation after those decisions. This does not approve `DAG-002`, create `APPROVAL_RECORD.md`, compute blocker readiness, refresh deliverable-local dependency mirrors, change lifecycle state, dispatch Type 2 work, run `PREPARATION`, or promote Chirality corpus material.

## Operating Rules

- Report lifecycle state and approved DAG coordination facts.
- Semantic readiness answers: is the task context prepared for bounded dispatch/review?
- Implementation readiness answers: can this item safely consume committed upstream artifacts?
- DEV-001 dispatch uses implementation readiness after the 2026-05-01 blocker-queue replacement.
- Compute implementation blocked/unblocked states only from approved `ACTIVE` DAG edges, the DEV-001 implementation evidence register, the `COMMITTED` evidence threshold, and satisfied `PKG-00` architecture-basis edges unless a later human ruling changes the rule. No approved revision `0.5` graph currently exists, so blocker computation is held.
- Do not use `CANDIDATE` edges for blocker queues, wave placement, readiness claims, schedule, staffing, or priority.
- `PKG-00` has reached the selected architecture readiness threshold; downstream product-development work still requires one sealed deliverable scope and explicit write targets.
- Do not require local dependency registers for `PKG-00`; use the scope-change/context-injection record as the evidence surface for `PKG-00` architecture basis.
- Treat a DEV-001 implementation projection that excludes `PKG-00` nodes and `ARCHITECTURE_BASIS` edges as a derived coordination view only; it does not replace `DAG-001` or remove `AB-00-*` brief injection.
- Treat non-`PKG-00` deliverable-local `Dependencies.csv` files as synchronized mirrors/evidence, not as independent sequencing authority. Resolve dependency ambiguity through `RECONCILIATION`; apply any committed file-state repairs through `CHANGE`.
- Keep dependency proposals labeled `PROPOSAL` until accepted by the human project authority.
- Keep `NEXT_INSTANCE_PROMPT.md` objective-neutral. Put mutable item state and immediate handoff facts in `NEXT_INSTANCE_STATE.md`; put durable coordination rulings here.
- Keep `init/NEXT_SESSION_PROMPT.md` objective-neutral. Use it only as the fresh-session bootstrap into the coordination control loop.
- Keep `NEXT_INSTANCE_STATE.md` in rotating handoff form: archive the previous latest completed task in the compact history table, then summarize only the just-completed task as the latest state.
- After SCA-002 acceptance, do not use `DAG-001`, old dispatch briefs, or the `DAG-002` proposal for new Type 2 dispatch until a revision `0.5` graph is approved and downstream context surfaces are refreshed through the appropriate workflow gates.
- Treat `execution/_DAG/DAG-002/` artifacts as unapproved proposal evidence until a later human graph approval record exists. Proposal artifacts must not drive blocker computation, lifecycle state, Type 2 dispatch, schedule, staffing, priority, or implementation-readiness claims.
- Treat `execution/_Coordination/DEV-001_BLOCKER_QUEUE.*` as a current hold-state queue, not as a blocked/unblocked implementation-readiness queue, until explicit graph approval enables recomputation.
- Treat hold-state queue edge counts as last-refresh context only if the unapproved `DAG-002` proposal has changed after the queue was generated.
- Treat the Chirality app docs corpus as quarantined reference material. Route any proposed promotion into OpenPipeStress through `SCOPE_CHANGE` or a human-approved architecture decision before development execution.
