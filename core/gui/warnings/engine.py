"""Warning and blocking state contracts for DEL-07-04."""

from __future__ import annotations

from copy import deepcopy
import json
from typing import Any, Mapping


WARNING_CLASSES = {
    "solve_required",
    "code_check_required",
    "provenance_missing",
    "assumption",
    "incomplete_data",
    "diagnostic",
    "professional_boundary",
}
BLOCKING_CLASSES = {"solve_required", "code_check_required", "incomplete_data"}


def build_warning_ux_contract(*, warning_set_id: str, conditions: list[Mapping[str, Any]]) -> dict[str, Any]:
    diagnostics: list[dict[str, str]] = []
    warnings = [_warning_record(item, diagnostics) for item in conditions]
    blocking = [item for item in warnings if item["blocks_progress"]]
    return {
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-07-04",
        "package_id": "PKG-07",
        "scope_item": "SOW-022",
        "objectives": ["OBJ-006", "OBJ-011"],
        "warning_set_id": str(warning_set_id),
        "warnings": sorted(warnings, key=canonical_json),
        "blocking_summary": {
            "has_blocking_items": bool(blocking),
            "blocking_warning_ids": sorted(item["warning_id"] for item in blocking),
        },
        "diagnostics": sorted(diagnostics, key=canonical_json),
        "auto_fill_missing_data": False,
        "software_makes_professional_acceptance_claim": False,
    }


def canonical_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _warning_record(condition: Mapping[str, Any], diagnostics: list[dict[str, str]]) -> dict[str, Any]:
    warning_id = _text(condition.get("warning_id"), "warning:TBD")
    warning_class = _text(condition.get("warning_class"), "diagnostic")
    if warning_class not in WARNING_CLASSES:
        diagnostics.append(_diagnostic("WARNING_CLASS_UNSUPPORTED", "blocking", warning_id))
    source_status = _text(condition.get("source_status"), "not_provided")
    if source_status in {"missing", "not_provided"} and warning_class != "provenance_missing":
        diagnostics.append(_diagnostic("WARNING_SOURCE_STATUS_UNRESOLVED", "warning", warning_id))
    return {
        "warning_id": warning_id,
        "warning_class": warning_class,
        "severity": _severity(condition, warning_class),
        "blocks_progress": bool(condition.get("blocks_progress", warning_class in BLOCKING_CLASSES)),
        "target_ref": deepcopy(condition.get("target_ref")),
        "message": _text(condition.get("message"), "TBD warning message"),
        "remediation": _text(condition.get("remediation"), "Provide source data or retain unresolved TBD."),
        "source_status": source_status,
        "assumption_ref": deepcopy(condition.get("assumption_ref")),
        "diagnostic_ref": deepcopy(condition.get("diagnostic_ref")),
        "professional_boundary_preserved": True,
    }


def _severity(condition: Mapping[str, Any], warning_class: str) -> str:
    if condition.get("severity"):
        return str(condition["severity"])
    return "blocking" if warning_class in BLOCKING_CLASSES else "warning"


def _diagnostic(code: str, severity: str, target_ref: str) -> dict[str, str]:
    return {"code": code, "severity": severity, "target_ref": target_ref}


def _text(value: Any, fallback: str) -> str:
    text = str(value).strip() if value is not None else ""
    return text or fallback
