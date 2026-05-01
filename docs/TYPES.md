---
doc_id: OPS-TYPES
doc_kind: governance.types
status: draft
created: 2026-04-30
refs:
  - rel: governed_by
    to: OPS-CONTRACT
---

# TYPES — OpenPipeStress Domain Vocabulary and Identity

## 1. Project hierarchy

OpenPipeStress agentic development uses a flat hierarchy:

```text
docs/_Decomposition/SOFTWARE_DECOMP.md
└── PKG-XX Package
    └── DEL-XX-YY Deliverable
```

There are no phases, sub-packages, or task sub-levels inside deliverables. Delivery sequencing may be managed separately, but package identity is a domain partition, not a schedule phase.

## 2. Stable identifiers

| Entity | Format | Example | Assigned by |
|---|---|---|---|
| Scope Item | `SOW-NNN` | `SOW-003` | SOFTWARE_DECOMP |
| Objective | `OBJ-NNN` | `OBJ-001` | SOFTWARE_DECOMP |
| Package | `PKG-XX` | `PKG-04` | SOFTWARE_DECOMP |
| Deliverable | `DEL-XX-YY` | `DEL-04-02` | SOFTWARE_DECOMP |
| Decision | `DEC-NNN` | `DEC-001` | Human / Type 1 agent draft |
| Open Issue | `OI-NNN` | `OI-004` | Type 1 agent draft / human |
| Risk | `RISK-NNN` | `RISK-002` | Type 1 agent draft / human |

Deliverable IDs are coupled to package IDs: `DEL-04-02` belongs to `PKG-04`. IDs persist across renames.

## 3. Software deliverable types

| Type | Meaning |
|---|---|
| `DOC_UPDATE` | Documentation, policy, guide, or report-template work. |
| `DATA_MODEL_CHANGE` | Schema, type, serialization, or domain object change. |
| `API_CONTRACT` | Public API, plugin, import/export, or data contract. |
| `BACKEND_FEATURE_SLICE` | Bounded backend/core behavior implementation. |
| `UX_UI_SLICE` | Bounded GUI feature or view. |
| `TEST_SUITE` | Verification, regression, validation, or lint suite. |
| `CI_CD_CHANGE` | Build, packaging, release, or pipeline change. |
| `SECURITY_CONTROL` | Privacy, redaction, storage, telemetry, or security control. |

## 4. Analysis-status vocabulary

| Status | Meaning | Authority level |
|---|---|---|
| `MODEL_INCOMPLETE` | Required physical data for solving is missing. | Software finding. |
| `MECHANICS_SOLVED` | Numerical mechanics solve completed. | Solver result only. |
| `RULE_INPUTS_INCOMPLETE` | Mechanics may be solved but rule-pack required values are missing. | Rule-pack finding. |
| `USER_RULE_CHECKED` | User-defined rule pack evaluated the result. | Software computation using user data. |
| `USER_RULE_FAILED` | User-defined rule pack produced a failing result. | Software computation using user data. |
| `HUMAN_REVIEW_REQUIRED` | Result requires competent engineering review before reliance. | Always true for professional use. |
| `HUMAN_APPROVED_FOR_PROJECT` | A human has recorded project-specific acceptance outside the solver core. | Human record only. |

The software must not use `CODE_COMPLIANT` as an automatic status.

The code-neutral analysis boundary is the data-model surface that keeps these
statuses from collapsing into one another. `schemas/analysis_boundary.schema.yaml`
requires an explicit authority model: mechanics statuses come from solver
results, user-rule statuses come from user-supplied rule-pack computation, and
human acceptance is an external hash-bound project record only.

## 5. Epistemic labels

| Label | Meaning |
|---|---|
| `FACT` | Directly observed in a cited source or model artifact. |
| `ASSUMPTION` | Reasonable inference requiring human validation. |
| `PROPOSAL` | Suggested design/development move requiring decision. |
| `TBD` | Unknown or unwarranted; must be resolved before reliance. |

## 6. Piping/software vocabulary

