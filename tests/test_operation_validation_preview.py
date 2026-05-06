#!/usr/bin/env python3
"""Focused tests for DEL-16-02 operation validation and diff preview."""

from __future__ import annotations

from copy import deepcopy
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.model_operations.validation_preview import (  # noqa: E402
    canonical_json,
    validate_and_preview_operations,
)


def ref(object_type, value):
    return {"object_type": object_type, "ref": value}


def professional_boundary():
    return {
        "human_review_required": True,
        "software_makes_compliance_claim": False,
        "software_makes_certification_claim": False,
        "software_makes_sealing_claim": False,
        "software_makes_approval_claim": False,
        "software_makes_authentication_claim": False,
    }


def validation():
    return {
        "schema_validation": "pending",
        "constraint_validation": "pending",
        "unit_validation": "pending",
        "diff_preview_status": "not_generated",
        "application_status": "not_applied",
    }


def model_state():
    return {
        "model_state": {"state_id": "state:accepted"},
        "entities": [
            {
                "stable_id": "component:pipe-1",
                "category": "Component",
                "diameter": {"value": 100.0, "unit": "mm", "dimension": "length"},
            }
        ],
    }


def operation_envelope(change):
    return {
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-16-01",
        "package_id": "PKG-16",
        "scope_item": "SOW-069",
        "objectives": ["OBJ-015"],
        "operation_set": {
            "operation_set_id": "ops:invented",
            "project_ref": ref("Project", "project:invented"),
            "model_ref": ref("Model", "model:invented"),
            "operations": [
                {
                    "operation_id": "op:resize",
                    "operation_kind": "modify",
                    "operation_status": "proposed",
                    "author_type": "agent",
                    "target_refs": [ref("Component", "component:pipe-1")],
                    "preconditions": {
                        "base_model_state_ref": ref("ModelState", "state:accepted"),
                        "required_current_hashes": [],
                        "required_refs": [ref("Component", "component:pipe-1")],
                        "assumptions": [],
                    },
                    "changes": [change],
                    "validation": validation(),
                    "diagnostics": [],
                    "diff_preview_refs": [],
                    "assumptions": [],
                    "professional_boundary": professional_boundary(),
                }
            ],
            "diagnostics": [],
            "professional_boundary": professional_boundary(),
        },
    }


def quantity_change():
    return {
        "change_id": "change:diameter",
        "change_kind": "set_field",
        "target_object_type": "Component",
        "target_ref": ref("Component", "component:pipe-1"),
        "value_payload": {
            "value_kind": "quantity",
            "scalar_values": [],
            "quantity_values": [{"value": 125.0, "unit": "mm", "dimension": "length"}],
            "reference_values": [],
            "structured_values": [{"diameter": {"value": 125.0, "unit": "mm", "dimension": "length"}}],
            "notes": [],
        },
        "unit_requirements": {
            "unit_metadata_required": True,
            "dimension_check_required": True,
            "missing_unit_behavior": "emit_diagnostic",
        },
    }


def test_valid_operation_generates_stable_preview_without_mutating_state():
    accepted = model_state()
    original = deepcopy(accepted)
    first = validate_and_preview_operations(operation_envelope(quantity_change()), accepted)
    second = validate_and_preview_operations(operation_envelope(quantity_change()), deepcopy(accepted))

    assert canonical_json(first) == canonical_json(second)
    assert accepted == original
    assert first["validation"]["diff_preview_status"] == "generated"
    assert first["validation"]["application_status"] == "not_applied"
    assert first["diff_preview"][0]["before"]["diameter"]["value"] == 100.0
    assert first["diff_preview"][0]["after"]["diameter"]["value"] == 125.0


def test_missing_unit_metadata_blocks_preview():
    change = quantity_change()
    change["value_payload"]["quantity_values"] = [{"value": 125.0}]
    result = validate_and_preview_operations(operation_envelope(change), model_state())

    codes = {item["code"] for item in result["diagnostics"]}
    assert "OP-UNIT-METADATA-MISSING" in codes
    assert result["validation"]["diff_preview_status"] == "blocked_by_validation"
    assert result["diff_preview"][0]["preview_status"] == "blocked_by_validation"


def test_unresolved_target_and_constraint_findings_are_blocking():
    change = quantity_change()
    change["target_ref"] = ref("Component", "component:missing")
    result = validate_and_preview_operations(
        operation_envelope(change),
        model_state(),
        constraint_diagnostics=[{"code": "CV-BLOCK", "severity": "blocking"}],
    )

    codes = {item["code"] for item in result["diagnostics"]}
    assert "OP-TARGET-REF-UNRESOLVED" in codes
    assert "OP-CONSTRAINT-BLOCKING" in codes
    assert result["validation"]["constraint_validation"] == "blocked"


def test_direct_mutation_request_is_rejected():
    envelope = operation_envelope(quantity_change())
    operation = envelope["operation_set"]["operations"][0]
    operation["validation"]["application_status"] = "applied"
    result = validate_and_preview_operations(envelope, model_state())

    assert "OP-DIRECT-MUTATION-BLOCKED" in {item["code"] for item in result["diagnostics"]}
    assert result["validation"]["application_status"] == "not_applied"


def test_output_boundary_language_does_not_make_prohibited_claims():
    result = validate_and_preview_operations(operation_envelope(quantity_change()), model_state())
    text = canonical_json(result).lower()

    for forbidden in [
        "code compliant",
        "certified",
        "sealed",
        "authenticated",
        "professional approval",
        "engineering acceptance",
    ]:
        assert forbidden not in text
    assert result["professional_boundary"]["software_makes_approval_claim"] is False


def main():
    test_valid_operation_generates_stable_preview_without_mutating_state()
    test_missing_unit_metadata_blocks_preview()
    test_unresolved_target_and_constraint_findings_are_blocking()
    test_direct_mutation_request_is_rejected()
    test_output_boundary_language_does_not_make_prohibited_claims()


if __name__ == "__main__":
    main()
