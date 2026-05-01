---
doc_id: DEV-001-DISPATCH-DEL-03-01
doc_kind: coordination.dispatch_brief
status: launched_single_bounded_item
created: 2026-05-01
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-03-01
package_id: PKG-03
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-03-01

## Dispatch Decision

The human project authority authorized exactly one next bounded DAG item:
`DEL-03-01 - Material library schema with provenance`.

This dispatch is not broad fan-out. It does not authorize lifecycle
transitions, candidate-edge promotion, blocker queue refresh, dependency
register edits, or work on any other deliverable.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-03-01` |
| PackageID | `PKG-03` |
| Name | Material library schema with provenance |
| Type | `DATA_MODEL_CHANGE` |
| Scope items | `SOW-017` |
| Objectives | `OBJ-004` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-01_Material library schema with provenance` |
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
| `DEL-02-01` | `SCHEMA_CONTRACT` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-02-02` | `UNIT_CONTRACT` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-01-02` | `GOVERNANCE_PREDECESSOR` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-01-03` | `GOVERNANCE_PREDECESSOR` | `SEMANTIC_READY` | `SATISFIED` |

No `CANDIDATE` edge currently gates `DEL-03-01`.

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

Resolved baseline to preserve: material values are governed, unit-aware,
provenance-marked data. Public schema slots are allowed; protected material
tables, code-specific allowables, proprietary catalog values, and private
libraries are not public defaults.

Remaining implementation-level TBDs are not resolved by this dispatch: accepted
public material source catalog, public material fixture value policy,
temperature interpolation policy, allowable value storage details, and concrete
material editor UI behavior.

## Applicable Contract Invariants

Minimum invariants carried into this sealed item:

- `OPS-K-IP-1` through `OPS-K-IP-3` - no protected standards/proprietary data,
  provenance required, suspected protected content quarantined.
- `OPS-K-DATA-1` through `OPS-K-DATA-3` - code-specific values are
  user-supplied/private, missing required values are explicit findings, and
  material data carries provenance.
- `OPS-K-UNIT-1` - material properties and imports are unit-aware and
  dimensionally checked.
- `OPS-K-PRIV-1` - private material libraries are not transmitted or committed
  publicly by default.
- `OPS-K-AUTH-1` - no certification, sealing, approval, authentication, or
  code-compliance claims for reliance.
- `OPS-K-AGENT-1` through `OPS-K-AGENT-4` - no invented engineering values,
  conflicts surfaced, sealed execution, draft until human accepted.

## Explicit Write Scope

Authorized product and evidence targets for this bounded item:

- `schemas/material.schema.yaml`
- `fixtures/material/invented_material_library_valid.json`
- `tests/test_material_schema.py`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-01_Material library schema with provenance/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-03-01.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch.

## Acceptance Criteria

This bounded item is acceptable for closeout when:

- `schemas/material.schema.yaml` defines material-library metadata, material
  records, temperature-dependent property slots, allowable slots, provenance,
  redistribution status, review status, completeness findings, diagnostics, and
  open decisions.
- Public fixture data is invented/non-engineering or schema-shape-only and does
  not contain protected material tables, code-specific allowables, proprietary
  catalog values, or private material data.
- Tests verify required provenance, redistribution, unit/completeness, and
  no-public-allowable controls.
- Documentation points to the material schema without claiming engineering
  approval, code compliance, or public material data suitability.
- No lifecycle state transition, dependency-register edit, candidate-edge
  change, or blocker-queue refresh occurs.
