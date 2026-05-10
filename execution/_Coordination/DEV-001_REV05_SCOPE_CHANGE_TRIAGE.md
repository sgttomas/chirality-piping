---
doc_id: DEV-001-REV05-SCOPE-CHANGE-TRIAGE
doc_kind: coordination.scope_change_triage
status: proposal_only_complete
created: 2026-05-09
source_acceptance_commit: c0303c4
source_release_plan: execution/_Coordination/DEV-001_REV05_RELEASE_READINESS_PLANNING.md
source_runtime_gap_ledger: execution/_Coordination/DEV-001_REV05_RUNTIME_PRODUCT_GAP_LEDGER.md
graph_authority: execution/_DAG/DAG-002
scope_change_applied: false
graph_mutation: not_authorized_not_performed
candidate_promotion: not_authorized_not_performed
dependency_mirror_refresh: not_authorized_not_performed
lifecycle_evidence_change: not_authorized_not_performed
type2_dispatch: not_performed
---

# DEV-001 Rev 0.5 Scope-Change Triage

## Purpose

This triage classifies likely post-DEV-001 revision `0.5` next steps by the
governed route needed before execution. It does not apply any scope change.

The accepted baseline remains bounded implementation-evidence closure only:
84 committed non-`PKG-00` implementation rows, 8 `PKG-00` architecture-basis
context rows, approved active `DAG-002`, non-gating candidate rows, and no
release/product/professional readiness claim.

## Triage Matrix

| Triage ID | Proposed direction | Route required | Why route is required | Current recommendation |
|---|---|---|---|---|
| SCT-01 | Retire seven candidate rows and retain `E0623` as non-gating security review | CHANGE graph patch plan, human approval, strict graph audit, mirror policy decision | Changes approved graph files and possibly downstream mirrors. | Prepare patch plan only if human wants graph cleanup. |
| SCT-02 | Promote any retained candidate row | RECONCILIATION decision, CHANGE graph patch, strict graph audit, human approval | Candidate promotion can change blocker semantics and may create cycles. | Not recommended now; promote none from current evidence. |
| SCT-03 | Release-readiness review without implementation | REVIEW/AUDIT planning route | Defines release gates but does not change product code. | Safe next planning route. |
| SCT-04 | Live CI provider/workflow implementation | Scope change or sealed Type 2 deliverable after release plan | Adds provider-specific automation and release evidence behavior beyond bounded skeleton. | Defer until release target is selected. |
| SCT-05 | Signing, notarization, attestation, publishing | Scope change plus release authority | Creates external release obligations and credentials/secrets handling. | Defer; requires human release authority. |
| SCT-06 | Full GUI desktop runtime | New product-assembly initiative or scope change | Existing PKG-07 evidence is contract-level, not full runtime. | Requires explicit product target and sealed write scopes. |
| SCT-07 | Headless/CLI product runtime | Scope change or product-assembly initiative | Existing runner contract is bounded; real runtime integration and release behavior remain unclaimed. | Candidate for a smaller product target than full GUI. |
| SCT-08 | Live solver/prover or external FEA execution | Scope change with validation/security/IP review | External execution, commercial parsers, protected standards data, and professional reliance are out of current scope. | Defer until target and data boundary are approved. |
| SCT-09 | Professional engineering acceptance workflow | Human/professional authority route | Software cannot infer professional approval, certification, compliance, or reliance acceptance. | Requires responsible professional decision, not agent dispatch. |
| SCT-10 | Private rule-pack/private library payload storage | Scope change with security and IP/data review | Current evidence covers metadata/reference controls, not payload storage, encryption, or secret manager integration. | Defer until storage/security architecture is selected. |
| SCT-11 | Production persistence container/migrations | Architecture decision plus product scope | Current persistence evidence is schema/contract bounded. | Requires product runtime target first. |
| SCT-12 | Promote Chirality app/harness corpus concepts | Scope change or architecture decision | Corpus is quarantined read-only reference, not OpenPipeStress implementation scope. | Keep quarantined unless human selects specific concepts for review. |

## Route Definitions

| Route | Output before execution |
|---|---|
| CHANGE graph patch plan | Proposed file edits, cycle/schema audit plan, mirror refresh policy, and human approval text. |
| Release-readiness review | Checklist of gates, owners, current evidence, missing evidence, waivers, and non-claims. |
| Product-assembly initiative | Product target, acceptance criteria, deliverable set, sealed write scopes, verification plan, and launch gates. |
| Scope change | Scope brief, affected decomposition/DAG/register surfaces, IP/security/professional implications, and human approval. |
| Professional authority route | Named responsible human/professional review obligations and permitted wording; no software-only approval. |

## Recommended Immediate Route

The lowest-risk next route is proposal-only release-readiness review. It can
decide whether the project wants:

- a release-evidence archive only;
- a headless/CLI release target;
- a full GUI runtime target;
- an external integration target;
- graph cleanup before roadmap work;
- or no release route yet.

Graph cleanup can be planned in parallel only as a `CHANGE` patch plan. It
should not be applied until the human project authority approves mutation of
`DAG-002` and a strict audit route.

## Recommended Next Gate

```text
APPROVE: prepare a proposal-only DEV-001 revision 0.5 release-readiness review
packet from the release-readiness plan, runtime/product gap ledger, and
scope-change triage. Include recommended release target options, gate checklist,
missing evidence, waiver candidates, and non-claims. Do not dispatch Type 2
work, mutate DAG-002, promote candidate edges, refresh dependency mirrors, or
change lifecycle/evidence state.
```
