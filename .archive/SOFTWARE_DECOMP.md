---
doc_id: OPS-SOFTWARE-DECOMP
package_role: working_surface
doc_kind: decomposition.software
status: draft
created: 2026-04-30
refs:
  - rel: depends_on
    to: OPS-INTENT
  - rel: depends_on
    to: OPS-PRD
  - rel: governed_by
    to: OPS-CONTRACT
---

# SOFTWARE_DECOMP — OpenPipeStress Agent-Executable Software Decomposition

## 1. Intake summary

OpenPipeStress is a proposed free/open-source, code-neutral piping stress analysis platform. It will implement open global piping flexibility mechanics, provide a GUI and reporting workflow, and require users to supply governing code data, material properties, component data, SIFs/flexibility factors, allowables, and rule-pack checks.

This decomposition intentionally separates:

- **open mechanics** from **protected standards/code data**;
- **mechanics solved** from **user-rule checked** from **human professional approval**;
- **global centerline analysis** from **specialized local FEA handoff**.

## 2. References

- `INTENT.md` — project intent and governing philosophy.
- `PRD.md` — product requirements and release strategy.
- `MWK 1956.pdf` — historical/foundational technical reference for classical piping flexibility analysis.
- `AGENT_SOFTWARE_DECOMP.md` — decomposition method and required output structure.
- `PROFESSIONAL_ENGINEERING.md` and `CHIRALITY_FRAMEWORK.md` — professional accountability and epistemic governance lens.

## 3. Vocabulary map

|CanonicalTerm|Synonyms|Notes|
|---|---|---|
|OpenPipeStress|Open Pipe Stress, FOSS piping stress tool|Working product name; final name TBD.|
|Rule Pack|code pack, check pack, B31-style rules|User-supplied artifact containing formulas/allowables/checks.|
|Mechanics Solve|analysis solve, pipe stress run|Numerical result only, not compliance.|
|User Rule Check|code check, stress check|Computation under user-supplied rule pack.|
|Protected Standards Data|ASME data, code tables, proprietary tables|Excluded from public repository.|
|Private Library|company library, local database|User-owned materials/components/rules.|
|Centerline Model|beam model, line-element model|Default global piping analysis representation.|
|Local FEA Handoff|solid model export, shell analysis export|Specialized local analysis workflow.|

## 4. Structured Scope of Work (SSOW)

