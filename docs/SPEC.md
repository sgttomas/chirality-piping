---
doc_id: OPS-SPEC
doc_kind: governance.technical_spec
status: draft
created: 2026-04-30
refs:
  - rel: depends_on
    to: OPS-CONTRACT
  - rel: depends_on
    to: OPS-TYPES
  - rel: implements
    to: OPS-PRD
---

# SPEC — OpenPipeStress Technical and Agentic Implementation Specification

## 1. Architectural overview

OpenPipeStress shall be organized as a code-neutral engineering workbench with five separable layers:

```text
GUI / UX Layer
  └── Application Services
        └── Domain Core
              ├── Geometry + Units + Data Schemas
              ├── Solver Core
              ├── Loads + Stress Recovery
              ├── Rule-Pack Evaluator
              └── Reporting/Audit Manifest
        └── Adapters
              ├── Private Libraries
              ├── Import/Export Plugins
              ├── Local FEA Handoff
              └── Storage/Packaging
```

The domain core owns mechanics and invariant enforcement. Adapters may import/export data but may not bypass unit checks, provenance checks, or public/private data boundaries.

## 2. Repository layout target

```text
open-pipe-stress/
├── docs/
├── schemas/
├── core/
│   ├── units/
│   ├── geometry/
│   ├── model/
│   ├── solver/
│   ├── loads/
│   ├── stress/
│   ├── rules/
│   └── reports/
├── gui/
├── adapters/
├── examples/
│   ├── invented_models/
│   └── rule_packs_invented_only/
├── validation/
│   ├── benchmarks/
│   ├── hand_calcs/
│   └── regression/
├── scripts/
└── tests/
```

## 3. Domain objects

Minimum domain objects:

| Object | Purpose | Required properties |
|---|---|---|
| `Project` | Project container and design basis metadata | ID, units, storage policy, rule-pack refs, report settings |
| `Model` | Piping system model | nodes, elements, components, supports, loads, libraries |
| `Node` | 3D coordinate and six-DOF state | coordinates, DOF flags, imposed displacement refs |
| `Element` | Analytical member between nodes | type, section/material refs, local coordinate system, results stations |
| `Component` | Piping-specific object | component type, geometry, user modifiers, source/provenance |
| `Material` | User/private material record | density, modulus, expansion, allowables, source status |
| `Section` | Pipe/section properties | OD, thickness, corrosion basis, section properties, source status |
| `Support` | Restraint/restraint behavior | type, direction, stiffness/gap/friction, active-state results |
| `LoadCase` | Primitive loading | type, magnitude, units, source |
| `Combination` | Algebraic result combination | expression, result basis, category |
| `RulePack` | User-defined code/design-basis check | required inputs, formulas, allowables, status, checksum |
| `Report` | Auditable calculation output | input manifest, warnings, results, rule-pack refs, notices |

## 4. Unit system and dimensional analysis

The domain core owns the unit contract. Every physical value that crosses a
schema, application-service, solver, import/export, report, or rule-evaluation
boundary must carry explicit unit metadata unless the field is explicitly
classified as dimensionless, ratio, percentage, or coefficient.

The unit contract is represented by `schemas/units.schema.yaml` and the
`core/units` module contract. It defines:

- unit-system records, unit definitions, dimensions, and dimension vectors;
- unit-bearing quantity records with magnitude, unit reference, dimension,
  missing-unit behavior, and provenance;
- conversion declarations with transform kind, provenance, redistribution
  status, and review status;
- dimension-check records and unit diagnostics;
- operation rules for same-dimension operations, derived-dimension operations,
  schema/import/export validation, and rule evaluation;
- test obligations and open decisions that block downstream schema/API freeze
  where decisions remain `TBD`.

Dimensionless values are not a fallback for missing units. Missing or ambiguous
unit metadata on a unit-bearing physical value is a diagnostic, and solve- or
rule-check-required missing unit data must not be silently supplied.

Conversion data is governed data. Public unit or conversion records require
provenance, redistribution status, contributor certification, and review
status. Suspected protected standards content, proprietary vendor data, or
undocumented commercial data must be quarantined and escalated under the
project IP/data-boundary policy.

Deterministic conversion tests that require numeric constants remain gated until
the project has accepted the unit catalog, conversion source set, numeric
representation, and tolerance policy. Until accepted, unsupported conversion
semantics such as offset temperature, gauge versus absolute pressure, and
angle/rotation treatment remain explicit `TBD` decisions or blocking
diagnostics.

Unit checks support mechanics and rule evaluation. They do not certify, seal,
approve, authenticate, or declare engineering code compliance for reliance.

## 5. Solver core requirements

