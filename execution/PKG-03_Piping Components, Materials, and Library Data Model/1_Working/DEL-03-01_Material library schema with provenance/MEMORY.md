# MEMORY - DEL-03-01 Material Library Schema With Provenance

## 2026-05-01 Bounded Product-Development Session

Human gate:

- Human project authority authorized `DEL-03-01 - Material library schema with
  provenance` as the next bounded DAG item.

Dispatch:

- Fresh sealed dispatch brief:
  `execution/_Coordination/DEV-001_DISPATCH_DEL-03-01.md`.
- Active upstream dependencies consumed from approved `DAG-001`: `DEL-00-01`,
  `DEL-00-02`, `DEL-00-04`, `DEL-00-06`, `DEL-00-07`, `DEL-00-08`,
  `DEL-02-01`, `DEL-02-02`, `DEL-01-02`, and `DEL-01-03`.
- `CANDIDATE` rows were not promoted or used as gates.

Files changed:

- `schemas/material.schema.yaml`
- `fixtures/material/invented_material_library_valid.json`
- `tests/test_material_schema.py`
- `docs/SPEC.md`
- `docs/TYPES.md`
- this `MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-03-01.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

Decisions and boundaries:

- Material schema defines slots, provenance, redistribution, review status,
  completeness findings, and diagnostics.
- Public fixture uses invented non-engineering/schema-shape data and omits real
  material values.
- Material allowables are represented as governed slots only; public protected
  or code-specific allowable tables remain excluded.
- Accepted public material source catalog, public fixture value policy,
  temperature interpolation policy, and allowable storage details remain `TBD`.
- No lifecycle state transition, blocker queue refresh, `DAG-001` edit,
  candidate-edge change, or dependency-register edit was authorized.
