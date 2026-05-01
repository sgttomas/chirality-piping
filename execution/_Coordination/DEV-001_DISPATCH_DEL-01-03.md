---
doc_id: DEV-001-DISPATCH-DEL-01-03
doc_kind: coordination.dispatch_brief
status: launched_single_bounded_item
created: 2026-05-01
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-01-03
package_id: PKG-01
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-01-03

## Dispatch Decision

The human project authority authorized exactly one bounded DAG item by stating
that ORCHESTRATOR may proceed as it wishes after ORCHESTRATOR recommended
`DEL-01-03 - Contributor certification workflow`.

This dispatch is not broad fan-out. It does not authorize lifecycle
transitions, candidate-edge promotion, blocker queue refresh, dependency
register edits, or work on any other deliverable.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-01-03` |
| PackageID | `PKG-01` |
| Name | Contributor certification workflow |
| Type | `DOC_UPDATE` |
| Scope items | `SOW-028`, `SOW-048` |
| Objectives | `OBJ-002` |
| Context envelope | `S` |
| Deliverable path | `execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-03_Contributor certification workflow` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| Target | Dependency type | Required maturity | Satisfaction |
|---|---|---|---|
| `DEL-00-01` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-02` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-06` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-08` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-01-01` | `GOVERNANCE_PREDECESSOR` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-01-02` | `GOVERNANCE_PREDECESSOR` | `SEMANTIC_READY` | `SATISFIED` |

No `CANDIDATE` edge currently gates `DEL-01-03`.

## Applicable Architecture Basis

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module responsibilities and no-bypass dependency rules.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no
  certification/compliance claims.
- `AB-00-08` - layered validation, provenance, protected-content, and
  acceptance gates.

Resolved baseline to preserve: contributor review must protect the public data
boundary, maintain provenance and redistribution status, keep unresolved legal
and license choices as `TBD`, and avoid professional-approval or compliance
claims.

Remaining implementation-level TBDs are not resolved by this dispatch: final
open-source license, maintainer roster, maintainer quorum, legal-review
authority, DCO/CLA adoption, and release policy details.

## Applicable Contract Invariants

Minimum invariants carried into this sealed item:

- `OPS-K-IP-1` - public repository must not contain protected standards text,
  copied tables, protected examples, proprietary commercial data, or private
  project data.
- `OPS-K-IP-2` - public data contributions require source, provenance,
  license/redistribution status, contributor certification, and review
  disposition.
- `OPS-K-IP-3` - suspected protected content must be quarantined and escalated;
  agents must not paraphrase protected tables into public data.
- `OPS-K-GOV-1` and `OPS-K-GOV-2` - license and governance authority remain
  `TBD` until recorded by the human project authority.
- `OPS-K-AUTH-1` - no certification, sealing, approval, authentication, or
  code-compliance claims for reliance.
- `OPS-K-AGENT-1` through `OPS-K-AGENT-4` - no invented values, conflicts
  surfaced, sealed execution, draft until human accepted.

## Explicit Write Scope

Authorized product and evidence targets for this bounded item:

- `CONTRIBUTING.md`
- `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md`
- `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`
- `governance/MAINTAINERS.md`
- `execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-03_Contributor certification workflow/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-01-03.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch.

## Acceptance Criteria

This bounded item is acceptable for closeout when:

- `CONTRIBUTING.md` contains a contributor certification workflow section that
  requires provenance, redistribution status, certification, review routing,
  and stop/escalation behavior.
- A contributor certification template exists with contributor, source,
  rights, redistribution, protected-content, private-data, and review fields.
- The contribution review checklist records the certification evidence needed
  to accept, reject, quarantine, or escalate a contribution.
- Maintainer policy points to the interim certification template while keeping
  the final DCO/CLA or other legal mechanism as `TBD`.
- The workflow rejects or quarantines missing-evidence, unknown-rights,
  private-only, or protected-suspected public data contributions.
- The workflow does not claim legal advice, standards-body approval,
  professional engineering approval, certification, sealing, endorsement, or
  automatic code compliance for reliance.
- No lifecycle state transition, dependency-register edit, candidate-edge
  change, or blocker-queue refresh occurs.
