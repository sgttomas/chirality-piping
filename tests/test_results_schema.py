#!/usr/bin/env python3
"""Stdlib checks for the result export schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "results.schema.yaml"
PRODUCT_PREVIEW_RESULT_PATH = (
    ROOT / "fixtures" / "product_preview" / "invented_mechanics_result.json"
)

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
    "ResultMetadata",
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


def load_product_preview_result():
    with PRODUCT_PREVIEW_RESULT_PATH.open(encoding="utf-8") as fixture_file:
        return json.load(fixture_file)


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
    assert "metadata" not in quantity_required
    assert (
        defs["QuantityResult"]["properties"]["metadata"]["$ref"]
        == "#/$defs/ResultMetadata"
    )
    assert {
        "component",
        "coordinate_system",
        "location",
        "basis",
        "sign_convention",
    } <= required_at(schema, "ResultMetadata")
    metadata = defs["ResultMetadata"]["properties"]
    assert {
        "axial_force",
        "torsional_moment",
        "bending_moment_y",
        "bending_moment_z",
        "axial_normal_stress",
        "bending_normal_stress_y",
        "bending_normal_stress_z",
        "torsional_shear_stress",
        "pressure_hoop_stress",
        "pressure_longitudinal_stress",
    } <= set(metadata["component"]["enum"])
    assert {"element_local", "pipe_section"} <= set(metadata["coordinate_system"]["enum"])
    assert {"end_i", "end_j"} <= set(metadata["location"]["enum"])
    assert (
        "recovered_from_local_element_stiffness"
        in set(metadata["basis"]["enum"])
    )
    assert (
        "recovered_from_open_mechanics_stress_components"
        in set(metadata["basis"]["enum"])
    )
    assert "interpolated_from_endpoint_resultants" in set(metadata["basis"]["enum"])
    assert "explicit_user_linear_combination" in set(metadata["basis"]["enum"])
    quantity_condition = defs["QuantityResult"]["allOf"][0]
    assert set(quantity_condition["if"]["properties"]["family"]["enum"]) == {
        "force",
        "moment",
    }
    assert "metadata" in quantity_condition["then"]["required"]
    assert REQUIRED_FAMILIES <= enum_at(schema, "ResultFamily")

    preview_result = load_product_preview_result()
    axial_force = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:force:pipe-P-120:axial"
    )
    axial_force_end_j = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:force:pipe-P-120:axial:end-j"
    )
    axial_force_midspan = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:force:pipe-P-120:midspan:axial"
    )
    axial_metadata = axial_force["metadata"]
    axial_end_j_metadata = axial_force_end_j["metadata"]
    axial_midspan_metadata = axial_force_midspan["metadata"]
    torsional_stress_end_j = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:stress:pipe-P-120:end-j:torsional-shear"
    )
    torsional_stress_midspan = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:stress:pipe-P-120:midspan:torsional-shear"
    )
    combination_axial_force = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial"
    )
    pressure_hoop = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:stress:pipe-P-120:end-i:pressure-hoop"
    )
    stress_summary = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:stress:pipe-P-120"
    )
    assert axial_force["unit"] == "N"
    assert axial_force["basis_ref"] == {"ref_type": "load_case", "ref_id": "load:L-100"}
    assert axial_metadata["component"] in metadata["component"]["enum"]
    assert axial_metadata["coordinate_system"] in metadata["coordinate_system"]["enum"]
    assert axial_metadata["location"] in metadata["location"]["enum"]
    assert axial_metadata["basis"] in metadata["basis"]["enum"]
    assert axial_metadata["sign_convention"]
    assert axial_force_end_j["unit"] == "N"
    assert axial_end_j_metadata["component"] == "axial_force"
    assert axial_end_j_metadata["coordinate_system"] == "element_local"
    assert axial_end_j_metadata["location"] == "end_j"
    assert axial_end_j_metadata["basis"] in metadata["basis"]["enum"]
    assert "j-end" in axial_end_j_metadata["sign_convention"]
    assert axial_force_midspan["unit"] == "N"
    assert axial_midspan_metadata["component"] == "axial_force"
    assert axial_midspan_metadata["coordinate_system"] == "element_local"
    assert axial_midspan_metadata["location"] == "midspan"
    assert axial_midspan_metadata["basis"] == "interpolated_from_endpoint_resultants"
    assert stress_summary["kind"] == "open_formula_stress_summary"
    assert "metadata" not in stress_summary
    assert torsional_stress_end_j["unit"] == "MPa"
    assert torsional_stress_end_j["metadata"]["component"] == "torsional_shear_stress"
    assert torsional_stress_end_j["metadata"]["coordinate_system"] == "element_local"
    assert torsional_stress_end_j["metadata"]["location"] == "end_j"
    assert (
        torsional_stress_end_j["metadata"]["basis"]
        == "recovered_from_open_mechanics_stress_components"
    )
    assert torsional_stress_midspan["unit"] == "MPa"
    assert torsional_stress_midspan["metadata"]["component"] == "torsional_shear_stress"
    assert torsional_stress_midspan["metadata"]["coordinate_system"] == "element_local"
    assert torsional_stress_midspan["metadata"]["location"] == "midspan"
    assert (
        torsional_stress_midspan["metadata"]["basis"]
        == "interpolated_from_endpoint_resultants"
    )
    assert pressure_hoop["unit"] == "MPa"
    assert pressure_hoop["metadata"]["component"] == "pressure_hoop_stress"
    assert pressure_hoop["metadata"]["coordinate_system"] == "pipe_section"
    assert combination_axial_force["unit"] == "N"
    assert combination_axial_force["basis_ref"] == {
        "ref_type": "combination",
        "ref_id": "combination:C-OPER-ALT",
    }
    assert combination_axial_force["source_result_refs"] == [
        "result:force:pipe-P-120:axial",
        "result:loadcase:load-L-200:force:pipe-P-120:axial",
    ]
    assert combination_axial_force["metadata"]["basis"] == "explicit_user_linear_combination"
    assert combination_axial_force["metadata"]["basis"] in metadata["basis"]["enum"]

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