|ScopeItemID|Status|Statement|SourceRef|Notes|
|---|---|---|---|---|
|SOW-001|IN|The project shall produce a free and open-source piping stress analysis platform.|INTENT.md §Core intent; PRD §1, §4|Open-source licensing choice remains TBD.|
|SOW-002|IN|The software shall remain code-neutral: the solver computes mechanics and user-supplied rule packs evaluate acceptability.|INTENT.md §Product identity; PRD §6.1|Do not encode a proprietary code as the default public product.|
|SOW-003|IN|The public repository shall not redistribute protected standards text, tables, figures, examples, material allowables, SIF tables, flexibility-factor tables, or protected dimensional tables.|INTENT.md §Copyright-respecting design principle; PRD §5, §17|Legal review required before accepting any public data contribution.|
|SOW-004|IN|The system shall require users to supply code-specific and project-specific data needed for code checks.|INTENT.md §User-supplied code and data layer; PRD §6.1, §12|Rule-pack completeness must be machine-checkable.|
|SOW-005|IN|The analytical core shall model piping as a 3D centerline/frame system with six degrees of freedom per node.|INTENT.md §Analytical engine intent; PRD §11.1-§11.2|Primary global model is line-element, not routine solid meshing.|
|SOW-006|IN|The solver shall implement straight pipe elements with open mechanics section and stiffness calculations.|PRD §11.3.1|Pipe dimensions come from user data or lawful libraries.|
|SOW-007|IN|The component model shall support bends and elbows with user-entered SIFs, flexibility factors, and bend geometry.|INTENT.md §Analytical engine intent; PRD §11.3.2|Do not bundle B31J-derived tables/equations.|
|SOW-008|IN|The component model shall support branch connections with user-entered reinforcement, SIF, flexibility, and local data fields.|PRD §11.3.3|Branch local checks may require future specialized modules.|
|SOW-009|IN|The component model shall support rigid and semi-rigid valves, flanges, reducers, and specialty items using user-supplied dimensions, weights, and centers of gravity.|PRD §11.3.4-§11.3.5|Public component data must be licensed.|
|SOW-010|IN|The component model shall support expansion joints using manufacturer/user-supplied stiffness, effective area, movement limits, and hardware data.|PRD §11.3.6|No invented expansion joint defaults.|
|SOW-011|IN|The solver shall support anchors, guides, line stops, vertical supports, springs, and imposed displacements.|PRD §11.4|Initial release may limit to linear supports.|
|SOW-012|IN|The solver shall support nonlinear support behavior such as one-way restraints, lift-off, gaps, and friction through a controlled iterative method.|PRD §11.4, §22.4|MVP may defer some nonlinear cases.|
|SOW-013|IN|The analysis engine shall support primitive load cases for weight, pressure, thermal expansion, imposed displacement, hydrotest, wind, seismic, and occasional loads.|PRD §11.5|Dynamic modules may be deferred.|
|SOW-014|IN|The analysis engine shall support unit-aware load-case algebra and user-defined combinations.|PRD §11.6|Code-specific combinations supplied by user rule packs.|
|SOW-015|IN|The stress module shall recover fundamental mechanics stresses from forces, moments, pressure, and section properties.|PRD §11.7|Code stress categories are rule-pack mappings.|
|SOW-016|IN|The product shall provide a private rule-pack schema for user-defined stress checks, allowables, formulas, and pass/fail criteria.|INTENT.md §Rule-pack intent; PRD §12|Public examples must use invented values.|
|SOW-017|IN|The product shall support private material libraries with temperature-dependent properties, allowables, and provenance fields.|PRD §13.1|No public ASME material allowable tables.|
|SOW-018|IN|The product shall support private pipe section and component libraries with provenance and redistribution status.|PRD §13.2-§13.4|Public data requires documented rights.|
|SOW-019|IN|The public project shall accept component data only when provenance and redistribution rights are documented.|PRD §13.4, §17.2|Contributor certification required.|
|SOW-020|IN|The GUI shall provide a 3D centerline modeler with model tree and piping component visualization.|PRD §14.1-§14.2|GUI framework TBD.|
|SOW-021|IN|The GUI shall provide editors for materials, sections, components, load cases, supports, rule packs, and private libraries.|PRD §14.3, §14.5-§14.6|Workflow should surface missing data early.|
|SOW-022|IN|The GUI shall distinguish data required for solving from data required for code checking and shall block/qualify results when required data is missing.|PRD §14.4|No silent engineering defaults.|
|SOW-023|IN|The GUI shall provide results review for displacements, rotations, forces, moments, restraint reactions, equipment loads, stresses, and ratios.|PRD §14.2, §15.1|Stress ratios depend on rule pack completeness.|
|SOW-024|IN|The product shall generate auditable calculation reports including inputs, sources, warnings, assumptions, results, rule-pack checksums, and limitations.|PRD §15|Reports must not reproduce protected standards content.|
|SOW-025|IN|All calculations, schemas, imports, exports, and rule evaluations shall be unit-aware.|PRD §6.6|Unit conversions must be deterministic and testable.|
|SOW-026|IN|The project shall maintain verification benchmarks, regression tests, and numerical quality checks for solver and stress recovery behavior.|PRD §16.2-§16.4|Benchmark sources must be public/original/permissive.|
|SOW-027|IN|The project shall maintain a validation manual that distinguishes mechanics verification from code compliance and professional signoff.|PRD §16.5|Human engineering judgment remains required.|
|SOW-028|IN|The repository shall include contributor governance, IP controls, and review procedures preventing copyrighted standards data from entering the public project.|PRD §17.1-§17.2|Use stop-and-escalate for suspected protected content.|
|SOW-029|IN|The product shall be local-first and shall protect private rule packs, private material data, private component data, and project models.|PRD §18.1, §18.3|Cloud services are out of MVP unless approved.|
|SOW-030|IN|The architecture shall support public APIs, plugins, and import/export adapters for integration with external tools.|PRD §19.3|Specific formats TBD.|
|SOW-031|IN|The product shall support optional handoff of selected local-detail problems to external shell/solid FEA workflows.|INTENT.md §Analytical engine intent; PRD §4.2, §19|Local FEA is not the normal global analysis method.|
|SOW-032|IN|The project shall provide reproducible build, packaging, and CI/CD workflows for supported platforms.|PRD §22|Supported OS/platforms TBD.|
|SOW-033|IN|The project shall provide user/developer documentation and invented-data examples for education and testing.|PRD §7.5, §16.5, §22|No protected examples.|
|SOW-034|IN|The product shall preserve professional responsibility boundaries: agents/software do not certify, approve, seal, or declare engineering code compliance for reliance.|PROFESSIONAL_ENGINEERING.md; CHIRALITY_FRAMEWORK.md; PRD §5, §17.4|Reports may show user-rule pass/fail, not human code compliance.|
|SOW-035|IN|The solver shall be designed for sparse numerical performance and reproducible results on practical piping models.|PRD §11.8, §20|Performance targets TBD.|
|SOW-036|IN|The GUI and reports shall support baseline accessibility and usability appropriate for engineering review.|PRD §21|Detailed WCAG target TBD.|
|SOW-037|IN|Telemetry, if implemented, shall be opt-in, privacy-preserving, and shall not transmit private project/code data.|PRD §18.2|MVP default is no telemetry.|
|SOW-038|IN|The software architecture shall be extensible without allowing plugins/adapters to bypass governance, unit safety, or data-boundary constraints.|PRD §19.1-§19.3|Plugin trust model TBD.|
|SOW-039|IN|The product shall produce reproducibility artifacts such as model hashes, solver version, rule-pack checksum, and input manifest.|PRD §15.3|Hash/canonicalization details TBD.|
|SOW-040|IN|The product shall implement private data handling controls, including redaction/export safeguards where reports or shared models may expose protected/private values.|PRD §18.3|Detailed threat model TBD.|
|SOW-041|IN|The project shall define machine-readable project, model, material, component, load, result, and report schemas.|PRD §19.1, Appendix A|Schema language TBD.|
|SOW-042|IN|Rule packs shall be versioned, source-noted, checksum-addressed, and explicitly marked for redistribution status.|INTENT.md §Rule-pack intent; PRD §12.4|Private rule packs should not be committed publicly.|
|SOW-043|IN|The report system shall prohibit automatic inclusion of protected code text, copied standards tables, or proprietary formulas in generated public reports/templates.|PRD §15.2|User-private report templates remain user responsibility.|
|SOW-044|IN|The product shall provide import mechanisms that record source/provenance/license metadata for component and material data.|PRD §13.5|Importer may reject/flag missing provenance.|
|SOW-045|IN|The rule-pack evaluator shall be sandboxed and unit-aware, not arbitrary executable code.|PRD §12.3|Expression language TBD.|
|SOW-046|IN|The product shall support result exports suitable for review, regression comparison, and downstream tooling.|PRD §15.1, §19.3|Export formats TBD.|
|SOW-047|IN|The product shall clearly distinguish mechanics solved, rule-pack checked, and professionally code-compliant/human-approved states.|INTENT.md §GUI intent; PRD §6.2, §17.4|Human approval is outside software authority.|
|SOW-048|IN|The project shall define open-source license, governance, release, and maintainer policies.|PRD §17.2, §22|License selection TBD.|
|SOW-049|IN|The project shall define criteria for when global beam analysis is sufficient and when local shell/solid analysis handoff is needed.|INTENT.md §Analytical engine intent; PRD §4.2|Criteria are guidance, not automatic certification.|

