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

## 8. Lifecycle states for development deliverables

```text
OPEN → INITIALIZED → IN_PROGRESS → CHECKING → ISSUED
```

`ISSUED` means accepted as a development artifact by the human project authority. It does not mean professional engineering authentication of any piping calculation.
