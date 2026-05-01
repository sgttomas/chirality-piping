---
doc_id: DEV-001-DISPATCH-DEL-03-07
doc_kind: coordination.dispatch_brief
status: launched_single_bounded_item
created: 2026-05-01
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-03-07
package_id: PKG-03
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-03-07

## Dispatch Decision

The human project authority authorized exactly one bounded DAG item:
`DEL-03-07 - Public/private library import provenance checker`.

This dispatch is not broad fan-out. It does not authorize lifecycle
transitions, candidate-edge promotion, blocker queue refresh, dependency
register edits, or work on any other deliverable.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-03-07` |
| PackageID | `PKG-03` |
| Name | Public/private library import provenance checker |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope items | `SOW-019`, `SOW-044` |
| Objectives | `OBJ-002`, `OBJ-004` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-07_Public-private library import provenance checker` |
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
| `DEL-03-01` | `DOMAIN_MODEL` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-03-02` | `DOMAIN_MODEL` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-01-02` | `GOVERNANCE_PREDECESSOR` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-01-03` | `GOVERNANCE_PREDECESSOR` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-02-04` | `SERVICE_API` | `SEMANTIC_READY` | `SATISFIED` |

No `CANDIDATE` edge currently gates `DEL-03-07`.

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

Resolved baseline to preserve: material, section, and component library imports
must carry provenance, redistribution, review, privacy, and unit metadata
through the import boundary. Public acceptance requires reviewed
public-permissive evidence. Private imports may remain local/private but cannot
bypass quarantine or missing-provenance diagnostics.

Remaining implementation-level TBDs are not resolved by this dispatch:
external import formats, legal/license interpretation, accepted public source
catalogs, UI/editor presentation, and downstream adapter mechanics.

## Applicable Contract Invariants

- `OPS-K-IP-1` through `OPS-K-IP-3` - no protected standards/proprietary data,
  provenance required, suspected protected content quarantined.
- `OPS-K-DATA-1` through `OPS-K-DATA-3` - code-specific values are
  user-supplied/private unless lawfully imported; missing values are explicit
  findings; material/component data carries provenance.
- `OPS-K-UNIT-1` - imported numeric values preserve unit and dimension
  metadata.
- `OPS-K-PRIV-1` - private libraries are not transmitted or committed publicly
  by default.
- `OPS-K-AUTH-1` - no certification, sealing, approval, authentication, legal
  acceptance, or code-compliance claims for reliance.
- `OPS-K-AGENT-1` through `OPS-K-AGENT-4` - no invented engineering values,
  conflicts surfaced, sealed execution, draft until human accepted.

## Explicit Write Scope

Authorized product and evidence targets for this bounded item:

- `core/library_import/provenance_checker.py`
- `core/library_import/README.md`
- `tests/test_library_import_provenance.py`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-07_Public-private library import provenance checker/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-03-07.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch.

## Acceptance Criteria

This bounded item is acceptable for closeout when:

- A library import provenance checker validates already-parsed material,
  section, and component payloads for required provenance, redistribution,
  review disposition, privacy posture, protected-content quarantine, and unit
  metadata preservation.
- The checker does not parse concrete external formats and does not make legal
  license conclusions.
- Tests cover accepted public imports, unresolved public rights, private-local
  handling, missing provenance, protected/quarantine metadata, and unit metadata
  preservation.
- Documentation identifies the checker and its boundaries without claiming
  engineering approval, legal acceptance, code compliance, public data
  suitability, or catalog completeness.
- No lifecycle state transition, dependency-register edit, candidate-edge
  change, or blocker-queue refresh occurs.
