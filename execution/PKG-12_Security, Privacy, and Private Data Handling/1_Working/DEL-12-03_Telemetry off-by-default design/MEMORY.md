# Memory: DEL-12-03 Telemetry off-by-default design

## Current Session

2026-05-02 - Implemented from sealed dispatch brief
`execution/_Coordination/DEV-001_DISPATCH_DEL-12-03.md`.

## Decisions And Rulings

- Human project authority authorized implementation from the sealed brief only
  after `DEL-12-05` closeout and `DEL-12-03` brief preparation were committed.
- Implementation stayed within the approved write scope:
  `docs/security/telemetry_policy.md`,
  `tests/security/test_telemetry_policy.py`, this `MEMORY.md`, the dispatch
  brief, and `NEXT_INSTANCE_STATE.md`.
- No lifecycle transition, implementation-evidence registration,
  dependency-register edit, blocker-queue refresh, `DAG-001` change,
  candidate-edge promotion, runtime telemetry code, vendor integration,
  endpoint selection, product config schema edit, plugin/runtime behavior
  change, or network behavior was performed.

## Source Basis

- `execution/_Coordination/DEV-001_DISPATCH_DEL-12-03.md`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design/_CONTEXT.md`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design/Specification.md`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design/Guidance.md`
- `docs/CONTRACT.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `docs/security/threat_model.md`

## Implementation Notes

- Added `docs/security/telemetry_policy.md` as the public telemetry policy.
- Defined default-off behavior for absent, unset, empty, unknown, unsupported,
  and malformed telemetry configuration.
- Recorded fail-closed behavior: telemetry remains disabled unless the user
  explicitly opts in through an approved surface and a human-approved event
  allowlist exists.
- Prohibited telemetry initialization of network transport, upload jobs,
  queues, persistence, endpoints, vendors, and external service clients without
  opt-in and allowlist evidence.
- Prohibited telemetry payloads from carrying private project models,
  code-specific rule data, private rule packs, private material/component
  libraries, generated reports, model hashes, local file paths, secrets,
  credentials, protected standards content, or professional/code-compliance
  claims.
- Kept product config schema, consent UI/CLI, endpoint, vendor, transport,
  retention policy, concrete event schema, and support-bundle workflow as
  `TBD`.

## Verification

- `python3 -m pytest tests/security/test_telemetry_policy.py` passed.
- `git diff --check` passed.
- Focused protected-content/prohibited-claim/real-secret scan found only
  guardrail and exclusion wording in the telemetry policy, tests, dispatch
  brief, memory, and state.

## Remaining TBDs

- Product configuration schema and storage location.
- Consent UI or CLI surface.
- Endpoint, vendor, transport, and retention policy.
- Concrete event schema and event allowlist.
- Explicit user-selected support bundle or payload workflow.
- Lifecycle/evidence/local dependency-register alignment and blocker-queue
  refresh for `DEL-12-03`, if later authorized.
