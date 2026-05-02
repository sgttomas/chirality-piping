---
doc_id: DEV-001-DISPATCH-DEL-08-01
doc_kind: coordination.dispatch_brief
status: sealed_brief_prepared
created: 2026-05-02
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-08-01
package_id: PKG-08
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-08-01 Calculation Report Generator

**Dispatch status:** implemented in working tree on 2026-05-02 after separate
human approval from the sealed brief.
**Coordination mode:** `FULL_GRAPH`
**Graph authority:** `execution/_DAG/DAG-001/DependencyEdges.csv`
**Implementation threshold:** upstream `COMMITTED` evidence

## Dispatch Decision

The human project authority authorized one bounded ORCHESTRATOR action:

- prepare `execution/_Coordination/DEV-001_DISPATCH_DEL-08-01.md`
  for `DEL-08-01 - Calculation report generator`.

This authorization prepares the implementation brief only. It does not
authorize implementation, lifecycle transition, implementation-evidence
registration, dependency-register edits, blocker-queue refresh, `DAG-001`
changes, candidate-edge promotion, staging, commit, or broad DAG execution.

The human project authority later authorized implementation from this sealed
brief with the instruction "proceed with implementation". That implementation
authorization does not authorize lifecycle transition, implementation-evidence
registration, dependency-register edits, blocker-queue refresh, `DAG-001`
changes, candidate-edge promotion, staging, commit, or broad DAG execution.

The eventual implementation scope should be constrained to a deterministic
calculation-report generation contract and bounded renderer/template support
surface that assembles already-governed report inputs. It may define a strict
report-generation schema/fixture contract, add a bounded report generator
support module, update focused documentation, and add deterministic tests. It
must not implement the protected-content linter, private-data redaction/export
controls, GUI presentation, CLI runtime, API transport, adapter behavior,
local FEA handoff packaging, protected standards content, private engineering
data, real secrets, or professional/code-compliance claims.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-08-01` |
| PackageID | `PKG-08` |
| Name | Calculation report generator |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope items | `SOW-024` |
| Objectives | `OBJ-007` |
| Context envelope | `L` |
| Deliverable path | `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| EdgeID | Upstream | Dependency type | Implementation-readiness satisfaction |
|---|---|---|---|
| `DAG-001-E0239` - `DAG-001-E0245` | Applicable `PKG-00` architecture basis | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DAG-001-E0522` | `DEL-02-05` Project persistence and round-trip serialization | `PERSISTENCE_CONTRACT` | `COMMITTED` evidence `742016e` |
| `DAG-001-E0523` | `DEL-05-03` Fundamental stress recovery module | `LOAD_STRESS_PREDECESSOR` | `COMMITTED` evidence `26dc805` |
| `DAG-001-E0524` | `DEL-05-04` Analysis status semantics | `DOMAIN_MODEL` | `COMMITTED` evidence `dbaf21e` |
| `DAG-001-E0525` | `DEL-06-04` Private rule-pack lifecycle and checksum handling | `RULE_PACK_PREDECESSOR` | `COMMITTED` evidence `ad270f6` |
| `DAG-001-E0526` | `DEL-08-02` Audit manifest and model hash | `REPORTING_PREDECESSOR` | `COMMITTED` evidence `061f1af` |
| `DAG-001-E0527` | `DEL-08-03` Warnings, assumptions, and provenance report section | `REPORTING_PREDECESSOR` | `COMMITTED` evidence `50f947a` |
| `DAG-001-E0528` | `DEL-01-04` Professional responsibility and product-claims policy | `GOVERNANCE_PREDECESSOR` | `COMMITTED` evidence `65f3119` |

Current implementation-readiness queue state:

- `DEL-08-01` is `UNBLOCKED`.
- `DEL-08-01` has `MISSING_EVIDENCE`; it is not yet recorded as implemented.
- Candidate edges are excluded.

Downstream impact if later implemented and committed:

- `DEL-08-01` currently blocks `DEL-08-05`, `DEL-11-01`, and `DEL-12-02` in
  the active implementation-readiness queue.
- No candidate edge currently targets or originates from `DEL-08-01`.

## Applicable Architecture Basis

Applicable basis IDs from `SCA-001`: `AB-00-01`, `AB-00-02`, `AB-00-03`,
`AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.

