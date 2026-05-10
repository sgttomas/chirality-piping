"""Schema-adjacent service functions for the invented desktop preview.

This module deliberately stays local and deterministic. It loads invented
fixtures, validates the product-preview shape used by the desktop slice, and
returns status envelopes without running external solvers or applying hidden
model mutations.
"""

from __future__ import annotations

from copy import deepcopy
import json
from pathlib import Path
from typing import Any, Mapping

from core.analysis_runs import build_preview_analysis_run_envelope


ROOT = Path(__file__).resolve().parents[2]
FIXTURE_DIR = ROOT / "fixtures" / "product_preview"

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
}

PROHIBITED_TEXT = (
    "code compliant",
    "certified",
    "sealed",
    "professional approval",
    "approved for construction",
    "release ready",
    "production ready",
)


def load_preview_model(path: Path | None = None) -> dict[str, Any]:
    """Load the invented preview model fixture."""

    return _read_json(path or FIXTURE_DIR / "invented_preview_model.json")


def load_design_knowledge(path: Path | None = None) -> dict[str, Any]:
    """Load invented design-knowledge records for the preview model."""

    return _read_json(path or FIXTURE_DIR / "invented_design_knowledge.json")


def run_preview_mechanics(model: Mapping[str, Any] | None = None) -> dict[str, Any]:
    """Return a deterministic mechanics/status envelope for the preview.

    The current technical preview consumes an invented result fixture. It does
    not perform external solver/prover execution or rule-pack checking.
    """

    model_record = deepcopy(dict(model or load_preview_model()))
    result = _read_json(FIXTURE_DIR / "invented_mechanics_result.json")
    diagnostics = list(result.get("diagnostics", []))
    diagnostics.extend(validate_preview_model(model_record)["diagnostics"])
    result["diagnostics"] = _stable(diagnostics)
    result["professional_boundary"] = deepcopy(PROFESSIONAL_BOUNDARY)
    result["accepted_model_state_mutated"] = False
    return result


def build_analysis_run_preview(model: Mapping[str, Any] | None = None) -> dict[str, Any]:
    """Build an immutable analysis-run record from the preview mechanics result."""

    mechanics_result = run_preview_mechanics(model)
    return build_preview_analysis_run_envelope(
        mechanics_result,
        model_state_ref={
            "object_type": "ModelState",
            "ref": f"state:{mechanics_result['model_ref']}:preview",
        },
        settings_ref={
            "object_type": "SolverSettings",
            "ref": "settings:preview-linear-static",
        },
        unit_system_ref={
            "object_type": "UnitSystem",
            "ref": "SI-preview-units",
        },
        load_basis_refs=[
            {
                "object_type": "LoadCase",
                "ref": "loadcase:LC-OPERATING-PREVIEW",
            }
        ],
    )


def build_agent_proposal_preview() -> dict[str, Any]:
    """Return a deterministic, non-mutating proposal preview."""

    proposal = _read_json(FIXTURE_DIR / "invented_agent_proposal.json")
    return {
        "schema_version": "0.1.0",
        "document_kind": "openpipestress.product_preview.agent_proposal_preview",
        "proposal": proposal,
        "validation": deepcopy(proposal["validation"]),
        "diff_preview": deepcopy(proposal["operation"]["changes"]),
        "application_status": "not_applied",
        "accepted_model_state_mutated": False,
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
        "diagnostics": [],
    }


def build_model_tree(model: Mapping[str, Any] | None = None) -> dict[str, Any]:
    """Build simple tree/inspector input records for the desktop UI."""

    record = deepcopy(dict(model or load_preview_model()))
    entities = []
    for collection_name, ref_type in (
        ("nodes", "node"),
        ("pipe_segments", "pipe"),
        ("supports", "support"),
        ("components", "component"),
    ):
        for item in record.get(collection_name, []):
            entities.append(
                {
                    "id": item["id"],
                    "type": ref_type,
                    "label": item.get("label", item["id"]),
                    "properties": _properties(item),
                }
            )
    return {
        "schema_version": "0.1.0",
        "document_kind": "openpipestress.product_preview.model_tree",
        "entities": sorted(entities, key=lambda item: (item["type"], item["id"])),
    }