## 5. Objectives

|ObjectiveID|Statement|Mapped Scope Items|Mapped Deliverables|
|---|---|---|---|
|OBJ-001|Deliver an open, auditable piping stress analysis platform that engineers can inspect and extend.|SOW-001,SOW-002,SOW-024,SOW-033|DEL-01-01, DEL-02-01, DEL-02-02, DEL-02-03, DEL-11-01, DEL-11-02, DEL-11-03, DEL-11-04, DEL-11-05|
|OBJ-002|Protect standards-body and vendor intellectual property by separating open mechanics from user-supplied code/data.|SOW-003,SOW-004,SOW-016,SOW-028,SOW-042,SOW-048|DEL-01-01, DEL-01-02, DEL-01-03, DEL-03-07, DEL-06-03, DEL-06-04, DEL-08-05, DEL-11-02, DEL-11-05|
|OBJ-003|Implement a robust global centerline/frame solver for practical piping flexibility analysis.|SOW-005,SOW-006,SOW-011,SOW-012,SOW-035|DEL-04-01, DEL-04-02, DEL-04-03, DEL-04-04, DEL-04-05, DEL-05-01, DEL-05-02, DEL-05-03, DEL-11-03|
|OBJ-004|Support piping-specific components and private libraries without bundling protected data.|SOW-007,SOW-008,SOW-009,SOW-010,SOW-017,SOW-018,SOW-019,SOW-044|DEL-03-01, DEL-03-02, DEL-03-03, DEL-03-04, DEL-03-05, DEL-03-06, DEL-03-07|
|OBJ-005|Provide user-defined rule packs that evaluate solver results against user-owned design bases.|SOW-004,SOW-016,SOW-042,SOW-045,SOW-047|DEL-05-02, DEL-05-04, DEL-06-01, DEL-06-02, DEL-06-03, DEL-06-04, DEL-06-05|
|OBJ-006|Provide a GUI workflow that makes model creation, missing data, results, and assumptions visible.|SOW-020,SOW-021,SOW-022,SOW-023,SOW-036|DEL-07-01, DEL-07-02, DEL-07-03, DEL-07-04, DEL-07-05, DEL-07-06|
|OBJ-007|Generate reproducible reports and result exports suitable for professional review.|SOW-024,SOW-039,SOW-043,SOW-046|DEL-07-05, DEL-08-01, DEL-08-02, DEL-08-03, DEL-08-04, DEL-08-05|
|OBJ-008|Maintain rigorous verification, validation, regression testing, and release gates.|SOW-026,SOW-027,SOW-032|DEL-04-05, DEL-09-01, DEL-09-02, DEL-09-03, DEL-09-04, DEL-09-05, DEL-10-04, DEL-11-04|
|OBJ-009|Enable interoperability and extensibility while preserving governance boundaries.|SOW-030,SOW-031,SOW-038,SOW-049|DEL-02-04, DEL-08-04, DEL-10-01, DEL-10-02, DEL-10-03, DEL-10-04|
|OBJ-010|Protect private project, code, rule-pack, and component data in local-first workflows.|SOW-029,SOW-037,SOW-040|DEL-12-01, DEL-12-02, DEL-12-03, DEL-12-04, DEL-12-05|
|OBJ-011|Preserve professional responsibility: software assists analysis but does not authenticate engineering work.|SOW-034,SOW-047|DEL-01-04, DEL-02-03, DEL-05-04, DEL-06-05, DEL-07-04, DEL-08-03, DEL-09-04, DEL-11-01|

## 6. Packages

|PackageID|Name|Scope Description|Assigned Scope Items|Exclusions|
|---|---|---|---|---|
|PKG-01|Governance, IP Boundary, and Professional Responsibility|Defines the product’s legal, governance, contribution, and professional-boundary rules.|SOW-001,SOW-003,SOW-028,SOW-034,SOW-048|Does not implement solver or GUI behavior except through requirements and policies.|
|PKG-02|Domain Model, Units, and Core Schemas|Defines the canonical software entities, unit system, serialization contracts, and extensibility boundaries.|SOW-002,SOW-025,SOW-038,SOW-041|Does not implement numerical solving or GUI views.|
|PKG-03|Piping Components, Materials, and Library Data Model|Defines material/component/section/library models and public/private data governance at the data-object level.|SOW-007,SOW-008,SOW-009,SOW-010,SOW-017,SOW-018,SOW-019,SOW-044|Does not implement the rule evaluator or global solver.|
|PKG-04|Solver Core and Numerical Methods|Implements global 3D centerline/frame mechanics, straight pipe behavior, supports, nonlinear support logic, and performance harnesses.|SOW-005,SOW-006,SOW-011,SOW-012,SOW-035|Does not decide code compliance; produces mechanical results.|
|PKG-05|Loads, Load Cases, and Stress Recovery|Implements primitive loads, load-case algebra, mechanical stress recovery, and analysis-status semantics.|SOW-013,SOW-014,SOW-015,SOW-047|Does not contain proprietary code load combinations or allowables.|
|PKG-06|Rule Packs and User-Supplied Code Check Engine|Implements the schema, sandboxed evaluator, required-input checks, and private lifecycle for rule packs.|SOW-004,SOW-016,SOW-042,SOW-045|Does not ship ASME or other proprietary rule content.|
|PKG-07|Graphical User Interface and Engineering Workflow|Implements the interactive modeler, editors, warning UX, and results views.|SOW-020,SOW-021,SOW-022,SOW-023,SOW-036|Does not silently supply missing code data.|
|PKG-08|Reporting, Audit, and Reproducibility|Implements calculation reports, audit manifests, hashes, report-content guardrails, and exports.|SOW-024,SOW-039,SOW-043,SOW-046|Does not authenticate or certify engineering work.|
|PKG-09|Verification, Validation, and Quality Oracles|Implements mechanics benchmarks, regression suites, validation manual structure, and release quality gates.|SOW-026,SOW-027|Does not replace professional review for project-specific reliance.|
|PKG-10|Build, Packaging, API, and Interoperability|Implements public API/plugin boundaries, import/export adapters, local FEA handoff contracts, and release packaging.|SOW-030,SOW-031,SOW-032,SOW-049|Does not embed external proprietary tool behavior.|
|PKG-11|Documentation, Examples, and Education|Produces user/developer guides, theory notes, tutorials, and invented-data example models.|SOW-033|Does not publish protected standards examples or commercial software examples.|
|PKG-12|Security, Privacy, and Private Data Handling|Implements local-first storage policies, redaction/export controls, telemetry constraints, and threat modeling.|SOW-029,SOW-037,SOW-040|Does not operate as a cloud service unless separately authorized.|