|Canonical Term|Definition|Notes|
|---|---|---|
|Open Mechanics|Public analytical mechanics implemented by the solver: geometry, stiffness, loads, stress recovery, units, reports.|May be shipped publicly.|
|Protected Standards Data|Code text, tables, figures, code-derived formulas, allowables, SIF/flexibility tables, protected dimensional tables.|Must not be shipped publicly unless explicit redistribution rights exist.|
|User-Supplied Code Data|Values, formulas, allowables, SIFs, flexibility factors, load combinations, and interpretations entered by the user.|May be stored in private project/rule-pack files.|
|Mechanics Solve|A solved numerical model with displacements, reactions, forces, moments, and mechanical stresses.|Not equivalent to a code check.|
|Rule-Pack Check|A user-defined expression/comparison applied to solver outputs.|Not equivalent to professional authentication.|
|Professional Approval|Human acceptance for reliance by a competent/responsible professional.|Outside software authority.|
|Centerline Model|A 3D line-element representation of the piping centerline and components.|Default global analysis model.|
|Local FEA Handoff|A transfer package for local shell/solid analysis.|Specialized workflow, not the default global solver.|
|Plugin Manifest|A declared extension metadata record that requests entrypoints, permissions, provenance, privacy posture, checksums, sandbox requirements, and no-bypass constraints.|A manifest grants nothing by itself.|
|Extension Point|A governed boundary where a plugin or adapter may request to import, export, validate, report, emit diagnostics, or expose user-owned rule-pack metadata.|Exact loader and transport remain `TBD`.|
|Domain Surface|The model, persistence, unit, analysis-boundary, rule-pack metadata, result, report, or diagnostic surface touched by an extension entrypoint.|Must be declared in the plugin manifest.|
|No-Bypass Constraint|A mandatory control that plugins and adapters must preserve, including units, provenance, privacy, protected-content, diagnostics, hashes, rule sandboxing, reports, solver boundaries, and human acceptance boundaries.|Cannot be waived by plugin code.|

## 7. Data provenance labels

| Label | Meaning |
|---|---|
| `USER_SUPPLIED_LICENSED_CODE` | User entered value from a licensed code/standard. |
| `USER_SUPPLIED_OWNER_SPEC` | User entered value from owner/company design basis. |
| `MANUFACTURER_PUBLIC_PERMISSIVE` | Manufacturer data with documented redistribution rights. |
| `MANUFACTURER_PRIVATE` | Manufacturer/vendor data for private project use only. |
| `PUBLIC_DOMAIN_OR_ORIGINAL` | Public-domain/original/invented data safe for public examples. |
| `UNKNOWN_SOURCE` | Source not documented; cannot be relied on without review. |
| `PROTECTED_CONTENT_SUSPECTED` | Quarantine and escalate before public use. |

## 8. Canonical domain object registry

The canonical machine-readable domain model is `schemas/model.schema.yaml`. That schema uses JSON Schema 2020-12, is written as strict JSON syntax in a `.yaml` path, and is intentionally code-neutral: it defines public object structure, identifiers, references, unit-bearing values, provenance, diagnostics, results, and report settings, but it does not bundle protected standards data, proprietary catalog values, code-specific allowables, SIF/flexibility tables, or compliance rules.

