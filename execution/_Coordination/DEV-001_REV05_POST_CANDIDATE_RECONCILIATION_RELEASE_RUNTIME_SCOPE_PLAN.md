---
doc_id: DEV-001-REV05-POST-CANDIDATE-RECONCILIATION-RELEASE-RUNTIME-SCOPE-PLAN
doc_kind: coordination.next_phase_plan
status: proposal_only_complete
created: 2026-05-09
source_acceptance_commit: c0303c4
source_handoff_commit: cdd164c
source_reconciliation: execution/_Reconciliation/Reconciliation_Run_Summary_2026-05-09_DEV001_REV05_CANDIDATE_EDGE_RECONCILIATION.md
graph_authority: execution/_DAG/DAG-002
graph_mutation: not_authorized_not_performed
candidate_promotion: not_authorized_not_performed
dependency_mirror_refresh: not_authorized_not_performed
lifecycle_evidence_change: not_authorized_not_performed
type2_dispatch: not_performed
---

# DEV-001 Rev 0.5 Post-Candidate Reconciliation Plan

## Purpose

This plan follows the accepted DEV-001 revision `0.5` implementation-evidence
closure and the candidate-edge reconciliation run. It organizes the next
decision phase into release-readiness planning, runtime/product gap planning,
and scope-change triage.

This is not a release, production, professional, or full GUI/runtime readiness
claim. It does not mutate `DAG-002`, promote candidate rows, refresh dependency
mirrors, change lifecycle/evidence state, or dispatch Type 2 work.

## Candidate Reconciliation Inputs

Use the reconciliation recommendations as planning constraints:

- Retire recommended as active DAG dependency candidates:
  `DAG-002-E0616`, `DAG-002-E0617`, `DAG-002-E0618`,
  `DAG-002-E0619`, `DAG-002-E0620`, `DAG-002-E0622`, and
  `DAG-002-E0624`.
- Retain non-gating pending release-security review: `DAG-002-E0623`.
- Promote now: none.

Rows `DAG-002-E0619`, `DAG-002-E0620`, and `DAG-002-E0622` should not be
promoted directly because they participate in known candidate-layer SCC
warnings. Their useful content should be routed into review or gap planning.

## Release-Readiness Planning

Release-readiness planning should be a review/gate definition exercise, not a
new implementation tranche.

| Area | Planning decision needed | Source signal | Route |
|---|---|---|---|
| Evaluator security | Decide whether `DEL-06-02` sandboxed evaluator needs additional release-security review or hardening before any release claim. | Retained candidate `DAG-002-E0623`; bounded evaluator evidence; threat model. | RELEASE-SECURITY review gate. |
| Adapter threat model | Decide whether adapter framework details require a threat-model addendum before release. | Retire-recommended `DAG-002-E0619`; known SCC risk if promoted. | RELEASE-SECURITY review or scoped doc update, not graph promotion. |
| CI/release gates | Decide what CI evidence is required before any release-readiness claim. | Retire-recommended `DAG-002-E0620`; release gate and CI pipeline deliverables. | RELEASE-GATE checklist review. |
| Validation sufficiency | Decide which validation suites, invented examples, regression tests, and numerical checks are required for release candidacy. | PKG-09 bounded evidence and current test surfaces. | VALIDATION readiness matrix. |
| Packaging/signing/publishing | Decide whether live packaging, signing, notarization, installer generation, or publication is in scope. | Current evidence explicitly excludes live CI/signing/publishing. | Scope-change if requested beyond planning. |
| Professional acceptance | Decide whether any professional engineer review, code-compliance claim, or certified output claim is sought. | Professional-boundary policy and acceptance record non-claims. | Human/professional authority route only. |
| Private/protected data | Confirm release artifacts do not include protected standards content, private project data, private libraries, secrets, or nonredistributable payloads. | IP/data boundary and security/private-data evidence. | Release compliance checklist. |

Recommended release-readiness output:

- `DEV-001_REV05_RELEASE_READINESS_PLANNING.md`, proposal-only.
- A release-readiness checklist separating evidence already present from
  required-but-not-yet-authorized work.
