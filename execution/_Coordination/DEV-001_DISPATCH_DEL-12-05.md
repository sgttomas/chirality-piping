---
doc_id: DEV-001-DISPATCH-DEL-12-05
doc_kind: coordination.dispatch_brief
status: implemented_awaiting_lifecycle_evidence_queue_gate
created: 2026-05-02
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-12-05
package_id: PKG-12
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-12-05 Security Threat Model

**Dispatch status:** implemented from sealed brief on 2026-05-02; lifecycle,
evidence, local dependency-register alignment, blocker-queue refresh, staging,
and commit await separate authorization.
**Coordination mode:** `FULL_GRAPH`
**Graph authority:** `execution/_DAG/DAG-001/DependencyEdges.csv`
**Implementation threshold:** upstream `COMMITTED` evidence

## Dispatch Decision

The human project authority authorized one bounded ORCHESTRATOR action:

- prepare `execution/_Coordination/DEV-001_DISPATCH_DEL-12-05.md`
  for `DEL-12-05 - Security threat model`.

The first authorization prepared the implementation brief only. A later human
message authorized implementation from this sealed brief. That implementation
authorization did not authorize lifecycle transition, implementation-evidence
registration, dependency-register edits, blocker-queue refresh, `DAG-001`
changes, candidate-edge promotion, staging, commit, or broad DAG execution.

The eventual implementation scope should be deliberately constrained to a
public threat-model document for OpenPipeStress private data, report sharing,
plugins, imports, local-first storage boundaries, telemetry posture, secrets,
and supply-chain exposure. It does not authorize cloud-service design,
encryption/key-management selection, plugin permission finalization, public API
transport selection, import/export format selection, protected standards
content, real private project data, real secrets, private libraries, legal
sufficiency claims, professional/code-compliance claims, or security
certification claims.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-12-05` |
| PackageID | `PKG-12` |
| Name | Security threat model |
| Type | `DOC_UPDATE` |
| Scope items | `SOW-040` |
| Objectives | `OBJ-010` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-05_Security threat model` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| EdgeID | Upstream | Dependency type | Implementation-readiness satisfaction |
|---|---|---|---|
| `DAG-001-E0382` - `DAG-001-E0388` | Applicable `PKG-00` architecture basis | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DAG-001-E0598` | `DEL-01-02` Copyright and protected-data boundary policy | `SECURITY_PREDECESSOR` | `COMMITTED` evidence `0d729cf` |
| `DAG-001-E0599` | `DEL-02-04` Plugin and extension domain contracts | `SECURITY_PREDECESSOR` | `COMMITTED` evidence `ef44f4c` |
| `DAG-001-E0600` | `DEL-01-04` Professional responsibility and product-claims policy | `SECURITY_PREDECESSOR` | `COMMITTED` evidence `65f3119` |

Current implementation-readiness queue state:

- `DEL-12-05` is `UNBLOCKED`.
- `DEL-12-05` has `MISSING_EVIDENCE`; it is not implemented.
- Candidate edges are excluded.
- `DEL-12-05` currently gates downstream consumers `DEL-10-02`,
  `DEL-10-04`, `DEL-12-01`, `DEL-12-02`, `DEL-12-03`, and `DEL-12-04`.

Candidate caveats retained as non-gating:

- `DAG-001-E0619` asks whether `DEL-12-05` may need `DEL-10-02` adapter
  details before finalization; this remains `CANDIDATE` and must not block this
  dispatch.
- `DAG-001-E0623` asks whether `DEL-06-02` may need threat-model review before
  implementation freeze; this remains `CANDIDATE` and must not change current
  evidence or ordering.

## Applicable Architecture Basis

Applicable basis IDs from `SCA-001`: `AB-00-01`, `AB-00-02`, `AB-00-03`,
`AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.

Resolved baseline to preserve: ADR traceability; layer/module responsibilities
for GUI, application services, domain core, solver, rules, reports, adapters,
storage, schemas, validation, and tests; schema-first command/query/job result
envelopes; deterministic persistence/canonical JSON hash basis where JSON
payloads are hashed; diagnostics/result-envelope warning classes; API/plugin
no-bypass boundaries; and layered validation/protected-content/security gates
where applicable.

The threat model must preserve these architecture boundaries without resolving
implementation-level TBDs. Exact dependency versions, encryption/key
management, plugin permission model, public API transport, import/export format
list, CI provider/thresholds, build signing process, secret storage, and
physical project package/container remain `TBD` unless separately approved by
the human project authority.

## Explicit Write Scope

The bounded implementation write scope, if separately authorized, is limited
to:

- `docs/security/threat_model.md`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-05_Security threat model/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-12-05.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch. In particular, do not edit
security-control implementation code, schemas, plugin manifests, report/export
code, storage/redaction/telemetry/secret-handling implementation, `DAG-001`,
candidate edges, deliverable-local `Dependencies.csv`,
`DEV-001_IMPLEMENTATION_EVIDENCE.csv`, or `DEV-001_BLOCKER_QUEUE.*` during
implementation unless separately authorized.

## Applicable Invariants

