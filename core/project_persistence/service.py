"""Schema-shaped project persistence helpers for TP-PER-01.

This module builds deterministic project persistence envelopes for invented or
cleared payloads. It does not choose a physical project container, open files,
write files, migrate stored projects, transmit data, or make engineering
acceptance claims.
"""

from __future__ import annotations

from copy import deepcopy
from hashlib import sha256
import json
from typing import Any, Mapping, Sequence


SCHEMA_VERSION = "0.1.0"
DOCUMENT_KIND = "openpipestress.project_persistence"
PROJECT_SCHEMA_REF = "schemas/project_persistence.schema.yaml"
MODEL_SCHEMA_REF = "schemas/model.schema.yaml"
MODEL_STATE_SCHEMA_REF = "schemas/model_state.schema.json"
ANALYSIS_RUN_SCHEMA_REF = "schemas/analysis_run.schema.json"

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
}

PRIVATE_DATA_MARKER = {
    "classification": "public_permissive",
    "redistribution_status": "public_permissive",
    "default_transmission_allowed": False,
    "quarantine_required": False,
    "review_status": "accepted",
}

ENGINE_PROVENANCE = {
    "source_name": "OpenPipeStress TP-PER-01 project persistence service",
    "source_location": "core/project_persistence/service.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_certification": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_status": "accepted",
}


def build_project_persistence_envelope(
    *,
    project_id: str,
    project_name: str,
    model_payload: Mapping[str, Any],
    model_state_refs: Sequence[Mapping[str, Any]] | None = None,
    analysis_run_records: Sequence[Mapping[str, Any]] | None = None,
    analysis_run_refs: Sequence[Mapping[str, Any]] | None = None,
    result_envelope_refs: Sequence[Mapping[str, Any]] | None = None,
    result_refs: Sequence[Mapping[str, Any]] | None = None,
    model_state_records: Sequence[Mapping[str, Any]] | None = None,
    provenance_manifest: Sequence[Mapping[str, Any]] | None = None,
) -> dict[str, Any]:
    """Build a deterministic persistence envelope around a canonical model."""

    model_payload_copy = deepcopy(dict(model_payload))
    run_history = _run_history(
        model_state_refs=model_state_refs,
        model_state_records=model_state_records,
        analysis_run_refs=analysis_run_refs,
        analysis_run_records=analysis_run_records,
        result_envelope_refs=result_envelope_refs,
        result_refs=result_refs,
    )
    project = {
        "project_id": project_id,
        "project_name": project_name,
        "unit_system_ref": _artifact_ref("schema", "unit-system:SI-preview"),
        "model_payload": model_payload_copy,
        "rule_pack_refs": [],
        "provenance_manifest": [
            deepcopy(item) for item in (provenance_manifest or [ENGINE_PROVENANCE])
        ],
        "private_data": deepcopy(PRIVATE_DATA_MARKER),
        "human_acceptance_refs": [],
        "run_history": run_history,
    }
    base_envelope = {
        "schema_version": SCHEMA_VERSION,
        "document_kind": DOCUMENT_KIND,
        "physical_container": {
            "status": "TBD",
            "decision_ref": "TBD",
            "notes": "Physical project container format remains deferred in TP-PER-01.",
        },
        "project": project,
        "migration": {
            "status": "current",
            "source_schema_version": SCHEMA_VERSION,
            "target_schema_version": SCHEMA_VERSION,
            "migration_framework": "TBD",
            "diagnostics": [],
        },
        "round_trip_manifest": {
            "serialization": "canonical_json_jcs",
            "semantic_equality": [
                "model_content",
                "unit_metadata",
                "loads",
                "rule_pack_refs",
                "provenance_metadata",
                "reproducibility_metadata",
            ],
            "volatile_fields": [],
            "normalization_rules": [
                "schema_approved_only",
                "no_silent_engineering_defaults",
                "documented_volatile_field_exclusion",
            ],
        },
        "validation_profile": {
            "schema_validation": True,
            "model_schema_delegation": MODEL_SCHEMA_REF,
            "unit_metadata_check": True,
            "provenance_check": True,
            "rule_pack_reference_check": True,
            "protected_content_check": True,
            "private_data_check": True,
            "professional_boundary_check": True,
            "telemetry_default": "off",
        },
        "service_operations": _service_operations(),
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
        "diagnostics": [],
    }
    base_envelope["hash"] = _hash_metadata(base_envelope)
    return base_envelope


