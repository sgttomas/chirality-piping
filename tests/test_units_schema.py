#!/usr/bin/env python3
"""Stdlib checks for the unit system schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "units.schema.yaml"

REQUIRED_DIMENSIONS = {
    "dimensionless",
    "length",
    "mass",
    "time",
    "temperature",
    "temperature_interval",
    "angle",
    "force",
    "moment",
    "pressure",
    "stress",
    "area",
    "volume",
    "density",
    "linear_stiffness",
    "rotational_stiffness",
}

REQUIRED_QUANTITY_FIELDS = {
    "quantity_id",
    "quantity_kind",
    "magnitude",
    "unit_ref",
    "dimension_id",
    "unit_required",
    "missing_unit_behavior",
    "provenance",
}

FORBIDDEN_DEFAULT_TERMS = {
    "assume zero",
    "assumed zero",
    "assume unity",
    "assumed unity",
    "defaults to",
    "default value",
    "fallback default",
    "implicit default",
    "silently",
}


def load_schema():
    with SCHEMA_PATH.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


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


def main():
    schema = load_schema()
    defs = schema["$defs"]

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))

    required_root = {
        "schema_version",
        "unit_system",
        "dimension_records",
        "quantity_records",
        "conversion_declarations",
        "dimension_checks",
        "operation_rules",
        "test_obligations",
        "open_decisions",
        "diagnostics",
    }
    assert required_root <= set(schema["required"])

    dimensions = set(defs["DimensionId"]["enum"])
    assert REQUIRED_DIMENSIONS <= dimensions

    vector = defs["DimensionVector"]
    assert {
        "length",
        "mass",
        "time",
        "temperature",
        "angle",
        "electric_current",
        "substance_amount",
        "luminous_intensity",
    } <= set(vector["required"])

    quantity = defs["QuantityRecord"]
    assert REQUIRED_QUANTITY_FIELDS <= set(quantity["required"])
    assert quantity["additionalProperties"] is False
    assert quantity["properties"]["missing_unit_behavior"]["enum"] == [
        "diagnostic_blocking",
        "diagnostic_warning",
        "not_applicable_explicit_dimensionless",
        "TBD",
    ]

    conversion = defs["ConversionDeclaration"]
    assert {
        "conversion_id",
        "source_unit_ref",
        "target_unit_ref",
        "dimension_id",
        "transform_kind",
        "factor_representation",
        "provenance",
        "review_status",
    } <= set(conversion["required"])

    operation = defs["OperationRule"]
    assert {
        "operation",
        "compatibility_rule",
        "unsupported_behavior",
        "diagnostic_codes",
        "review_status",
    } <= set(operation["required"])
    assert {
        "same_dimension_required",
        "derived_dimension_required",
        "explicit_dimensionless_classification_required",
        "unsupported_until_decision",
    } <= set(operation["properties"]["compatibility_rule"]["enum"])
    assert {
        "addition",
        "subtraction",
        "comparison",
        "conversion",
        "multiplication",
        "division",
        "power",
        "dimensionless_classification",
        "import_validation",
        "export_validation",
        "rule_evaluation",
    } <= set(operation["properties"]["operation"]["enum"])

    test_obligation = defs["TestObligation"]
    assert {
        "test_id",
        "test_kind",
        "required_for",
        "fixture_data_policy",
        "gating_status",
        "evidence_ref",
    } <= set(test_obligation["required"])
    assert "conversion_gated_pending_constants" in test_obligation["properties"]["test_kind"]["enum"]
    assert (
        "no_numeric_conversion_constants_until_approved"
        in test_obligation["properties"]["fixture_data_policy"]["enum"]
    )
    assert "blocked_pending_decision" in test_obligation["properties"]["gating_status"]["enum"]

    open_decision = defs["OpenDecision"]
    assert {
        "decision_id",
        "topic",
        "status",
        "blocking_scope",
        "required_before",
        "owner",
        "notes",
    } <= set(open_decision["required"])
    assert {
        "unit_catalog",
        "base_dimension_vector",
        "dimensionless_classification",
        "conversion_tolerance_policy",
        "offset_temperature_semantics",
        "gauge_absolute_pressure_semantics",
        "diagnostic_code_namespace",
    } <= set(open_decision["properties"]["topic"]["enum"])

    diagnostic_codes = set(defs["UnitDiagnosticCode"]["enum"])
    assert {
        "UNIT_MISSING",
        "UNIT_UNKNOWN",
        "UNIT_AMBIGUOUS",
        "DIMENSION_MISMATCH",
        "CONVERSION_UNSUPPORTED",
        "DIMENSIONLESS_CLASSIFICATION_REQUIRED",
        "PROTECTED_UNIT_DATA_SUSPECTED",
        "CONVERSION_TEST_GATED",
    } <= diagnostic_codes
    assert defs["UnitDiagnostic"]["properties"]["code"]["$ref"] == "#/$defs/UnitDiagnosticCode"

    all_text = "\n".join(walk_strings(schema)).lower()
    for forbidden in FORBIDDEN_DEFAULT_TERMS:
        assert forbidden not in all_text


if __name__ == "__main__":
    main()