## 7. Deliverables

### PKG-01 — Governance, IP Boundary, and Professional Responsibility

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-01-01|Project governance baseline|DOC_UPDATE|SOW-001,SOW-048|OBJ-001,OBJ-002|M|Single governance surface; policy language only.|
|DEL-01-02|Copyright and protected-data boundary policy|DOC_UPDATE|SOW-003,SOW-028|OBJ-002|M|Requires careful wording and legal-review flags.|
|DEL-01-03|Contributor certification workflow|DOC_UPDATE|SOW-028,SOW-048|OBJ-002|S|Bounded policy artifact.|
|DEL-01-04|Professional responsibility and product-claims policy|DOC_UPDATE|SOW-034|OBJ-011|S|Policy wording; no code.|
### PKG-02 — Domain Model, Units, and Core Schemas

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-02-01|Canonical domain model schema|DATA_MODEL_CHANGE|SOW-041|OBJ-001|M|Core schema touches many entity definitions but one domain.|
|DEL-02-02|Unit system and dimensional-analysis core contract|BACKEND_FEATURE_SLICE|SOW-025|OBJ-001|M|Foundation for solver and rule engine.|
|DEL-02-03|Code-neutral analysis boundary model|DATA_MODEL_CHANGE|SOW-002|OBJ-001,OBJ-011|S|Important semantic boundary; limited implementation surface.|
|DEL-02-04|Plugin and extension domain contracts|API_CONTRACT|SOW-038|OBJ-009|M|Cross-cutting but kept to domain/API definitions.|
### PKG-03 — Piping Components, Materials, and Library Data Model

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-03-01|Material library schema with provenance|DATA_MODEL_CHANGE|SOW-017|OBJ-004|M|Schema plus tests; no proprietary data.|
|DEL-03-02|Pipe section and component library schema|DATA_MODEL_CHANGE|SOW-018|OBJ-004|M|Library schema only; no protected tables.|
|DEL-03-03|Bend and elbow component model fields|BACKEND_FEATURE_SLICE|SOW-007|OBJ-004|S|Single component family.|
|DEL-03-04|Branch connection component model fields|BACKEND_FEATURE_SLICE|SOW-008|OBJ-004|M|Component model has more local fields but remains bounded.|
|DEL-03-05|Rigid component models for valves, flanges, reducers, and specialty items|BACKEND_FEATURE_SLICE|SOW-009|OBJ-004|M|Multiple related rigid components; no data tables.|
|DEL-03-06|Expansion joint component model|BACKEND_FEATURE_SLICE|SOW-010|OBJ-004|M|Specialized but bounded component type.|
|DEL-03-07|Public/private library import provenance checker|BACKEND_FEATURE_SLICE|SOW-019,SOW-044|OBJ-002,OBJ-004|M|Connects schema with governance; no external import formats yet.|
### PKG-04 — Solver Core and Numerical Methods

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-04-01|3D frame stiffness kernel|BACKEND_FEATURE_SLICE|SOW-005,SOW-035|OBJ-003|L|Central solver kernel; large but single domain and foundational.|
|DEL-04-02|Straight pipe element|BACKEND_FEATURE_SLICE|SOW-006|OBJ-003|M|Single element type.|
|DEL-04-03|Linear support and restraint models|BACKEND_FEATURE_SLICE|SOW-011|OBJ-003|M|Support families are related and can share validation.|
|DEL-04-04|Nonlinear support active-set solver|BACKEND_FEATURE_SLICE|SOW-012|OBJ-003|L|Complex; remains one numerical domain.|
|DEL-04-05|Sparse solver performance harness|TEST_SUITE|SOW-035|OBJ-003,OBJ-008|M|Performance harness separate from solver logic.|
### PKG-05 — Loads, Load Cases, and Stress Recovery

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-05-01|Primitive load case engine|BACKEND_FEATURE_SLICE|SOW-013|OBJ-003|M|Primitive categories; code combinations remain external.|
|DEL-05-02|Load-case algebra engine|BACKEND_FEATURE_SLICE|SOW-014|OBJ-003,OBJ-005|M|Bounded algebra surface.|
|DEL-05-03|Fundamental stress recovery module|BACKEND_FEATURE_SLICE|SOW-015|OBJ-003|M|Does not encode code stress equations.|
|DEL-05-04|Analysis status semantics|DATA_MODEL_CHANGE|SOW-047|OBJ-005,OBJ-011|S|Small state model with high governance value.|
### PKG-06 — Rule Packs and User-Supplied Code Check Engine

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-06-01|Rule-pack schema|DATA_MODEL_CHANGE|SOW-016,SOW-042|OBJ-005|M|Schema and examples only.|
|DEL-06-02|Sandboxed unit-aware expression evaluator|BACKEND_FEATURE_SLICE|SOW-045|OBJ-005|L|Security-sensitive and numerically important.|
|DEL-06-03|Required-input completeness checker|BACKEND_FEATURE_SLICE|SOW-004|OBJ-002,OBJ-005|M|Connects user data to rule-pack gating.|
|DEL-06-04|Private rule-pack lifecycle and checksum handling|BACKEND_FEATURE_SLICE|SOW-042|OBJ-002,OBJ-005|M|Local lifecycle only; access control details defer to PKG-12.|
|DEL-06-05|Invented non-code example rule pack|DOC_UPDATE|SOW-016|OBJ-005,OBJ-011|S|Example only; no proprietary content.|
### PKG-07 — Graphical User Interface and Engineering Workflow

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-07-01|3D viewport and centerline editor|UX_UI_SLICE|SOW-020|OBJ-006|L|GUI surface is broad but bounded to viewport/editor.|
|DEL-07-02|Model tree and property inspector|UX_UI_SLICE|SOW-020,SOW-021|OBJ-006|M|Single UI work surface.|
|DEL-07-03|Material, component, and rule-pack editors|UX_UI_SLICE|SOW-021|OBJ-006|L|Multiple editors but same GUI domain; may split later.|
|DEL-07-04|Missing-data warning and blocking UX|UX_UI_SLICE|SOW-022|OBJ-006,OBJ-011|M|High-value UX slice.|
|DEL-07-05|Results viewer|UX_UI_SLICE|SOW-023|OBJ-006,OBJ-007|L|Results surface is substantial but one domain.|
|DEL-07-06|Accessibility and usability baseline|UX_UI_SLICE|SOW-036|OBJ-006|M|May refine once GUI framework chosen.|
### PKG-08 — Reporting, Audit, and Reproducibility

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-08-01|Calculation report generator|BACKEND_FEATURE_SLICE|SOW-024|OBJ-007|L|Report is broad but single artifact family.|
|DEL-08-02|Audit manifest and model hash|BACKEND_FEATURE_SLICE|SOW-039|OBJ-007|M|Canonicalization TBD but bounded.|
|DEL-08-03|Warnings, assumptions, and provenance report section|BACKEND_FEATURE_SLICE|SOW-024|OBJ-007,OBJ-011|M|Leverages warning data from GUI/core.|
|DEL-08-04|Result export format|API_CONTRACT|SOW-046|OBJ-007,OBJ-009|M|Format TBD but bounded.|
|DEL-08-05|Report protected-content linter|TEST_SUITE|SOW-043|OBJ-002,OBJ-007|M|Heuristic plus review; cannot be sole legal control.|
### PKG-09 — Verification, Validation, and Quality Oracles

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-09-01|Mechanics benchmark suite|TEST_SUITE|SOW-026|OBJ-008|M|Uses original/public mechanics cases.|
|DEL-09-02|Stress recovery benchmark suite|TEST_SUITE|SOW-026|OBJ-008|M|No code formulas.|
|DEL-09-03|Nonlinear support regression suite|TEST_SUITE|SOW-026|OBJ-008|M|Depends on nonlinear solver maturity.|
|DEL-09-04|Validation manual skeleton|DOC_UPDATE|SOW-027|OBJ-008,OBJ-011|M|Documentation-oriented.|
|DEL-09-05|Release quality gate checklist|CI_CD_CHANGE|SOW-026,SOW-027|OBJ-008|M|Process + CI; bounded.|
### PKG-10 — Build, Packaging, API, and Interoperability

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-10-01|Public API and plugin boundary|API_CONTRACT|SOW-030|OBJ-009|M|API details TBD.|
|DEL-10-02|Import/export adapter framework|BACKEND_FEATURE_SLICE|SOW-030|OBJ-009|L|Actual external formats can be later deliverables.|
|DEL-10-03|Local FEA handoff data contract|API_CONTRACT|SOW-031,SOW-049|OBJ-009|M|Guidance only, no external FEA implementation.|
|DEL-10-04|Build, packaging, and CI/CD pipeline|CI_CD_CHANGE|SOW-032|OBJ-008,OBJ-009|L|Platform list TBD.|
### PKG-11 — Documentation, Examples, and Education

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-11-01|User guide skeleton|DOC_UPDATE|SOW-033|OBJ-001,OBJ-011|M|Documentation only.|
|DEL-11-02|Developer guide for solver and rule packs|DOC_UPDATE|SOW-033|OBJ-001,OBJ-002|M|Documentation only.|
|DEL-11-03|Theory notes: classical to modern centerline analysis|DOC_UPDATE|SOW-033|OBJ-001,OBJ-003|M|Should cite public/permissive sources; no formula copying from standards.|
|DEL-11-04|Invented educational example models|DOC_UPDATE|SOW-033|OBJ-001,OBJ-008|M|Examples must be clearly non-code.|
|DEL-11-05|Contributor tutorial and onboarding|DOC_UPDATE|SOW-033|OBJ-001,OBJ-002|S|Documentation only.|
### PKG-12 — Security, Privacy, and Private Data Handling

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-12-01|Local-first storage and private data paths|SECURITY_CONTROL|SOW-029|OBJ-010|M|Implementation depends on chosen stack.|
|DEL-12-02|Private data redaction and export controls|SECURITY_CONTROL|SOW-040|OBJ-010|M|Security-sensitive but bounded.|
|DEL-12-03|Telemetry off-by-default design|SECURITY_CONTROL|SOW-037|OBJ-010|S|May be no-op in MVP.|
|DEL-12-04|Secret and private-library handling|SECURITY_CONTROL|SOW-040,SOW-029|OBJ-010|M|Avoids cloud assumptions.|
|DEL-12-05|Security threat model|DOC_UPDATE|SOW-040|OBJ-010|M|Document now; update as architecture matures.|

