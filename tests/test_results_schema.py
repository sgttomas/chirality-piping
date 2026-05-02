#!/usr/bin/env python3
"""Stdlib checks for the result export schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "results.schema.yaml"

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_item",
    "objectives",
    "export_format_status",
    "result_envelope",
}

REQUIRED_DEFS = {
    "AnalysisStatus",
    "Checksum",
    "Diagnostic",
    "DimensionId",
    "DownstreamUse",
    "ExportFormatStatus",
    "ProfessionalBoundary",
    "Provenance",
    "QuantityResult",
    "Reference",
    "Reproducibility",
    "ResultEnvelope",
    "ResultFamily",
    "ResultSet",
    "RulePackRef",
    "SolverVersion",
}

REQUIRED_FAMILIES = {
    "displacement",
    "rotation",
    "force",
    "moment",
    "reaction",
    "stress",
    "ratio",
    "rule_check",
}

REQUIRED_DIAGNOSTIC_CLASSES = {
    "SOLVE_BLOCKING",
    "RULE_CHECK_BLOCKING",
    "PROVENANCE_WARNING",
    "ASSUMPTION_WARNING",
    "NONLINEAR_WARNING",
    "IP_BOUNDARY_WARNING",
    "UNIT_WARNING",
    "EXPORT_BLOCKING",
}

FORBIDDEN_STATUS = {
    "HUMAN_APPROVED_FOR_PROJECT",
    "CODE_COMPLIANT",
    "CERTIFIED",
    "SEALED",
    "APPROVED",
}

FORBIDDEN_FORMAT_COMMITMENTS = {
    "csv",
    "spreadsheet",
    "hdf5",
    "local_fea",
    "openapi_transport",
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


def main():
    schema = load_schema()
    defs = schema["$defs"]

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))
    assert REQUIRED_ROOT <= set(schema["required"])
    assert REQUIRED_DEFS <= set(defs)

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-08-04"
    assert schema["properties"]["package_id"]["const"] == "PKG-08"
    assert schema["properties"]["scope_item"]["const"] == "SOW-046"
    assert {"OBJ-007", "OBJ-009"} <= set(
        schema["properties"]["objectives"]["items"]["enum"]
    )

    export_status = defs["ExportFormatStatus"]["properties"]
    assert export_status["baseline_format"]["const"] == (
        "schema_first_json_result_envelope"
    )
    assert export_status["additional_formats"]["const"] == "TBD"
    assert export_status["public_transport_protocol"]["const"] == "TBD"
    assert export_status["local_fea_package_format"]["const"] == "TBD"
    assert export_status["external_adapter_formats"]["const"] == "TBD"
    for name in FORBIDDEN_FORMAT_COMMITMENTS:
        assert name not in {
            export_status["additional_formats"]["const"].lower(),
            export_status["public_transport_protocol"]["const"].lower(),
            export_status["local_fea_package_format"]["const"].lower(),
            export_status["external_adapter_formats"]["const"].lower(),
        }

    envelope_required = required_at(schema, "ResultEnvelope")
    assert {
        "schema_version",
        "envelope_id",
        "model_ref",
        "run_ref",
        "solver_version",
        "unit_system_ref",
        "load_basis_refs",
        "result_sets",
        "diagnostics",
        "provenance",
        "reproducibility",
        "analysis_status",
        "professional_boundary",
        "downstream_use",
    } <= envelope_required

    status = enum_at(schema, "AnalysisStatus")
    assert {
        "MODEL_INCOMPLETE",
        "MECHANICS_SOLVED",
        "RULE_INPUTS_INCOMPLETE",
        "USER_RULE_CHECKED",
        "USER_RULE_FAILED",
        "HUMAN_REVIEW_REQUIRED",
    } <= status
    assert status.isdisjoint(FORBIDDEN_STATUS)
    assert (
        defs["ResultEnvelope"]["properties"]["analysis_status"]["contains"]["const"]
        == "HUMAN_REVIEW_REQUIRED"
    )

    quantity_required = required_at(schema, "QuantityResult")
    assert {
        "result_id",
        "family",
        "object_ref",
        "basis_ref",
        "magnitude",
        "unit",
        "dimension",
        "provenance",
    } <= quantity_required
    assert REQUIRED_FAMILIES <= enum_at(schema, "ResultFamily")

    diagnostic_required = required_at(schema, "Diagnostic")
    assert {
        "code",
        "class",
        "severity",
        "source",
        "affected_object",
        "message",
        "remediation",
        "provenance",
    } <= diagnostic_required
    assert REQUIRED_DIAGNOSTIC_CLASSES <= set(
        defs["Diagnostic"]["properties"]["class"]["enum"]
    )

    rule_pack_required = required_at(schema, "RulePackRef")
    assert {
        "rule_pack_id",
        "version",
        "checksum",
        "source_notice",
        "redistribution_status",
        "completeness_status",
        "private_payload_redacted",
    } <= rule_pack_required
    assert defs["RulePackRef"]["properties"]["private_payload_redacted"]["const"] is True

    boundary = defs["ProfessionalBoundary"]["properties"]
    assert boundary["human_review_required"]["const"] is True
    assert boundary["software_makes_compliance_claim"]["const"] is False
    assert boundary["software_makes_certification_claim"]["const"] is False
    assert boundary["software_makes_sealing_claim"]["const"] is False
    assert boundary["software_makes_approval_claim"]["const"] is False
    assert boundary["software_makes_authentication_claim"]["const"] is False

    downstream = defs["DownstreamUse"]["properties"]
    assert downstream["review"]["const"] is True
    assert downstream["regression_comparison"]["const"] is True
    assert downstream["report_consumption"]["const"] is True
    assert downstream["headless_automation"]["const"] is True
    assert downstream["governed_downstream_tooling"]["const"] is True
    assert downstream["additional_export_formats"]["const"] == "TBD"


if __name__ == "__main__":
    main()