- `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`
- `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`
- `OPS-K-AUTH-1`, `OPS-K-AUTH-2`
- `OPS-K-MECH-2`
- `OPS-K-RULE-2`, `OPS-K-RULE-3`
- `OPS-K-REPORT-1`, `OPS-K-REPORT-2`
- `OPS-K-PRIV-1`, `OPS-K-PRIV-2`
- `OPS-K-GOV-4`
- `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`, `OPS-K-AGENT-4`

## Acceptance Criteria

This bounded item is acceptable for implementation closeout when:

- `docs/security/threat_model.md` exists and is traceable to `DEL-12-05`,
  `PKG-12`, `SOW-040`, and `OBJ-010`.
- The document covers at least these asset/surface families: private project
  files, private rule packs, private material/component libraries, reports,
  exports/shared models, diagnostics/logs, bug reports, telemetry, plugins,
  import/export adapters, FEA handoff, secrets, build/release artifacts, and
  supply-chain dependencies.
- Trust boundaries preserve local-first behavior, public repository boundaries,
  report/export disclosure boundaries, bug-report/diagnostic disclosure
  boundaries, plugin/import no-bypass boundaries, rule-evaluator sandbox
  boundaries, and supply-chain/build boundaries.
- Threat rows identify conservative controls or explicit `TBD` decisions for
  redaction, export warnings, provenance, redistribution status, protected
  content suspicion, quarantine, diagnostics, public/private markings,
  checksums, private storage, secrets, telemetry opt-in, dependency/release
  review, and build/package integrity.
- The threat model states that cloud operation is out of MVP unless separately
  approved, and that telemetry is disabled by default and cannot include
  private engineering/code data without explicit user action.
- Plugin/import/adapter/FEA handoff threats preserve the no-bypass rule for
  schema validation, unit checks, provenance checks, privacy/private-data
  controls, protected-content screening, diagnostics, report controls,
  rule-pack sandboxing, solver boundaries, and human acceptance boundaries.
- Report/export/template threats preserve the IP and professional boundaries:
  no public protected standards text, tables, copied formulas, proprietary
  benchmark content, private project data, or automatic professional/code
  compliance claims.
- All unresolved implementation choices remain `TBD` rather than being
  invented.
- Deliverable `MEMORY.md` records the work, source basis, verification, and
  remaining open decisions.
- Verification includes `git diff --check` and focused scans for prohibited
  protected-content, real-secret, private-data, and certification/compliance
  claim patterns.
- No lifecycle transition, dependency-register edit, candidate-edge change,
  blocker-queue refresh, implementation-evidence registration, protected
  standards data, proprietary engineering value, real private data, real
  secret, or professional/security certification claim occurs unless separately
  authorized.

## Dispatch Task Shape

```markdown
PURPOSE: Implement the sealed OpenPipeStress deliverable within explicit scope.
RequestedBy: WORKING_ITEMS

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-05_Security threat model
DeliverablePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-05_Security threat model
TaskProfile: security-privacy

DeliverableID: DEL-12-05
PackageID: PKG-12

Tasks:
  - Implement only the artifacts authorized for DEL-12-05.
  - Create or update docs/security/threat_model.md as the public product
    threat model.
  - Preserve all applicable contract invariants and architecture-basis
    constraints.
  - Cover private data, report sharing, plugins, imports, local-first storage
    boundaries, telemetry, secrets, and supply chain.
  - Keep implementation choices such as encryption, permission model, API
    transport, import/export formats, secret storage, signing, and project
    container format as TBD unless already approved by a human ruling.
  - Do not implement security controls, edit schemas, change plugin/runtime
    behavior, promote candidate edges, or introduce protected/private content.

Return:
  - Files changed.
  - Tests, scans, and validation commands run.
  - Threat surface coverage summary.
  - Remaining TBDs and blocked decisions.
```

## Implementation Summary

Implemented within the sealed write scope:

- Added `docs/security/threat_model.md` as the public product threat model.
- Added deliverable working memory at
  `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-05_Security threat model/MEMORY.md`.
- Covered private project files, private rule packs, private material/component
  libraries, reports, exports/shared models, diagnostics/logs, bug reports,
  telemetry, plugins, import/export adapters, FEA handoff, secrets,
  build/release artifacts, and supply-chain dependencies.
- Preserved local-first behavior, telemetry-off-by-default posture,
  plugin/import no-bypass constraints, rule-evaluator sandbox boundaries,
  report/export boundaries, and professional-boundary constraints.
- Kept unresolved choices as `TBD`, including encryption, secret storage,
  plugin permission model, API transport, import/export format list, redaction
  workflow, bug-report bundle format, telemetry event schema, signing process,
  and reproducible build policy.
- No lifecycle transition, evidence registration, dependency-register edit,
  blocker-queue refresh, `DAG-001` edit, candidate-edge promotion, security
  control implementation, schema edit, plugin/runtime behavior change,
  protected standards content, real private data, real secret, or professional,
  code-compliance, or security-certification claim was introduced.

Verification performed:

- `git diff --check` passed.
- Focused protected-content/prohibited-claim/real-secret scan passed with only
  guardrail and exclusion wording.

## Current Stop Point

Stop after implementation from this sealed brief. Lifecycle/evidence/local
dependency-register alignment, blocker-queue refresh, staging, and commit
require separate human authorization.
