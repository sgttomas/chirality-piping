#!/usr/bin/env python3
"""Stdlib checks for the code-neutral analysis boundary schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "analysis_boundary.schema.yaml"

FORBIDDEN_AUTOMATIC = {
    "CODE_COMPLIANT",
    "COMPLIANT",
    "CERTIFIED",
    "SEALED",
    "APPROVED",
    "HUMAN_APPROVED_FOR_PROJECT",
}

FORBIDDEN_CLAIM_FIELDS = {
    "software_makes_compliance_claim",
    "software_makes_certification_claim",
    "software_makes_sealing_claim",
    "software_makes_professional_acceptance_claim",
}

REQUIRED_RULE_PACK_REF_FIELDS = {
    "rule_pack_id",
    "version",
    "checksum",
    "source_notice",
    "supplied_by_user",
    "redistribution_status",
    "provenance",
}

REQUIRED_PROVENANCE_FIELDS = {
    "source_name",
    "source_location",
    "source_license",
    "contributor",
    "redistribution_status",
    "review_status",
}

REQUIRED_FORBIDDEN_CLAIMS = {
    "code_compliance",
    "certification",
    "sealing",
    "professional_acceptance",
}


def load_schema():
    with SCHEMA_PATH.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


def enum_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["enum"])


def required_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["required"])


def main():
    schema = load_schema()
    defs = schema["$defs"]

    assert "authority_model" in schema["required"]
    authority_model = defs["AuthorityModel"]
    assert authority_model["properties"]["mechanics_authority"]["const"] == (
        "solver_result_only"
    )
    assert authority_model["properties"]["rule_check_authority"]["const"] == (
        "software_computation_using_user_data"
    )
    assert authority_model["properties"]["human_acceptance_authority"]["const"] == (
        "external_hash_bound_human_record_only"
    )
    assert authority_model["properties"]["automatic_status_scope"]["items"][
        "$ref"
    ].endswith("/AutomaticAnalysisStatus")
    forbidden_claim_values = set(
        authority_model["properties"]["forbidden_software_claims"]["items"]["enum"]
    )
    assert REQUIRED_FORBIDDEN_CLAIMS <= forbidden_claim_values

    automatic = enum_at(schema, "AutomaticAnalysisStatus")
    assert automatic.isdisjoint(FORBIDDEN_AUTOMATIC)
    for status in automatic:
        assert not any(forbidden == status for forbidden in FORBIDDEN_AUTOMATIC)

    mechanics_statuses = set(defs["MechanicsSolveBoundary"]["properties"]["status"]["enum"])
    assert mechanics_statuses == {"MODEL_INCOMPLETE", "MECHANICS_SOLVED"}

    rule_statuses = set(defs["UserRuleCheckBoundary"]["properties"]["status"]["enum"])
    assert rule_statuses == {
        "RULE_INPUTS_INCOMPLETE",
        "USER_RULE_CHECKED",
        "USER_RULE_FAILED",
    }
    assert rule_statuses.isdisjoint(FORBIDDEN_AUTOMATIC)

    professional_boundary = defs["ProfessionalBoundary"]["properties"]
    for field in FORBIDDEN_CLAIM_FIELDS:
        assert professional_boundary[field]["const"] is False
    assert professional_boundary["human_review_required"]["const"] is True

    user_rule_check = defs["UserRuleCheckBoundary"]
    assert "rule_pack_ref" in user_rule_check["required"]
    assert user_rule_check["properties"]["rule_pack_ref"]["$ref"].endswith("/RulePackRef")

    rule_pack_ref = defs["RulePackRef"]
    assert REQUIRED_RULE_PACK_REF_FIELDS <= required_at(schema, "RulePackRef")
    assert rule_pack_ref["properties"]["supplied_by_user"]["const"] is True
    assert rule_pack_ref["properties"]["provenance"]["$ref"].endswith("/Provenance")

    assert REQUIRED_PROVENANCE_FIELDS <= required_at(schema, "Provenance")

    human_ref = defs["HumanAcceptanceRef"]
    assert human_ref["properties"]["human_status"]["const"] == "HUMAN_APPROVED_FOR_PROJECT"
    assert "bound_evidence_hashes" in human_ref["required"]
    assert "human_acceptance_refs" in schema["required"]

    missing_input = defs["MissingInputFinding"]
    assert "diagnostic_ref" in missing_input["required"]
    assert set(missing_input["properties"]["status"]["enum"]) == {
        "MODEL_INCOMPLETE",
        "RULE_INPUTS_INCOMPLETE",
        "TBD",
    }


if __name__ == "__main__":
    main()