| Object | Registry meaning | Boundary note |
|---|---|---|
| `Project` | Project container for units, privacy/storage posture, models, rule-pack references, report settings, reports, diagnostics, and hashes. | Project-private engineering data remains user controlled unless intentionally contributed with documented rights. |
| `Model` | Centerline model container for nodes, elements, components, materials, sections, supports, load cases, combinations, results, and diagnostics. | Solver and GUI behavior are downstream consumers, not encoded as claims in the schema. |
| `Node` | Addressable point with unit-aware coordinates and six-degree-of-freedom state fields. | Missing imposed values or constraints are findings, not silent defaults. |
| `Element` | Analytical member connecting node references with material, section, optional component, local-coordinate, and result-station hooks. | Mechanics implementation remains separate from schema structure. |
| `Component` | Piping-specific object such as bend, branch, reducer, valve, flange, expansion joint, rigid, or other user-defined component. | User modifiers and code/manufacturer values require provenance and may be private. |
| `Material` | User/private or permissively sourced material record with unit-bearing properties and provenance. | Public schema does not provide protected material allowables or code tables. |
| `Section` | Pipe/section record with unit-bearing properties and provenance. | Protected dimensional tables and proprietary catalog data are not public defaults. |
| `Support` | Restraint/support record with target reference, directions, unit-bearing properties, and active-state result hook. | Nonlinear solution behavior and convergence semantics are solver-owned. |
| `LoadCase` | Primitive loading record with target references, units, and provenance. | Code-specific load requirements are user/rule-pack supplied. |
| `Combination` | Algebraic grouping of load-case factors with an explicit basis. | No public code-specific combination defaults are implied. |
| `RulePackRef` | Reference to a user/public rule-pack by identity, version, checksum, source notice, redistribution status, and required-input links. | Rule formulas, protected interpretations, and evaluator implementation are outside this schema. |
| `Quantity` | Unit-bearing value record with magnitude, unit, dimension, and provenance. | Dimensionless values must be explicit; missing or incompatible units are findings, not hidden defaults. |
| `Provenance` | Source, license, contributor, redistribution, and review-status record used by reliance-affecting public/private data. | Unknown or suspected protected sources must remain reviewable and cannot be treated as accepted public data. |
| `Reference` | Typed stable-object reference used instead of positional coupling where editing, solving, reporting, or audit depends on identity. | External references remain explicit and do not bypass schema validation. |
| `Diagnostic` | Result-envelope warning/finding record with code, class, severity, source, affected object, message, remediation, and provenance. | Diagnostics expose missing data and boundary risks without becoming compliance claims. |
| `Checksum` | Hash metadata record with algorithm, canonicalization, payload reference, and value. | JSON payload hashes use the JCS-compatible basis where applicable; physical package/container details remain owned by persistence work. |
| `ProjectPersistenceEnvelope` | Versioned create/open/save envelope with schema version, project payload, migration status, hash metadata, diagnostics, round-trip manifest, validation profile, and service-operation contract records. | Delegates model structure to `schemas/model.schema.yaml`; physical project package/container remains `TBD`. |
| `PersistenceOperation` | Application-service contract record for create, open, save, validate, version-check, and migrate operations. | Operations must not bypass schema, unit, provenance, rule-pack reference, private-data, protected-content, or professional-boundary checks. |
| `RoundTripManifest` | Persistence comparison record naming semantic equality categories, serialization basis, volatile fields, and normalization rules. | Round trips preserve engineering-relevant data and do not insert silent defaults. |
| `HumanAcceptanceRef` | External human-review or project-acceptance reference bound to specific hashes. | Invalidates on bound-hash changes and is not an automatic software approval, certification, sealing, authentication, or code-compliance status. |
| `Result` | Mechanical, diagnostic, stress, reaction, or user-rule-check result envelope with analysis statuses and unit-aware values. | Result statuses do not mean automatic code compliance. |
| `ReportSettings` | Report configuration for manifests, provenance summaries, notices, result references, and rule-pack references. | Report rendering and professional acceptance records remain separate workflows. |
| `Report` | Auditable report-facing record for manifests, hashes, statuses, diagnostics, rule-pack refs, provenance summary, and professional-boundary notice. | Reports are decision support and must not claim certification, sealing, or professional approval by the software. |

Companion boundary schemas:

| Schema | Purpose | Boundary note |
|---|---|---|
| `schemas/analysis_boundary.schema.yaml` | Separates mechanics solve authority, user-rule-check authority, missing-input findings, diagnostics, and external human acceptance references. | Human acceptance is reference-only and hash-bound; it is not emitted by solver or rule-pack computation. |
| `schemas/analysis_status.schema.yaml` | Defines result-envelope status semantics and external human acceptance records. | `HUMAN_APPROVED_FOR_PROJECT` is not an automatic software status. |
| `schemas/project_persistence.schema.yaml` | Defines the versioned project persistence envelope, round-trip manifest, migration status, hash metadata, validation profile, service operations, private-data markers, and external human acceptance references. | Public fixtures must not contain protected standards data, private rule content, proprietary values, or automatic compliance/professional-approval claims. |

## 9. Lifecycle states for development deliverables

```text
OPEN → INITIALIZED → IN_PROGRESS → CHECKING → ISSUED
```

`ISSUED` means accepted as a development artifact by the human project authority. It does not mean professional engineering authentication of any piping calculation.
