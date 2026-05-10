#!/usr/bin/env python3
"""Focused tests for TP-PER-01 project persistence service behavior."""

import json
import sys
from copy import deepcopy
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.project_persistence import (
    canonical_json,
    project_hash_manifest,
    round_trip_project_envelope,
    validate_project_persistence_envelope,
)


FIXTURE_PATH = ROOT / "fixtures" / "persistence" / "invented_persisted_preview_project.json"


def persisted_project():
    return json.loads(FIXTURE_PATH.read_text(encoding="utf-8"))


def test_invented_persistence_fixture_validates_with_run_history_refs():
    envelope = persisted_project()
    run_history = envelope["project"]["run_history"]

    assert envelope["document_kind"] == "openpipestress.project_persistence"
    assert envelope["physical_container"]["status"] == "TBD"
    assert envelope["validation_profile"]["telemetry_default"] == "off"
    assert envelope["project"]["private_data"]["default_transmission_allowed"] is False
    assert validate_project_persistence_envelope(envelope) == []
    assert {item["ref"] for item in run_history["analysis_run_refs"]} == {
        "run:preview-linear-static-001"
    }
    assert "result-envelope:run:preview-linear-static-001" in {
        item["ref"] for item in run_history["result_envelope_refs"]
    }
    assert "result:stress:pipe-P-120:end-j:torsional-shear" in {
        item["ref"] for item in run_history["result_refs"]
    }


def test_canonical_hashes_are_stable_and_cover_project_payloads():
    envelope = persisted_project()
    cloned = json.loads(canonical_json(envelope))

    assert canonical_json(envelope) == canonical_json(cloned)
    assert project_hash_manifest(envelope) == envelope["hash"]["hash_manifest"]
    scopes = {item["payload_scope"] for item in envelope["hash"]["hash_manifest"]}
    assert {"project_payload", "model_payload", "project_envelope"} <= scopes


def test_round_trip_keeps_semantic_equality_and_hashes():
    round_trip = round_trip_project_envelope(persisted_project())

    assert round_trip["serialization"] == "canonical_json_jcs"
    assert round_trip["semantic_equal"] is True
    assert round_trip["diagnostics"] == []
    assert round_trip["source_hash"]["value"] == round_trip["round_trip_hash"]["value"]


def test_mutation_changes_hash_and_validation_reports_mismatch():
    envelope = persisted_project()
    mutated = deepcopy(envelope)
    mutated["project"]["model_payload"]["project"]["name"] = "Changed invented project"

    assert project_hash_manifest(envelope) != project_hash_manifest(mutated)
    codes = {item["code"] for item in validate_project_persistence_envelope(mutated)}
    assert "PERSISTENCE_PROJECT_HASH_MISMATCH" in codes
    assert "PERSISTENCE_HASH_MANIFEST_MISMATCH" in codes


def test_missing_boundary_fields_return_structured_diagnostics():
    envelope = persisted_project()
    del envelope["project"]["provenance_manifest"]
    del envelope["project"]["private_data"]
    del envelope["professional_boundary"]
    envelope["project"]["run_history"]["result_refs"] = []

    codes = {item["code"] for item in validate_project_persistence_envelope(envelope)}

    assert "PERSISTENCE_PROVENANCE_MISSING" in codes
    assert "PERSISTENCE_PRIVATE_DATA_BOUNDARY_INVALID" in codes
    assert "PERSISTENCE_PROFESSIONAL_BOUNDARY_VIOLATION" in codes
    assert "PERSISTENCE_RUN_HISTORY_REFS_MISSING" in codes
