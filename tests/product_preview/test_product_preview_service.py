#!/usr/bin/env python3
"""Tests for the TP-MAC-01 product-preview service slice."""

from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.product_preview import (  # noqa: E402
    build_agent_proposal_preview,
    build_model_tree,
    load_design_knowledge,
    load_preview_model,
    run_preview_mechanics,
    validate_preview_model,
)
from core.product_preview.service import canonical_json  # noqa: E402


def test_preview_model_fixture_is_valid_and_invented():
    model = load_preview_model()
    validation = validate_preview_model(model)

    assert model["document_kind"] == "openpipestress.product_preview.model"
    assert validation["status"] == "passed"
    assert model["data_boundary"]["public_examples_policy"] == "invented_or_cleared_data_only"
    assert "protected_owner_or_standards_data" in model["data_boundary"]["protected_source_policy"]
    assert "code compliant" not in canonical_json(model).lower()


def test_model_tree_preserves_stable_entity_ids():
    tree = build_model_tree()
    entity_ids = {item["id"] for item in tree["entities"]}

    assert {"node:N-100", "pipe:P-120", "support:S-130", "component:C-140"} <= entity_ids
    assert all(item["properties"] for item in tree["entities"])


def test_design_knowledge_has_visible_provenance_and_diagnostics():
    knowledge = load_design_knowledge()

    assert knowledge["records"]
    assert all(record["provenance"] == "invented_example" for record in knowledge["records"])
    assert {item["code"] for item in knowledge["diagnostics"]} == {
        "RULE_CHECK_NOT_PERFORMED",
        "SUPPORT_STIFFNESS_UNRESOLVED",
    }


def test_mechanics_result_keeps_status_boundaries_separate():
    result = run_preview_mechanics()

    assert result["status"]["mechanics"] == "MECHANICS_SOLVED"
    assert result["status"]["rule_check"] == "RULE_INPUTS_INCOMPLETE"
    assert result["status"]["professional_acceptance"] == "NOT_PROVIDED"
    assert result["accepted_model_state_mutated"] is False
    assert "RULE_CHECK_INPUTS_MISSING" in {item["code"] for item in result["diagnostics"]}
    assert result["summary"]["max_displacement"]["result_ref"] == "result:disp:node-N-140"


def test_agent_proposal_is_review_only_and_non_mutating():
    preview = build_agent_proposal_preview()

    assert preview["application_status"] == "not_applied"
    assert preview["accepted_model_state_mutated"] is False
    assert preview["proposal"]["audit_boundary"]["requires_user_acceptance"] is True
    assert preview["proposal"]["audit_boundary"]["mutates_accepted_model_state"] is False
    assert "certified" not in canonical_json(preview).lower()
