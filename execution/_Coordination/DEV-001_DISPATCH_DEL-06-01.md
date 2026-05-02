---
doc_id: DEV-001-DISPATCH-DEL-06-01
doc_kind: coordination.dispatch_brief
status: implemented_awaiting_lifecycle_evidence_gate
created: 2026-05-02
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-06-01
package_id: PKG-06
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-06-01

## Dispatch Decision

The human project authority authorized preparation of one sealed dispatch brief:

- `DEL-06-01 - Rule-pack schema`

This authorization prepares the implementation brief only. It does not
authorize implementation, lifecycle transition, implementation-evidence
mutation, dependency-register edits, blocker-queue refresh, `DAG-001` changes,
candidate-edge promotion, or broad DAG execution.

The human project authority later authorized implementation from this sealed
brief. Implementation has been completed within the bounded write scope.
Lifecycle transition, implementation evidence registration, dependency-register
alignment, blocker-queue refresh, `DAG-001` changes, and commit have not been
performed by this implementation pass.

The eventual implementation scope should be deliberately constrained to a
code-neutral, schema-first rule-pack artifact contract: metadata, required
inputs, formula declaration slots, user-supplied allowable slots, provenance,
redistribution status, checksum metadata, lifecycle/status fields, diagnostics,
and professional-boundary markings. It does not authorize protected standards
text, copied standards formulas, bundled code-specific allowables, public
rule-pack examples, a sandboxed evaluator implementation, GUI editors, private
storage lifecycle implementation, report generation, or professional/code
compliance claims.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-06-01` |
| PackageID | `PKG-06` |
| Name | Rule-pack schema |
| Type | `DATA_MODEL_CHANGE` |
| Scope items | `SOW-016`, `SOW-042` |
| Objectives | `OBJ-005` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-01_Rule-pack schema` |
| Current lifecycle | `SEMANTIC_READY` |

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
| `DEL-02-01` | `SCHEMA_CONTRACT` | `COMMITTED` evidence `7b256f3` |
| `DEL-02-02` | `UNIT_CONTRACT` | `COMMITTED` evidence `a458cba` |
| `DEL-02-03` | `DOMAIN_MODEL` | `COMMITTED` evidence `abc1306` |
| `DEL-01-02` | `GOVERNANCE_PREDECESSOR` | `COMMITTED` evidence `0d729cf` |
| `DEL-01-04` | `GOVERNANCE_PREDECESSOR` | `COMMITTED` evidence `65f3119` |

Current implementation-readiness queue state:

- `DEL-06-01` is `UNBLOCKED`.
- `DEL-06-01` has `MISSING_EVIDENCE` for its own implementation.
- Candidate edges are excluded.
- `DEL-06-01` currently gates downstream consumers `DEL-06-02`,
  `DEL-06-03`, `DEL-06-04`, `DEL-06-05`, `DEL-07-03`, and `DEL-11-02`.

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

Resolved baseline to preserve: rule packs are user-owned artifacts that may
declare user-rule checks over mechanics results, required user-supplied inputs,
allowable slots, and formula declaration slots without embedding protected
standards content or arbitrary executable code. The schema must carry
provenance, redistribution status, checksum metadata, diagnostics, and
professional-boundary fields so downstream evaluator, completeness, private
lifecycle, GUI editor, report, and documentation work can consume the contract
without bypassing IP, privacy, unit, diagnostics, or human-approval constraints.

Remaining implementation-level TBDs are not resolved by this dispatch:
rule expression grammar/library, evaluator execution semantics, concrete
formula AST, public example content, private storage location, checksum library,
non-JSON asset handling, public API transport, GUI editor presentation, and
final rule-check result-envelope integration.

## Explicit Write Scope

The bounded implementation write scope, if separately authorized, is limited to:

- `schemas/rule_pack.schema.yaml`
- `tests/test_rule_pack_schema.py`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-01_Rule-pack schema/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-06-01.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch. In particular, do not edit
`DAG-001`, candidate edges, deliverable-local `Dependencies.csv`,
`DEV-001_IMPLEMENTATION_EVIDENCE.csv`, or `DEV-001_BLOCKER_QUEUE.*` during
implementation unless separately authorized.

## Implementation Summary

Implemented within the sealed write scope:

- Added `schemas/rule_pack.schema.yaml` as a strict JSON-syntax JSON Schema
  2020-12 contract for user-owned rule-pack artifacts.
- Defined schema surfaces for metadata, public/private classification,
  required inputs, declarative formula slots, user-supplied value slots, check
  definitions, diagnostics, checksums, provenance, professional-boundary flags,
  and open decisions.
- Added `tests/test_rule_pack_schema.py` with focused stdlib checks for
  required fields, provenance/redistribution controls, checksum basis,
  required-input structure, declarative formula constraints, value slots,
  status boundaries, diagnostics, and prohibited claim wording.
