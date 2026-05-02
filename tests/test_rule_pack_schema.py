#!/usr/bin/env python3
"""Stdlib checks for the rule-pack schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "rule_pack.schema.yaml"

REQUIRED_TOP_LEVEL = {
    "metadata",
    "classification",
    "required_inputs",
    "formula_declarations",
    "value_slots",
    "check_definitions",
    "diagnostics",
    "checksums",
    "provenance",
    "professional_boundary",
}

REQUIRED_PROVENANCE = {
    "source_name",
    "source_location",
    "source_license",
    "contributor",
    "contributor_certification",
    "redistribution_status",
    "review_status",
}

FORBIDDEN_SCHEMA_TEXT = {
    "CODE_COMPLIANT",
    "certified",
    "sealed",
    "automatic compliance",
    "professional approval by the software",
}


def load_schema():
    with SCHEMA_PATH.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


def definition(schema, name):
    return schema["$defs"][name]


def required_at(schema, definition_name):
    return set(definition(schema, definition_name)["required"])


def walk_strings(value):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for item in value.values():
            yield from walk_strings(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_strings(item)


def main():
    schema = load_schema()
    defs = schema["$defs"]

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert REQUIRED_TOP_LEVEL <= set(schema["required"])

    metadata = definition(schema, "RulePackMetadata")
    assert {
        "rule_pack_id",
        "schema_version",
        "rule_pack_version",
        "lifecycle_status",
        "source_notice",
    } <= set(metadata["required"])

    classification = definition(schema, "RulePackClassification")
    assert classification["properties"]["protected_content_review_required"][
        "const"
    ] is True
    assert classification["properties"]["private_values_allowed"]["const"] is True
    assert "private_user_data_not_committed" in classification["properties"][
        "public_repository_policy"
    ]["enum"]

    provenance = definition(schema, "ProvenanceRecord")
    assert REQUIRED_PROVENANCE <= set(provenance["required"])

    checksum = definition(schema, "Checksum")
    assert {"algorithm", "canonicalization", "payload_ref", "value"} <= set(
        checksum["required"]
    )
    assert {"JCS", "NONE", "TBD"} <= set(
        checksum["properties"]["canonicalization"]["enum"]
    )

    required_input = definition(schema, "RequiredInput")
    assert {
        "quantity_intent",
        "completeness_status",
        "missing_value_diagnostic",
        "provenance_required",
        "redistribution_status_required",
    } <= set(required_input["required"])
    assert required_input["properties"]["provenance_required"]["const"] is True
    assert required_input["properties"]["redistribution_status_required"][
        "const"
    ] is True

    formula = definition(schema, "FormulaDeclaration")
    assert formula["properties"]["arbitrary_code_execution_allowed"]["const"] is False
    assert {
        "declarative_ast",
        "symbolic_reference",
        "structured_expression",
    } <= set(formula["properties"]["declaration_form"]["enum"])
    assert "no_protected_text_tables_or_copied_formulas" in formula["properties"][
        "protected_content_policy"
    ]["enum"]

    value_slot = definition(schema, "UserSuppliedValueSlot")
    assert {
        "quantity_intent",
        "value_status",
        "provenance",
        "redistribution_status",
        "review_status",
        "completeness_status",
    } <= set(value_slot["required"])

    check = definition(schema, "CheckDefinition")
    assert {"RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"} <= set(
        definition(schema, "AnalysisStatus")["enum"]
    )
    assert "HUMAN_APPROVED_FOR_PROJECT" not in definition(schema, "AnalysisStatus")[
        "enum"
    ]
    assert {"acceptability_basis", "result_statuses", "diagnostic_policy"} <= set(
        check["required"]
    )

    diagnostic_codes = set(definition(schema, "RulePackDiagnosticCode")["enum"])
    assert {
        "RULE_CHECK_BLOCKING",
        "RULE_INPUT_MISSING",
        "RULE_UNIT_MISMATCH",
        "RULE_PROVENANCE_WARNING",
        "RULE_REDISTRIBUTION_WARNING",
        "RULE_PROTECTED_CONTENT_WARNING",
        "RULE_EVALUATOR_ERROR",
        "RULE_INCOMPLETE_DATA",
    } <= diagnostic_codes

    professional_boundary = definition(schema, "ProfessionalBoundary")
    assert (
        professional_boundary["properties"]["software_makes_compliance_claim"][
            "const"
        ]
        is False
    )
    assert professional_boundary["properties"]["human_review_required"]["const"] is True
    assert (
        professional_boundary["properties"][
            "human_acceptance_record_software_generated"
        ]["const"]
        is False
    )

    all_text = "\n".join(walk_strings(schema))
    for forbidden in FORBIDDEN_SCHEMA_TEXT:
        assert forbidden not in all_text


if __name__ == "__main__":
    main()
