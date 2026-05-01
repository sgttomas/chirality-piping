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
    "analysis_boundary_controls",
    "persistence_controls",
    "human_acceptance_controls",
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
    assert sandbox["properties"]["capability_declaration_required"]["const"] is True

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
    assert privacy["properties"]["redaction_supported"]["const"] is True

    compatibility = definition(schema, "ApiBoundaryCompatibility")
    assert compatibility["properties"]["domain_contract_ref"]["const"] == (
        "docs/architecture/extension_domain_contracts.md"
    )
    assert compatibility["properties"]["api_boundary_contract_ref"]["const"] == (
        "api/api_boundary_contract.yaml"
    )
    assert compatibility["properties"]["schema_version_contract"]["enum"][0] == (
        "JSON Schema 2020-12"
    )
    assert compatibility["properties"]["result_envelope_required"]["const"] is True

    entrypoint = definition(schema, "Entrypoint")
    assert "domain_surface" in entrypoint["required"]
    assert "canonical_model" in entrypoint["properties"]["domain_surface"]["enum"]
    assert "project_persistence" in entrypoint["properties"]["domain_surface"]["enum"]

    professional_boundary = definition(schema, "ProfessionalBoundary")
    assert (
        professional_boundary["properties"][
            "human_acceptance_record_software_generated"
        ]["const"]
        is False
    )


if __name__ == "__main__":
    main()
