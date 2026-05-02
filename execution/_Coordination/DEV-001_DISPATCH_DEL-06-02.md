---
doc_id: DEV-001-DISPATCH-DEL-06-02
doc_kind: coordination.dispatch_brief
status: implemented_awaiting_lifecycle_evidence_gate
created: 2026-05-02
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-06-02
package_id: PKG-06
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-06-02

## Dispatch Decision

The human project authority authorized preparation of one sealed dispatch brief:

- `DEL-06-02 - Sandboxed unit-aware expression evaluator`

This authorization prepared the implementation brief only. It did not authorize
lifecycle transition, implementation-evidence mutation, dependency-register
edits, blocker-queue refresh, `DAG-001` changes, candidate-edge promotion, or
broad DAG execution.

The human project authority later authorized implementation from this sealed
brief. Implementation has been completed within the bounded write scope.
Lifecycle transition, implementation evidence registration, dependency-register
alignment, blocker-queue refresh, and commit remain separate gated actions.

The eventual implementation scope should be deliberately constrained to a
sandboxed, deterministic, unit-aware evaluator for declarative rule-pack
expressions: variable binding from declared rule-pack inputs and supplied result
fields, dimensional compatibility checks, deterministic findings for invalid or
unsafe expressions, and explicit preservation of analysis-status and
professional-boundary semantics. It does not authorize arbitrary executable
code, host-language evaluation, filesystem/network/process access, protected
standards text, copied standards formulas, bundled code allowables, public
rule-pack examples, private rule-pack lifecycle/checksum handling, completeness
checker behavior, GUI editor behavior, report generation, or professional/code
compliance claims.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-06-02` |
| PackageID | `PKG-06` |
| Name | Sandboxed unit-aware expression evaluator |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope items | `SOW-045` |
| Objectives | `OBJ-005` |
| Context envelope | `L` |
| Deliverable path | `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-02_Sandboxed unit-aware expression evaluator` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| Target | Dependency type | Implementation-readiness satisfaction |
|---|---|---|
| `DEL-00-01` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-02` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-03` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-04` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-06` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-07` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-08` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-06-01` | `RULE_PACK_PREDECESSOR` | `COMMITTED` evidence `20241f9` |
| `DEL-02-02` | `UNIT_CONTRACT` | `COMMITTED` evidence `a458cba` |

Current implementation-readiness queue state:

- `DEL-06-02` is `UNBLOCKED`.
- `DEL-06-02` has `MISSING_EVIDENCE` for its own implementation.
- Candidate edges are excluded.
- Candidate edge `DAG-001-E0623` (`DEL-06-02` may need `DEL-12-05`) remains
  non-gating pending later `RECONCILIATION`; it is not used for dispatch
  readiness, scheduling, implementation blockers, or write scope.
- `DEL-06-02` currently gates downstream consumer `DEL-06-05`.

## Applicable Architecture Basis

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module responsibilities and no-bypass dependency rules.
- `AB-00-03` - schema-first command/query/job/result-envelope service
  boundaries, including distinct mechanics-solved, user-rule-checked, and
  human-approved states.
- `AB-00-04` - JSON Schema 2020-12, schema versioning, canonical JSON, and
  JCS-compatible hash basis for JSON payloads.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no
  certification/compliance claims.
- `AB-00-07` - internal/public API and adapter no-bypass constraints.
- `AB-00-08` - layered validation, provenance, protected-content, and
  acceptance gates.

Resolved baseline to preserve: rule packs are data artifacts, not executable
programs. The evaluator may interpret a deliberately small declarative
expression surface from the committed `DEL-06-01` schema and bind variables
only from declared rule-pack inputs, user-supplied values, and explicit supplied
result fields. Unit and dimensional checks are part of evaluator correctness.
Missing variables, unsupported operators, invalid references, non-finite
values, unsafe constructs, and dimension mismatches are deterministic findings,
not silent defaults. Evaluation results may support user-rule checked or
user-rule failed states, but they must not represent certification, sealing,
authentication, human approval, or code compliance for reliance.