def validate_project_persistence_envelope(envelope: Mapping[str, Any]) -> list[dict[str, Any]]:
    """Return structured diagnostics for TP-PER-01 persistence invariants."""

    diagnostics: list[dict[str, Any]] = []
    if not isinstance(envelope, Mapping):
        return [_diagnostic("PERSISTENCE_ENVELOPE_INVALID", "SCHEMA_VALIDATION", "project_envelope")]

    for field in ("schema_version", "document_kind", "project", "hash", "migration"):
        if not envelope.get(field):
            diagnostics.append(_diagnostic("PERSISTENCE_FIELD_MISSING", "SCHEMA_VALIDATION", field))

    if envelope.get("document_kind") != DOCUMENT_KIND:
        diagnostics.append(_diagnostic("PERSISTENCE_DOCUMENT_KIND_INVALID", "SCHEMA_VALIDATION", "document_kind"))

    physical_container = envelope.get("physical_container", {})
    if isinstance(physical_container, Mapping) and physical_container.get("status") != "TBD":
        diagnostics.append(_diagnostic("PERSISTENCE_PHYSICAL_CONTAINER_OVERSPECIFIED", "MIGRATION", "physical_container"))

    project = envelope.get("project", {})
    if not isinstance(project, Mapping):
        diagnostics.append(_diagnostic("PERSISTENCE_PROJECT_INVALID", "SCHEMA_VALIDATION", "project"))
        project = {}

    if not project.get("model_payload"):
        diagnostics.append(_diagnostic("PERSISTENCE_MODEL_PAYLOAD_MISSING", "SCHEMA_VALIDATION", "project.model_payload"))

    if not _valid_private_data(project.get("private_data")):
        diagnostics.append(_diagnostic("PERSISTENCE_PRIVATE_DATA_BOUNDARY_INVALID", "PRIVATE_DATA", "project.private_data"))

    provenance_manifest = project.get("provenance_manifest", [])
    if not provenance_manifest or any(not _valid_provenance(item) for item in provenance_manifest):
        diagnostics.append(_diagnostic("PERSISTENCE_PROVENANCE_MISSING", "PROVENANCE_WARNING", "project.provenance_manifest"))

    if envelope.get("professional_boundary") != PROFESSIONAL_BOUNDARY:
        diagnostics.append(_diagnostic("PERSISTENCE_PROFESSIONAL_BOUNDARY_VIOLATION", "PROFESSIONAL_BOUNDARY", "professional_boundary"))

    validation_profile = envelope.get("validation_profile", {})
    if isinstance(validation_profile, Mapping) and validation_profile.get("telemetry_default") != "off":
        diagnostics.append(_diagnostic("PERSISTENCE_TELEMETRY_DEFAULT_INVALID", "PRIVATE_DATA", "validation_profile.telemetry_default"))

    run_history = project.get("run_history", {})
    if not _valid_run_history(run_history):
        diagnostics.append(_diagnostic("PERSISTENCE_RUN_HISTORY_REFS_MISSING", "SCHEMA_VALIDATION", "project.run_history"))

    expected_hash = _hash_metadata(_envelope_without_hash(envelope))
    actual_hash = envelope.get("hash", {})
    if isinstance(actual_hash, Mapping):
        if actual_hash.get("project_payload_hash", {}).get("value") != expected_hash["project_payload_hash"]["value"]:
            diagnostics.append(_diagnostic("PERSISTENCE_PROJECT_HASH_MISMATCH", "SCHEMA_VALIDATION", "hash.project_payload_hash"))
        if _manifest_values(actual_hash.get("hash_manifest", [])) != _manifest_values(expected_hash["hash_manifest"]):
            diagnostics.append(_diagnostic("PERSISTENCE_HASH_MANIFEST_MISMATCH", "SCHEMA_VALIDATION", "hash.hash_manifest"))

    return sorted(diagnostics, key=canonical_json)