Resolved baseline to preserve: ADR traceability; layer/module responsibilities;
schema-first command/query/job/result-envelope boundaries; JSON Schema 2020-12
contracts; canonical JSON/JCS-compatible hash basis where JSON payloads are
hashed; diagnostics/result-envelope warning classes and fields; API/plugin/
adapter no-bypass boundaries; local-first private/public path controls; and
layered validation/protected-content/security gates where applicable.

The calculation report generator must preserve `DEL-02-05` persistence and
round-trip boundaries, `DEL-05-03` mechanics stress-result boundaries,
`DEL-05-04` analysis-status/professional-authority separation, `DEL-06-04`
rule-pack identity/version/checksum posture, `DEL-08-02` audit-manifest/model
hash posture, `DEL-08-03` warnings/assumptions/provenance section posture, and
`DEL-01-04` professional-boundary notices. Final GUI presentation, CLI command
shape, public API transport, adapter execution, private-data redaction/export
controls, protected-content lint implementation, release-template integration,
and final report styling/layout policy remain `TBD` unless separately approved
by the human project authority.

## Explicit Write Scope

The bounded implementation write scope, if separately authorized, is limited
to:

- `schemas/report_generator.schema.yaml`
- `core/reporting/report_generator/`
- `fixtures/reports/invented/`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `tests/test_report_generator_contract.py`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-08-01.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch. In particular, do not edit GUI
code, CLI runtime code, API transport bindings, adapter implementations, local
FEA handoff artifacts, protected-content linter code, redaction/export control
code, package manifests, CI workflows, release scripts, `DAG-001`, candidate
edges, deliverable-local `Dependencies.csv`, `DEV-001_IMPLEMENTATION_EVIDENCE.csv`,
  or `DEV-001_BLOCKER_QUEUE.*` during implementation unless separately
authorized.

## Applicable Invariants

- `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`
- `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`
- `OPS-K-AUTH-1`, `OPS-K-AUTH-2`
- `OPS-K-MECH-2`
- `OPS-K-UNIT-1`
- `OPS-K-RULE-1`, `OPS-K-RULE-3`
- `OPS-K-REPORT-1`, `OPS-K-REPORT-2`
- `OPS-K-PRIV-1`, `OPS-K-PRIV-2`
- `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`, `OPS-K-AGENT-4`

## Acceptance Criteria

This bounded item is acceptable for implementation closeout when:

- `schemas/report_generator.schema.yaml` exists, remains strict JSON syntax
  parseable by Python `json`, and is traceable to `DEL-08-01`, `PKG-08`,
  `SOW-024`, and `OBJ-007`.
- The contract defines a calculation-report envelope that can assemble model
  input summary, load cases/combinations, mechanics results, stress results,
  diagnostics, warnings, assumptions, provenance notes, audit-manifest/hash
  references, rule-pack references, limitations, and professional-boundary
  notices.
- Report payloads reference persisted project/model inputs, result-export
  envelopes, audit manifests, report-section records, and rule-pack lifecycle
  records by stable identifiers, checksums, source notes, privacy class, and
  review state where applicable.
- The renderer/support module, if implemented, provides bounded in-memory
  report assembly, deterministic ordering, template-slot validation, and
  export to a neutral text/structured representation suitable for tests. It
  must not read arbitrary project files, invoke solver internals, run GUI/CLI/
  API/adapter workflows, implement protected-content linting, implement
  redaction/export controls, choose filesystem roots, or access network/process
  capabilities.
- Report templates and invented fixtures are original project artifacts and do
  not copy protected standards text, protected tables, protected examples,
  proprietary formulas, proprietary engineering values, private project data,
  private rule-pack payloads, private library content, or real secrets.
- Report output preserves `DEL-05-04` status semantics: mechanics solved,
  rule-input incomplete, user-rule checked, user-rule failed, human review
  required, and human acceptance reference states remain distinct. Software
  must not emit automatic `CODE_COMPLIANT`, certification, sealing, approval,
  authentication, endorsement, or professional-reliance statuses.
- Rule-pack references carry identity, version, checksum, source note,
  redistribution/private-public status, and completeness/status information
  without copying private formulas, protected standards text, protected tables,
  proprietary engineering values, or private rule-pack payloads into public
  artifacts.
- Numeric report values include explicit unit and dimensional metadata or an
  explicit diagnostic/finding; missing solve-required or rule-check-required
  values surface as findings and are never silently defaulted.
