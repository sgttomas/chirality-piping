# MEMORY: DEL-02-01 Canonical domain model schema

## Session 2026-04-30

Human ruling consumed:

- Accepted the completed `DEL-01-01` pilot pattern.
- Authorized exactly one next bounded DAG item: `DEL-02-01`.
- Explicitly prohibited broad fan-out.

Bounded write scope used:

- `schemas/model.schema.yaml`
- `docs/TYPES.md`
- `tests/test_model_schema.py`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-02-01.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`
- `execution/_Coordination/_COORDINATION.md`
- this `MEMORY.md`

Work notes:

- Preserved JSON Schema 2020-12 and strict JSON syntax in `schemas/model.schema.yaml`.
- Tightened canonical schema requirements for model collections, unit-bearing quantities, diagnostics, hash payload scope, result/report evidence, rule-pack references, support/load provenance, and common object coverage.
- Updated `docs/TYPES.md` Section 8 with schema syntax and common-record vocabulary.
- Added focused stdlib checks in `tests/test_model_schema.py`.

Guardrails preserved:

- No lifecycle state transition.
- No `Dependencies.csv`, `_DEPENDENCIES.md`, `DAG-001`, candidate-edge, or blocker-queue edits.
- No protected standards text, protected tables, proprietary values, private data, or automatic compliance/certification/sealing claims introduced.

Open items:

- `C-02-01-001` objective mapping remains unresolved: `DEL-02-01` owns `OBJ-001`; `SOW-041` also maps `OBJ-012` in the scope ledger.
- `C-02-01-002` stale metadata pointer remains unresolved in local references.
- Physical project container and migration framework remain owned by persistence work unless later human authority changes that boundary.

## Session 2026-05-03 - Revision 0.5 Supplement

Human rulings consumed:

- `DEL-02-01` supplemental revision `0.5` schema/context work is required
  before `DAG-002` graph approval.
- Execute the supplement now as bounded `DEL-02-01` work.

Bounded write scope used:

- `schemas/model.schema.yaml`
- `tests/test_model_schema.py`
- `docs/TYPES.md`
- `docs/SPEC.md`
- this deliverable `_CONTEXT.md`
- this `MEMORY.md`
- coordination/DAG approval-review state needed to close the control loop

Work notes:

- Refreshed `_CONTEXT.md` to revision `0.5`, adding `SOW-065`, `OBJ-012`, and
  `OBJ-014`.
- Extended the canonical `Model` schema with a `model_role`, unresolved
  assumptions, traceability links, and typed references to design knowledge,
  constraints, equipment interfaces, operations, states/runs, comparisons,
  handoff packages, and external-prover metadata.
- Added `Assumption`, `ModelRole`, and `TraceabilityLink` definitions while
  leaving detailed PKG-13 through PKG-16 contracts to their owning deliverables.
- Updated `docs/TYPES.md`, `docs/SPEC.md`, and schema tests for the revision
  `0.5` vocabulary.

Guardrails preserved:

- No lifecycle state transition.
- No `Dependencies.csv`, `_DEPENDENCIES.md`, blocker queue, or graph approval
  record edits.
- No Type 2 dispatch, `PREPARATION` scaffold, or Chirality corpus promotion.
- No protected standards text, protected tables, proprietary values, private
  data, public code-specific defaults, automatic compliance/certification/
  sealing claims, or software-generated professional approval claims
  introduced.

Open items:

- Downstream PKG-13 through PKG-16 deliverables still own detailed schemas and
  services for design knowledge, constraints, states/runs, comparisons,
  handoff, external-prover metadata, and structured model operations.
- Physical project container and migration implementation details remain owned
  by persistence work unless later human authority changes that boundary.