def canonical_json(value: Any) -> str:
    """Return deterministic JSON used for TP-PER-01 hashes."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def project_hash_manifest(envelope: Mapping[str, Any]) -> list[dict[str, Any]]:
    """Return deterministic hash records for a persistence envelope."""

    return _hash_metadata(_envelope_without_hash(envelope))["hash_manifest"]


def round_trip_project_envelope(envelope: Mapping[str, Any]) -> dict[str, Any]:
    """Serialize and parse an envelope through canonical JSON."""

    source = deepcopy(dict(envelope))
    restored = json.loads(canonical_json(source))
    source_hash = _checksum(_artifact_ref("project_persistence", "round-trip:source"), "project_envelope", _envelope_without_hash(source))
    restored_hash = _checksum(_artifact_ref("project_persistence", "round-trip:restored"), "project_envelope", _envelope_without_hash(restored))
    return {
        "serialization": "canonical_json_jcs",
        "semantic_equal": canonical_json(source) == canonical_json(restored),
        "source_hash": source_hash,
        "round_trip_hash": restored_hash,
        "diagnostics": validate_project_persistence_envelope(restored),
        "envelope": restored,
    }


def _run_history(
    *,
    model_state_refs: Sequence[Mapping[str, Any]] | None,
    model_state_records: Sequence[Mapping[str, Any]] | None,
    analysis_run_refs: Sequence[Mapping[str, Any]] | None,
    analysis_run_records: Sequence[Mapping[str, Any]] | None,
    result_envelope_refs: Sequence[Mapping[str, Any]] | None,
    result_refs: Sequence[Mapping[str, Any]] | None,
) -> dict[str, Any]:
    analysis_records = [deepcopy(dict(item)) for item in (analysis_run_records or [])]
    state_records = [deepcopy(dict(item)) for item in (model_state_records or [])]
    inferred_analysis_refs = [
        _artifact_ref("analysis_run", item.get("analysis_run", {}).get("run_id", "analysis-run:unknown"))
        for item in analysis_records
    ]
    inferred_result_refs = _result_refs_from_analysis_records(analysis_records)
    history = {
        "model_state_refs": [deepcopy(dict(item)) for item in (model_state_refs or [])],
        "model_state_records": state_records,
        "analysis_run_refs": [deepcopy(dict(item)) for item in (analysis_run_refs or inferred_analysis_refs)],
        "analysis_run_records": analysis_records,
        "result_envelope_refs": [deepcopy(dict(item)) for item in (result_envelope_refs or [])],
        "result_refs": [deepcopy(dict(item)) for item in (result_refs or inferred_result_refs)],
        "hash_manifest": [],
    }
    history["hash_manifest"] = _history_record_hashes(history)
    return history


def _hash_metadata(envelope_without_hash: Mapping[str, Any]) -> dict[str, Any]:
    project = deepcopy(dict(envelope_without_hash.get("project", {})))
    run_history = project.get("run_history", {}) if isinstance(project, Mapping) else {}
    manifest = [
        _checksum(_artifact_ref("project_persistence", "project-payload"), "project_payload", project),
        _checksum(_artifact_ref("model_payload", "model-payload"), "model_payload", project.get("model_payload", {})),
        _checksum(_artifact_ref("project_persistence", "project-envelope-without-hash"), "project_envelope", envelope_without_hash),
    ]
    project_payload_hash = manifest[0]
    if isinstance(run_history, Mapping):
        manifest.extend(deepcopy(dict(item)) for item in run_history.get("hash_manifest", []))
        for ref in run_history.get("result_envelope_refs", []):
            if isinstance(ref, Mapping) and ref.get("hash"):
                manifest.append(deepcopy(dict(ref["hash"])))
        for ref in run_history.get("result_refs", []):
            if isinstance(ref, Mapping) and ref.get("hash"):
                manifest.append(deepcopy(dict(ref["hash"])))
    return {
        "canonicalization": "JCS",
        "project_payload_hash": project_payload_hash,
        "hash_manifest": manifest,
        "payload_partition_status": "json_payloads_defined",
        "excluded_fields": [],
    }


def _history_record_hashes(run_history: Mapping[str, Any]) -> list[dict[str, Any]]:
    hashes: list[dict[str, Any]] = []
    for record in run_history.get("model_state_records", []):
        state_id = record.get("model_state", {}).get("state_id", "model-state:unknown")
        hashes.append(_checksum(_artifact_ref("model_state", state_id), "model_state_record", record))
    for record in run_history.get("analysis_run_records", []):
        run_id = record.get("analysis_run", {}).get("run_id", "analysis-run:unknown")
        hashes.append(_checksum(_artifact_ref("analysis_run", run_id), "analysis_run_record", record))
    return hashes


def _envelope_without_hash(envelope: Mapping[str, Any]) -> dict[str, Any]:
    value = deepcopy(dict(envelope))
    value.pop("hash", None)
    return value


def _checksum(payload_ref: Mapping[str, Any], payload_scope: str, payload: Any) -> dict[str, Any]:
    return {
        "algorithm": "sha256",
        "canonicalization": "JCS",
        "payload_ref": deepcopy(dict(payload_ref)),
        "payload_scope": payload_scope,
        "value": sha256(canonical_json(payload).encode("utf-8")).hexdigest(),
    }


def _artifact_ref(ref_kind: str, ref: str, hash_value: Mapping[str, Any] | None = None) -> dict[str, Any]:
    value = {
        "ref_kind": ref_kind,
        "ref": ref,
    }
    if hash_value is not None:
        value["hash"] = deepcopy(dict(hash_value))
    return value


def _result_refs_from_analysis_records(records: Sequence[Mapping[str, Any]]) -> list[dict[str, Any]]:
    refs: list[dict[str, Any]] = []
    seen: set[str] = set()
    for record in records:
        for item in record.get("analysis_run", {}).get("result_refs", []):
            result_ref = item.get("result_ref", {})
            result_id = result_ref.get("ref") or result_ref.get("id")
            if not result_id or result_id in seen:
                continue
            seen.add(str(result_id))
            refs.append(_artifact_ref("result", str(result_id)))
    return refs


def _valid_private_data(value: Any) -> bool:
    if not isinstance(value, Mapping):
        return False
    return (
        value.get("default_transmission_allowed") is False
        and value.get("classification") != "protected_suspected"
        and value.get("redistribution_status") != "protected_suspected"
    )


def _valid_provenance(value: Any) -> bool:
    if not isinstance(value, Mapping):
        return False
    required = (
        "source_name",
        "source_location",
        "source_license",
        "contributor",
        "contributor_certification",
        "redistribution_status",
        "review_status",
    )
    return all(str(value.get(field, "")).strip() for field in required) and value.get("redistribution_status") != "protected_suspected"


def _valid_run_history(value: Any) -> bool:
    if not isinstance(value, Mapping):
        return False
    return all(value.get(field) for field in ("model_state_refs", "analysis_run_refs", "result_envelope_refs", "result_refs"))


def _manifest_values(manifest: Any) -> list[tuple[str, str, str]]:
    if not isinstance(manifest, Sequence) or isinstance(manifest, (str, bytes)):
        return []
    values = []
    for item in manifest:
        if isinstance(item, Mapping):
            payload_ref = item.get("payload_ref", {})
            ref = payload_ref.get("ref") if isinstance(payload_ref, Mapping) else ""
            values.append((str(item.get("payload_scope", "")), str(ref), str(item.get("value", ""))))
    return sorted(values)


def _diagnostic(code: str, diagnostic_class: str, affected_ref: str) -> dict[str, Any]:
    return {
        "code": code,
        "class": diagnostic_class,
        "severity": "blocking",
        "source": "core/project_persistence/service.py",
        "affected_ref": _artifact_ref("project_persistence", affected_ref),
        "message": f"{code} at {affected_ref}",
        "remediation": "Provide a schema-shaped persistence envelope with explicit provenance, privacy, hashes, and professional-boundary fields.",
    }


def _service_operations() -> list[dict[str, Any]]:
    diagnostic_classes = [
        "SCHEMA_VALIDATION",
        "MIGRATION",
        "UNIT_METADATA",
        "PROVENANCE_WARNING",
        "RULE_CHECK_BLOCKING",
        "IP_BOUNDARY_WARNING",
        "PRIVATE_DATA",
        "PROFESSIONAL_BOUNDARY",
    ]
    return [
        {
            "operation": operation,
            "boundary": "application_service",
            "minimum_inputs": ["project_envelope"],
            "minimum_outputs": ["project_envelope", "diagnostics", "hash_manifest"],
            "diagnostic_classes": diagnostic_classes,
            "bypass_prohibited": True,
        }
        for operation in ("create_project", "open_project", "save_project", "validate_project", "version_check")
    ]