- Deterministic tests under `tests/test_report_generator_contract.py` cover
  JSON parseability, metadata traceability, required report sections,
  deterministic section ordering, template-slot validation, audit/hash
  references, report-section integration, result-export integration,
  rule-pack reference shape, unit/dimension requirements, professional-boundary
  exclusions, privacy/provenance fields, and unresolved `TBD` runtime/redaction
  decisions.
- Deliverable `MEMORY.md` records the work, source basis, verification, and
  remaining open decisions.
- Verification includes `python3 tests/test_report_generator_contract.py`,
  `python3 tests/test_report_sections_contract.py`,
  `python3 tests/test_results_schema.py`,
  `python3 tests/test_analysis_status_schema.py`,
  `python3 tests/test_persistence_schema.py`,
  `python3 tests/test_rule_pack_schema.py`, `git diff --check`, and focused
  scans for protected standards content, private data, real secrets, runtime
  redaction/linter commitments, and certification/compliance claim patterns.
- No lifecycle transition, dependency-register edit, candidate-edge change,
  blocker-queue refresh, implementation-evidence registration, protected
  standards data, proprietary engineering value, real private data, real
  secret, GUI/report-preview/CLI/API/adapter runtime behavior, redaction/export
  control implementation, protected-content linter implementation, or
  professional/security/code-compliance claim occurs unless separately
  authorized.

## Implementation Summary

Implemented within the sealed write scope:

- Added `schemas/report_generator.schema.yaml` as the strict-JSON JSON Schema
  2020-12 contract for calculation-report assembly.
- Added `core/reporting/report_generator/` as a bounded Rust support crate for
  in-memory report validation, template-slot validation, and deterministic
  neutral section ordering.
- Added invented report fixture data under `fixtures/reports/invented/`.
- Added deterministic schema/fixture checks under
  `tests/test_report_generator_contract.py`.
- Updated focused `docs/SPEC.md` and `docs/TYPES.md` sections only for the
  report-generator boundary.
- Added deliverable `MEMORY.md`.

Verification completed:

- `python3 tests/test_report_generator_contract.py` passed.
- `cargo fmt --manifest-path core/reporting/report_generator/Cargo.toml -- --check`
  passed.
- `cargo test --manifest-path core/reporting/report_generator/Cargo.toml` passed
  with 5 unit tests.
- `python3 tests/test_report_sections_contract.py` passed.
- `python3 tests/test_results_schema.py` passed.
- `python3 tests/test_analysis_status_schema.py` passed.
- `python3 tests/test_persistence_schema.py` passed.
- `python3 tests/test_rule_pack_schema.py` passed.
- `git diff --check` passed.

Focused protected-content/private-secret/prohibited-claim scan was reviewed;
matches were guardrail/prohibition terms in docs, tests, schema, memory, and
this dispatch brief, not protected data, private secrets, concrete runtime
redaction/linter commitments, or positive compliance/professional/security
claims.

No lifecycle transition, dependency-register edit, candidate-edge change,
blocker-queue refresh, implementation-evidence registration, `DAG-001` change,
GUI/report-preview/CLI/API/adapter runtime behavior, private redaction/export
control implementation, protected-content linter implementation, protected
standards data, private data, real secret, or professional/security/
code-compliance claim occurred during implementation.

## Dispatch Task Shape

