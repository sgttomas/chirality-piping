# MEMORY - DEL-02-04 Plugin and extension domain contracts

## Session 2026-05-01

Human project authority authorized one bounded DAG item of ORCHESTRATOR's
choosing. ORCHESTRATOR selected `DEL-02-04` to complete the remaining PKG-02
extension/API boundary foundation after `DEL-02-01`, `DEL-02-02`,
`DEL-02-03`, and `DEL-02-05`.

### Scope Used

- Sealed dispatch brief:
  `execution/_Coordination/DEV-001_DISPATCH_DEL-02-04.md`.
- Active upstream dependencies from approved `DAG-001` only.
- Candidate edges excluded.
- No lifecycle transition, dependency-register edit, or blocker queue refresh.

### Decisions And Constraints

- Plugin manifests are denied by default; requests are not runtime grants.
- Entrypoints must declare both extension point and domain surface.
- Domain/API compatibility must carry the DEL-02-04 domain contract,
  `api/api_boundary_contract.yaml`, JSON Schema 2020-12 basis, and host API
  status.
- No-bypass controls now explicitly cover analysis-boundary state,
  persistence, and external human-acceptance boundaries.
- Privacy posture must preserve local-first behavior, telemetry off by default,
  export permission checks, and redaction support.
- Sandbox posture must declare capabilities and deny arbitrary code execution,
  filesystem access, network access, and process spawning by default.

### Remaining TBDs

- Runtime plugin loader and isolation technology.
- Permission grant storage, consent UX, revocation, signing, and update
  mechanism.
- Public API transport and concrete external import/export formats.
- Rule expression grammar and sandbox implementation details.
- Canonicalization edge cases for non-JSON payloads.
- CI gates for plugin submission, protected-content screening, and
  security/privacy review.
