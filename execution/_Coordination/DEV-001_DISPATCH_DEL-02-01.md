---
doc_id: DEV-001-DISPATCH-DEL-02-01
doc_kind: coordination.dispatch_brief
status: launched_single_bounded_item
created: 2026-04-30
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-02-01
package_id: PKG-02
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-02-01

## Dispatch Decision

The human project authority accepted the completed `DEL-01-01` pilot pattern
and authorized exactly one next bounded DAG item: `DEL-02-01 - Canonical
domain model schema`.

This dispatch is not broad fan-out. It does not authorize lifecycle
transitions, candidate-edge promotion, blocker queue refresh, or work on any
other deliverable.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-02-01` |
| PackageID | `PKG-02` |
| Name | Canonical domain model schema |
| Type | `DATA_MODEL_CHANGE` |
| Scope items | `SOW-041` |
| Objectives | `OBJ-001` from `Deliverables.csv`; `OBJ-012` remains scope-ledger context pending human ruling |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-01_Canonical domain model schema` |
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

No `CANDIDATE` edge currently gates `DEL-02-01`.

## Applicable Architecture Basis

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module responsibilities and no-bypass dependency rules.
- `AB-00-03` - command/query/job/result envelope boundaries and distinct mechanics/rule/human states.
- `AB-00-04` - deterministic, versioned, unit-aware, provenance-preserving, schema-governed JSON persistence compatibility and JCS-compatible hash basis where JSON payload hashes are used.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no certification/compliance claims.
- `AB-00-07` - public/internal API boundary and adapter/plugin validation constraints.
- `AB-00-08` - layered schema, unit, validation, and protected-content/provenance gates.

Resolved baseline to preserve: JSON Schema 2020-12 contracts, schema-first
result-envelope compatibility, canonical JSON/JCS-compatible hash basis where
JSON payloads are hashed, and validation/protected-content test gates as
applicable.

Remaining implementation-level TBDs are not resolved by this dispatch: exact
dependency versions, solver numerical library, rule expression grammar/library,
public API transport, import/export format list, CI provider/coverage
thresholds, and physical project package/container.

## Applicable Contract Invariants

Minimum invariants carried into this sealed item:

- `OPS-K-IP-1` through `OPS-K-IP-3` - protected standards/code/proprietary data stay out of public content.
- `OPS-K-DATA-1` through `OPS-K-DATA-3` - code-specific values are user supplied, missing values are findings, and reliance-affecting records carry provenance.
- `OPS-K-AUTH-1` and `OPS-K-AUTH-2` - no certification, sealing, approval, or reliance claims by software/agents.
- `OPS-K-UNIT-1` - schema-visible dimensional values must be unit-aware.
- `OPS-K-MECH-1` and `OPS-K-MECH-2` - global centerline/frame model and mechanics/rule/human authority separation.
- `OPS-K-REPORT-1` and `OPS-K-REPORT-2` - report-facing schema records expose audit data without protected standards content.
- `OPS-K-AGENT-1` through `OPS-K-AGENT-4` - no invented values, conflicts surfaced, sealed execution, draft until human accepted.

## Explicit Write Scope

Authorized product and evidence targets for this bounded item:

- `schemas/model.schema.yaml`
- `docs/TYPES.md`
- `tests/test_model_schema.py`
- `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-01_Canonical domain model schema/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-02-01.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`
- `execution/_Coordination/_COORDINATION.md`

No other files are authorized by this dispatch.

## Acceptance Criteria

This bounded item is acceptable for closeout when:

- `schemas/model.schema.yaml` remains strict JSON syntax using JSON Schema 2020-12.
- The schema covers Project, Model, Node, Element, Material, Component, Section, Support, LoadCase, Combination, RulePackRef, Result, ReportSettings, and Report records.
- Unit-bearing quantities require unit/dimension/provenance fields.
- Diagnostics require code/class/severity/source/affected object/message/remediation/provenance fields.
- Results and reports can carry diagnostics, provenance, rule-pack references, report notices, input manifests, and hash metadata.
- JSON payload hash records explicitly carry canonicalization and payload scope metadata.
- The schema and type registry do not introduce protected standards text, copied code formulas, protected tables, proprietary values, private data, or automatic code-compliance/certification/sealing claims.
- Focused stdlib tests pass for `tests/test_model_schema.py` and the existing schema tests.
- No lifecycle state transition, dependency-register edit, candidate-edge change, or blocker-queue refresh occurs.
