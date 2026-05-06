#!/usr/bin/env python3
"""Focused tests for DEL-13-04 physical-to-analytical transform contract."""

import sys
from copy import deepcopy
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.model_transform.physical_to_analytical.contract import (  # noqa: E402
    transform_physical_to_analytical,
)


FORBIDDEN_CLAIMS = {
    "certification",
    "certified",
    "sealing",
    "sealed",
    "authentication",
    "code compliant",
    "professional approval",
    "engineering acceptance",
}


def provenance(source_name="invented transform fixture"):
    return {
        "source_name": source_name,
        "source_location": f"fixtures/{source_name.replace(' ', '_')}",
        "source_license": "public_permissive",
        "contributor": "DEL-13-04 test",
        "contributor_certification": "invented non-engineering transform fixture",
        "redistribution_status": "public_permissive",
        "review_status": "pending",
    }


def ref(object_type, item_id):
    return {"object_type": object_type, "id": item_id}


def quantity(value=1.0, unit="m", dimension="length"):
    return {
        "value": value,
        "unit": unit,
        "dimension": dimension,
        "provenance": provenance("quantity source"),
    }


def node(node_id, x):
    return {
        "id": node_id,
        "coordinates": {
            "x": quantity(x),
            "y": quantity(0.0),
            "z": quantity(0.0),
        },
        "degrees_of_freedom": {
            "UX": "free",
            "UY": "free",
            "UZ": "free",
            "RX": "free",
            "RY": "free",
            "RZ": "free",
        },
    }


def physical_model():
    return {
        "id": "PHYS-1",
        "name": "Invented physical model",
        "model_role": "physical_source_of_truth",
        "coordinate_system": {"type": "cartesian", "axes": ["X", "Y", "Z"]},
        "nodes": [node("N-2", 2.0), node("N-1", 0.0)],
        "elements": [
            {
                "id": "E-1",
                "element_type": "straight_pipe",
                "start_node_ref": ref("Node", "N-1"),
                "end_node_ref": ref("Node", "N-2"),
                "material_ref": ref("Material", "MAT-1"),
                "section_ref": ref("Section", "SEC-1"),
                "local_coordinate_system": {"type": "cartesian", "axes": ["X", "Y", "Z"]},
                "result_stations": [quantity(0.0), quantity(1.0), quantity(2.0)],
            }
        ],
        "components": [],
        "materials": [
            {
                "id": "MAT-1",
                "name": "Invented material",
                "properties": {
                    "elastic_modulus": quantity(1.0, "Pa", "stress"),
                    "density": quantity(1.0, "kg/m3", "density"),
                },
                "provenance": provenance("material source"),
            }
        ],
        "sections": [
            {
                "id": "SEC-1",
                "section_type": "pipe",
                "properties": {
                    "area": quantity(1.0, "m2", "area"),
                    "second_moment_area": quantity(1.0, "m4", "second_moment_area"),
                },
                "provenance": provenance("section source"),
            }
        ],
        "supports": [
            {
                "id": "SUP-1",
                "support_type": "anchor",
                "target_ref": ref("Node", "N-1"),
                "directions": ["UX", "UY", "UZ", "RX", "RY", "RZ"],
                "properties": {"stiffness": quantity(1.0, "N/m", "stiffness")},
                "provenance": provenance("support source"),
            }
        ],
        "load_cases": [
            {
                "id": "LC-1",
                "name": "Invented load case",
                "load_type": "weight",
                "loads": [
                    {
                        "target_ref": ref("Element", "E-1"),
                        "direction": "Y",
                        "quantity": quantity(1.0, "N", "force"),
                        "provenance": provenance("load source"),
                    }
                ],
                "provenance": provenance("load case source"),
            }
        ],
        "combinations": [],
        "results": [],
        "diagnostics": [],
        "unresolved_assumptions": [],
        "traceability_links": [],
        "design_knowledge_refs": [],
        "constraint_refs": [],
        "equipment_interface_refs": [],
        "operation_refs": [],
        "model_state_refs": [],
        "analysis_run_refs": [],
        "comparison_refs": [],
        "handoff_package_refs": [],
        "external_reference_refs": [],
        "provenance": provenance("model source"),
    }


def codes(result):
    return {item["code"] for item in result.diagnostics}


def all_text(value):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for item in value.values():
            yield from all_text(item)
    elif isinstance(value, (list, tuple)):
        for item in value:
            yield from all_text(item)


def test_transform_is_deterministic_traceable_and_preserves_source_model():
    source = physical_model()
    original = deepcopy(source)

    first = transform_physical_to_analytical(source).to_dict()
    second = transform_physical_to_analytical(deepcopy(source)).to_dict()

    assert first == second
    assert source == original
    analytical = first["analytical_model"]
    assert analytical["model_role"] == "analytical_solver_model"
    assert analytical["source_model_ref"] == ref("Model", "PHYS-1")
    assert [item["id"] for item in analytical["nodes"]] == ["N-1", "N-2"]
    assert [item["id"] for item in analytical["elements"]] == ["E-1"]
    assert {link["source_ref"]["id"] for link in analytical["traceability_links"]} >= {
        "N-1",
        "N-2",
        "E-1",
        "MAT-1",
        "SEC-1",
        "SUP-1",
        "LC-1",
    }
    assert not first["has_blocking_findings"]


def test_missing_units_and_unsupported_physical_records_are_explicit_findings():
    source = physical_model()
    source["nodes"][0]["coordinates"]["x"]["unit"] = ""
    source["elements"].append(
        {
            "id": "E-SOLID",
            "element_type": "solid",
            "start_node_ref": ref("Node", "N-1"),
            "end_node_ref": ref("Node", "N-2"),
            "material_ref": ref("Material", "MAT-1"),
            "section_ref": ref("Section", "SEC-1"),
            "local_coordinate_system": {"type": "cartesian", "axes": ["X", "Y", "Z"]},
            "result_stations": [quantity(0.0)],
        }
    )

    result = transform_physical_to_analytical(source)

    assert {
        "PTA-NODE-COORDINATE-UNIT",
        "PTA-ELEMENT-TYPE-UNSUPPORTED",
        "PTA-ELEMENT-END-NODE-UNRESOLVED",
    } <= codes(result)
    assert result.has_blocking_findings
    assert [item["id"] for item in result.analytical_model["nodes"]] == ["N-1"]
    assert [item["id"] for item in result.analytical_model["elements"]] == []
    assert any(
        link["source_ref"]["id"] == "E-SOLID"
        and link["target_ref"]["object_type"] == "Diagnostic"
        for link in result.traceability_links
    )


def test_transform_output_contains_no_prohibited_authority_claims():
    result = transform_physical_to_analytical(physical_model()).to_dict()
    rendered = "\n".join(all_text(result)).lower()

    for claim in FORBIDDEN_CLAIMS:
        assert claim not in rendered


if __name__ == "__main__":
    test_transform_is_deterministic_traceable_and_preserves_source_model()
    test_missing_units_and_unsupported_physical_records_are_explicit_findings()
    test_transform_output_contains_no_prohibited_authority_claims()
