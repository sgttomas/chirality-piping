---
doc_id: DEV-001-DISPATCH-DEL-10-02
doc_kind: coordination.dispatch_brief
status: sealed_brief_prepared
created: 2026-05-02
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-10-02
package_id: PKG-10
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-10-02 Import/Export Adapter Framework

**Dispatch status:** implemented in working tree on 2026-05-02 after separate
human approval from the sealed brief.
**Coordination mode:** `FULL_GRAPH`
**Graph authority:** `execution/_DAG/DAG-001/DependencyEdges.csv`
**Implementation threshold:** upstream `COMMITTED` evidence

## Dispatch Decision

The human project authority authorized one bounded ORCHESTRATOR action:

- prepare `execution/_Coordination/DEV-001_DISPATCH_DEL-10-02.md`
  for `DEL-10-02 - Import/export adapter framework`.

This authorization prepares the implementation brief only. It does not
authorize implementation, lifecycle transition, implementation-evidence
registration, dependency-register edits, blocker-queue refresh, `DAG-001`
changes, candidate-edge promotion, staging, commit, or broad DAG execution.

The eventual implementation scope should be constrained to a format-neutral
adapter framework contract/support surface. It may define adapter interface
contracts, a small in-memory adapter support module, a sample invented adapter
fixture, focused docs, and deterministic tests. It must not choose concrete
external import/export formats, public transport, endpoint syntax, plugin
runtime/loading/signing, package manifests, CI/release behavior, local FEA
handoff format, GUI/report runtime behavior, protected standards content,
private engineering data, real secrets, or professional/code-compliance or
security-certification claims.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-10-02` |
| PackageID | `PKG-10` |
| Name | Import/export adapter framework |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope items | `SOW-030` |
| Objectives | `OBJ-009` |
| Context envelope | `L` |
| Deliverable path | `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-02_Import-export adapter framework` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| EdgeID | Upstream | Dependency type | Implementation-readiness satisfaction |
|---|---|---|---|
| `DAG-001-E0301` - `DAG-001-E0307` | Applicable `PKG-00` architecture basis | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DAG-001-E0556` | `DEL-10-01` Public API and plugin boundary | `INTEROP_PREDECESSOR` | `COMMITTED` evidence `53cc3d6` |
| `DAG-001-E0557` | `DEL-03-07` Public/private library import provenance checker | `GOVERNANCE_PREDECESSOR` | `COMMITTED` evidence `4d880b3` |
| `DAG-001-E0558` | `DEL-02-04` Plugin and extension domain contracts | `SERVICE_API` | `COMMITTED` evidence `ef44f4c` |
| `DAG-001-E0559` | `DEL-12-01` Local-first storage and private data paths | `SECURITY_PREDECESSOR` | `COMMITTED` evidence `84e0a73` |
| `DAG-001-E0560` | `DEL-12-05` Security threat model | `SECURITY_PREDECESSOR` | `COMMITTED` evidence `b97121d` |

Current implementation-readiness queue state:

- `DEL-10-02` is `UNBLOCKED`.
- `DEL-10-02` has `MISSING_EVIDENCE`; it is not yet recorded as implemented.
- Candidate edges are excluded.

Downstream impact if later implemented and committed:

- No active implementation-readiness blocker currently lists `DEL-10-02` as a
  missing upstream provider.
- Candidate edge `DAG-001-E0619` mentions possible reverse coupling from
  `DEL-12-05` to `DEL-10-02`; it remains non-gating and must not affect
  dispatch readiness unless later reconciled and promoted.

## Applicable Architecture Basis