## 8. Scope ledger summary

The authoritative companion register is `_Registers/ScopeLedger.csv`.

|ScopeItemID|Status|Statement|SourceRef|PackageID|DeliverableID(s)|ObjectiveID(s)|OpenIssue|Notes|
|---|---|---|---|---|---|---|---|---|
|SOW-001|IN|The project shall produce a free and open-source piping stress analysis platform.|INTENT.md §Core intent; PRD §1, §4|PKG-01|DEL-01-01|OBJ-001|FALSE|Open-source licensing choice remains TBD.|
|SOW-002|IN|The software shall remain code-neutral: the solver computes mechanics and user-supplied rule packs evaluate acceptability.|INTENT.md §Product identity; PRD §6.1|PKG-02|DEL-02-03|OBJ-001|FALSE|Do not encode a proprietary code as the default public product.|
|SOW-003|IN|The public repository shall not redistribute protected standards text, tables, figures, examples, material allowables, SIF tables, flexibility-factor tables, or protected dimensional tables.|INTENT.md §Copyright-respecting design principle; PRD §5, §17|PKG-01|DEL-01-02|OBJ-002|FALSE|Legal review required before accepting any public data contribution.|
|SOW-004|IN|The system shall require users to supply code-specific and project-specific data needed for code checks.|INTENT.md §User-supplied code and data layer; PRD §6.1, §12|PKG-06|DEL-06-03|OBJ-002, OBJ-005|FALSE|Rule-pack completeness must be machine-checkable.|
|SOW-005|IN|The analytical core shall model piping as a 3D centerline/frame system with six degrees of freedom per node.|INTENT.md §Analytical engine intent; PRD §11.1-§11.2|PKG-04|DEL-04-01|OBJ-003|FALSE|Primary global model is line-element, not routine solid meshing.|
|SOW-006|IN|The solver shall implement straight pipe elements with open mechanics section and stiffness calculations.|PRD §11.3.1|PKG-04|DEL-04-02|OBJ-003|FALSE|Pipe dimensions come from user data or lawful libraries.|
|SOW-007|IN|The component model shall support bends and elbows with user-entered SIFs, flexibility factors, and bend geometry.|INTENT.md §Analytical engine intent; PRD §11.3.2|PKG-03|DEL-03-03|OBJ-004|FALSE|Do not bundle B31J-derived tables/equations.|
|SOW-008|IN|The component model shall support branch connections with user-entered reinforcement, SIF, flexibility, and local data fields.|PRD §11.3.3|PKG-03|DEL-03-04|OBJ-004|FALSE|Branch local checks may require future specialized modules.|
|SOW-009|IN|The component model shall support rigid and semi-rigid valves, flanges, reducers, and specialty items using user-supplied dimensions, weights, and centers of gravity.|PRD §11.3.4-§11.3.5|PKG-03|DEL-03-05|OBJ-004|FALSE|Public component data must be licensed.|
|SOW-010|IN|The component model shall support expansion joints using manufacturer/user-supplied stiffness, effective area, movement limits, and hardware data.|PRD §11.3.6|PKG-03|DEL-03-06|OBJ-004|FALSE|No invented expansion joint defaults.|
|SOW-011|IN|The solver shall support anchors, guides, line stops, vertical supports, springs, and imposed displacements.|PRD §11.4|PKG-04|DEL-04-03|OBJ-003|FALSE|Initial release may limit to linear supports.|
|SOW-012|IN|The solver shall support nonlinear support behavior such as one-way restraints, lift-off, gaps, and friction through a controlled iterative method.|PRD §11.4, §22.4|PKG-04|DEL-04-04|OBJ-003|FALSE|MVP may defer some nonlinear cases.|
|SOW-013|IN|The analysis engine shall support primitive load cases for weight, pressure, thermal expansion, imposed displacement, hydrotest, wind, seismic, and occasional loads.|PRD §11.5|PKG-05|DEL-05-01|TBD|FALSE|Dynamic modules may be deferred.|
|SOW-014|IN|The analysis engine shall support unit-aware load-case algebra and user-defined combinations.|PRD §11.6|PKG-05|DEL-05-02|TBD|FALSE|Code-specific combinations supplied by user rule packs.|
|SOW-015|IN|The stress module shall recover fundamental mechanics stresses from forces, moments, pressure, and section properties.|PRD §11.7|PKG-05|DEL-05-03|TBD|FALSE|Code stress categories are rule-pack mappings.|
|SOW-016|IN|The product shall provide a private rule-pack schema for user-defined stress checks, allowables, formulas, and pass/fail criteria.|INTENT.md §Rule-pack intent; PRD §12|PKG-06|DEL-06-01, DEL-06-05|OBJ-002, OBJ-005|FALSE|Public examples must use invented values.|
|SOW-017|IN|The product shall support private material libraries with temperature-dependent properties, allowables, and provenance fields.|PRD §13.1|PKG-03|DEL-03-01|OBJ-004|FALSE|No public ASME material allowable tables.|
|SOW-018|IN|The product shall support private pipe section and component libraries with provenance and redistribution status.|PRD §13.2-§13.4|PKG-03|DEL-03-02|OBJ-004|FALSE|Public data requires documented rights.|
|SOW-019|IN|The public project shall accept component data only when provenance and redistribution rights are documented.|PRD §13.4, §17.2|PKG-03|DEL-03-07|OBJ-004|FALSE|Contributor certification required.|
|SOW-020|IN|The GUI shall provide a 3D centerline modeler with model tree and piping component visualization.|PRD §14.1-§14.2|PKG-07|DEL-07-01, DEL-07-02|OBJ-006|FALSE|GUI framework TBD.|
|SOW-021|IN|The GUI shall provide editors for materials, sections, components, load cases, supports, rule packs, and private libraries.|PRD §14.3, §14.5-§14.6|PKG-07|DEL-07-02, DEL-07-03|OBJ-006|FALSE|Workflow should surface missing data early.|
|SOW-022|IN|The GUI shall distinguish data required for solving from data required for code checking and shall block/qualify results when required data is missing.|PRD §14.4|PKG-07|DEL-07-04|OBJ-006|FALSE|No silent engineering defaults.|
|SOW-023|IN|The GUI shall provide results review for displacements, rotations, forces, moments, restraint reactions, equipment loads, stresses, and ratios.|PRD §14.2, §15.1|PKG-07|DEL-07-05|OBJ-006|FALSE|Stress ratios depend on rule pack completeness.|
|SOW-024|IN|The product shall generate auditable calculation reports including inputs, sources, warnings, assumptions, results, rule-pack checksums, and limitations.|PRD §15|PKG-08|DEL-08-01, DEL-08-03|OBJ-001, OBJ-007|FALSE|Reports must not reproduce protected standards content.|
|SOW-025|IN|All calculations, schemas, imports, exports, and rule evaluations shall be unit-aware.|PRD §6.6|PKG-02|DEL-02-02|TBD|FALSE|Unit conversions must be deterministic and testable.|
|SOW-026|IN|The project shall maintain verification benchmarks, regression tests, and numerical quality checks for solver and stress recovery behavior.|PRD §16.2-§16.4|PKG-09|DEL-09-01, DEL-09-02, DEL-09-03, DEL-09-05|OBJ-008|FALSE|Benchmark sources must be public/original/permissive.|
|SOW-027|IN|The project shall maintain a validation manual that distinguishes mechanics verification from code compliance and professional signoff.|PRD §16.5|PKG-09|DEL-09-04, DEL-09-05|OBJ-008|FALSE|Human engineering judgment remains required.|
|SOW-028|IN|The repository shall include contributor governance, IP controls, and review procedures preventing copyrighted standards data from entering the public project.|PRD §17.1-§17.2|PKG-01|DEL-01-02, DEL-01-03|OBJ-002|FALSE|Use stop-and-escalate for suspected protected content.|
|SOW-029|IN|The product shall be local-first and shall protect private rule packs, private material data, private component data, and project models.|PRD §18.1, §18.3|PKG-12|DEL-12-01, DEL-12-04|OBJ-010|FALSE|Cloud services are out of MVP unless approved.|
|SOW-030|IN|The architecture shall support public APIs, plugins, and import/export adapters for integration with external tools.|PRD §19.3|PKG-10|DEL-10-01, DEL-10-02|OBJ-009|FALSE|Specific formats TBD.|
|SOW-031|IN|The product shall support optional handoff of selected local-detail problems to external shell/solid FEA workflows.|INTENT.md §Analytical engine intent; PRD §4.2, §19|PKG-10|DEL-10-03|OBJ-009|FALSE|Local FEA is not the normal global analysis method.|
|SOW-032|IN|The project shall provide reproducible build, packaging, and CI/CD workflows for supported platforms.|PRD §22|PKG-10|DEL-10-04|OBJ-008|FALSE|Supported OS/platforms TBD.|
|SOW-033|IN|The project shall provide user/developer documentation and invented-data examples for education and testing.|PRD §7.5, §16.5, §22|PKG-11|DEL-11-01, DEL-11-02, DEL-11-03, DEL-11-04, DEL-11-05|OBJ-001|FALSE|No protected examples.|
|SOW-034|IN|The product shall preserve professional responsibility boundaries: agents/software do not certify, approve, seal, or declare engineering code compliance for reliance.|PROFESSIONAL_ENGINEERING.md; CHIRALITY_FRAMEWORK.md; PRD §5, §17.4|PKG-01|DEL-01-04|OBJ-011|FALSE|Reports may show user-rule pass/fail, not human code compliance.|
|SOW-035|IN|The solver shall be designed for sparse numerical performance and reproducible results on practical piping models.|PRD §11.8, §20|PKG-04|DEL-04-01, DEL-04-05|OBJ-003|FALSE|Performance targets TBD.|
|SOW-036|IN|The GUI and reports shall support baseline accessibility and usability appropriate for engineering review.|PRD §21|PKG-07|DEL-07-06|OBJ-006|FALSE|Detailed WCAG target TBD.|
|SOW-037|IN|Telemetry, if implemented, shall be opt-in, privacy-preserving, and shall not transmit private project/code data.|PRD §18.2|PKG-12|DEL-12-03|OBJ-010|FALSE|MVP default is no telemetry.|
|SOW-038|IN|The software architecture shall be extensible without allowing plugins/adapters to bypass governance, unit safety, or data-boundary constraints.|PRD §19.1-§19.3|PKG-02|DEL-02-04|OBJ-009|FALSE|Plugin trust model TBD.|
|SOW-039|IN|The product shall produce reproducibility artifacts such as model hashes, solver version, rule-pack checksum, and input manifest.|PRD §15.3|PKG-08|DEL-08-02|OBJ-007|FALSE|Hash/canonicalization details TBD.|
|SOW-040|IN|The product shall implement private data handling controls, including redaction/export safeguards where reports or shared models may expose protected/private values.|PRD §18.3|PKG-12|DEL-12-02, DEL-12-04, DEL-12-05|OBJ-010|FALSE|Detailed threat model TBD.|
|SOW-041|IN|The project shall define machine-readable project, model, material, component, load, result, and report schemas.|PRD §19.1, Appendix A|PKG-02|DEL-02-01|TBD|FALSE|Schema language TBD.|
|SOW-042|IN|Rule packs shall be versioned, source-noted, checksum-addressed, and explicitly marked for redistribution status.|INTENT.md §Rule-pack intent; PRD §12.4|PKG-06|DEL-06-01, DEL-06-04|OBJ-002, OBJ-005|FALSE|Private rule packs should not be committed publicly.|
|SOW-043|IN|The report system shall prohibit automatic inclusion of protected code text, copied standards tables, or proprietary formulas in generated public reports/templates.|PRD §15.2|PKG-08|DEL-08-05|OBJ-007|FALSE|User-private report templates remain user responsibility.|
|SOW-044|IN|The product shall provide import mechanisms that record source/provenance/license metadata for component and material data.|PRD §13.5|PKG-03|DEL-03-07|OBJ-004|FALSE|Importer may reject/flag missing provenance.|
|SOW-045|IN|The rule-pack evaluator shall be sandboxed and unit-aware, not arbitrary executable code.|PRD §12.3|PKG-06|DEL-06-02|OBJ-005|FALSE|Expression language TBD.|
|SOW-046|IN|The product shall support result exports suitable for review, regression comparison, and downstream tooling.|PRD §15.1, §19.3|PKG-08|DEL-08-04|OBJ-007|FALSE|Export formats TBD.|
|SOW-047|IN|The product shall clearly distinguish mechanics solved, rule-pack checked, and professionally code-compliant/human-approved states.|INTENT.md §GUI intent; PRD §6.2, §17.4|PKG-05|DEL-05-04|OBJ-005, OBJ-011|FALSE|Human approval is outside software authority.|
|SOW-048|IN|The project shall define open-source license, governance, release, and maintainer policies.|PRD §17.2, §22|PKG-01|DEL-01-01, DEL-01-03|OBJ-002|FALSE|License selection TBD.|
|SOW-049|IN|The project shall define criteria for when global beam analysis is sufficient and when local shell/solid analysis handoff is needed.|INTENT.md §Analytical engine intent; PRD §4.2|PKG-10|DEL-10-03|OBJ-009|FALSE|Criteria are guidance, not automatic certification.|