### 5.1 Degrees of freedom

Each node carries six degrees of freedom:

```text
Ux, Uy, Uz, Rx, Ry, Rz
```

### 5.2 Straight pipe/frame element

The straight-pipe element shall support axial, torsional, and bending stiffness in local coordinates, local-to-global transformation, thermal strain, distributed loads, and element end-force recovery.

### 5.3 Bend and component treatment

The open core shall provide fields and mechanics interfaces for bends, branches, reducers, valves, flanges, expansion joints, and rigid elements. User-supplied SIFs, flexibility factors, stress indices, manufacturer stiffnesses, and local data are inputs, not public defaults.

### 5.4 Nonlinear supports

Nonlinear supports shall use an active-set or equivalent iterative method. Results must record active/inactive states, gaps, lift-off, friction state, convergence tolerance, iteration count, and non-convergence warnings.

### 5.5 Numerical quality

Solver results must be deterministic for the same model, units, solver version, and rule-pack inputs. Sparse-solver settings, tolerances, and conditioning warnings must be reportable.

## 6. Loads and stress recovery

Primitive loads include weight, pressure, temperature, imposed displacement, hydrotest, wind, seismic, and user occasional loads. Code-specific load combinations are not public defaults; they are user rule-pack or project inputs.

Stress recovery shall calculate open mechanics quantities such as axial stress, bending stress, torsional shear stress, pressure membrane stresses, and resultants. Code-category equations are rule-pack mappings.

## 7. Rule-pack evaluator

Rule packs are private or user-owned design-basis artifacts. The public project ships schemas and invented examples only.

Minimum rule-pack sections:

```yaml
rule_pack:
  id: string
  name: string
  version: string
  source_notice: string
  redistribution_status: private|public_permissive|unknown
  checksum: sha256:...
required_inputs: []
variables: []
checks: []
report_notice: string
```

The evaluator must be sandboxed, unit-aware, deterministic, and incapable of arbitrary code execution.

## 8. GUI requirements

The GUI shall expose:

- 3D centerline viewport;
- model tree;
- property inspector;
- material/component/rule-pack editors;
- load case and combination manager;
- support/restraint editor;
- results viewer;
- warnings and blocking messages;
- report preview/export.

Missing data warnings must distinguish:

| Warning class | Meaning |
|---|---|
| `SOLVE_BLOCKING` | Required physical input missing. |
| `RULE_CHECK_BLOCKING` | Mechanics can solve, but rule-pack check lacks required user/code data. |
| `PROVENANCE_WARNING` | Value exists but source is missing or weak. |
| `ASSUMPTION_WARNING` | User or model assumption requires review. |
| `NONLINEAR_WARNING` | Convergence or active-state uncertainty exists. |
| `IP_BOUNDARY_WARNING` | Public contribution/report may contain protected or private data. |

## 9. Reporting and audit

Reports must include:

- software version and solver version;
- model hash and input manifest;
- unit system;
- material/component/rule-pack provenance summary;
- load cases and combinations;
- displacements, rotations, forces, moments, reactions, stresses;
- warnings, assumptions, missing data;
- rule-pack name/version/checksum;
- statement that code-specific data is user-supplied;
- statement that professional reliance requires competent human review.

Reports must not reproduce protected code text, protected standards tables, or proprietary formulas in public templates/examples.

## 10. Verification and validation mechanics

The project shall maintain:

- unit tests for units, schemas, expression evaluation, and status semantics;
- hand-calculation benchmark cases for mechanics;
- regression tests for solver and stress recovery;
- nonlinear support convergence tests;
- report reproducibility tests;
- public validation manual using original/public/permissive examples only.

## 11. Agentic development mechanics

Downstream implementation should use the decomposition package as the source of work identity:

```text
PKG-XX_<PackageLabel>/
└── 1_Working/
    └── DEL-XX-YY_<DeliverableLabel>/
        ├── _STATUS.md
        ├── _CONTEXT.md
        ├── _REFERENCES.md
        ├── _DEPENDENCIES.md
        ├── Datasheet.md
        ├── Specification.md
        ├── Guidance.md
        └── Procedure.md
```

Each deliverable must be executable by a bounded Type 2 specialist. If a deliverable becomes cross-domain or too large, it must be split or explicitly accepted as a human-approved open issue.

## 12. Acceptance semantics

A software increment may be accepted for development use only when:

1. scope and deliverable ID match the decomposition;
2. tests and/or documentation artifacts listed in the deliverable exist;
3. no protected public data has been introduced;
4. missing data and assumptions are surfaced;
5. solver/rule/report changes pass required validation gates;
6. a human accepts the work at the review gate.