- Updated `docs/SPEC.md` and `docs/TYPES.md` with the bounded rule-pack schema
  surface.
- Added deliverable `MEMORY.md` with implementation notes and downstream open
  items.

Verification performed:

- `python3 tests/test_rule_pack_schema.py` passed.
- `python3 tests/test_units_schema.py` passed.
- `python3 tests/test_analysis_boundary_schema.py` passed.
- `python3 tests/test_analysis_status_schema.py` passed.
- `python3 tests/test_model_schema.py` passed.
- `python3 tests/test_plugin_manifest_schema.py` passed.
- `python3 tests/test_persistence_schema.py` passed.
- `python3 tests/test_material_schema.py` passed.
- `python3 tests/test_component_section_schema.py` passed.
- `git diff --check` passed.
- Focused protected-content/prohibited-claim scan found only existing
  governance guardrail text and dispatch protocol wording; no protected
  standards data, proprietary engineering values, or software-generated
  professional/code-compliance claim was introduced.

## Acceptance Criteria

This bounded item is acceptable for implementation closeout when:

- The rule-pack schema uses JSON Schema 2020-12 conventions and can be loaded by
  the repository's existing stdlib schema-check style.
- The schema defines required identity, schema version, rule-pack version,
  lifecycle/status, source notes, public/private classification, redistribution
  status, checksum metadata, and professional-boundary fields.
- The schema supports required-input declarations with unit/dimensional intent,
  provenance requirements, completeness status, and missing-input diagnostics.
- The schema supports formula declaration slots as declarative data only, with
  no arbitrary executable code and no protected standards text or copied
  standards formulas.
- The schema supports user-supplied allowable/value slots with units,
  provenance, source notes, redistribution status, review status, and
  completeness status.
- The schema represents user-rule pass/fail/incomplete states while preserving
  the distinction between mechanics solved, user-rule checked, and human
  professional approval.
- Diagnostics cover rule-check blocking, provenance warnings, unit mismatch,
  protected-content warning, evaluator error, redistribution warning, and
  incomplete data conditions.
- Tests cover required fields, provenance/redistribution controls, checksum
  metadata, required-input structure, declarative formula constraints,
  user-supplied allowable slots, status boundaries, and prohibited claim/code
  wording.
- Public examples are not added unless separately authorized; any fixture data
  used by tests is invented, non-code, and not suitable for engineering use.
- No lifecycle transition, dependency-register edit, candidate-edge change,
  blocker-queue refresh, protected standards data, proprietary engineering
  value, or professional/code-compliance claim occurs unless separately
  authorized.

## Dispatch Task Shape

```markdown
PURPOSE: Implement the sealed OpenPipeStress deliverable within explicit scope.
RequestedBy: WORKING_ITEMS

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-01_Rule-pack schema
DeliverablePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-01_Rule-pack schema
TaskProfile: rule-pack-engine

DeliverableID: DEL-06-01
PackageID: PKG-06

Tasks:
  - Implement only the rule-pack schema, focused schema tests, docs updates, and memory update authorized for this deliverable.
  - Preserve all applicable contract invariants and architecture-basis constraints.
  - Keep all candidate edges non-gating; do not implement `DEL-06-02`, `DEL-06-03`, `DEL-06-04`, `DEL-06-05`, GUI editor behavior, or developer-guide content.
  - Record a run summary and open issues.

RuntimeOverrides:
  DecompositionPath: docs/_Decomposition/SOFTWARE_DECOMP.md
  DecompositionRevision: "0.4"
  DAGPath: execution/_DAG/DAG-001/DependencyEdges.csv
  DAGApprovalRecord: execution/_DAG/DAG-001/APPROVAL_RECORD.md
  CoordinationMode: FULL_GRAPH
  BlockerComputation: ENABLED_ACTIVE_EDGES_ONLY
  ScopeChangeID: SCA-001

CustomInstructions:
  - Read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and existing schema/test patterns before editing.
  - Apply only applicable `AB-00-*` constraints; do not copy full `PKG-00` prose.
  - Treat protected standards/code data as out of scope.
  - Unknown expression-grammar, evaluator, checksum-library, storage, API, GUI, and report-integration details remain `TBD`.
  - Do not claim certification, approval, sealing, authentication, or code compliance for reliance.
  - Do not edit files outside this sealed write scope.
  - Do not recompute or mutate blocker queues unless explicitly assigned.

ExpectedOutputs:
  - `schemas/rule_pack.schema.yaml` implementing the bounded schema contract.
  - `tests/test_rule_pack_schema.py` with focused stdlib schema checks.
  - `docs/SPEC.md` and `docs/TYPES.md` updates for the bounded schema surface.
  - Deliverable `MEMORY.md` update.
  - Open issue list for unresolved `TBD`, assumptions, or cross-deliverable dependencies.
```
