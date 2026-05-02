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

## 4.1 Material library schema

Material library records are governed data. The public schema may define
material identity, temperature-dependent property slots, allowable-value slots,
unit metadata, provenance, redistribution status, review status, and
completeness findings. The public repository must not bundle protected material
allowable tables, code-specific allowable values, proprietary catalog data, or
private user material libraries as defaults.

The material-library contract is represented by `schemas/material.schema.yaml`.
It defines:

- material-library metadata, privacy class, and provenance;
- material records with unit-bearing property slots and review status;
- allowable slots for private or reviewed user-rule data without public
  protected/code-specific values;
- completeness rules and diagnostics for missing solve-required or
  rule-check-required material data;
- open decisions for public fixture policy, accepted source catalogs,
  allowable-value storage, and temperature interpolation.

Public material fixtures must be invented non-engineering examples or
permissively sourced records with completed review evidence. Missing or
unreviewed material values remain explicit findings; they are not supplied by
silent defaults.

## 4.2 Section and component library schemas

Pipe section and component library records are governed data. The public schema
may define user-entered dimensions, weights, centers of gravity, component
modifier slots, derived-property slots, provenance, redistribution status,
review status, and completeness findings. The public repository must not bundle
protected dimensional tables, code-specific component modifiers, proprietary
catalog values, or private user component libraries as defaults.

The section and component library contracts are represented by
`schemas/section.schema.yaml` and `schemas/component.schema.yaml`. They define:

- section-library metadata, pipe/section dimension slots, derived-property
  slots, provenance, review status, and missing-data diagnostics;
- component-library metadata, component field slots for dimensions, weights,
  centers of gravity, stiffness, effective area, movement limits, and user
  modifier inputs;
- bend/elbow family contracts for user-entered centerline radius, included
  angle, plane/orientation metadata, source references, and user-supplied SIF
  or flexibility-factor slots;
- branch-connection family contracts for user-entered run/header geometry,
  connection angle/type slots, reinforcement source slots, and user-supplied
  SIF or flexibility-factor slots;
- rigid/semi-rigid component family contracts for valves, flanges, reducers,
  rigid parts, and specialty items using user-entered dimensions, weights,
  centers of gravity, stiffness behavior, and source references;
- expansion-joint family contracts for user/manufacturer-supplied stiffnesses,
  effective area, movement limits, hardware references, and manufacturer
  provenance slots;
- explicit public repository value policies for SIF, flexibility, proprietary
  catalog, and user/private values;
- completeness rules and diagnostics for solve-required, property-calculation,
  rule-check-required, and import/review-required data;
- open decisions for fixture policy, accepted source catalogs, import formats,
  section-property calculation policy, and component value policy.

Public section/component fixtures must be invented non-engineering examples or
schema-shape-only records unless a public-permissive source has completed
review evidence. Missing or unreviewed section/component values remain explicit
findings; they are not supplied by silent defaults.

The pipe section property calculator is represented by
`core/section_properties/calculator.py`. It calculates circular pipe section
properties and mass-per-length values from user-entered dimensions and density
inputs only. It does not provide pipe schedule tables, protected dimensional
tables, material defaults, catalog values, unit conversion constants, or
code-specific values. Unit-bearing inputs must already carry compatible unit and
dimension metadata; otherwise the calculator returns blocking diagnostics such
as `SECTION_UNIT_MISSING`, `SECTION_DIMENSION_INCONSISTENT`, or
`SECTION_CALCULATION_INPUT_INVALID`.

The library import provenance checker is represented by
`core/library_import/provenance_checker.py`. It validates already-parsed
material, section, and component library payloads for required provenance,
redistribution status, review disposition, private-only handling, protected
content quarantine, and unit metadata preservation. It does not parse external
formats and does not make legal conclusions; unresolved rights remain review
findings for the human project authority.

Bend and elbow records define schema slots and mechanics-interface metadata
only. Public repository fixtures may show the shape of bend geometry fields but
must not supply protected bend tables, code-specific SIF or flexibility values,
or proprietary catalog geometry as defaults.

