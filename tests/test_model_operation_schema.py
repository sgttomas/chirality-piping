#!/usr/bin/env python3
"""Stdlib checks for the structured model operation schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "model_operation.schema.json"

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_item",
    "objectives",
    "operation_contract_status",
    "operation_set",
}

REQUIRED_DEFS = {
    "AssumptionRecord",
    "Checksum",
    "Diagnostic",
    "DiffPreviewRef",
    "Id",
    "ModelOperationRecord",
    "OperationAuthorType",
    "OperationChange",
    "OperationContractStatus",
    "OperationKind",
    "OperationPrecondition",
    "OperationSet",
    "OperationStatus",
    "OperationValidation",
    "OperationValuePayload",
    "PrivacyClassification",
    "ProfessionalBoundary",
    "Provenance",
    "Quantity",
    "RedistributionStatus",
    "Reference",
    "ReviewStatus",
    "UnitRequirements",
    "ValidationState",
}

REQUIRED_OPERATION_KINDS = {
    "add",
    "move",
    "modify",
    "delete",
    "reconnect",
    "constraint",
    "load",
    "support",
    "design_knowledge",
}

FORBIDDEN_STATUS = {
    "agent_accepted_engineering_state",
    "auto_approved",
    "code_compliant",
    "certified",
    "sealed",
}

FORBIDDEN_SCHEMA_TEXT = {
    "code compliant",
    "certified by software",
    "sealed by software",
    "professional approval by the software",
}


def load_schema():
    with SCHEMA_PATH.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


def required_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["required"])


def enum_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["enum"])


def walk_keys(value):
    if isinstance(value, dict):
        for key, item in value.items():
            yield key
            yield from walk_keys(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_keys(item)


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
    assert "default" not in set(walk_keys(schema))
    assert REQUIRED_ROOT <= set(schema["required"])
    assert REQUIRED_DEFS <= set(defs)

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-16-01"
    assert schema["properties"]["package_id"]["const"] == "PKG-16"
    assert schema["properties"]["scope_item"]["const"] == "SOW-069"
    assert schema["properties"]["objectives"]["contains"]["const"] == "OBJ-015"

    contract = defs["OperationContractStatus"]["properties"]
    assert contract["record_contract"]["const"] == "schema_first_model_operation_records"
    assert contract["mutation_route"]["const"] == "structured_operations_only"
    assert contract["direct_model_mutation_allowed"]["const"] is False
    assert contract["user_acceptance_boundary"]["const"] == (
        "downstream_user_acceptance_required"
    )
    assert contract["diff_preview_binding"]["const"] == "downstream_DEL-16-02"
    assert contract["audit_trail_binding"]["const"] == "downstream_DEL-16-03"

    assert REQUIRED_OPERATION_KINDS <= enum_at(schema, "OperationKind")
    assert {"user", "agent", "import_adapter", "project_template"} <= enum_at(
        schema, "OperationAuthorType"
    )
    operation_status = enum_at(schema, "OperationStatus")
    assert {
        "proposed",
        "schema_validated",
        "blocked_by_diagnostics",
        "ready_for_user_review",
        "rejected",
    } <= operation_status
    assert operation_status.isdisjoint(FORBIDDEN_STATUS)

    set_required = required_at(schema, "OperationSet")
    assert {
        "operation_set_id",
        "project_ref",
        "model_ref",
        "operations",
        "diagnostics",
        "provenance",
        "professional_boundary",
    } <= set_required

    operation_required = required_at(schema, "ModelOperationRecord")
    assert {
        "operation_id",
        "operation_kind",
        "operation_status",
        "author_type",
        "target_refs",
        "preconditions",
        "changes",
        "validation",
        "diagnostics",
        "diff_preview_refs",
        "assumptions",
        "provenance",
        "professional_boundary",
    } <= operation_required

    precondition_required = required_at(schema, "OperationPrecondition")
    assert {
        "base_model_state_ref",
        "required_current_hashes",
        "required_refs",
        "assumptions",
    } <= precondition_required

    change_required = required_at(schema, "OperationChange")
    assert {
        "change_id",
        "change_kind",
        "target_object_type",
        "value_payload",
        "unit_requirements",
        "provenance",
    } <= change_required
    assert {
        "add_object",
        "remove_object",
        "set_field",
        "move_geometry",
        "reconnect",
        "update_constraint",
        "update_load",
        "update_support",
        "attach_design_knowledge",
    } <= set(defs["OperationChange"]["properties"]["change_kind"]["enum"])

    validation_required = required_at(schema, "OperationValidation")
    assert {
        "schema_validation",
        "constraint_validation",
        "unit_validation",
        "diff_preview_status",
        "application_status",
    } <= validation_required
    assert {
        "not_applied",
        "held_for_user_review",
        "downstream_application_required",
    } <= set(defs["OperationValidation"]["properties"]["application_status"]["enum"])

    units = defs["UnitRequirements"]["properties"]
    assert units["unit_metadata_required"]["const"] is True
    assert units["dimension_check_required"]["const"] is True
    assert units["missing_unit_behavior"]["const"] == "emit_diagnostic"

    professional = defs["ProfessionalBoundary"]["properties"]
    assert professional["human_review_required"]["const"] is True
    assert professional["software_makes_compliance_claim"]["const"] is False
    assert professional["software_makes_certification_claim"]["const"] is False
    assert professional["software_makes_sealing_claim"]["const"] is False
    assert professional["software_makes_approval_claim"]["const"] is False
    assert professional["software_makes_authentication_claim"]["const"] is False

    joined_strings = "\n".join(walk_strings(schema)).lower()
    for forbidden in FORBIDDEN_SCHEMA_TEXT:
        assert forbidden.lower() not in joined_strings


if __name__ == "__main__":
    main()
