#!/usr/bin/env python3
"""Stdlib checks for the canonical domain model schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "model.schema.yaml"

REQUIRED_DEFS = {
    "Assumption",
    "Project",
    "Model",
    "ModelRole",
    "Node",
    "Element",
    "Component",
    "Material",
    "Section",
    "Support",
    "LoadCase",
    "Combination",
    "RulePackRef",
    "Result",
    "ReportSettings",
    "Report",
    "Quantity",
    "Provenance",
    "Diagnostic",
    "Checksum",
    "Reference",
    "TraceabilityLink",
}

REQUIRED_ANALYSIS_STATUSES = {
    "MODEL_INCOMPLETE",
    "MECHANICS_SOLVED",
    "RULE_INPUTS_INCOMPLETE",
    "USER_RULE_CHECKED",
    "USER_RULE_FAILED",
    "HUMAN_REVIEW_REQUIRED",
    "HUMAN_APPROVED_FOR_PROJECT",
}

FORBIDDEN_SCHEMA_TEXT = {
    "CODE_COMPLIANT",
    "ASME",
    "B31",
    "certified",
    "sealed",
    "automatic compliance",
    "professional approval by the software",
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


def required_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["required"])


def enum_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["enum"])


def main():
    schema = load_schema()
    defs = schema["$defs"]

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))
    assert {"schema_version", "project"} <= set(schema["required"])
    assert REQUIRED_DEFS <= set(defs)

    assert REQUIRED_ANALYSIS_STATUSES <= enum_at(schema, "AnalysisStatus")
    assert "CODE_COMPLIANT" not in enum_at(schema, "AnalysisStatus")

    assert {
        "id",
        "name",
        "unit_system",
        "privacy_class",
        "storage_policy",
        "models",
        "rule_pack_refs",
        "report_settings",
        "reports",
        "diagnostics",
        "hashes",
    } <= required_at(schema, "Project")

    assert {
        "model_role",
        "nodes",
        "elements",
        "components",
        "materials",
        "sections",
        "supports",
        "load_cases",
        "combinations",
        "results",
        "diagnostics",
        "unresolved_assumptions",
        "traceability_links",
        "design_knowledge_refs",
        "constraint_refs",
        "equipment_interface_refs",
        "operation_refs",
        "model_state_refs",
        "analysis_run_refs",
        "comparison_refs",
        "handoff_package_refs",
        "external_reference_refs",
        "provenance",
    } <= required_at(schema, "Model")
    assert {
        "physical_source_of_truth",
        "analytical_solver_model",
        "derived_view",
        "TBD",
    } <= enum_at(schema, "ModelRole")

    assert {"id", "statement", "status", "affected_refs", "provenance"} <= required_at(
        schema, "Assumption"
    )
    assert {"unresolved", "resolved", "rejected", "TBD"} <= set(
        defs["Assumption"]["properties"]["status"]["enum"]
    )

    assert {
        "id",
        "trace_type",
        "source_ref",
        "target_ref",
        "diagnostics",
        "provenance",
    } <= required_at(schema, "TraceabilityLink")
    assert {
        "physical_to_analytical",
        "operation_application",
        "state_snapshot",
        "analysis_run",
        "comparison",
        "handoff",
        "external_reference",
        "TBD",
    } <= set(defs["TraceabilityLink"]["properties"]["trace_type"]["enum"])

    assert {
        "DesignKnowledge",
        "Constraint",
        "EquipmentInterface",
        "ModelOperation",
        "ModelState",
        "AnalysisRun",
        "Comparison",
        "HandoffPackage",
        "ExternalProverReference",
        "Assumption",
        "TraceabilityLink",
    } <= set(defs["Reference"]["properties"]["object_type"]["enum"])

    assert {"value", "unit", "dimension", "provenance"} <= required_at(
        schema, "Quantity"
    )
    assert {
        "source_name",
        "source_location",
        "source_license",
        "contributor",
        "contributor_certification",
        "redistribution_status",
        "review_status",
    } <= required_at(schema, "Provenance")

    assert {
        "code",
        "class",
        "severity",
        "source",
        "affected_object",
        "message",
        "remediation",
        "provenance",
    } <= required_at(schema, "Diagnostic")
    assert {
        "SOLVE_BLOCKING",
        "RULE_CHECK_BLOCKING",
        "PROVENANCE_WARNING",
        "ASSUMPTION_WARNING",
        "NONLINEAR_WARNING",
        "IP_BOUNDARY_WARNING",
    } <= set(defs["Diagnostic"]["properties"]["class"]["enum"])

    assert {"algorithm", "canonicalization", "payload_ref", "value"} <= required_at(
        schema, "Checksum"
    )
    assert {"JCS", "NONE", "TBD"} <= set(
        defs["Checksum"]["properties"]["canonicalization"]["enum"]
    )

    assert {"target_ref", "quantity", "provenance"} <= required_at(
        schema, "LoadRecord"
    )
    assert {
        "id",
        "support_type",
        "target_ref",
        "directions",
        "properties",
        "provenance",
    } <= required_at(schema, "Support")

    assert {"diagnostics", "provenance"} <= required_at(schema, "Result")
    assert {
        "input_manifest",
        "hashes",
        "analysis_statuses",
        "diagnostics",
        "rule_pack_refs",
        "provenance_summary",
        "professional_boundary_notice",
    } <= required_at(schema, "Report")
    assert {"checksum", "source_notice", "required_input_refs"} <= required_at(
        schema, "RulePackRef"
    )

    all_text = "\n".join(walk_strings(schema))
    for forbidden in FORBIDDEN_SCHEMA_TEXT:
        assert forbidden not in all_text


if __name__ == "__main__":
    main()
