---
doc_id: DEV-001-DISPATCH-DEL-06-04
doc_kind: coordination.dispatch_brief
status: committed_evidence_queue_refreshed
created: 2026-05-02
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-06-04
package_id: PKG-06
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-06-04

## Dispatch Decision

The human project authority authorized one bounded item:

- `DEL-06-04 - Private rule-pack lifecycle and checksum handling`

This dispatch authorizes implementation within the explicit write scope below.
It does not authorize broad DAG execution, candidate-edge promotion, protected
standards data, private rule-pack payloads, storage/encryption/access-control
policy, GUI/editor work, report generation, public API transport, lifecycle
transition, implementation-evidence registration, local dependency-register
edits, blocker-queue refresh, or commit unless separately approved.

Implementation has been completed in the working tree within the bounded write
scope. Lifecycle transition, implementation-evidence registration,
local-dependency annotation, blocker-queue refresh, and DAG validation have now
been performed. Implementation and initial alignment were committed as
`ad270f6 core: add rule pack lifecycle handling`; final evidence and queue
artifacts now record `DEL-06-04` as `COMMITTED`.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-06-04` |
| PackageID | `PKG-06` |
| Name | Private rule-pack lifecycle and checksum handling |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope items | `SOW-042` |
| Objectives | `OBJ-002`, `OBJ-005` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-04_Private rule-pack lifecycle and checksum handling` |
| Current lifecycle | `CHECKING` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| Target | Dependency type | Implementation-readiness satisfaction |
|---|---|---|
| `DEL-00-01` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-02` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-03` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-04` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-06` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-07` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-08` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-06-01` | `RULE_PACK_PREDECESSOR` | `COMMITTED` evidence `20241f9` |
| `DEL-02-05` | `PERSISTENCE_CONTRACT` | `COMMITTED` evidence `742016e` |

Current implementation-readiness queue state:

- `DEL-06-04` is `UNBLOCKED`.
- `DEL-06-04` has `COMMITTED` evidence `ad270f6`.
- Candidate edges are excluded.
- `DEL-06-04` currently gates downstream consumers `DEL-07-03`,
  `DEL-08-01`, `DEL-08-02`, `DEL-12-02`, and `DEL-12-04`.

## Applicable Architecture Basis

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module responsibilities and no-bypass dependency rules.
- `AB-00-03` - schema-first command/query/job/result-envelope service
  boundaries, including distinct mechanics-solved, user-rule-checked, and
  human-approved states.
- `AB-00-04` - JSON Schema 2020-12, schema versioning, canonical JSON, and
  JCS-compatible hash basis for JSON payloads.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no
  certification/compliance claims.
- `AB-00-07` - internal/public API and adapter no-bypass constraints.
- `AB-00-08` - layered validation, provenance, protected-content, and
  acceptance gates.

Resolved baseline to preserve: rule-pack lifecycle records identify the
rule-pack identity, version, source/provenance notice, redistribution/private
status, checksum metadata, and audit/report-facing manifest references without
publishing private formulas, protected standards text, proprietary values,
owner standards, or private rule-pack payloads. JSON payload checksums use a
JCS-compatible canonical payload basis when the caller supplies canonical JSON
bytes. Storage location, encryption defaults, access policy, secret handling,
non-JSON payload partitioning, GUI/report/API integration, and private content
review workflow remain `TBD` or deferred to later deliverables.

## Explicit Write Scope

Implementation write scope is limited to:

- `core/rules/rule_pack_lifecycle/`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-04_Private rule-pack lifecycle and checksum handling/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-06-04.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

## Acceptance Criteria

This bounded item is acceptable for implementation closeout when:

- A rule-pack lifecycle/checksum module records identity, version, privacy,
  redistribution, provenance/source notice, review status, checksum records,
  and audit-manifest hooks.
- JSON checksum handling records algorithm, canonicalization, payload scope,
  payload reference, and value using deterministic SHA-256 over caller-supplied
  canonical payload bytes.
- Diagnostics cover missing source notice, missing/unknown redistribution
  status, missing or stale checksum, suspected protected content, attempted
  public export of private content, and professional-boundary violations.
- Audit/report-facing references expose rule-pack identity/version/checksum and
  source note without exposing private rule-pack content.
- The module does not implement private storage, encryption, access control,
  secret handling, GUI/editor behavior, report generation, API transport, or
  final result-envelope integration.
- Tests verify deterministic checksum behavior, lifecycle diagnostics, private
  export blocking, manifest hook redaction, stale checksum detection, and
  absence of software-generated compliance/certification/sealing claims.
- No protected standards text, protected tables, copied formulas, proprietary
  values, private project data, private rule-pack payloads, or professional
  approval/code-compliance claims are introduced.

## Implementation Summary

Implemented within the sealed write scope:

- Added `core/rules/rule_pack_lifecycle` with a bounded Rust metadata/checksum
  module and focused unit tests.
- Implemented SHA-256 checksum generation over caller-supplied canonical bytes,
  recording JCS-compatible metadata for JSON payloads.
- Implemented lifecycle validation findings for missing metadata, missing or
  stale checksum, suspected protected content, blocked public export of private
  content, and professional-boundary violations.
- Implemented audit-manifest references that expose only identity, version,
  source notice, privacy/redistribution/review status, checksum metadata, and
  private-payload redaction state.
- Updated `docs/SPEC.md`, `docs/TYPES.md`, and deliverable `MEMORY.md`.

Verification performed:

- `cargo fmt --manifest-path core/rules/rule_pack_lifecycle/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/rules/rule_pack_lifecycle/Cargo.toml`
  passed: 8 tests.
- `python3 tests/test_rule_pack_schema.py` passed.
- `cargo test --manifest-path core/rules/expression_evaluator/Cargo.toml`
  passed: 14 tests.
- `git diff --check` passed.
- Focused protected-content/prohibited-claim scan found only guardrail,
  exclusion, and diagnostic wording.

Lifecycle/evidence/queue closeout:

- `DEL-06-04` lifecycle moved from `SEMANTIC_READY` to `CHECKING`.
- `DEL-06-04` local dependency mirror rows `DAG-001-E0472` and
  `DAG-001-E0473` record satisfied upstreams `DEL-06-01` and `DEL-02-05`.
- `DEV-001_IMPLEMENTATION_EVIDENCE.csv` records `DEL-06-04` as
  `COMMITTED` evidence `ad270f6`.
- `DEV-001_BLOCKER_QUEUE.*` was refreshed from approved active `DAG-001` edges
  and implementation evidence; queue changed to 54 unblocked / 19 blocked.
- `DEL-06-04` no longer appears as a missing upstream blocker; `DEL-08-02` is
  newly unblocked.
- Aggregate `DAG-001` was validated and left unchanged.