## 9. Coverage and telemetry

|Metric|Value|
|---|---|
|ScopeItemCount|49|
|PackageCount|12|
|DeliverableCount|59|
|ObjectiveCount|11|
|UnassignedScopeItems|0|
|ScopeItemsWithoutDeliverableMapping|0|
|UnmappedObjectives|0|
|ContextEnvelopeCounts|S=8, M=42, L=9, XL=0|
|OpenIssues|8|
|Revision|0.1 Draft — 2026-04-30|

## 10. Open issues

|OpenIssueID|Issue|Related Scope|Owner/Resolution Path|
|---|---|---|---|
|OI-001|License selection is TBD: permissive vs copyleft requires human decision.|SOW-001,SOW-048|Human project authority|
|OI-002|Implementation stack is TBD: language, GUI framework, solver libraries, and packaging targets.|SOW-032,SOW-036|Architecture decision|
|OI-003|Legal review is required for public component/material data policy before accepting community data.|SOW-003,SOW-019,SOW-028|Legal/human review|
|OI-004|Specific supported import/export formats are TBD.|SOW-030,SOW-046|Architecture/product decision|
|OI-005|Numerical tolerances and performance targets are TBD pending solver prototype.|SOW-026,SOW-035|Solver lead/human|
|OI-006|Rule-pack expression language is TBD.|SOW-045|Architecture decision|
|OI-007|Human approval workflow for project-specific calculation reports is TBD and outside solver authority.|SOW-034,SOW-047|Human/project governance|
|OI-008|Telemetry policy defaults to no telemetry; any telemetry design requires explicit human approval.|SOW-037|Security/product decision|