Applicable basis IDs from `SCA-001`: `AB-00-01`, `AB-00-02`, `AB-00-03`,
`AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.

Resolved baseline to preserve: ADR traceability; layer/module responsibilities
for GUI, application services, domain core, solver, storage, reporting,
private libraries, plugins, adapters, schemas, validation, and tests;
schema-first command/query/job/result-envelope concepts; JSON Schema 2020-12
contracts; canonical JSON/JCS-compatible hash basis where JSON payloads are
hashed; diagnostics/result-envelope warning classes; API/plugin/adapter
no-bypass boundaries; local-first private/public path controls; and layered
validation/protected-content/security gates where applicable.

The adapter framework must preserve the `DEL-10-01` public API/plugin
boundary, `DEL-03-07` provenance checker posture, `DEL-02-04` extension-domain
contract, `DEL-12-01` local-first storage policy, and `DEL-12-05` plugin/import
threat-model constraints. Concrete external formats, public API transport,
endpoint syntax, OpenAPI binding, adapter execution/loading model, plugin
runtime, permission persistence, physical project package/container, local FEA
handoff package, redaction workflow, CI provider, release matrix, and
format-specific parse/write behavior remain `TBD` unless separately approved
by the human project authority.

## Explicit Write Scope

The bounded implementation write scope, if separately authorized, is limited
to:

- `schemas/adapter_framework.schema.yaml`
- `core/adapters/framework/`
- `fixtures/adapters/invented/`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `tests/test_adapter_framework_contract.py`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-02_Import-export adapter framework/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-10-02.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch. In particular, do not edit GUI
code, report renderer code, runtime API server code, plugin loader/runtime
code, concrete import/export adapter implementations, local FEA handoff
artifacts, package manifests, CI workflows, release scripts, public API
transport bindings, storage runtime code, `DAG-001`, candidate edges,
deliverable-local `Dependencies.csv`, `DEV-001_IMPLEMENTATION_EVIDENCE.csv`,
or `DEV-001_BLOCKER_QUEUE.*` during implementation unless separately
authorized.

## Applicable Invariants

- `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`
- `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`
- `OPS-K-AUTH-1`, `OPS-K-AUTH-2`
- `OPS-K-MECH-2`
- `OPS-K-UNIT-1`
- `OPS-K-RULE-1`, `OPS-K-RULE-2`, `OPS-K-RULE-3`
- `OPS-K-REPORT-1`, `OPS-K-REPORT-2`
- `OPS-K-PRIV-1`, `OPS-K-PRIV-2`
- `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`, `OPS-K-AGENT-4`

## Acceptance Criteria

This bounded item is acceptable for implementation closeout when:

- `schemas/adapter_framework.schema.yaml` exists, remains strict JSON syntax
  parseable by Python `json`, and is traceable to `DEL-10-02`, `PKG-10`,
  `SOW-030`, and `OBJ-009`.
- The contract defines a schema-first adapter framework envelope for import,
  export, adapter capability declaration, validation planning, diagnostics,
  provenance, privacy classification, checksum/audit references, and
  public/private contribution review without choosing final external formats,
  public transport, endpoint syntax, runtime plugin loading, package scripts,
  CI provider, release matrix, local FEA package format, or GUI/report runtime
  behavior.
- Adapter interfaces route through public API/plugin command, query, job, and
  result-envelope boundaries consistent with `DEL-10-01`; adapters cannot
  bypass unit validation, provenance capture, redistribution review,
  private/public data classification, protected-content screening,
  diagnostics, rule-pack sandboxing, persistence/hash controls, report
  controls, or human-acceptance boundaries.
- Import paths distinguish syntactic parse readiness, schema validation, unit
  and dimensional validation, provenance completeness, redistribution status,
  privacy/publication posture, protected-content suspicion, mechanics
  readiness, rule-check readiness, and human-review-needed state.
- Export paths require explicit private/public boundary checks before payloads
  may leave the local project context, and must surface diagnostics for private
  data, protected-suspected content, missing provenance, unresolved
  redistribution rights, or unresolved redaction/export controls.
- The support module, if implemented, provides bounded in-memory adapter
  declarations, capability validation, invented adapter fixture validation, and
  deterministic result/finding construction only. It does not parse real
  external files, implement concrete commercial/open external formats, access
  the network, invoke external processes, choose filesystem roots, mutate
  project storage, or perform GUI/report/local-FEA runtime behavior.
- The sample invented adapter fixture uses invented, non-code,
  non-proprietary data only; it records source/provenance, redistribution
  status, contributor certification, privacy classification, and review
  disposition. It must not embed protected standards text, protected tables,
  copied code formulas, material allowables, SIF/flexibility tables, protected
  dimensional tables, proprietary vendor data, private project data, private
  rule-pack payloads, real secrets, real user paths, or real commercial format
  examples.
- Diagnostics use structured fields compatible with the architecture baseline:
  code, class, severity, source, affected object, message, remediation, and
  provenance.
- Rule-pack hook metadata, if included, preserves sandboxed deterministic
  evaluation, unit awareness, required-input diagnostics, rule-pack identity,
  version, checksum, source notice, redistribution status, and private/public
  marking without exposing private formulas or executing arbitrary code.
- Report/export hook metadata, if included, preserves warnings, assumptions,
  provenance, limitations, protected-content controls, professional-boundary
  notices, and result-envelope references.
- The adapter framework preserves the code-neutral authority boundary:
  mechanics solve, user-rule checking, and human professional acceptance are
  distinct; adapter outputs do not emit automatic `CODE_COMPLIANT`,
  certification, sealing, approval, endorsement, authentication, security
  certification, compliance attestation, or professional-reliance statuses.
- Deterministic tests under `tests/test_adapter_framework_contract.py` cover
  JSON parseability, metadata traceability, unresolved `TBD` format/transport/
  runtime/package decisions, operation categories, no-bypass controls,
  provenance and redistribution requirements, unit/dimension requirements,
  privacy/export controls, diagnostic shape, invented fixture constraints, and
  absence of prohibited professional/compliance/security-certification terms.
- Deliverable `MEMORY.md` records the work, source basis, verification, and
  remaining open decisions.
- Verification includes `python3 tests/test_adapter_framework_contract.py`,
  `python3 tests/test_api_boundary_contract.py`,
  `python3 tests/test_library_import_provenance.py`, `git diff --check`, and
  focused scans for protected standards content, private data, real secrets,
  real external format commitments, network/process/filesystem commitments,
  package/CI/release commitments, and certification/compliance/security claim
  patterns.
- No lifecycle transition, dependency-register edit, candidate-edge change,
  blocker-queue refresh, implementation-evidence registration, protected
  standards data, proprietary engineering value, real private data, real
  secret, final external format choice, final public transport choice, package
  manifest/script change, CI provider choice, release matrix choice,
  GUI/report/local-FEA runtime implementation, or professional/security/
  code-compliance claim occurs unless separately authorized.

## Dispatch Task Shape

```markdown
PURPOSE: Implement the sealed OpenPipeStress deliverable within explicit scope.
RequestedBy: WORKING_ITEMS

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-02_Import-export adapter framework
DeliverablePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-02_Import-export adapter framework
TaskProfile: interop-build

