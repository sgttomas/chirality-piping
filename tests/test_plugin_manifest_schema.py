#!/usr/bin/env python3
"""Stdlib checks for the plugin manifest schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "plugin_manifest.schema.yaml"

REQUIRED_TOP_LEVEL = {
    "metadata",
    "api_boundary_compatibility",
    "entrypoints",
    "permissions",
    "provenance",
    "privacy",
    "checksums",
    "sandbox",
    "no_bypass_constraints",
    "professional_boundary",
}

REQUIRED_NO_BYPASS = {
    "unit_controls",
    "provenance_controls",
    "privacy_controls",
    "rule_sandbox_controls",
}


def load_schema():
    with SCHEMA_PATH.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


def definition(schema, name):
    return schema["$defs"][name]


def main():
    schema = load_schema()
    defs = schema["$defs"]

    assert REQUIRED_TOP_LEVEL <= set(schema["required"])
    assert schema["additionalProperties"] is False

    permissions = definition(schema, "PermissionModel")
    assert "denied_by_default" in permissions["required"]
    assert permissions["properties"]["denied_by_default"]["const"] is True
    assert permissions["properties"]["grant_state"]["enum"][0] == "not_granted"

    sandbox = definition(schema, "SandboxDeclaration")
    assert "sandbox_required" in sandbox["required"]
    assert sandbox["properties"]["sandbox_required"]["const"] is True
    assert sandbox["properties"]["arbitrary_code_execution_allowed"]["const"] is False
    assert sandbox["properties"]["filesystem_access_default"]["enum"][0] == "denied"
    assert sandbox["properties"]["network_access_default"]["enum"][0] == "denied"
    assert sandbox["properties"]["process_spawn_default"]["enum"] == ["denied"]

    provenance = definition(schema, "ProvenanceRecord")
    required_provenance = {
        "source_name",
        "source_location",
        "source_license",
        "contributor",
        "contributor_certification",
        "redistribution_status",
        "review_status",
    }
    assert required_provenance <= set(provenance["required"])

    no_bypass = definition(schema, "NoBypassConstraints")
    assert REQUIRED_NO_BYPASS <= set(no_bypass["required"])
    for control in REQUIRED_NO_BYPASS:
        assert no_bypass["properties"][control]["const"] is True

    privacy = definition(schema, "PrivacyDeclaration")
    assert privacy["properties"]["local_first"]["const"] is True
    assert privacy["properties"]["private_data_transmission_default"]["const"] is False
    assert privacy["properties"]["telemetry_enabled_by_default"]["const"] is False

    compatibility = definition(schema, "ApiBoundaryCompatibility")
    assert compatibility["properties"]["api_boundary_contract_ref"]["const"] == (
        "api/api_boundary_contract.yaml"
    )
    assert compatibility["properties"]["result_envelope_required"]["const"] is True


if __name__ == "__main__":
    main()