Remaining implementation-level TBDs are not resolved by this dispatch:
final expression grammar/library selection, parser dependency policy, complete
quantity representation, conversion constants, final diagnostic code taxonomy,
comparison tolerance policy, variable namespace/result-field binding contract,
threat-model review depth, public API transport, GUI editor presentation,
private rule-pack storage, checksum lifecycle, and report/result-envelope
integration.

## Explicit Write Scope

The bounded implementation write scope, if separately authorized, is limited to:

- `core/rules/expression_evaluator/.gitignore`
- `core/rules/expression_evaluator/Cargo.toml`
- `core/rules/expression_evaluator/README.md`
- `core/rules/expression_evaluator/src/lib.rs`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-02_Sandboxed unit-aware expression evaluator/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-06-02.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch. In particular, do not edit
`DAG-001`, candidate edges, deliverable-local `Dependencies.csv`,
`DEV-001_IMPLEMENTATION_EVIDENCE.csv`, or `DEV-001_BLOCKER_QUEUE.*` during
implementation unless separately authorized.

## Acceptance Criteria

This bounded item is acceptable for implementation closeout when:

- The evaluator accepts a small declarative expression representation and never
  invokes host-language code, imports, reflection, macros, filesystem access,
  network access, process execution, environment access, or hidden side
  effects.
- The implementation binds variables only from explicit caller-supplied
  bindings that correspond to declared rule-pack inputs, user-supplied values,
  or result fields allowed by the sealed interface.
- The evaluator performs deterministic unit/dimension checks for numeric
  operands, comparisons, and outputs using the existing unit/dimension contract
  rather than inventing a parallel unrestricted unit model.
- Unsupported operators, malformed expressions, invalid references, duplicate
  or missing bindings, missing required values, non-finite values, division by
  zero, and dimension mismatches produce deterministic findings.
- Evaluation outputs preserve the analysis boundary: mechanics solved,
  rule-inputs incomplete, user-rule checked, user-rule failed, and human review
  required remain distinct states.
- Tests cover successful invented expressions, unsafe construct rejection,
  unsupported expression forms, missing variable binding, invalid reference,
  unit mismatch, non-finite input, division by zero, deterministic diagnostics,
  and absence of professional/code-compliance claims.
- Public tests and docs use invented non-code examples only; they do not embed
  protected standards text, copied standards formulas, protected tables,
  proprietary engineering values, material allowables, SIF/flexibility data,
  owner standards, or private rule packs.
- No lifecycle transition, dependency-register edit, candidate-edge change,
  blocker-queue refresh, protected standards data, proprietary engineering
  value, or professional/code-compliance claim occurs unless separately
  authorized.

## Implementation Summary

Implemented within the sealed write scope:

- Added the `open_pipe_stress_expression_evaluator` Rust crate under
  `core/rules/expression_evaluator`.
- Implemented explicit expression-tree evaluation for literals, variable
  references, unary negation, arithmetic, dimensionless scaling, same-dimension
  ratios, and same-dimension comparisons.
- Added bounded dimension metadata for common mechanics/unit-contract
  dimensions without adding conversion constants or protected data.
- Added deterministic findings for unsafe constructs, unsupported expression
  forms, missing variables, duplicate bindings, invalid references, missing
  required values, non-finite inputs, division by zero, dimension mismatches,
  type mismatches, and human-approval status boundary violations.
- Preserved analysis-status boundaries, including rejection of external human
  approval as an automatic evaluator output.
- Updated `docs/SPEC.md`, `docs/TYPES.md`, and deliverable `MEMORY.md`.

Verification performed:

- `rustfmt --edition 2021 core/rules/expression_evaluator/src/lib.rs` passed.
- `rustc --edition=2021 --test
  core/rules/expression_evaluator/src/lib.rs -o
  /tmp/open_pipe_stress_expression_evaluator_tests` passed.
- `/tmp/open_pipe_stress_expression_evaluator_tests` passed: 14 tests.
- `git diff --check` passed.

