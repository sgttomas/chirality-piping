# DEV-001 Dispatch - DEL-06-03 Required-input Completeness Checker

**Dispatch status:** approved by human project authority on 2026-05-02
**Coordination mode:** `FULL_GRAPH`
**Graph authority:** `execution/_DAG/DAG-001/DependencyEdges.csv`
**Implementation threshold:** upstream `COMMITTED` evidence

## Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-06-03` |
| PackageID | `PKG-06` |
| Name | Required-input completeness checker |
| Type | `BACKEND_FEATURE_SLICE` |
| Register objective | Implement rule-pack completeness checks that block code-check status when required user-supplied inputs are missing. |
| Anticipated artifacts | rule completeness checker; tests |

## Authorized Write Scope

- `core/rules/completeness_checker/`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-03_Required-input completeness checker/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-06-03.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md` for session handoff

Lifecycle, dependency-register, implementation-evidence, blocker-queue, staging,
and commit changes require explicit follow-on CHANGE/human approval unless
separately authorized.

## Active Upstream Dependencies

| EdgeID | Upstream | Evidence |
|---|---|---|
| `DAG-001-E0169` - `DAG-001-E0175` | Applicable `PKG-00` architecture basis | Satisfied by accepted architecture baseline |
| `DAG-001-E0469` | `DEL-06-01` Rule-pack schema | `COMMITTED` |
| `DAG-001-E0470` | `DEL-02-03` Code-neutral analysis boundary model | `COMMITTED` |
| `DAG-001-E0471` | `DEL-05-04` Analysis status semantics | `COMMITTED` |

## Applicable Invariants

- `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`
- `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`
- `OPS-K-AUTH-1`, `OPS-K-MECH-2`, `OPS-K-UNIT-1`
- `OPS-K-RULE-2`, `OPS-K-RULE-3`
- `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`

## Architecture Basis

Applicable basis IDs from `SCA-001`: `AB-00-01`, `AB-00-02`, `AB-00-03`,
`AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.

Resolved baseline: Rust core/application services, JSON Schema 2020-12
contracts, schema-first command/query/job result envelopes, canonical
JSON/JCS-compatible hash basis where JSON payloads are hashed, and layered
Cargo/validation/protected-content test gates as applicable.

## Acceptance Criteria

- Implement a bounded Rust completeness-checker module from declarative
  required-input declarations and supplied input evidence.
- Missing rule-check-required values produce explicit blocking findings and
  `RULE_INPUTS_INCOMPLETE` readiness semantics.
- Provenance, redistribution, protected-content suspicion, review status,
  unit, and dimension gaps are explicit findings.
- Mechanics solve status remains distinct from rule-check readiness.
- No protected standards/code data, proprietary engineering values, public
  code-specific defaults, private payloads, or professional/code-compliance
  claims are introduced.
- Focused Cargo tests pass.

## Exclusions

- No formula evaluation or expression parsing.
- No rule-pack JSON parsing/canonicalization.
- No private storage, encryption, access control, or secret handling.
- No GUI, report, API, CLI, or final result-envelope integration.
- No lifecycle transition, evidence update, blocker queue refresh, staging, or
  commit without explicit follow-on approval.
