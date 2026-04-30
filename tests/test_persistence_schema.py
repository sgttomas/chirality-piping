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

    defs = schema["$defs"]
    project = defs["ProjectEnvelope"]
    assert {"project_id", "unit_system_ref", "model_payload", "private_data"} <= set(
        project["required"]
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
    } <= set(hash_metadata["required"])

    migration = defs["MigrationStatus"]
    assert {"status", "source_schema_version", "target_schema_version"} <= set(
        migration["required"]
    )
    assert {
        "current",
        "migration_needed",
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


if __name__ == "__main__":
    main()
