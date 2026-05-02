---
doc_id: DEV-001-DISPATCH-DEL-12-03
doc_kind: coordination.dispatch_brief
status: sealed_brief_prepared
created: 2026-05-02
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-12-03
package_id: PKG-12
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-12-03 Telemetry Off-By-Default Design

**Dispatch status:** sealed dispatch brief prepared on 2026-05-02.
**Coordination mode:** `FULL_GRAPH`
**Graph authority:** `execution/_DAG/DAG-001/DependencyEdges.csv`
**Implementation threshold:** upstream `COMMITTED` evidence

## Dispatch Decision

The human project authority authorized one bounded ORCHESTRATOR action after
`DEL-12-05` lifecycle/evidence/local dependency alignment and blocker-queue
refresh passed:

- prepare `execution/_Coordination/DEV-001_DISPATCH_DEL-12-03.md`
  for `DEL-12-03 - Telemetry off-by-default design`.

This authorization prepares the implementation brief only. It does not
authorize implementation, lifecycle transition, implementation-evidence
registration, dependency-register edits, blocker-queue refresh, `DAG-001`
changes, candidate-edge promotion, staging, commit, or broad DAG execution.

The eventual implementation scope should be deliberately constrained to
telemetry policy/default behavior that proves telemetry is absent or disabled
by default unless explicitly opted in. It does not authorize cloud-service
behavior, telemetry vendor selection, endpoint selection, broad product config
schema design, private data transmission, real telemetry events, plugin/runtime
behavior changes, protected standards content, real private project data, real
secrets, or professional/code-compliance/security certification claims.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-12-03` |
| PackageID | `PKG-12` |
| Name | Telemetry off-by-default design |
| Type | `SECURITY_CONTROL` |
| Scope items | `SOW-037` |
| Objectives | `OBJ-010` |
| Context envelope | `S` |
| Deliverable path | `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| EdgeID | Upstream | Dependency type | Implementation-readiness satisfaction |
|---|---|---|---|
| `DAG-001-E0368` - `DAG-001-E0374` | Applicable `PKG-00` architecture basis | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DAG-001-E0604` | `DEL-12-05` Security threat model | `SECURITY_PREDECESSOR` | `COMMITTED` evidence `b97121d` |
| `DAG-001-E0605` | `DEL-01-02` Copyright and protected-data boundary policy | `SECURITY_PREDECESSOR` | `COMMITTED` evidence `0d729cf` |

Current implementation-readiness queue state:

- `DEL-12-03` is `UNBLOCKED`.
- `DEL-12-03` has `MISSING_EVIDENCE`; it is not implemented.
- Candidate edges are excluded.

## Applicable Architecture Basis

Applicable basis IDs from `SCA-001`: `AB-00-01`, `AB-00-02`, `AB-00-03`,
`AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.

Resolved baseline to preserve: ADR traceability; layer/module responsibilities
for GUI, application services, storage, schemas, validation, adapters, and
tests; schema-first command/query/job result envelopes where telemetry touches
services; deterministic persistence/canonical JSON hash basis where JSON
payloads are hashed; diagnostics/result-envelope warning classes; API/plugin
no-bypass boundaries; and layered validation/protected-content/security gates
where applicable.

The telemetry design must preserve local-first behavior and the `DEL-12-05`
threat-model posture: telemetry is disabled by default, opt-in only if ever
implemented, and cannot include private engineering/code data unless a user
explicitly chooses a payload. Exact config schema, storage location, event
schema, consent UI, endpoint, vendor, retention policy, and transport remain
`TBD` unless separately approved by the human project authority.

## Explicit Write Scope

The bounded implementation write scope, if separately authorized, is limited
to:

- `docs/security/telemetry_policy.md`
- `tests/security/test_telemetry_policy.py`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-12-03.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch. In particular, do not edit
runtime networking code, GUI code, product config schemas, plugin manifests,
adapter code, telemetry vendor integrations, `DAG-001`, candidate edges,
deliverable-local `Dependencies.csv`, `DEV-001_IMPLEMENTATION_EVIDENCE.csv`,
or `DEV-001_BLOCKER_QUEUE.*` during implementation unless separately
authorized.

## Applicable Invariants

- `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`
- `OPS-K-DATA-1`, `OPS-K-DATA-2`
- `OPS-K-AUTH-1`, `OPS-K-AUTH-2`
- `OPS-K-RULE-2`
- `OPS-K-REPORT-2`
- `OPS-K-PRIV-1`, `OPS-K-PRIV-2`
- `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`, `OPS-K-AGENT-4`

## Acceptance Criteria

This bounded item is acceptable for implementation closeout when:

- `docs/security/telemetry_policy.md` exists and is traceable to `DEL-12-03`,
  `PKG-12`, `SOW-037`, and `OBJ-010`.
- The policy states that telemetry is disabled by default, absent/unset/
  malformed telemetry configuration resolves to disabled, and MVP may remain a
  no-op with no telemetry transport.
- The policy states that telemetry cannot initialize network transport,
  background upload jobs, queues, persistence, endpoints, or vendors unless a
  user explicitly opts in and a human-approved event allowlist exists.
- The policy prohibits telemetry payloads from including private project
  models, code-specific rule data, private rule packs, private material or
  component libraries, generated reports, model hashes, local file paths,
  secrets, credentials, protected standards content, or professional/code
  compliance claims.
- Tests under `tests/security/test_telemetry_policy.py` provide deterministic
  checks for default-off semantics, malformed/unknown config fail-closed
  behavior, event allowlist requirements, and forbidden payload fields without
  adding network behavior or product runtime dependencies.
- The implementation keeps product config schema, consent UI, endpoint,
  vendor, retention policy, and concrete event schema as `TBD`.
- Deliverable `MEMORY.md` records the work, source basis, verification, and
  remaining open decisions.
- Verification includes `git diff --check` and focused scans for prohibited
  protected-content, private-data, real-secret, and certification/compliance
  claim patterns.
- No lifecycle transition, dependency-register edit, candidate-edge change,
  blocker-queue refresh, implementation-evidence registration, protected
  standards data, proprietary engineering value, real private data, real
  secret, telemetry vendor integration, network behavior, or professional/
  security/code-compliance claim occurs unless separately authorized.

## Dispatch Task Shape

```markdown
PURPOSE: Implement the sealed OpenPipeStress deliverable within explicit scope.
RequestedBy: WORKING_ITEMS

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design
DeliverablePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design
TaskProfile: security-privacy

DeliverableID: DEL-12-03
PackageID: PKG-12

Tasks:
  - Implement only the artifacts authorized for DEL-12-03.
  - Create docs/security/telemetry_policy.md as the public telemetry policy.
  - Add deterministic policy tests under tests/security/test_telemetry_policy.py
    without adding runtime networking behavior or telemetry dependencies.
  - Preserve all applicable contract invariants, architecture-basis
    constraints, and the DEL-12-05 threat-model posture.
  - Keep config schema, consent UI, endpoint, vendor, retention policy, and
    concrete event schema as TBD unless already approved by a human ruling.
  - Do not implement telemetry transport, add a telemetry vendor, edit schemas,
    change plugin/runtime behavior, promote candidate edges, or introduce
    protected/private content.

Return:
  - Files changed.
  - Tests, scans, and validation commands run.
  - Default-off telemetry behavior summary.
  - Remaining TBDs and blocked decisions.
```

## Current Stop Point

Stop after sealed dispatch brief preparation. Implementation requires a
separate human approval gate from this brief.
