---
doc_id: DEV-001-DISPATCH-DEL-08-05
doc_kind: coordination.dispatch_brief
status: sealed_brief_prepared
created: 2026-05-02
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-08-05
package_id: PKG-08
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
reconciliation_basis: execution/_Reconciliation/Reconciliation_Run_Summary_2026-05-02_DEL0805_CANDIDATE_E0621.md
---

# DEV-001 Dispatch - DEL-08-05 Report Protected-Content Linter

**Dispatch status:** implemented in working tree on 2026-05-02 after separate
human approval from the sealed brief.
**Coordination mode:** `FULL_GRAPH`
**Graph authority:** `execution/_DAG/DAG-001/DependencyEdges.csv`
**Implementation threshold:** upstream `COMMITTED` evidence

## Dispatch Decision

The human project authority authorized one bounded ORCHESTRATOR action:

- work on `DEL-08-05 - Report protected-content linter` next, with a small
  `RECONCILIATION` pass on the `DEL-08-05` / `DEL-11-04` candidate edge before
  dispatch.

The reconciliation pass recorded
`execution/_Reconciliation/Reconciliation_Run_Summary_2026-05-02_DEL0805_CANDIDATE_E0621.md`.
It keeps `DAG-001-E0621` as `CANDIDATE` and non-gating. `DEL-08-05` may proceed
without `DEL-11-04` by using invented/synthetic linter fixtures. The active
edge `DAG-001-E0593` remains the governing production order: later public
invented educational example models consume the committed linter.

This authorization prepares the implementation brief only. It does not
authorize implementation, lifecycle transition, implementation-evidence
registration, dependency-register edits, blocker-queue refresh, `DAG-001`
changes, candidate-edge promotion, staging, commit, or broad DAG execution.

The eventual implementation scope should be constrained to a deterministic
protected-content linter contract and bounded scanning support for authorized
public report/template/example surfaces. It may define a strict linter finding
schema, add a bounded linter support module, add invented/synthetic fixtures,
update focused documentation, and add deterministic tests. It must not copy
protected standards content, use protected examples as fixtures, scan private
user data by default, implement broad CI/release workflows, implement
redaction/export controls, edit educational example models, or make legal,
professional, security, certification, or code-compliance sufficiency claims.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-08-05` |
| PackageID | `PKG-08` |
| Name | Report protected-content linter |
| Type | `TEST_SUITE` |
| Scope items | `SOW-043` |
| Objectives | `OBJ-002`, `OBJ-007` |
| Context envelope | `M` |
| Context notes | Heuristic plus review; cannot be sole legal control. |
| Deliverable path | `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-05_Report protected-content linter` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| EdgeID | Upstream | Dependency type | Implementation-readiness satisfaction |
|---|---|---|---|
| `DAG-001-E0267` - `DAG-001-E0273` | Applicable `PKG-00` architecture basis | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DAG-001-E0529` | `DEL-08-01` Calculation report generator | `REPORTING_PREDECESSOR` | `COMMITTED` evidence `9e21716` |
| `DAG-001-E0530` | `DEL-01-02` Copyright and protected-data boundary policy | `GOVERNANCE_PREDECESSOR` | `COMMITTED` evidence `0d729cf` |
| `DAG-001-E0531` | `DEL-01-04` Professional responsibility and product-claims policy | `GOVERNANCE_PREDECESSOR` | `COMMITTED` evidence `65f3119` |

Current implementation-readiness queue state:

- `DEL-08-05` is `UNBLOCKED`.
- `DEL-08-05` has `MISSING_EVIDENCE`; it is not yet recorded as implemented.
- Candidate edges are excluded.

Downstream impact if later implemented and committed:

- `DEL-08-05` currently blocks `DEL-09-05`, `DEL-10-04`, and `DEL-11-04` in
  the active implementation-readiness queue.
- `DAG-001-E0621` remains a retained non-gating candidate caveat and must not
  be promoted by this dispatch.

## Candidate Edge Reconciliation

`DAG-001-E0621` states that `DEL-08-05` may need `DEL-11-04` as a predecessor
because the protected-content linter may need educational example fixtures. The
small reconciliation pass keeps this edge non-gating for this dispatch.

