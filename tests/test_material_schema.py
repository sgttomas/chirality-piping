#!/usr/bin/env python3
"""Stdlib checks for the material library schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "material.schema.yaml"
FIXTURE_PATH = ROOT / "fixtures" / "material" / "invented_material_library_valid.json"

REQUIRED_TOP_LEVEL = {
    "schema_version",
    "material_library",
    "material_records",
    "property_definitions",
    "completeness_rules",
    "diagnostics",
    "open_decisions",
}

REQUIRED_MATERIAL_FIELDS = {
    "material_id",
    "name",
    "material_family",
    "privacy_class",
    "redistribution_status",
    "properties",
    "allowables",
    "completeness",
    "provenance",
    "review_status",
}

REQUIRED_PROVENANCE_FIELDS = {
    "source_name",
    "source_location",
    "source_license",
    "contributor",
    "contributor_certification",
    "redistribution_status",
    "review_status",
}

FORBIDDEN_PUBLIC_DATA_TEXT = {
    "ASME",
    "B31",
    "ASTM",
    "CODE_COMPLIANT",
    "certified material",
    "sealed",
    "automatic compliance",
    "professional approval by the software",
}


def load_json(path):
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def walk_strings(value):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for item in value.values():
            yield from walk_strings(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_strings(item)


def walk_keys(value):
    if isinstance(value, dict):
        for key, item in value.items():
            yield key
            yield from walk_keys(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_keys(item)


def required_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["required"])


def enum_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["enum"])


def main():
    schema = load_json(SCHEMA_PATH)
    fixture = load_json(FIXTURE_PATH)
    defs = schema["$defs"]

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))
    assert REQUIRED_TOP_LEVEL <= set(schema["required"])

    assert REQUIRED_MATERIAL_FIELDS <= required_at(schema, "MaterialRecord")
    assert REQUIRED_PROVENANCE_FIELDS <= required_at(schema, "Provenance")
    assert {
        "public_permissive",
        "private_only",
        "unknown",
        "protected_suspected",
        "rejected",
        "TBD",
    } <= enum_at(schema, "RedistributionStatus")

    assert {
        "density",
        "elastic_modulus",
        "poisson_ratio",
        "thermal_expansion_coefficient",
        "allowable_stress",
        "user_defined",
        "TBD",
    } <= enum_at(schema, "MaterialPropertyKind")
    assert {
        "density",
        "stress",
        "temperature",
        "thermal_expansion_coefficient",
        "dimensionless",
        "TBD",
    } <= enum_at(schema, "MaterialPropertyDimension")

    allowable = defs["MaterialAllowableSlot"]
    assert {
        "allowable_id",
        "allowable_kind",
        "value_status",
        "public_repository_value_policy",
        "required_for",
        "provenance",
        "review_status",
    } <= set(allowable["required"])
    assert "no_public_code_specific_values" in set(
        allowable["properties"]["public_repository_value_policy"]["enum"]
    )
    assert "private_user_supplied_only" in set(
        allowable["properties"]["public_repository_value_policy"]["enum"]
    )

    completeness_rule = defs["CompletenessRule"]
    assert {
        "rule_id",
        "applies_to",
        "required_property_kinds",
        "required_for",
        "missing_behavior",
        "diagnostic_code",
        "review_status",
    } <= set(completeness_rule["required"])
    assert "diagnostic_blocking" in set(
        completeness_rule["properties"]["missing_behavior"]["enum"]
    )

    diagnostic_codes = enum_at(schema, "MaterialDiagnosticCode")
    assert {
        "MATERIAL_PROPERTY_MISSING",
        "MATERIAL_UNIT_MISSING",
        "MATERIAL_PROVENANCE_MISSING",
        "MATERIAL_PROTECTED_CONTENT_SUSPECTED",
        "MATERIAL_ALLOWABLE_NOT_PUBLIC",
    } <= diagnostic_codes

    open_decision = defs["OpenDecision"]
    assert {
        "public_material_fixture_policy",
        "accepted_material_source_catalog",
        "allowable_value_storage_policy",
        "temperature_interpolation_policy",
    } <= set(open_decision["properties"]["topic"]["enum"])

    assert fixture["material_library"]["library_scope"] == "public_schema_fixture"
    assert fixture["material_records"][0]["redistribution_status"] == "TBD"
    assert (
        fixture["material_records"][0]["allowables"][0][
            "public_repository_value_policy"
        ]
        == "no_public_code_specific_values"
    )
    assert fixture["material_records"][0]["completeness"][0]["status"] == "incomplete"
    assert fixture["diagnostics"][0]["code"] == "MATERIAL_PROPERTY_MISSING"

    all_text = "\n".join([*walk_strings(schema), *walk_strings(fixture)])
    for forbidden in FORBIDDEN_PUBLIC_DATA_TEXT:
        assert forbidden not in all_text


if __name__ == "__main__":
    main()
