---
doc_id: DEV-001-DISPATCH-DEL-02-05
doc_kind: coordination.dispatch_brief
status: launched_single_bounded_item
created: 2026-05-01
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-02-05
package_id: PKG-02
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-02-05

## Dispatch Decision

The human project authority authorized exactly one bounded DAG item of
ORCHESTRATOR's choosing. ORCHESTRATOR selected `DEL-02-05 - Project
persistence and round-trip serialization`.

This dispatch is not broad fan-out. It does not authorize lifecycle
transitions, candidate-edge promotion, blocker queue refresh, dependency
register edits, or work on any other deliverable.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-02-05` |
| PackageID | `PKG-02` |
| Name | Project persistence and round-trip serialization |
| Type | `DATA_MODEL_CHANGE` |
| Scope items | `SOW-050`, `SOW-041` |
| Objectives | `OBJ-001`, `OBJ-012` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| Target | Dependency type | Required maturity | Satisfaction |
|---|---|---|---|
| `DEL-00-01` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-02` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-03` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-04` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-06` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-07` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-08` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-02-01` | `PERSISTENCE_CONTRACT` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-02-02` | `UNIT_CONTRACT` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-02-03` | `DOMAIN_MODEL` | `SEMANTIC_READY` | `SATISFIED` |

No `CANDIDATE` edge currently gates `DEL-02-05`.

## Applicable Architecture Basis

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module responsibilities and no-bypass dependency rules.
- `AB-00-03` - command/query/job/result envelope boundaries and distinct
  mechanics/rule/human states.
- `AB-00-04` - deterministic, versioned, provenance-preserving,
  schema-governed JSON persistence compatibility and JCS-compatible hash basis
  where JSON payload hashes are used.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no
  certification/compliance claims.
- `AB-00-07` - public/internal API boundary and adapter/plugin validation
  constraints.
- `AB-00-08` - layered schema, validation, unit, provenance, and
  protected-content gates.

Resolved baseline to preserve: Rust core/application services; JSON Schema
2020-12 contracts; schema-first command/query/job/result-envelope compatibility;
canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; and
Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.

Remaining implementation-level TBDs are not resolved by this dispatch: exact
dependency versions, solver numerical library, rule expression grammar/library,
public API transport, import/export format list, CI provider/coverage
thresholds, physical project package/container, migration framework/tooling,
canonical JSON library, and non-JSON/binary hash payload partitioning.

## Applicable Contract Invariants

Minimum invariants carried into this sealed item:

- `OPS-K-IP-1` through `OPS-K-IP-3` - protected standards/code/proprietary data
  stay out of public content.
- `OPS-K-DATA-1` through `OPS-K-DATA-3` - code-specific values are user
  supplied, missing values are findings, and reliance-affecting records carry
  provenance.
- `OPS-K-AUTH-1` and `OPS-K-AUTH-2` - no certification, sealing, approval, or
  reliance claims by software/agents; any human acceptance reference is
  external and hash-bound.
- `OPS-K-UNIT-1` - persisted quantities, imports, exports, and rule-facing
  records remain unit-aware and dimensionally checkable.
- `OPS-K-RULE-2` and `OPS-K-RULE-3` - rule-pack references remain sandboxed
  consumers, versioned, checksummed, source-noted, and public/private marked.
- `OPS-K-REPORT-1` and `OPS-K-REPORT-2` - report-facing manifests preserve
  warnings, assumptions, provenance, and professional-boundary notices without
  protected standards content.
- `OPS-K-PRIV-1` and `OPS-K-PRIV-2` - private project/rule/library data is not
  transmitted or committed publicly by default.
- `OPS-K-AGENT-1` through `OPS-K-AGENT-4` - no invented values, conflicts
  surfaced, sealed execution, draft until human accepted.

## Explicit Write Scope

Authorized product and evidence targets for this bounded item:

- `schemas/project_persistence.schema.yaml`
- `docs/architecture/persistence_contract.md`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `tests/test_persistence_schema.py`
- `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-02-05.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch.

## Acceptance Criteria

This bounded item is acceptable for closeout when:

- `schemas/project_persistence.schema.yaml` remains strict JSON syntax using
  JSON Schema 2020-12.
- The persistence envelope defines schema version, document kind, physical
  container TBD marker, project payload, hash metadata, migration status,
  round-trip manifest, diagnostics, validation profile, and service-operation
  contract records.
- The project payload delegates model structure to `schemas/model.schema.yaml`
  and preserves unit-system references, rule-pack references, provenance
  manifest, private-data markers, and optional external human acceptance
  references without treating them as software approval.
- Hash metadata records canonicalization, payload scope, checksum manifest,
  excluded/volatile fields, and JCS-compatible JSON basis without deciding the
  future non-JSON/binary partition.
- Migration and service-operation records expose structured statuses and
  diagnostics for create/open/save/validate/version-check/migrate flows.
- Round-trip criteria cover model content, unit metadata, loads, rule-pack
  references, provenance metadata, reproducibility metadata, and documented
  volatile fields.
- Documentation in `docs/architecture/persistence_contract.md`,
  `docs/SPEC.md`, and `docs/TYPES.md` records the persistence contract,
  service operations, boundary constraints, and remaining TBDs.
- Focused stdlib tests pass for `tests/test_persistence_schema.py` and
  relevant existing schema tests.
- No lifecycle state transition, dependency-register edit, candidate-edge
  change, or blocker-queue refresh occurs.