Implementation must satisfy linter fixture needs with invented/synthetic
markers and safe negative fixtures created specifically for linter tests. It
must not depend on future `DEL-11-04` educational example models, and it must
not create or edit `examples/models/invented/*` or tutorials owned by
`DEL-11-04`.

If implementation later proves actual educational example models are required
to finish the linter, stop and route that ambiguity back to `RECONCILIATION`.

## Applicable Architecture Basis

Applicable basis IDs from `SCA-001`: `AB-00-01`, `AB-00-02`, `AB-00-03`,
`AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.

Resolved baseline to preserve: ADR traceability; layer/module responsibilities;
schema-first command/query/job/result-envelope boundaries; JSON Schema 2020-12
contracts; canonical JSON/JCS-compatible hash basis where JSON payloads are
hashed; diagnostics/result-envelope warning classes and fields; API/plugin/
adapter no-bypass boundaries; local-first private/public path controls; and
layered validation/protected-content/security gates where applicable.

The report protected-content linter must preserve `DEL-08-01` report generator
surface boundaries, `DEL-01-02` protected-data/quarantine policy, and
`DEL-01-04` professional-claims boundaries. CI provider, release pipeline,
public API transport, GUI/report preview, private-data redaction/export
controls, actual public educational examples, and final legal-review workflow
remain `TBD` unless separately approved by the human project authority.

## Explicit Write Scope

The bounded implementation write scope, if separately authorized, is limited
to:

- `schemas/report_protected_content_linter.schema.yaml`
- `core/reporting/protected_content_linter/`
- `fixtures/report_lint/invented/`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `tests/test_report_protected_content_linter.py`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-05_Report protected-content linter/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-08-05.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch. In particular, do not edit GUI
code, CLI runtime code, API transport bindings, adapter implementations, local
FEA handoff artifacts, private-data redaction/export controls, release/CI
workflow files, `examples/models/invented/*`, tutorials, `DAG-001`, candidate
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
- `OPS-K-GOV-4`
- `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`, `OPS-K-AGENT-4`

## Acceptance Criteria

This bounded item is acceptable for implementation closeout when:

- `schemas/report_protected_content_linter.schema.yaml` exists, remains strict
  JSON syntax parseable by Python `json`, and is traceable to `DEL-08-05`,
  `PKG-08`, `SOW-043`, `OBJ-002`, and `OBJ-007`.
- The contract defines deterministic linter input configuration, public-surface
  path targeting, finding codes/classes/severities, source locations,
  remediation/review-routing fields, and explicit review status fields.
- The linter distinguishes at least protected-content risk, private-data risk,
  proprietary-source risk, prohibited professional/code-compliance claim risk,
  unknown redistribution/provenance risk, and safe metadata allowance.
- The linter treats clean heuristic output as review evidence only, not legal
  clearance, professional approval, certification, sealing, endorsement,
  authentication, code-compliance proof, or a security guarantee.
- The support module, if implemented, provides bounded local in-memory/file
  scanning over explicitly configured public report-template/example surfaces
  and deterministic finding generation. It must not scan user-private paths by
  default, access network/process capabilities, read arbitrary project stores,
  mutate source files, quarantine files itself, implement redaction/export
  controls, run GUI/CLI/API/adapter workflows, or choose CI/release policy.
- Fixtures are invented/synthetic linter fixtures created for `DEL-08-05` only.
  They must not copy protected standards text, protected tables, protected
  examples, proprietary formulas, material allowables, SIF/flexibility tables,
  protected dimensional tables, vendor catalogs, commercial software examples,
  private project data, private rule-pack payloads, private library content, or
  real secrets.
- Safe negative fixtures demonstrate permitted metadata such as rule-pack ID,
  version, checksum, source notice, redistribution status, review state,
  provenance summary, diagnostics, and professional-boundary notices without
  embedding private formulas, protected content, or proprietary values.
- Prohibited-claim checks flag public report language that claims the software
  certifies, seals, approves, authenticates, endorses, or declares engineering
  code compliance for reliance while permitting decision-support and
  human-review-needed wording.
- Deterministic tests under `tests/test_report_protected_content_linter.py`
  cover JSON parseability, metadata traceability, public-surface targeting,
  synthetic protected-risk findings, safe metadata allowance, prohibited-claim
  findings, unknown provenance behavior, deterministic output ordering,
  review/quarantine routing fields, and candidate-edge non-dependence on
  `DEL-11-04`.
- Focused documentation updates in `docs/SPEC.md` and `docs/TYPES.md` describe
  only the linter boundary and keep CI provider, release policy, redaction/
  export controls, GUI/CLI/API/adapter integration, and legal-review workflow
  as downstream or `TBD` unless already approved.
- Deliverable `MEMORY.md` records the work, source basis, verification, the
  `DAG-001-E0621` reconciliation disposition, and remaining open decisions.
- Verification includes `python3 tests/test_report_protected_content_linter.py`,
  relevant adjacent report/schema tests, `git diff --check`, and focused scans
  for protected standards content, private data, real secrets, prohibited
  professional/code-compliance/security/legal sufficiency claims, and accidental
  edits outside the sealed write scope.
- No lifecycle transition, dependency-register edit, candidate-edge change,
  blocker-queue refresh, implementation-evidence registration, `DAG-001`
  change, protected standards data, proprietary engineering value, real private
  data, real secret, GUI/CLI/API/adapter runtime behavior, redaction/export
  control implementation, CI/release workflow implementation, educational
  example model implementation, or professional/legal/code-compliance claim
  occurs unless separately authorized.

## Dispatch Task Shape

```markdown
PURPOSE: Implement the sealed OpenPipeStress deliverable within explicit scope.
RequestedBy: WORKING_ITEMS

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-05_Report protected-content linter
DeliverablePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-05_Report protected-content linter
TaskProfile: report-audit

DeliverableID: DEL-08-05
PackageID: PKG-08

Tasks:
  - Implement only the artifacts authorized for DEL-08-05.
  - Define schemas/report_protected_content_linter.schema.yaml as the
    strict-JSON JSON Schema 2020-12 protected-content linter contract.
  - Add a bounded core/reporting/protected_content_linter support module only
    if needed for deterministic public-surface scanning and finding generation.
  - Add invented/synthetic linter fixtures under fixtures/report_lint/invented
    only if needed for tests; fixtures must not copy protected or private
    content.
  - Add deterministic schema/contract checks under
    tests/test_report_protected_content_linter.py.
  - Preserve all applicable contract invariants, architecture-basis
    constraints, and upstream DEL-08-01, DEL-01-02, and DEL-01-04 contracts.
  - Treat DAG-001-E0621 as retained CANDIDATE/non-gating; do not depend on
    DEL-11-04 actual educational examples.
  - Keep CI provider/release workflow, redaction/export controls, GUI/CLI/API/
    adapter integration, educational example model publication, quarantine
    file movement, and final legal-review workflow as TBD unless already
    approved by a human ruling.
  - Do not implement GUI, CLI, API transport, adapter, local FEA handoff,
    private-data redaction/export control, release/CI workflow, educational
    example models, dependency graph, or queue/evidence changes.

RuntimeOverrides:
  DecompositionPath: docs/_Decomposition/SOFTWARE_DECOMP.md
  DecompositionRevision: "0.4"
  DAGPath: execution/_DAG/DAG-001/DependencyEdges.csv
  DAGApprovalRecord: execution/_DAG/DAG-001/APPROVAL_RECORD.md
  CoordinationMode: FULL_GRAPH
  BlockerComputation: ENABLED_ACTIVE_EDGES_ONLY
  ScopeChangeID: SCA-001
  ReconciliationBasis: execution/_Reconciliation/Reconciliation_Run_Summary_2026-05-02_DEL0805_CANDIDATE_E0621.md

CustomInstructions:
  - Read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`,
    existing production artifacts, the reconciliation summary, and this
    dispatch brief before editing.
  - Apply only applicable `AB-00-*` constraints; do not copy full `PKG-00`
    prose into deliverable artifacts.
  - Treat protected standards/code data and private user data as out of scope.
  - Unknown engineering, legal, CI, release, redaction, and security-sufficiency
    details remain `TBD`.
  - Do not claim legal clearance, security sufficiency, certification,
    approval, sealing, authentication, endorsement, or code compliance for
    reliance.
  - Do not edit files outside this sealed write scope unless explicitly
    authorized by a later human gate.
  - Do not recompute or mutate blocker queues unless explicitly assigned.

AllowedWriteTargets:
  - schemas/report_protected_content_linter.schema.yaml
  - core/reporting/protected_content_linter/
  - fixtures/report_lint/invented/
  - docs/SPEC.md
  - docs/TYPES.md
  - tests/test_report_protected_content_linter.py
  - execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-05_Report protected-content linter/MEMORY.md
  - execution/_Coordination/DEV-001_DISPATCH_DEL-08-05.md
  - execution/_Coordination/NEXT_INSTANCE_STATE.md

ExpectedOutputs:
  - Protected-content linter schema and/or bounded linter support artifacts
    listed in this sealed brief.
  - Tests or validation evidence appropriate to the deliverable.
  - Deliverable MEMORY.md with source basis, verification, reconciliation
    disposition, and open issues.
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

## Implementation Summary

Implemented within the sealed write scope:

- Added `schemas/report_protected_content_linter.schema.yaml` as the
  strict-JSON JSON Schema 2020-12 contract for deterministic protected-content
  linter configuration, public-surface targets, findings, source locations,
  review routes, dispositions, summaries, and provenance.
- Added `core/reporting/protected_content_linter/` as a bounded, dependency-free
  Rust support crate for caller-supplied public-surface text scanning and
  deterministic finding generation.
- Added invented/synthetic linter fixtures under `fixtures/report_lint/invented/`.
- Added deterministic schema/fixture checks under
  `tests/test_report_protected_content_linter.py`.
- Updated focused `docs/SPEC.md` and `docs/TYPES.md` sections only for the
  linter boundary.
- Added deliverable `MEMORY.md`.

Verification completed:

- `python3 tests/test_report_protected_content_linter.py` passed.
- `cargo fmt --manifest-path core/reporting/protected_content_linter/Cargo.toml -- --check`
  passed.
- `cargo test --manifest-path core/reporting/protected_content_linter/Cargo.toml`
  passed with 4 unit tests.
- `python3 tests/test_report_generator_contract.py` passed.
- `python3 tests/test_report_sections_contract.py` passed.
- `python3 tests/test_results_schema.py` passed.
- `python3 tests/test_analysis_status_schema.py` passed.
- `git diff --check` passed.
- Focused protected-content/private-secret/prohibited-claim scan was reviewed;
  matches were guardrail/prohibition terms in docs, schema, memory, dispatch,
  state, and synthetic linter pattern lists/tests, not protected data, private
  secrets, or positive legal/professional/security/code-compliance claims.

No lifecycle transition, dependency-register edit, candidate-edge change,
blocker-queue refresh, implementation-evidence registration, `DAG-001` change,
GUI/CLI/API/adapter runtime behavior, private redaction/export control
implementation, CI/release workflow implementation, educational example model
implementation, protected standards data, private data, real secret, legal
sufficiency claim, security sufficiency claim, or professional/code-compliance
claim occurred during implementation.

## Implementation Closeout

Implementation changed only files inside the sealed implementation write scope:

- `schemas/report_protected_content_linter.schema.yaml`
- `core/reporting/protected_content_linter/`
- `fixtures/report_lint/invented/`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `tests/test_report_protected_content_linter.py`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-05_Report protected-content linter/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-08-05.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No lifecycle transition, implementation-evidence registration,
dependency-register edit, blocker-queue refresh, `DAG-001` change,
candidate-edge promotion, staging, commit, redaction/export control
implementation, CI/release workflow implementation, or educational example
model implementation occurred during implementation.

Lifecycle/evidence/queue closeout remains unauthorized and requires a separate
human gate.

Lifecycle/evidence/queue closeout was later authorized by the human project
authority with the instruction "perform lifecycle transition,
dependency-register edits, evidence registration, blocker queue refresh,
staging, and commit".

Closeout actions performed before implementation/closeout commit:

- Set `DEL-08-05` lifecycle display state to `CHECKING`.
- Annotated local `Dependencies.csv` rows `DAG-001-E0529` through
  `DAG-001-E0531` as `SATISFIED` from committed upstream evidence.
- Added `DEL-08-05` to `DEV-001_IMPLEMENTATION_EVIDENCE.csv` as
  `WORKING_TREE` evidence before commit.
- Rebuilt `DEV-001_BLOCKER_QUEUE.*`; queue remained 67 unblocked / 6 blocked
  because `DEL-08-05` evidence was not yet commit-backed.
