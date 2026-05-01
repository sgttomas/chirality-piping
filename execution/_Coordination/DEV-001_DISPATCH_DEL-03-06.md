---
doc_id: DEV-001-DISPATCH-DEL-03-06
doc_kind: coordination.dispatch_brief
status: launched_single_bounded_item
created: 2026-05-01
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-03-06
package_id: PKG-03
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-03-06

## Dispatch Decision

The human project authority authorized exactly one bounded DAG item:
`DEL-03-06 - Expansion joint component model`.

This dispatch is not broad fan-out. It does not authorize lifecycle
transitions, candidate-edge promotion, blocker queue refresh, dependency
register edits, or work on any other deliverable.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-03-06` |
| PackageID | `PKG-03` |
| Name | Expansion joint component model |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope items | `SOW-010` |
| Objectives | `OBJ-004` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-06_Expansion joint component model` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| Target | Dependency type | Required maturity | Satisfaction |
|---|---|---|---|
| `DEL-00-01` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-02` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-04` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-06` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-07` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-08` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-03-02` | `DOMAIN_MODEL` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-02-02` | `UNIT_CONTRACT` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-01-02` | `GOVERNANCE_PREDECESSOR` | `SEMANTIC_READY` | `SATISFIED` |

No `CANDIDATE` edge currently gates `DEL-03-06`.

## Applicable Architecture Basis

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module responsibilities and no-bypass dependency rules.
- `AB-00-04` - JSON Schema 2020-12, canonical JSON, and versioned
  persistence/hash basis.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no
  certification/compliance claims.
- `AB-00-07` - schema-first API/adapter boundaries and no-bypass provenance,
  units, diagnostics, and data-boundary constraints.
- `AB-00-08` - layered validation, provenance, protected-content, and
  acceptance gates.

Resolved baseline to preserve: expansion joint stiffnesses, effective areas,
movement limits, hardware data, and manufacturer references are user-supplied,
manufacturer-supplied with review, or private-library values. Public schema
slots are allowed; manufacturer catalog values, proprietary data, private
library values, and invented engineering defaults are not public defaults.

Remaining implementation-level TBDs are not resolved by this dispatch:
accepted public expansion-joint source catalogs, public fixture value policy,
stiffness degree-of-freedom mapping, hardware taxonomy, exact solver
consumption details, concrete import formats, and downstream component editor
behavior.

## Applicable Contract Invariants

- `OPS-K-IP-1` through `OPS-K-IP-3` - no protected standards/proprietary data,
  provenance required, suspected protected content quarantined.
- `OPS-K-DATA-1` through `OPS-K-DATA-3` - code-specific and manufacturer
  values are user-supplied/private unless lawfully imported; missing values are
  explicit findings; component data carries provenance.
- `OPS-K-UNIT-1` - stiffnesses, effective areas, movement limits, and hardware
  quantities are unit-aware or explicitly dimensionless/reference data.
- `OPS-K-PRIV-1` - private component libraries are not transmitted or committed
  publicly by default.
- `OPS-K-AUTH-1` - no certification, sealing, approval, authentication, or
  code-compliance claims for reliance.
- `OPS-K-AGENT-1` through `OPS-K-AGENT-4` - no invented engineering values,
  conflicts surfaced, sealed execution, draft until human accepted.

## Explicit Write Scope

Authorized product and evidence targets for this bounded item:

- `schemas/component.schema.yaml`
- `fixtures/component/invented_section_component_library_valid.json`
- `tests/test_component_section_schema.py`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-06_Expansion joint component model/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-03-06.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch.

## Acceptance Criteria

This bounded item is acceptable for closeout when:

- `schemas/component.schema.yaml` has explicit expansion-joint diagnostic and
  field contracts for stiffnesses, effective area, movement limits, hardware
  data, manufacturer references, mechanics-interface use, diagnostics, and open
  decisions.
- Public fixture data remains invented, schema-shape-only, or missing-value
  evidence; it does not contain manufacturer values, proprietary catalog data,
  private library values, protected standards text, or invented engineering
  defaults.
- Tests verify expansion-joint component type coverage, stiffness/effective
  area/movement-limit/hardware slots, family contract coverage, and
  protected-content guardrails.
- Documentation points to the expansion-joint fields without claiming
  engineering approval, code compliance, public data suitability, or catalog
  completeness.
- No lifecycle state transition, dependency-register edit, candidate-edge
  change, or blocker-queue refresh occurs.
