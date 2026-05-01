---
doc_id: DEV-001-DISPATCH-DEL-01-04
doc_kind: coordination.dispatch_brief
status: launched_single_bounded_item
created: 2026-05-01
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-01-04
package_id: PKG-01
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-01-04

## Dispatch Decision

The human project authority authorized exactly one next bounded DAG item.
ORCHESTRATOR selected `DEL-01-04 - Professional responsibility and
product-claims policy`.

This dispatch is not broad fan-out. It does not authorize lifecycle
transitions, candidate-edge promotion, blocker queue refresh, dependency
register edits, or work on any other deliverable.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-01-04` |
| PackageID | `PKG-01` |
| Name | Professional responsibility and product-claims policy |
| Type | `DOC_UPDATE` |
| Scope items | `SOW-034` |
| Objectives | `OBJ-011` |
| Context envelope | `S` |
| Deliverable path | `execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-04_Professional responsibility and product-claims policy` |
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

No `CANDIDATE` edge currently gates `DEL-01-04`.

## Applicable Architecture Basis

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module responsibilities and no-bypass dependency rules.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no
  certification/compliance claims.
- `AB-00-08` - layered validation, provenance, protected-content, and
  acceptance gates.

Resolved baseline to preserve: no certification, sealing, professional
approval, standards-body endorsement, or automatic code-compliance claims;
status and report language must distinguish mechanics results, user-rule
checks, and human professional acceptance.

Remaining implementation-level TBDs are not resolved by this dispatch: final
project license, maintainer roster, legal-review authority, report template
format, human acceptance record storage, and release policy details.

## Applicable Contract Invariants

Minimum invariants carried into this sealed item:

- `OPS-K-AUTH-1` - software and agents must not claim to certify, seal,
  approve, authenticate, or declare engineering code compliance for reliance.
- `OPS-K-AUTH-2` - human acceptance records, if used, bind to specific
  model/rule/report hashes and do not survive content changes without
  re-review.
- `OPS-K-MECH-2` - solver mechanics, rule-pack evaluation, and professional
  compliance judgment remain separate.
- `OPS-K-REPORT-1` and `OPS-K-REPORT-2` - reports disclose assumptions,
  warnings, provenance, and limitations and must not reproduce protected
  standards content.
- `OPS-K-GOV-3` - public releases disclose scope, validation status, known
  limitations, data-boundary constraints, and professional-responsibility
  limitations.
- `OPS-K-AGENT-1` through `OPS-K-AGENT-4` - no invented values, conflicts
  surfaced, sealed execution, draft until human accepted.

## Explicit Write Scope

Authorized product and evidence targets for this bounded item:

- `docs/PROFESSIONAL_BOUNDARY.md`
- `docs/report_notice_template.md`
- `execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-04_Professional responsibility and product-claims policy/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-01-04.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch.

## Acceptance Criteria

This bounded item is acceptable for closeout when:

- `docs/PROFESSIONAL_BOUNDARY.md` defines permitted and prohibited product,
  report, documentation, release, and agent claims.
- A report notice template exists and distinguishes mechanics solve output,
  user-supplied rule-pack checks, assumptions/warnings, and human professional
  acceptance.
- Policy language states that OpenPipeStress outputs are decision support until
  accepted by competent human review for project use.
- Human acceptance, if recorded, binds to specific model, rule-pack, and report
  hashes and is invalidated or marked stale after relevant content changes.
- The policy and template do not claim legal advice, standards-body approval,
  professional engineering approval, certification, sealing, endorsement, or
  automatic code compliance for reliance.
- No lifecycle state transition, dependency-register edit, candidate-edge
  change, or blocker-queue refresh occurs.
