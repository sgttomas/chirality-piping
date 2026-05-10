# Coordination Record

**Active phase:** `Post TP-MAC-08`
**Active objective:** Preserve the closed TP-MAC-08 explicit mechanics-basis
load-combination preview baseline and open a new governed plan only if the
next instance is asked or receives an explicit sealed brief.
**Latest closed plan:** `plans/TP-MAC-08_CODE_NEUTRAL_LOAD_COMBINATION_PREVIEW_PLAN.md`
**Latest next-step assessment:** `execution/_Coordination/TP-MAC-08_NEXT_TRANCHE_ASSESSMENT.md`
**Latest state:** `execution/_Coordination/NEXT_INSTANCE_STATE.md`
**Historical archive:** `execution/_Coordination/_Archive/`

## Current Authority

The current development phase is no longer the `RT-1` evidence archive route,
the `TP-MAC-01` desktop-hardening tranche, an open TP-MAC-03 continuation, an
open TP-MAC-04 continuation, TP-RUN-01, or TP-PER-01. Those routes are
historical or closed baseline context only.

TP-MAC-08 is the latest implemented mechanics authority:

- `plans/TP-MAC-08_CODE_NEUTRAL_LOAD_COMBINATION_PREVIEW_PLAN.md`
- `plans/TP-MAC-07_MIDSPAN_STATION_PREVIEW_PLAN.md`
- `plans/TP-MAC-06_UNIFORM_THERMAL_AXIAL_PREVIEW_PLAN.md`

TP-PER-01 is the latest implemented persistence-spine authority:

- `plans/TP-PER-01_PROJECT_PERSISTENCE_AND_RUN_HISTORY_PLAN.md`

TP-RUN-01 is the latest implemented runtime-spine authority:

- `plans/TP-RUN-01_PREVIEW_RUNTIME_SPINE_PLAN.md`

TP-MAC-05, TP-MAC-04, TP-MAC-03, and TP-MAC-02 are completed mechanics and
desktop baselines:

- `plans/TP-MAC-05_ENDPOINT_STRESS_COMPONENT_RESULTS_PLAN.md`
- `plans/TP-MAC-04_ENDPOINT_RESULT_CONSUMER_AND_END_J_RECOVERY_PLAN.md`
- `plans/TP-MAC-03_RESULT_INTERPRETATION_AND_REVIEW_WORKFLOW_PLAN.md`
- `plans/TP-MAC-02_PHYSICS_FIRST_APPLICATION_PLAN.md`
- `docs/CONTRACT.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `AGENTS.md`

Historical REV05/DAG coordination files remain available in this folder for
audit traceability, but they are not the first context surface for TP-MAC-06
work. Do not reload the full historical coordination stack unless the next
task explicitly asks for DAG/evidence/lifecycle reconciliation.

## Active Product Context

The bounded product physics adapter consumes invented preview data and existing
mechanics crates:

- `core/product_physics/`
- `core/solver/frame_kernel`
- `core/solver/straight_pipe`
- `core/solver/linear_supports`
- `core/loads/primitive_loads`
- `core/loads/stress_recovery`

The current computed result envelope includes:

- displacement magnitudes;
- support reaction resultants;
- local element force and moment components at end-i and end-j with structured
  metadata;
- open-mechanics endpoint stress components at end-i and end-j with structured
  metadata;
- pipe-level open-formula stress summaries where explicit preview inputs
  support them;
- uniform axial thermal equivalent loads and thermal fixed-end correction for
  straight-pipe temperature-change preview loads with explicit invented
  material expansion input;
- deterministic midspan force, moment, and stress rows recovered from
  interpolated endpoint resultants;
- explicit mechanics-basis user load-combination rows with `basis_ref` and
  `source_result_refs`;
- explicit diagnostics for unsupported, incomplete, or review-required paths;
- professional-boundary and non-mutation fields.

TP-MAC-08 added only the planned explicit load-combination preview slice:

- every preview load case is solved independently;
- the first/default load case keeps legacy unqualified result IDs;
- non-default load-case rows use
  `result:loadcase:{load_case_suffix}:{base_result_tail}`;
- explicit mechanics-basis combination rows use
  `result:combination:{combination_suffix}:{base_result_tail}`;
- matching scalar rows are combined through `core/loads/load_case_algebra`
  using explicit user factors only;
- `open_formula_stress_summary` rows are skipped for combinations with
  diagnostics;
- code/rule combinations remain deferred.

TP-MAC-07 added only the planned midspan station preview slice:

- one deterministic `midspan` station per solved straight-pipe preview
  element;
- midspan force, moment, and stress rows with
  `basis: "interpolated_from_endpoint_resultants"`;
- pressure membrane midspan rows only when explicit pressure basis exists;
- endpoint-pair display remains endpoint-only;
- existing endpoint result IDs preserved.

TP-MAC-06 added only the planned uniform axial thermal slice:

- optional preview material input
  `thermal_expansion_coefficient: { value, unit: "1/degC" }`;
- required thermal expansion coefficient only when a thermal load targets a
  pipe using that material;
- existing primitive load shape with `category: "thermal"`, element target,
  `dimension: "temperature_change"`, and unit `degC`;
- thermal load `direction` treated as legacy syntactic input, not a mechanics
  direction control;
- open-mechanics axial thermal strain and equivalent axial load behavior for
  straight-pipe preview elements;
- thermal fixed-end correction applied before endpoint stress recovery;
- existing result ID patterns preserved.

The desktop workflow now includes:

- result grouping by displacement, reaction, force, moment, and stress;
- selectable result rows and diagnostic rows;
- result detail interpretation with metadata, linked diagnostics, linked
  knowledge, DEL-14-02 run context, and professional-boundary messaging;
- endpoint-pair display for selected local endpoint force/moment and stress
  component results;
- selected-result and selected-diagnostic review-only proposal narratives;
- read-only report-packet context including selected result refs and
  analysis-run hashes;
- load-basis refs for emitted load cases and explicit combinations;
- a mechanics gap ledger marking midspan station preview recovery, explicit
  mechanics-basis load combinations, and uniform axial thermal preview behavior
  implemented while broader station, combination, and thermal behavior remain
  deferred.

TP-RUN-01 connected runtime surfaces to this product-physics baseline. The
desktop Tauri command accepts an optional preview model payload while
preserving fixture fallback. The React preview service passes the loaded model
into the desktop runtime when Tauri is available. The headless runner has an
in-memory bridge that executes product physics and returns runner metadata,
deterministic result refs, audit context, and hashes.

TP-PER-01 adds the schema-shaped project persistence baseline:

- `core/project_persistence` builds, validates, hashes, and round-trips
  `openpipestress.project_persistence` envelopes;
- `schemas/project_persistence.schema.yaml` carries optional run-history refs
  or records for model states, analysis runs, result envelopes, result refs,
  and hash manifests;
- `fixtures/persistence/invented_persisted_preview_project.json` is the
  invented persisted-preview fixture with a canonical-model payload;
- physical project container format, desktop save/open UX, final CLI syntax,
  migrations, external storage, and professional acceptance remain deferred.

## Boundaries

The active posture must preserve these constraints:

- invented or cleared public example data only;
- no protected standards tables, owner data, private project data, allowables,
  SIF/flexibility tables, compliance checks, certification, sealing, approval,
  release, or production-readiness claims;
- no autonomous accepted-model mutation from agent proposals;
- unsupported mechanics paths return explicit diagnostics or gap-ledger entries
  instead of hidden defaults;
- professional status remains `NOT_PROVIDED` / human review required;
- pressure primitive loads may support stress recovery context but remain
  unapplied to the frame load vector until a separate governed plan changes
  that behavior;
- thermal loads must not silently fall through the existing uniform
  force-per-length load path;
- midspan station recovery must remain labeled as interpolation from endpoint
  resultants, not exact internal force diagram recovery;
- explicit combination rows must remain mechanics-basis user combinations, not
  code/rule combinations or protected design-basis defaults;
- persistence envelopes remain schema-shaped application-service records and
  do not choose a physical container, external storage, desktop file UX, or
  professional acceptance workflow.

## Next Gate

No next governed tranche is selected after TP-MAC-08 closeout. Open a new
assessment or governed plan only when the next user request scopes one.

Do not expand TP-MAC-07 or TP-MAC-06 into arbitrary station sweeps, exact
internal force diagrams, shear recovery, temperature-dependent material
interpolation, broader thermal load-combination behavior, code/rule
combinations, expansion-joint behavior, pressure-to-frame load conversion,
equivalent/principal stress, protected rule/check criteria,
release/professional claims, or professional acceptance workflows without a new
governed plan.
