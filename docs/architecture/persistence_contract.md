---
doc_id: OPS-PERSISTENCE-CONTRACT
doc_kind: architecture.data_model
status: draft
created: 2026-04-30
deliverable_id: DEL-02-05
package_id: PKG-02
scope_items:
  - SOW-050
  - SOW-041
---

# Project Persistence Contract

This document defines the DEL-02-05 persistence and deterministic round-trip contract. The companion schema is `schemas/project_persistence.schema.yaml`, written as strict JSON syntax so it can be parsed without a YAML dependency.

## Boundary

The persisted project document is a versioned, schema-governed domain artifact. It may contain private project data under user control, but public repository fixtures and examples must not contain protected standards text, copied tables, proprietary values, private rule-pack contents, or professional approval claims.

The project document envelope owns persistence metadata: schema version, project identity, migration status, canonical JSON/JCS hash metadata, private-data markers, diagnostics, and round-trip manifest entries. The actual model payload is delegated to `schemas/model.schema.yaml`.

The physical project package/container remains `TBD`. This contract does not decide whether a user-facing project file is one JSON document, a packaged archive, a directory bundle, or another container.

## Required Envelope

Every persisted project envelope must include:

- `schema_version`: semantic version for this persistence envelope.
- `project`: project identity, declared unit-system reference, canonical model payload, rule-pack references when present, provenance manifest when present, and private-data marker.
- `hash`: canonicalization and checksum metadata for the project payload and hash manifest.
- `migration`: current, stale, unsupported, migrated, failed, newer-than-supported, or `TBD` migration state.

Optional but expected fields include `document_kind`, `physical_container`, `round_trip_manifest`, and `diagnostics`.

## Deterministic Serialization

Round-trip serialization means:

1. Parse the project document.
2. Validate it against the persistence envelope and delegated model schema.
3. Normalize only schema-approved representation details.
4. Serialize JSON payloads using JCS-compatible canonical JSON for hashable payloads.
5. Parse the serialized output again.
6. Compare semantic equality for model content, unit metadata, loads, rule-pack references, provenance metadata, and reproducibility metadata.

Round-trip checks must preserve engineering-relevant data. They must not insert silent defaults for missing units, provenance, rule-pack values, material data, component data, SIF/flexibility inputs, allowables, or load basis values. Missing solve-required or rule-check-required values are diagnostics.

Volatile/session-only fields may be excluded from deterministic comparison only when the exclusion is documented in the hash or round-trip manifest. The exact payload/manifest partition for future non-JSON or binary assets remains `TBD`.

## Hash Rules

JSON payload hashes use canonical JSON with JCS-compatible canonicalization. The schema records this with `hash.canonicalization = "JCS"` and checksum entries carrying `algorithm`, `canonicalization`, `payload_ref`, and `value`.

Hashes must identify what was hashed: project payload, model payload, rule-pack reference metadata, input manifest, report-facing manifest, or external artifact reference. A hash over environment-local paths, timestamps, UI session state, or other volatile fields is not a reproducibility hash unless the volatile-field treatment is explicitly documented.

Human review records, if added by later work, must bind to specific model, rule-pack, result, and report hashes. They do not survive content changes and do not imply software certification or automatic code compliance.

## Migration Status

The persistence envelope records migration state separately from model content:

- `current`: source and target schema versions match supported persistence behavior.
- `migration_needed`: the project is stale and requires migration before being treated as current.
- `migrated`: a migration has produced the target schema version.
- `unsupported_schema`: the project cannot be opened without a future migration path.
- `failed`: migration was attempted and failed with diagnostics.
- `newer_than_supported`: the project was created by a newer schema than this implementation supports.
- `TBD`: the exact migration state cannot yet be classified.

The migration framework and tooling remain `TBD`. Unsupported, stale, failed, or newer-than-supported cases must produce structured diagnostics rather than silent coercion.

## Private Data Handling

Private project, material, component, library, and rule-pack data is local-first and user-controlled. Persistence metadata must mark privacy classification, redistribution status, default transmission permission, quarantine requirements where applicable, and review status.

`default_transmission_allowed` is false in private-data markers. Public export, telemetry, repository contribution, issue attachment, or cloud synchronization are separate reviewed actions and are not implied by create/open/save.

If protected or proprietary content is suspected, persistence validation must emit an IP/private-data diagnostic and route the content to quarantine/human review. Public artifacts for this deliverable must use invented, original, public-domain, or permissively licensed data only.

## Service Contract

Persistence operations are application-service boundaries:

- Create project: accepts project identity, unit-system reference, storage/private-data policy, and optional template reference; returns a versioned envelope or diagnostics.
- Open project: accepts a project artifact reference; returns parsed envelope, schema/version result, migration status, validation diagnostics, and hash evidence where available.
- Save project: accepts a validated envelope and target artifact reference; writes deterministic payload representation and returns hash/manifest evidence plus diagnostics.
- Validate project: checks schema shape, model schema delegation, unit metadata, provenance, rule-pack references, private-data markers, protected-content boundary, and professional-boundary language.
- Version check: classifies current, stale, unsupported, failed, newer-than-supported, or `TBD` migration state.
- Migrate project: returns a migrated envelope or structured failure diagnostics. Migration tooling remains `TBD`.

Adapters and plugins may translate storage formats, but they must not bypass schema validation, unit checks, provenance checks, diagnostics, private-data controls, protected-content screening, or professional-boundary restrictions.

## Remaining TBDs

- Physical project package/container.
- Migration framework/tooling and rollback semantics.
- Exact payload partition for hashes involving non-JSON or binary artifacts.
- Dependency/library choices for canonical JSON/JCS serialization.
- Final diagnostic code namespace and user-facing migration wording.