## 11. Decision log

|DecisionID|Decision|Basis|Status|
|---|---|---|---|
|DEC-001|Adopt code-neutral architecture: mechanics in public core, code-specific values in user/private rule packs.|Conversation intent; INTENT.md; PRD §6.1|Accepted as draft design basis; human final acceptance pending.|
|DEC-002|Adopt 3D centerline/frame solver as primary global analysis model.|Conversation intent; MWK 1956 lineage; PRD §11|Accepted as draft design basis; local FEA remains specialized handoff.|
|DEC-003|Exclude protected standards/code data from public examples, libraries, reports, and rule packs.|INTENT.md; PRD §5, §17|Accepted as draft design basis; legal review required.|
|DEC-004|Use Chirality-style flat package/deliverable decomposition with context envelopes.|AGENT_SOFTWARE_DECOMP.md|Accepted for this draft decomposition.|
|DEC-005|Treat generated software results as decision support; human professional responsibility remains non-transferable.|PROFESSIONAL_ENGINEERING.md; CHIRALITY_FRAMEWORK.md|Accepted as professional-boundary principle.|

## 12. Gate posture

This decomposition is complete enough to initialize downstream work, but it is still a **draft candidate** until the human project authority confirms:

1. SSOW scope items are correct.
2. Package boundaries are acceptable and non-overlapping.
3. Deliverable granularity is acceptable.
4. Open issues are accurate.
5. The generated registers may be treated as the current decomposition basis.