Branch connection records define schema slots and mechanics-interface metadata
only. Public repository fixtures may show the shape of branch geometry and
reinforcement-reference fields but must not supply protected branch tables,
code-specific SIF or flexibility values, reinforcement values, or proprietary
catalog geometry as defaults.

Rigid and semi-rigid component records define schema slots and
mechanics-interface metadata only. Public repository fixtures may show the
shape of valve, flange, reducer, rigid, and specialty-item fields but must not
supply protected dimensional/rating tables, manufacturer weights, centers of
gravity, stiffness values, or proprietary catalog geometry as defaults.

Expansion joint component records define schema slots and mechanics-interface
metadata only. Public repository fixtures may show stiffness, effective-area,
movement-limit, hardware, and manufacturer-reference field shapes but must not
supply manufacturer values, proprietary catalog data, private library values,
or invented engineering defaults.

### 4.3 Code-neutral analysis boundary

The analysis boundary is represented by
`schemas/analysis_boundary.schema.yaml`. It separates three authority domains:

- mechanics solve authority: solver results and diagnostics only;
- user-rule-check authority: software computation using user-supplied rule-pack
  data, rule-pack version, checksum, source notice, redistribution status, and
  provenance;
- human acceptance authority: external project records bound to reviewed
  evidence hashes.

