# Coordination Record

**Active phase:** `TP-PER-01 Project Persistence And Run History Spine` implemented
**Active objective:** Preserve the implemented schema-shaped persistence
baseline together with the closed TP-MAC-05 mechanics and TP-RUN-01 runtime
behavior.
**Active plan:** `plans/TP-PER-01_PROJECT_PERSISTENCE_AND_RUN_HISTORY_PLAN.md`
**Latest state:** `execution/_Coordination/NEXT_INSTANCE_STATE.md`
**Historical archive:** `execution/_Coordination/_Archive/`

## Current Authority

The current development phase is no longer the `RT-1` evidence archive route,
the `TP-MAC-01` desktop-hardening tranche, an open TP-MAC-03 continuation, or
an open TP-MAC-04 continuation. Those routes are historical context only.

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
audit traceability, but they are not the first context surface for TP-RUN-01
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
- explicit diagnostics for unsupported, incomplete, or review-required paths;
- professional-boundary and non-mutation fields.

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
- a mechanics gap ledger that marks endpoint-j local force/moment and endpoint
  stress component preview recovery implemented and keeps remaining unsupported
  mechanics explicit.

TP-RUN-01 should connect the runtime surfaces to this product-physics baseline.
The desktop Tauri command now accepts an optional preview model payload while
preserving fixture fallback. The React preview service passes the loaded model
into the desktop runtime when Tauri is available. The headless runner now has
an in-memory bridge that executes product physics and returns runner metadata,
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
- persistence envelopes remain schema-shaped application-service records and
  do not choose a physical container, external storage, desktop file UX, or
  professional acceptance workflow.

## Next Gate

No successor implementation gate is open in this coordination record. A new
governed plan is required before adding desktop save/open UX, final CLI syntax,
physical project containers, migrations, external execution, new mechanics,
release/professional claims, or professional acceptance workflows.