DeliverableID: DEL-10-02
PackageID: PKG-10

Tasks:
  - Implement only the artifacts authorized for DEL-10-02.
  - Add schemas/adapter_framework.schema.yaml as a strict-JSON JSON Schema
    2020-12 contract for the format-neutral import/export adapter framework.
  - Add core/adapters/framework/ as a bounded support module for in-memory
    adapter declaration validation and deterministic finding/result construction.
  - Add fixtures/adapters/invented/ with invented, non-code,
    non-proprietary sample adapter payloads only.
  - Add deterministic contract tests under tests/test_adapter_framework_contract.py.
  - Update focused docs/SPEC.md and docs/TYPES.md sections only as needed for
    the adapter framework boundary.
  - Preserve all applicable contract invariants, architecture-basis
    constraints, and upstream DEL-10-01/03-07/02-04/12-01/12-05 contracts.
  - Keep concrete external formats, public transport, endpoint syntax, plugin
    runtime/loading/signing, permission persistence, package scripts, CI
    provider, release matrix, physical project container, local FEA package,
    redaction workflow, and GUI/report runtime behavior as TBD unless already
    approved by a human ruling.
  - Do not implement real external file parsing, commercial/proprietary format
    behavior, network/process access, filesystem root selection, storage
    runtime behavior, API server code, plugin runtime code, GUI code, report
    renderer code, local FEA handoff code, package manifests, CI workflows,
    release scripts, candidate-edge changes, dependency-register edits, or
    protected/private content.

Return:
  - Files changed.
  - Tests, scans, and validation commands run.
  - Adapter framework contract summary.
  - Remaining TBDs and blocked decisions.
```

## Current Stop Point

Human project authority later authorized implementation from this sealed brief
only. Implementation stayed within the approved write scope:

- `schemas/adapter_framework.schema.yaml`
- `core/adapters/framework/`
- `fixtures/adapters/invented/`
- `tests/test_adapter_framework_contract.py`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-02_Import-export adapter framework/MEMORY.md`
- this dispatch brief
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

Implementation summary:

- Added a strict-JSON JSON Schema 2020-12 adapter framework contract for
  format-neutral adapter declarations, validation plans, operation results,
  diagnostics, provenance, privacy, checksums, audit references, and
  result-envelope compatibility.
- Added a bounded Python support module for in-memory adapter declaration
  validation and deterministic operation-result construction.
- Added an invented adapter fixture with invented, non-code, non-proprietary
  data only.
- Added deterministic stdlib tests and focused docs/type updates.

Lifecycle transition, dependency-register edits, implementation-evidence
registration, blocker-queue refresh, candidate-edge promotion, staging, commit,
and broad DAG execution require a separate human approval gate.
