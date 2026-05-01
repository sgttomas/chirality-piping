---
doc_id: DEV-001-DISPATCH-DEL-01-02
doc_kind: coordination.dispatch_brief
status: launched_single_bounded_item
created: 2026-05-01
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-01-02
package_id: PKG-01
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-01-02

## Dispatch Decision

The human project authority authorized exactly one bounded DAG item of
ORCHESTRATOR's choosing. ORCHESTRATOR selected `DEL-01-02 - Copyright and
protected-data boundary policy`.

This dispatch is not broad fan-out. It does not authorize lifecycle
transitions, candidate-edge promotion, blocker queue refresh, dependency
register edits, or work on any other deliverable.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-01-02` |
| PackageID | `PKG-01` |
| Name | Copyright and protected-data boundary policy |
| Type | `DOC_UPDATE` |
| Scope items | `SOW-003`, `SOW-028` |
| Objectives | `OBJ-002` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-02_Copyright and protected-data boundary policy` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| Target | Dependency type | Required maturity | Satisfaction |
|---|---|---|---|
| `DEL-00-01` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-02` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-06` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-08` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |

No `CANDIDATE` edge currently gates `DEL-01-02`.

## Applicable Architecture Basis

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module responsibilities and no-bypass dependency rules.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no
  certification/compliance claims.
- `AB-00-08` - layered validation, provenance, protected-content, and
  acceptance gates.

Resolved baseline to preserve: schema-first governance evidence where
applicable; protected-content and provenance gates; no certification,
sealing, approval, code-compliance, or professional-reliance claims.

Remaining implementation-level TBDs are not resolved by this dispatch: final
project license, contributor certification mechanism, maintainer roster,
legal-review authority, public release policy details, and exact automated
protected-content scanner implementation.

## Applicable Contract Invariants

Minimum invariants carried into this sealed item:

- `OPS-K-IP-1` through `OPS-K-IP-3` - protected standards/code/proprietary data
  stay out of public content, public data requires provenance/review evidence,
  and suspected protected content is quarantined and escalated.
- `OPS-K-DATA-1` through `OPS-K-DATA-3` - code-specific values are user
  supplied, missing values are findings, and reliance-affecting records carry
  provenance.
- `OPS-K-AUTH-1` - no certification, sealing, approval, authentication, or
  engineering code-compliance claim.
- `OPS-K-RULE-1`, `OPS-K-REPORT-2`, and `OPS-K-GOV-4` - public examples,
  report templates, and contributions must pass protected-content and
  provenance review.
- `OPS-K-PRIV-1` - private project/rule/library data is not transmitted or
  committed publicly by default.
- `OPS-K-AGENT-1` through `OPS-K-AGENT-4` - no invented values, conflicts
  surfaced, sealed execution, draft until human accepted.

## Explicit Write Scope

Authorized product and evidence targets for this bounded item:

- `docs/IP_AND_DATA_BOUNDARY.md`
- `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`
- `governance/MAINTAINERS.md`
- `execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-02_Copyright and protected-data boundary policy/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-01-02.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch.

## Acceptance Criteria

This bounded item is acceptable for closeout when:

- `docs/IP_AND_DATA_BOUNDARY.md` defines allowed public content, prohibited
  public content, private user data, required provenance, quarantine, report
  boundaries, and public contribution review.
- A repo-level contribution review checklist exists with fields for source,
  provenance, redistribution/license status, contributor certification,
  protected-content risk, private-data risk, tests/evidence, reviewer
  disposition, and quarantine/escalation.
- Suspected protected standards text, tables, copied formulas, proprietary
  vendor data, private user data, and commercial examples without permission
  are routed to quarantine or rejection, not accepted into public examples.
- Unknown legal, license, source, or redistribution status remains `TBD` or
  `unknown` and blocks acceptance until reviewed.
- The policy and checklist do not claim legal advice, standards-body approval,
  professional engineering approval, certification, sealing, endorsement, or
  code compliance for reliance.
- No lifecycle state transition, dependency-register edit, candidate-edge
  change, or blocker-queue refresh occurs.
