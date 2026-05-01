#!/usr/bin/env python3
"""Stdlib checks for the project persistence schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "project_persistence.schema.yaml"


def load_schema():
    with SCHEMA_PATH.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


def ref_name(ref):
    return ref.rsplit("/", 1)[-1]


def main():
    schema = load_schema()

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False

    required = set(schema["required"])
    assert {"schema_version", "project", "hash", "migration"} <= required

    properties = schema["properties"]
    assert ref_name(properties["project"]["$ref"]) == "ProjectEnvelope"
    assert ref_name(properties["hash"]["$ref"]) == "HashMetadata"
    assert ref_name(properties["migration"]["$ref"]) == "MigrationStatus"
    assert ref_name(properties["validation_profile"]["$ref"]) == "ValidationProfile"
    assert (
        ref_name(properties["service_operations"]["items"]["$ref"])
        == "PersistenceOperation"
    )

    defs = schema["$defs"]
    project = defs["ProjectEnvelope"]
    assert {"project_id", "unit_system_ref", "model_payload", "private_data"} <= set(
        project["required"]
    )
    assert (
        ref_name(project["properties"]["human_acceptance_refs"]["items"]["$ref"])
        == "HumanAcceptanceRef"
    )

    model_payload_ref = project["properties"]["model_payload"]["$ref"]
    assert ref_name(model_payload_ref) == "ModelPayload"
    model_payload = defs["ModelPayload"]
    assert {"$ref": "model.schema.yaml"} in model_payload["allOf"]

    hash_metadata = defs["HashMetadata"]
    assert hash_metadata["properties"]["canonicalization"]["const"] == "JCS"
    assert {
        "canonicalization",
        "project_payload_hash",
        "hash_manifest",
        "payload_partition_status",
    } <= set(hash_metadata["required"])
    assert "non_json_or_binary_partition_TBD" in set(
        hash_metadata["properties"]["payload_partition_status"]["enum"]
    )

    checksum = defs["Checksum"]
    assert {"algorithm", "canonicalization", "value"} <= set(checksum["required"])
    assert {
        "project_envelope",
        "project_payload",
        "model_payload",
        "rule_pack_reference",
        "input_manifest",
        "report_manifest",
        "external_artifact",
        "TBD",
    } <= set(checksum["properties"]["payload_scope"]["enum"])

    migration = defs["MigrationStatus"]
    assert {"status", "source_schema_version", "target_schema_version"} <= set(
        migration["required"]
    )
    assert {
        "current",
        "migration_needed",
        "stale",
        "migrated",
        "unsupported_schema",
        "failed",
        "newer_than_supported",
        "TBD",
    } <= set(migration["properties"]["status"]["enum"])
    assert migration["properties"]["migration_framework"]["const"] == "TBD"

    private_data = defs["PrivateDataMarker"]
    assert {
        "classification",
        "redistribution_status",
        "default_transmission_allowed",
    } <= set(private_data["required"])
    assert private_data["properties"]["default_transmission_allowed"]["const"] is False

    physical = defs["PhysicalContainer"]
    assert physical["required"] == ["status"]
    assert physical["properties"]["status"]["const"] == "TBD"
    assert properties["physical_container"]["$ref"].endswith("/PhysicalContainer")

    round_trip = defs["RoundTripManifest"]
    assert {"serialization", "semantic_equality"} <= set(round_trip["required"])
    assert "canonical_json_jcs" in round_trip["properties"]["serialization"]["enum"]
    assert {
        "schema_approved_only",
        "no_silent_engineering_defaults",
        "documented_volatile_field_exclusion",
        "TBD",
    } <= set(round_trip["properties"]["normalization_rules"]["items"]["enum"])

    validation_profile = defs["ValidationProfile"]
    assert {
        "schema_validation",
        "model_schema_delegation",
        "unit_metadata_check",
        "provenance_check",
        "private_data_check",
        "professional_boundary_check",
    } <= set(validation_profile["required"])
    assert (
        validation_profile["properties"]["model_schema_delegation"]["const"]
        == "schemas/model.schema.yaml"
    )
    assert validation_profile["properties"]["telemetry_default"]["const"] == "off"

    operation = defs["PersistenceOperation"]
    assert {
        "create_project",
        "open_project",
        "save_project",
        "validate_project",
        "version_check",
        "migrate_project",
        "TBD",
    } <= set(operation["properties"]["operation"]["enum"])
    assert operation["properties"]["boundary"]["const"] == "application_service"
    assert operation["properties"]["bypass_prohibited"]["const"] is True
    assert {
        "SCHEMA_VALIDATION",
        "MIGRATION",
        "UNIT_METADATA",
        "PROVENANCE_WARNING",
        "RULE_CHECK_BLOCKING",
        "IP_BOUNDARY_WARNING",
        "PRIVATE_DATA",
        "PROFESSIONAL_BOUNDARY",
        "TBD",
    } <= set(operation["properties"]["diagnostic_classes"]["items"]["enum"])

    human_acceptance = defs["HumanAcceptanceRef"]
    assert {
        "acceptance_ref",
        "authority_kind",
        "binding_hashes",
        "invalidates_on_hash_change",
    } <= set(human_acceptance["required"])
    assert (
        human_acceptance["properties"]["invalidates_on_hash_change"]["const"] is True
    )
    assert "external_human_review" in set(
        human_acceptance["properties"]["authority_kind"]["enum"]
    )

    rule_pack_ref = defs["RulePackRef"]
    assert {
        "public_permissive",
        "private_only",
        "unknown",
        "protected_suspected",
        "TBD",
    } <= set(rule_pack_ref["properties"]["redistribution_status"]["enum"])


if __name__ == "__main__":
    main()
