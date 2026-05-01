# MEMORY - DEL-02-05 Project Persistence and Round-Trip Serialization

## Session 2026-05-01

Human project authority authorized one bounded DAG item of ORCHESTRATOR's
choosing. ORCHESTRATOR selected `DEL-02-05 - Project persistence and
round-trip serialization` because `DEL-02-01`, `DEL-02-02`, and `DEL-02-03`
are completed predecessors and persistence anchors downstream storage, hashes,
reports, CLI/headless runs, GUI editing, and private-data handling.

Fresh dispatch brief:

- `execution/_Coordination/DEV-001_DISPATCH_DEL-02-05.md`

Approved write scope:

- `schemas/project_persistence.schema.yaml`
- `docs/architecture/persistence_contract.md`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `tests/test_persistence_schema.py`
- this `MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-02-05.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

Implemented/tightened contract:

- Added explicit persistence validation profile coverage.
- Added application-service `PersistenceOperation` records for create, open,
  save, validate, version-check, and migrate flows.
- Added checksum payload-scope and payload-partition status fields.
- Added external hash-bound `HumanAcceptanceRef` records that invalidate on
  bound-hash changes and do not imply software approval or code compliance.
- Added round-trip normalization rules that prohibit silent engineering
  defaults.
- Preserved the physical project package/container, migration framework,
  canonical JSON library choice, and non-JSON/binary hash partition as `TBD`.

Guardrails:

- No lifecycle state transition was made.
- No blocker queue refresh was run.
- No `DAG-001`, candidate-edge, `Dependencies.csv`, or `_DEPENDENCIES.md`
  mutation occurred.
- No protected standards text, protected tables, proprietary engineering
  values, private data, or automatic compliance/certification/sealing claims
  were introduced.