def validate_preview_model(model: Mapping[str, Any] | None = None) -> dict[str, Any]:
    """Validate the bounded product-preview shape and guardrail text."""

    record = deepcopy(dict(model or load_preview_model()))
    diagnostics: list[dict[str, str]] = []

    if record.get("document_kind") != "openpipestress.product_preview.model":
        diagnostics.append(_diagnostic("PREVIEW_DOCUMENT_KIND_INVALID", "blocking", "model"))
    if not record.get("schema_version"):
        diagnostics.append(_diagnostic("PREVIEW_SCHEMA_VERSION_MISSING", "blocking", "model"))
    if not record.get("nodes") or not record.get("pipe_segments"):
        diagnostics.append(_diagnostic("PREVIEW_GEOMETRY_INCOMPLETE", "blocking", "model"))

    for collection_name in ("nodes", "pipe_segments", "supports", "materials", "load_cases"):
        for item in record.get(collection_name, []):
            _validate_identity_and_provenance(item, collection_name, diagnostics)
            if collection_name == "load_cases":
                for load in item.get("primitive_loads", []):
                    _validate_identity_and_provenance(load, "primitive_loads", diagnostics)

    node_ids = {node.get("id") for node in record.get("nodes", [])}
    for segment in record.get("pipe_segments", []):
        if segment.get("from") not in node_ids or segment.get("to") not in node_ids:
            diagnostics.append(_diagnostic("PREVIEW_PIPE_ENDPOINT_UNKNOWN", "blocking", segment.get("id", "pipe:TBD")))
        section = segment.get("section", {})
        for field in ("outside_diameter", "wall_thickness"):
            quantity = section.get(field)
            if not isinstance(quantity, Mapping) or "value" not in quantity or "unit" not in quantity:
                diagnostics.append(_diagnostic("PREVIEW_UNIT_METADATA_MISSING", "blocking", segment.get("id", "pipe:TBD")))

    for support in record.get("supports", []):
        if support.get("node") not in node_ids:
            diagnostics.append(_diagnostic("PREVIEW_SUPPORT_NODE_UNKNOWN", "blocking", support.get("id", "support:TBD")))

    serialized = canonical_json(record).lower()
    for phrase in PROHIBITED_TEXT:
        if phrase in serialized:
            diagnostics.append(_diagnostic("PREVIEW_PROHIBITED_CLAIM_TEXT", "blocking", phrase))

    return {
        "schema_version": "0.1.0",
        "document_kind": "openpipestress.product_preview.validation",
        "status": "blocked" if any(item["severity"] == "blocking" for item in diagnostics) else "passed",
        "diagnostics": _stable(diagnostics),
        "status_boundaries": deepcopy(record.get("analysis_status", {})),
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
    }


def canonical_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _read_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def _properties(item: Mapping[str, Any]) -> list[dict[str, Any]]:
    result = []
    for key, value in item.items():
        if key == "position" and isinstance(value, Mapping):
            result.extend(
                {"field": f"position.{axis}", "value": value.get(axis), "unit": "m"}
                for axis in ("x", "y", "z")
            )
        elif key not in {"section"}:
            result.append({"field": key, "value": value})
    if isinstance(item.get("section"), Mapping):
        for key, quantity in item["section"].items():
            result.append({"field": f"section.{key}", "value": quantity.get("value"), "unit": quantity.get("unit")})
    return result


def _diagnostic(code: str, severity: str, target_ref: str) -> dict[str, str]:
    return {"code": code, "severity": severity, "target_ref": str(target_ref)}


def _validate_identity_and_provenance(
    item: Mapping[str, Any], collection_name: str, diagnostics: list[dict[str, str]]
) -> None:
    item_id = str(item.get("id", ""))
    if not item_id.strip():
        diagnostics.append(_diagnostic("PREVIEW_ID_MISSING", "blocking", collection_name))
    provenance = str(item.get("provenance", "")).lower()
    if "invented" not in provenance and "cleared" not in provenance:
        diagnostics.append(_diagnostic("PREVIEW_PROVENANCE_MISSING", "blocking", item_id or collection_name))


def _stable(items: list[Mapping[str, Any]]) -> list[dict[str, Any]]:
    return sorted((dict(item) for item in items), key=canonical_json)
