#!/usr/bin/env python3
"""Stdlib checks for the section and component library schemas."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SECTION_SCHEMA_PATH = ROOT / "schemas" / "section.schema.yaml"
COMPONENT_SCHEMA_PATH = ROOT / "schemas" / "component.schema.yaml"
FIXTURE_PATH = ROOT / "fixtures" / "component" / "invented_section_component_library_valid.json"

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
    "CODE_COMPLIANT",
    "certified component",
    "certified section",
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
    section_schema = load_json(SECTION_SCHEMA_PATH)
    component_schema = load_json(COMPONENT_SCHEMA_PATH)
    fixture = load_json(FIXTURE_PATH)

    for schema in (section_schema, component_schema):
        assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
        assert schema["additionalProperties"] is False
        assert "default" not in set(walk_keys(schema))
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
        "schema_version",
        "section_library",
        "section_records",
        "dimension_definitions",
        "property_definitions",
        "completeness_rules",
        "diagnostics",
        "open_decisions",
    } <= set(section_schema["required"])
    assert {
        "section_id",
        "name",
        "section_type",
        "privacy_class",
        "redistribution_status",
        "dimensions",
        "properties",
        "completeness",
        "provenance",
        "review_status",
    } <= required_at(section_schema, "SectionRecord")
    assert {
        "outside_diameter",
        "wall_thickness",
        "corrosion_allowance",
        "nominal_size_label",
        "user_defined",
        "TBD",
    } <= enum_at(section_schema, "SectionDimensionKind")
    assert {
        "cross_section_area",
        "inside_diameter",
        "moment_of_inertia",
        "section_modulus",
        "mass_per_length",
        "user_defined",
        "TBD",
    } <= enum_at(section_schema, "SectionPropertyKind")
    assert {
        "SECTION_DIMENSION_MISSING",
        "SECTION_UNIT_MISSING",
        "SECTION_PROVENANCE_MISSING",
        "SECTION_PROTECTED_CONTENT_SUSPECTED",
        "SECTION_CATALOG_VALUE_NOT_PUBLIC",
    } <= enum_at(section_schema, "SectionDiagnosticCode")

    assert {
        "schema_version",
        "component_library",
        "component_family_contracts",
        "component_records",
        "field_definitions",
        "completeness_rules",
        "diagnostics",
        "open_decisions",
    } <= set(component_schema["required"])
    assert {
        "component_id",
        "name",
        "component_type",
        "privacy_class",
        "redistribution_status",
        "fields",
        "completeness",
        "provenance",
        "review_status",
    } <= required_at(component_schema, "ComponentRecord")
    assert {
        "bend",
        "elbow",
        "branch",
        "reducer",
        "valve",
        "flange",
        "expansion_joint",
        "rigid",
        "specialty",
        "other",
        "TBD",
    } <= enum_at(component_schema, "ComponentType")
    assert {
        "bend_centerline_radius",
        "bend_included_angle",
        "bend_plane_orientation",
        "bend_geometry_source_reference",
        "weight",
        "center_of_gravity",
        "stiffness",
        "effective_area",
        "movement_limit",
        "sif_user_value",
        "flexibility_factor_user_value",
        "manufacturer_reference",
        "TBD",
    } <= enum_at(component_schema, "ComponentFieldKind")
    assert {
        "COMPONENT_FIELD_MISSING",
        "COMPONENT_UNIT_MISSING",
        "COMPONENT_PROVENANCE_MISSING",
        "COMPONENT_PROTECTED_CONTENT_SUSPECTED",
        "COMPONENT_MODIFIER_NOT_PUBLIC",
        "BEND_GEOMETRY_INCOMPLETE",
        "BEND_RULE_INPUT_MISSING",
    } <= enum_at(component_schema, "ComponentDiagnosticCode")
    assert {
        "contract_id",
        "component_types",
        "geometry_field_kinds",
        "rule_modifier_field_kinds",
        "source_metadata_field_kinds",
        "mechanics_interface",
        "protected_value_policy",
        "review_status",
    } <= required_at(component_schema, "ComponentFamilyContract")

    assert fixture["section_library"]["library_scope"] == "public_schema_fixture"
    assert fixture["section_records"][0]["redistribution_status"] == "TBD"
    assert fixture["section_records"][0]["dimensions"][0]["value_status"] == "missing"
    assert fixture["section_records"][0]["completeness"][0]["status"] == "incomplete"
    assert fixture["section_diagnostics"][0]["code"] == "SECTION_DIMENSION_MISSING"

    assert fixture["component_library"]["library_scope"] == "public_schema_fixture"
    bend_contract = fixture["component_family_contracts"][0]
    assert {"bend", "elbow"} <= set(bend_contract["component_types"])
    assert "bend_centerline_radius" in bend_contract["geometry_field_kinds"]
    assert "sif_user_value" in bend_contract["rule_modifier_field_kinds"]
    assert bend_contract["protected_value_policy"] == "schema_slots_only"
    assert fixture["component_records"][0]["redistribution_status"] == "TBD"
    fixture_field_kinds = {
        field["field_kind"] for field in fixture["component_records"][0]["fields"]
    }
    assert "bend_centerline_radius" in fixture_field_kinds
    assert "bend_included_angle" in fixture_field_kinds
    assert (
        fixture["component_records"][0]["fields"][0]["public_repository_value_policy"]
        == "schema_shape_only"
    )
    assert fixture["component_records"][0]["completeness"][0]["status"] == "incomplete"
    assert fixture["component_diagnostics"][0]["code"] == "BEND_GEOMETRY_INCOMPLETE"

    all_text = "\n".join(
        [
            *walk_strings(section_schema),
            *walk_strings(component_schema),
            *walk_strings(fixture),
        ]
    )
    for forbidden in FORBIDDEN_PUBLIC_DATA_TEXT:
        assert forbidden not in all_text


if __name__ == "__main__":
    main()