Software-generated statuses may report `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`,
`RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, and
`HUMAN_REVIEW_REQUIRED`. Software must not emit human approval, certification,
sealing, authentication, or code-compliance labels as automatic analysis
statuses.

Missing solve-required inputs and missing rule-check-required inputs are
explicit findings with diagnostics and provenance. They are not defaulted by the
solver, rule-pack evaluator, adapters, reports, or GUI.

### 4.4 Project persistence and round-trip serialization

The project persistence envelope is represented by
`schemas/project_persistence.schema.yaml`. It is a versioned, schema-governed
JSON document contract for create/open/save, version checks, migration status,
canonical hash metadata, private-data markers, diagnostics, and deterministic
round-trip manifests. The envelope delegates model structure to
`schemas/model.schema.yaml` rather than redefining model objects.

Persistence service operations are application-service boundaries. Create,
open, save, validate, version-check, and migrate flows must return structured
outputs and diagnostics; adapters and plugins must not bypass schema
validation, unit metadata checks, provenance checks, rule-pack reference checks,
private-data controls, protected-content screening, or professional-boundary
checks.

JSON payload hashes use the accepted JCS-compatible canonical JSON basis where
the payload is JSON. Hash records must identify payload scope, such as project
envelope, project payload, model payload, rule-pack reference, input manifest,
report manifest, or external artifact. Non-JSON and binary payload partitioning
remains `TBD` and must be explicit in hash or round-trip manifests.

Round-trip acceptance compares semantic equality for model content, unit
metadata, loads, rule-pack references, provenance metadata, reproducibility
metadata, and documented volatile-field exclusions. Round trips must not insert
silent engineering defaults for units, provenance, rule-pack values, material
data, component data, SIF/flexibility inputs, allowables, or load basis values.

Optional human acceptance references, when present in persisted projects, are
external and hash-bound. They invalidate on bound-hash changes and do not imply
software certification, sealing, authentication, professional approval, or
automatic code compliance.

### 4.5 Plugin and extension domain contracts

The plugin manifest contract is represented by
`schemas/plugin_manifest.schema.yaml`. It defines domain-level extension
metadata, entrypoints, permissions, provenance, privacy posture, checksums,
sandbox requirements, no-bypass controls, and professional-boundary constraints.

Plugins and adapters are denied by default. A manifest request is not a runtime
grant. Each entrypoint must identify the domain surface it touches, such as the
canonical model, project persistence, units, analysis-boundary state, rule-pack
metadata, results, reports, or diagnostics.

Plugin and adapter paths must preserve the same controls as first-party code:
schema validation, unit checks, provenance checks, private-data controls,
protected-content screening, diagnostics/result envelopes, persistence hashes,
rule-pack sandboxing, report boundaries, solver boundaries, and external
hash-bound human acceptance records. They must not create software-generated
professional approval records or automatic code-compliance statuses.

## 5. Solver core requirements

### 5.1 Degrees of freedom

Each node carries six degrees of freedom:

```text
Ux, Uy, Uz, Rx, Ry, Rz
```

### 5.2 Straight pipe/frame element

The straight-pipe element shall support axial, torsional, and bending stiffness in local coordinates, local-to-global transformation, thermal strain, distributed loads, and element end-force recovery.

The first frame-kernel slice is `core/solver/frame_kernel`. It defines the
six-degree-of-freedom node ordering, two-node frame element stiffness,
local-axis transformation, dense assembly, boundary-condition reduction, and a
temporary deterministic dense solve interface for small verification cases. The
sparse numerical library, solver tolerances, and canonical calculation unit
basis remain `TBD` until accepted through a later solver/architecture gate.
Frame-kernel inputs are numeric mechanics quantities that must arrive through
unit-aware upstream contracts; missing, non-finite, non-positive, degenerate,
invalid-node, repeated-restraint, or singular-system conditions are explicit
errors, not silent defaults.

### 5.3 Bend and component treatment

The open core shall provide fields and mechanics interfaces for bends, branches, reducers, valves, flanges, expansion joints, and rigid elements. User-supplied SIFs, flexibility factors, stress indices, manufacturer stiffnesses, and local data are inputs, not public defaults.

### 5.3.1 Linear supports and restraints

The linear support and restraint slice is `core/solver/linear_supports`. It
models anchors, guides, line stops, vertical supports, linear springs, and
imposed displacement boundary data as explicit mechanics-boundary inputs for
the frame-kernel six-DOF order. It prepares restrained DOFs, linear spring
entries, imposed displacement entries, and deterministic findings for missing
or invalid solve-required support data.

Support quantities retain dimension intent, including translational stiffness,
rotational stiffness, displacement, and rotation. Unit conversion constants,
canonical calculation units, support coordinate-frame convention,
constraint-elimination or penalty strategy, sparse-solver integration, and
final result-envelope integration remain `TBD` until accepted through later
gates.

The linear support slice does not implement nonlinear gap, lift-off, one-way,
friction, or active-set behavior; that behavior remains assigned to
`DEL-04-04`. It also does not provide public support stiffness defaults,
catalog values, protected standards data, rule-pack checks, load-case algebra,
or professional/code-compliance claims.

### 5.4 Nonlinear supports

Nonlinear supports shall use an active-set or equivalent iterative method. Results must record active/inactive states, gaps, lift-off, friction state, convergence tolerance, iteration count, and non-convergence warnings.

The nonlinear support active-set slice is `core/solver/nonlinear_supports`. It
models one-way supports, gap/lift-off/contact state, and friction stick/slip
classification as explicit mechanics-boundary decisions driven by supplied
trial displacement and reaction data. It records active-set changes, residual
state, convergence, iteration limits, and nonconvergence diagnostics through
the solver diagnostics contract.

This slice does not assemble the global nonlinear system, select a sparse
solver, define release-quality tolerance policy, perform load-case algebra,
provide public support/catalog defaults, run rule-pack checks, or make
professional/code-compliance claims. Sign conventions for activation and gap
closure remain explicit model/solver inputs rather than hidden defaults; final
support coordinate convention and production solve integration remain `TBD`.

### 5.5 Numerical quality

Solver results must be deterministic for the same model, units, solver version, and rule-pack inputs. Sparse-solver settings, tolerances, and conditioning warnings must be reportable.

The solver diagnostics slice is `core/solver/diagnostics`. It maps
frame-kernel failures into deterministic solver diagnostic records for singular
systems, invalid restraints, invalid topology, invalid numeric inputs,
conditioning warnings/failures, nonconvergence, and explicit sparse-solver or
tolerance-policy `TBD` states. These diagnostics are mechanics-solver findings
only; they do not perform rule-pack checks, professional review, certification,
sealing, or code-compliance assessment.

The sparse-solver performance harness slice is
`core/solver/performance_harness`. It records deterministic repeat-run evidence
for invented/public frame fixtures at the current solve boundary, including
matrix size, nonzero-count, residual, repeatability, provenance, assumptions,
limitations, and conditioning diagnostics. It is an observer/regression surface:
it does not modify solver logic, choose the production sparse numerical
library, set release timing or memory thresholds, import proprietary benchmark
models, or make professional/code-compliance claims.

The straight-pipe element slice is `core/solver/straight_pipe`. It adapts
explicit section properties into the frame-kernel element boundary, exposes
mass/weight-per-length hooks for later load-case work, and recovers local
mechanical element forces from element or model displacement vectors. Inputs
must come from governed unit/provenance/section-property paths; the slice does
not provide public pipe tables, material defaults, code-specific values, stress
checks, load combinations, or professional/code-compliance claims.

## 6. Loads and stress recovery

Primitive loads include weight, pressure, temperature, imposed displacement, hydrotest, wind, seismic, and user occasional loads. Code-specific load combinations are not public defaults; they are user rule-pack or project inputs.

The primitive load case slice is `core/loads/primitive_loads`. It records
weight, pressure, thermal, imposed-displacement, hydrotest, wind, seismic, and
occasional primitive loads as explicit mechanics inputs with dimension intent,
target references, and deterministic findings for missing or invalid
solve-required data. It prepares nodal, element-uniform, and
imposed-displacement contributions for later load-case algebra, stress
recovery, GUI, and headless execution consumers.

This slice does not define code-specific load combinations, wind/seismic design
procedures, pressure stress formulas, public catalog defaults, protected
standards data, rule-pack checks, result envelopes, or professional/code
compliance claims. Wind and seismic loads are currently represented only as
user-supplied equivalent mechanics loads; dynamic or code-procedure generation
remains `TBD`.

The load-case algebra slice is `core/loads/load_case_algebra`. It combines
explicit primitive-load or result-state operands using user-defined factors,
subtraction, and range envelopes while preserving unit/dimension intent and
analysis-status boundaries. Missing operands, incompatible dimensions,
non-finite factors, duplicate operands, unsupported expression shapes, and
human-approval status are deterministic findings, not silent defaults. This
slice does not provide code-specific public combination defaults, a general
rule-pack expression evaluator, stress recovery, protected standards data, or
professional/code-compliance claims.

The stress recovery slice is `core/loads/stress_recovery`. It recovers
code-neutral mechanics components from explicit element force resultants,
section properties, and optional pressure basis inputs: axial normal stress,
bending normal stress components, torsional shear stress, and thin-wall
pressure membrane components. Missing resultants, missing section or pressure
inputs, non-finite values, non-positive properties, incomplete mechanics
status, and human-approval status are deterministic findings, not silent
defaults. This slice does not provide design-code stress equations, allowables,
stress indices, SIF/flexibility tables, protected standards content, public
pipe tables, unit conversion constants, rule-pack checks, result export, or
professional/code-compliance claims.

Stress recovery shall calculate open mechanics quantities such as axial stress, bending stress, torsional shear stress, pressure membrane stresses, and resultants. Code-category equations are rule-pack mappings.

## 7. Rule-pack evaluator

Rule packs are private or user-owned design-basis artifacts. The public project ships schemas and invented examples only.

The rule-pack artifact contract is represented by
`schemas/rule_pack.schema.yaml`. It defines structure for:

- rule-pack identity, schema version, rule-pack version, lifecycle/status, and
  source notices;
- public/private classification, redistribution status, protected-content
  review requirement, and public repository policy;
- required input declarations with unit/dimensional intent, provenance
  requirements, completeness status, and missing-value diagnostics;
- declarative formula slots that are data records only, with arbitrary code
  execution disallowed and protected text, tables, or copied formulas excluded;
- user-supplied value slots for allowables, limits, coefficients, thresholds,
  and source references, with provenance, redistribution, review, and
  completeness status;
- user-rule check definitions that may produce `RULE_INPUTS_INCOMPLETE`,
  `USER_RULE_CHECKED`, `USER_RULE_FAILED`, and `HUMAN_REVIEW_REQUIRED`
  statuses, but not human approval or code-compliance statuses;
- checksum metadata using the accepted canonical JSON/JCS-compatible basis for
  JSON rule-pack payloads;
- diagnostics for missing inputs, unit mismatch, provenance gaps,
  redistribution gaps, suspected protected content, evaluator errors, and
  incomplete data;
- professional-boundary flags preventing software-generated certification,
  sealing, compliance, or human-acceptance records.

The schema is a contract for downstream evaluator, completeness-check,
private-lifecycle, GUI-editor, report, and documentation work. It does not
select the expression grammar, implement evaluation, create public example rule
packs, define private storage, choose the checksum library, or integrate final
result envelopes. Those details remain `TBD` until assigned to later
deliverables.

`core/rules/expression_evaluator` implements the first bounded evaluator
surface. It evaluates explicit expression trees only; it does not parse
arbitrary text or use host-language `eval`. The supported surface is deliberately
small: numeric literals, declared variable references, unary negation, basic
arithmetic, and same-dimension comparisons. Multiplication and division are
bounded to dimensionless scaling and same-dimension ratios until a later unit
algebra decision extends them.

`core/rules/rule_pack_lifecycle` implements private rule-pack lifecycle and
checksum evidence. It records rule-pack identity, version, privacy class,
redistribution status, review state, source notice, protected-content review
state, SHA-256 checksum records, JCS-compatible canonicalization metadata for
caller-supplied canonical JSON payload bytes, and audit-manifest references.
It emits deterministic findings for missing source notices, missing or unknown
redistribution state, pending review state, missing or stale checksums,
suspected protected content, attempted public export of private content, and
professional-boundary violations. It does not parse/canonicalize JSON, store
private rule-pack payloads, choose private storage paths, implement encryption
or access control, handle secrets, run GUI/report/API workflows, evaluate rule
expressions, or make professional/code-compliance claims.

`core/rules/completeness_checker` implements required-input completeness
checking for user-owned rule packs. It compares declarative required-input
records against caller-supplied input evidence for values, units, dimensions,
provenance, redistribution status, and review status. Blocking findings map to
`RULE_INPUTS_INCOMPLETE` and `RULE_CHECK_BLOCKING` semantics while preserving
the distinction between mechanics solve status and user-rule-check readiness.
It does not evaluate formulas, parse rule-pack files, store private data,
provide code-specific values, import protected standards content, or make
professional/code-compliance claims.

The evaluator reports deterministic findings for unsafe host-access requests,
unsupported expression forms, missing or duplicate bindings, invalid
references, missing required values, non-finite inputs, division by zero,
dimension mismatches, type mismatches, and human-approval status boundary
violations. It preserves the separation among mechanics solved, rule inputs
incomplete, user-rule checked, user-rule failed, and human review required. It
does not emit certification, sealing, authentication, human approval, or
code-compliance statuses.

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

`core/reporting/audit_manifest` implements the bounded audit-manifest and model
hash surface. It hashes structured canonical JSON values using sorted object
keys, records non-JSON asset hashes as separate manifest entries, captures
solver version stamps, unit-system references, rule-pack checksum references,
privacy/redaction metadata, and deterministic findings for missing manifest
evidence. It does not parse project files, choose a physical project container,
store private rule-pack payloads, import protected standards content,
authenticate engineering work, or make professional/code-compliance claims.

Analysis-status envelopes must preserve the project authority boundary:
software may emit `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`,
`RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, and
`HUMAN_REVIEW_REQUIRED`, but it must not emit `HUMAN_APPROVED_FOR_PROJECT` or
any code-compliance, certification, sealing, approval, authentication, or
professional-reliance equivalent as an automatic status. Any human acceptance
record is external, human-actor-owned, and bound to reviewed payload hashes.

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