```markdown
PURPOSE: Implement the sealed OpenPipeStress deliverable within explicit scope.
RequestedBy: WORKING_ITEMS

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator
DeliverablePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator
TaskProfile: report-audit

DeliverableID: DEL-08-01
PackageID: PKG-08

Tasks:
  - Implement only the artifacts authorized for DEL-08-01.
  - Define schemas/report_generator.schema.yaml as the strict-JSON JSON Schema
    2020-12 calculation-report contract.
  - Add a bounded core/reporting/report_generator support module only if
    needed for deterministic report assembly and template-slot validation.
  - Add invented report fixtures under fixtures/reports/invented only if
    needed for tests; fixtures must be original, non-code, and free of
    protected/private data.
  - Add deterministic schema/contract checks under
    tests/test_report_generator_contract.py.
  - Preserve all applicable contract invariants, architecture-basis
    constraints, and upstream DEL-02-05, DEL-05-03, DEL-05-04, DEL-06-04,
    DEL-08-02, DEL-08-03, and DEL-01-04 contracts.
  - Keep GUI presentation, CLI command syntax, public API transport,
    adapter execution, local FEA packaging, protected-content linting,
    redaction/export controls, release-template integration, and final
    report styling/layout policy as TBD unless already approved by a human
    ruling.
  - Do not implement GUI, CLI, API transport, adapter, local FEA handoff,
    protected-content linter, redaction/export control, packaging,
    dependency graph, or queue/evidence changes.

RuntimeOverrides:
  DecompositionPath: docs/_Decomposition/SOFTWARE_DECOMP.md
  DecompositionRevision: "0.4"
  DAGPath: execution/_DAG/DAG-001/DependencyEdges.csv
  DAGApprovalRecord: execution/_DAG/DAG-001/APPROVAL_RECORD.md
  CoordinationMode: FULL_GRAPH
  BlockerComputation: ENABLED_ACTIVE_EDGES_ONLY
  ScopeChangeID: SCA-001

CustomInstructions:
  - Read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`,
    existing production artifacts, and this dispatch brief before editing.
  - Apply only applicable `AB-00-*` constraints; do not copy full `PKG-00`
    prose into deliverable artifacts.
  - Treat protected standards/code data as out of scope.
  - Unknown engineering values remain `TBD`.
  - Do not claim certification, approval, sealing, authentication, or code
    compliance for reliance.
  - Do not edit files outside this sealed write scope unless explicitly
    authorized by a later human gate.
  - Do not recompute or mutate blocker queues unless explicitly assigned.

AllowedWriteTargets:
  - schemas/report_generator.schema.yaml
  - core/reporting/report_generator/
  - fixtures/reports/invented/
  - docs/SPEC.md
  - docs/TYPES.md
  - tests/test_report_generator_contract.py
  - execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/MEMORY.md
  - execution/_Coordination/DEV-001_DISPATCH_DEL-08-01.md
  - execution/_Coordination/NEXT_INSTANCE_STATE.md

ExpectedOutputs:
  - Calculation report schema and/or bounded report generator support artifacts
    listed in this sealed brief.
  - Tests or validation evidence appropriate to the deliverable.
  - Deliverable MEMORY.md with source basis, verification, and open issues.
  - Open issue list for unresolved `TBD`, assumptions, or downstream
    integration dependencies.

EXCLUSIONS:
  - No protected standards text, tables, examples, copied code formulas, or
    proprietary engineering values.
  - No private project, private rule-pack, private library, or real secret
    content.
  - No edits outside the sealed write scope.
  - No lifecycle state transition unless explicitly authorized by the human.
  - No local dependency-register edits unless explicitly assigned.
  - No implementation-evidence or blocker-queue updates unless separately
    authorized after implementation.
  - No candidate-edge promotion.
```

## Implementation Closeout

Implementation changed only files inside the sealed implementation write scope:

- `schemas/report_generator.schema.yaml`
- `core/reporting/report_generator/`
- `fixtures/reports/invented/`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `tests/test_report_generator_contract.py`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-08-01.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No lifecycle transition, implementation-evidence registration,
dependency-register edit, blocker-queue refresh, `DAG-001` change,
candidate-edge promotion, staging, commit, protected standards data, private
data, real secret, GUI/CLI/API/adapter runtime behavior, redaction/export
control implementation, protected-content linter implementation, or
professional/code-compliance claim occurred during implementation.

Lifecycle/evidence/queue closeout was later authorized by the human project
authority with the instruction "perform lifecycle transition,
dependency-register edits, evidence registration, blocker queue refresh,
staging, and commit".

Closeout actions performed before implementation/closeout commit:

- Set `DEL-08-01` lifecycle display state to `CHECKING`.
- Annotated local `Dependencies.csv` rows `DAG-001-E0522` through
  `DAG-001-E0528` as `SATISFIED` from committed upstream evidence.
- Added `DEL-08-01` to `DEV-001_IMPLEMENTATION_EVIDENCE.csv` as
  `WORKING_TREE` evidence before commit.
- Rebuilt `DEV-001_BLOCKER_QUEUE.*`; queue remained 65 unblocked / 8 blocked
  because `DEL-08-01` evidence was not yet commit-backed.

No aggregate `DAG-001` change, candidate-edge promotion, protected standards
data, private data, real secret, GUI/CLI/API/adapter runtime behavior,
redaction/export control implementation, protected-content linter
implementation, or professional/code-compliance claim occurred during closeout.