Lifecycle/evidence/queue closeout:

- No lifecycle transition was performed.
- No deliverable-local `Dependencies.csv` edit was performed.
- No `DEV-001_IMPLEMENTATION_EVIDENCE.csv` update was performed.
- No `DEV-001_BLOCKER_QUEUE.*` refresh was performed.
- Aggregate `DAG-001` was left unchanged.

## Dispatch Task Shape

```markdown
PURPOSE: Implement the sealed OpenPipeStress deliverable within explicit scope.
RequestedBy: WORKING_ITEMS

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-02_Sandboxed unit-aware expression evaluator
DeliverablePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-02_Sandboxed unit-aware expression evaluator
TaskProfile: rule-pack-engine

DeliverableID: DEL-06-02
PackageID: PKG-06

Tasks:
  - Implement only the sandboxed unit-aware expression evaluator, focused evaluator tests, docs updates, and memory update authorized for this deliverable.
  - Preserve all applicable contract invariants and architecture-basis constraints.
  - Keep all candidate edges non-gating; do not implement `DEL-06-03`, `DEL-06-04`, `DEL-06-05`, private rule-pack lifecycle/checksum handling, GUI editor behavior, report generation, public API transport, or developer-guide content.
  - Record a run summary and open issues.

RuntimeOverrides:
  DecompositionPath: docs/_Decomposition/SOFTWARE_DECOMP.md
  DecompositionRevision: "0.4"
  DAGPath: execution/_DAG/DAG-001/DependencyEdges.csv
  DAGApprovalRecord: execution/_DAG/DAG-001/APPROVAL_RECORD.md
  CoordinationMode: FULL_GRAPH
  BlockerComputation: ENABLED_ACTIVE_EDGES_ONLY
  ScopeChangeID: SCA-001

CustomInstructions:
  - Read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `Specification.md`, `Guidance.md`, `Procedure.md`, and existing rule-pack/unit/schema/test patterns before editing.
  - Apply only applicable `AB-00-*` constraints; do not copy full `PKG-00` prose.
  - Treat protected standards/code data as out of scope.
  - Keep the expression surface small and declarative; no arbitrary executable code, host-language `eval`, imports, reflection, filesystem access, network access, process execution, or hidden side effects.
  - Unknown parser/library, quantity representation, tolerance, API, GUI, report, private-storage, checksum, and threat-model details remain `TBD` unless resolved inside this sealed scope by explicit human approval.
  - Do not claim certification, approval, sealing, authentication, or code compliance for reliance.
  - Do not edit files outside this sealed write scope.
  - Do not recompute or mutate blocker queues unless explicitly assigned.

ExpectedOutputs:
  - `core/rules/expression_evaluator/` Rust crate implementing the bounded sandboxed evaluator contract.
  - Focused evaluator tests in the crate.
  - `docs/SPEC.md` and `docs/TYPES.md` updates for the bounded evaluator surface.
  - Deliverable `MEMORY.md` update.
  - Open issue list for unresolved `TBD`, assumptions, or cross-deliverable dependencies.

StopConditions:
  - Need to introduce a third-party parser/evaluator dependency without an explicit architecture/security decision.
  - Need to copy protected standards text, formulas, examples, tables, or proprietary engineering values.
  - Need to change `DAG-001`, candidate edges, implementation evidence, blocker queue, lifecycle state, or local dependency registers.
  - Need to implement sibling deliverables or broaden into GUI, report, API, private storage, checksum lifecycle, or completeness-checker scope.
```

## Closeout Instructions

If implementation is later authorized and completed, the session must close the
control loop before ending:

- Run focused evaluator tests and any adjacent tests needed for touched docs or
  contracts.
- Run `git diff --check`.
- Run a focused protected-content/prohibited-claim scan over touched files.
- Do not update lifecycle state, implementation evidence, local dependency
  satisfaction rows, or blocker queues unless the human explicitly authorizes
  that closeout step.
- Update this dispatch brief and `NEXT_INSTANCE_STATE.md` with actual
  implementation results and remaining `TBD` items.