- Explicit non-claims for release, production, professional acceptance, and
  full GUI/runtime readiness unless later accepted by human authority.

## Runtime/Product Gap Planning

Runtime/product gap planning should identify product assembly gaps that remain
after bounded implementation-evidence closure.

| Gap | Candidate/source signal | Planning route |
|---|---|---|
| Shared expression grammar/library | `DAG-002-E0616` retire recommendation; load-case algebra and rule-pack evaluator are separate bounded implementations. | Architecture/runtime decision: keep separate evaluators or authorize a shared expression service. |
| Result envelope compatibility | `DAG-002-E0617` and `DAG-002-E0618` retire recommendations; viewer, export, and FEA handoff can remain separate but must interoperate in product flows. | Product integration matrix for viewer/export/handoff envelopes. |
| Shared job orchestration | `DAG-002-E0624` retire recommendation; solve UX and headless runner share architecture basis but not a proven direct dependency. | Application-service runtime assembly plan. |
| Nonlinear diagnostic refinement | `DAG-002-E0622` retire recommendation; nonlinear cases can refine diagnostics without reversing DAG direction. | Solver validation/runtime diagnostics gap. |
| Full GUI/runtime assembly | Tranche L/M bounded GUI contracts are not full running GUI completion. | Product assembly scope route before claiming full runtime. |
| Live CI/build/package path | Bounded build/packaging evidence does not equal live release automation. | Release/product scope route before live packaging/signing/publishing. |
| External integrations | Provider-specific SDKs, external standards data, private libraries, and FEA tool integrations remain outside accepted evidence unless specifically scoped. | Scope-change route with data-boundary review. |

Recommended runtime/product output:

- `DEV-001_REV05_RUNTIME_PRODUCT_GAP_PLAN.md`, proposal-only.
- A gap ledger with owner route: architecture decision, release readiness,
  product assembly, validation, or scope change.
- No Type 2 implementation dispatch until the human project authority selects
  specific gap closures and write scopes.

## Scope-Change Triage

Use `SCOPE_CHANGE` rather than direct Type 2 dispatch when a requested next
step exceeds accepted DEV-001 revision `0.5` evidence closure.

| Proposed next step | Scope route |
|---|---|
| Mutating `DAG-002`, retiring candidate rows in graph files, or promoting any candidate row | CHANGE-managed graph patch plan, human approval, strict graph audit, then dependency mirror policy decision. |
| Full GUI/runtime completion claim | Scope change or new product-assembly initiative with explicit deliverables and acceptance criteria. |
| Release, production, signing, publishing, installer distribution, or live CI claims | Release-readiness scope route with packaging/security/professional boundaries. |
| Professional engineering acceptance, code compliance, or certification claims | Human/professional authority route; cannot be inferred from implementation evidence. |
| Provider-specific imports/exports, FEA tool integrations, private rule packs, or external protected standards data | Scope change with IP/data-boundary and security review before implementation. |
| Promotion of `docs/_ScopeChange/chirality-app-docs/` into active OpenPipeStress scope | Scope change or architecture decision; quarantined corpus remains read-only reference until then. |

## Recommended Sequence

1. Prepare proposal-only release-readiness planning from this plan and the
   candidate reconciliation summary.
2. Prepare proposal-only runtime/product gap ledger from this plan.
3. Prepare scope-change triage records only for items the human project
   authority selects for actual next work.
4. If graph cleanup is desired, ask for a separate CHANGE patch plan to retire
   seven candidates and retain `DAG-002-E0623` as non-gating pending
   release-security review. Do not apply that patch without approval.

## Recommended Next Gate

```text
APPROVE: prepare proposal-only DEV-001 revision 0.5 release-readiness planning
from the post-candidate reconciliation plan. Include evaluator/security review,
adapter threat-model feedback, CI/release-gate feedback, validation gates,
packaging/signing/publish boundaries, professional-acceptance boundaries, and
private/protected-data release checks. Do not dispatch Type 2 work, mutate
DAG-002, promote candidate edges, refresh dependency mirrors, or change
lifecycle/evidence state.
```
