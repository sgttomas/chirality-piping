#!/usr/bin/env python3
"""Stdlib smoke checks for invented educational model examples."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
EXAMPLE_DIR = ROOT / "examples" / "models" / "invented"
EXAMPLE_FILES = {
    "mechanics_only_toy_span.json",
    "fake_rule_pack_toy_model.json",
}

REQUIRED_PROJECT_KEYS = {
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
}

REQUIRED_MODEL_KEYS = {
    "id",
    "name",
    "model_role",
    "coordinate_system",
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
}

REQUIRED_NOTICE_TEXT = {
    "invented",
    "non-code",
    "non-project",
    "not suitable for engineering reliance",
}

FORBIDDEN_TEXT = {
    "A" + "SME",
    "B" + "31",
    "B" + "31J",
    "allowable " + "stress",
    "stress " + "intensification",
    "SIF " + "table",
    "vendor " + "catalog",
    "real " + "secret",
    "private " + "key",
    "BEGIN " + "RSA",
    "BEGIN " + "OPENSSH",
    "pass" + "word",
    "token" + "=",
    "CAE" + "SAR",
    "Auto" + "PIPE",
    "RO" + "HR2",
    "code " + "compliant",
    "certified by " + "openpipestress",
    "sealed by " + "openpipestress",
    "approved by " + "openpipestress",
    "professional " + "approval " + "by the software",
    "OPS_SYNTHETIC_" + "PROTECTED_TABLE",
    "OPS_SYNTHETIC_" + "CODE_FORMULA",
    "OPS_SYNTHETIC_" + "PRIVATE_RULE_PAYLOAD",
    "OPS_SYNTHETIC_" + "VENDOR_CATALOG",
}


def load_examples():
    assert EXAMPLE_DIR.is_dir()
    paths = sorted(EXAMPLE_DIR.glob("*.json"))
    assert {path.name for path in paths} == EXAMPLE_FILES
    return [(path, json.loads(path.read_text(encoding="utf-8"))) for path in paths]


def walk_strings(value):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for item in value.values():
            yield from walk_strings(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_strings(item)


def test_invented_examples_follow_model_contract_shape():
    for path, example in load_examples():
        assert example["schema_version"] == "0.1.0", path
        project = example["project"]
        assert REQUIRED_PROJECT_KEYS <= set(project), path
        assert project["privacy_class"] == "public", path
        assert project["storage_policy"] == "public_example", path
        assert project["models"], path

        for model in project["models"]:
            assert REQUIRED_MODEL_KEYS <= set(model), (path, model["id"])
            assert model["model_role"] == "analytical_solver_model"
            assert model["nodes"]
            assert model["elements"]
            assert model["materials"]
            assert model["sections"]
            assert model["load_cases"]
            assert model["combinations"]
            assert model["results"]


def test_invented_examples_carry_non_reliance_notices():
    for path, example in load_examples():
        text = "\n".join(walk_strings(example)).lower()
        for required in REQUIRED_NOTICE_TEXT:
            assert required in text, (path, required)
        assert "HUMAN_" + "APPROVED_FOR_PROJECT" not in text


def test_invented_examples_do_not_contain_protected_or_private_markers():
    for path, example in load_examples():
        text = "\n".join(walk_strings(example))
        text_lower = text.lower()
        for forbidden in FORBIDDEN_TEXT:
            haystack = text if forbidden != forbidden.lower() else text_lower
            needle = forbidden if forbidden != forbidden.lower() else forbidden.lower()
            assert needle not in haystack, (path, forbidden)


def test_fake_rule_example_links_only_invented_public_rule_pack():
    _, fake_rule = next(
        (path, example)
        for path, example in load_examples()
        if path.name == "fake_rule_pack_toy_model.json"
    )
    refs = fake_rule["project"]["rule_pack_refs"]
    assert len(refs) == 1
    ref = refs[0]
    assert ref["id"] == "INV_FAKE_RULE_PACK_REF"
    assert ref["redistribution_status"] == "public_permissive"
    assert "invented" in ref["source_notice"].lower()
    assert "engineering acceptance basis" in ref["source_notice"].lower()
