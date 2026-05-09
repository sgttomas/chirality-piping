"""Deterministic result-viewer records for DEL-07-05."""

from __future__ import annotations

from copy import deepcopy
import json
from math import isfinite
from typing import Any, Mapping


RESULT_KINDS = {"displacement", "rotation", "force", "moment", "reaction", "stress", "ratio"}


def build_results_viewer_contract(*, result_set_id: str, result_items: list[Mapping[str, Any]]) -> dict[str, Any]:
    diagnostics: list[dict[str, str]] = []
    views = [_result_view(item, diagnostics) for item in result_items]
    return {
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-07-05",
        "package_id": "PKG-07",
        "scope_item": "SOW-023",
        "objectives": ["OBJ-006", "OBJ-007"],
        "result_set_id": str(result_set_id),
        "views": sorted(views, key=canonical_json),
        "diagnostics": sorted(diagnostics, key=canonical_json),
        "viewer_policy": "tabular_and_overlay_descriptors_only",
        "solver_execution": "not_performed",
        "software_makes_professional_acceptance_claim": False,
    }


def canonical_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _result_view(item: Mapping[str, Any], diagnostics: list[dict[str, str]]) -> dict[str, Any]:
    result_id = _text(item.get("result_id"), "result:TBD")
    kind = _text(item.get("result_kind"), "unknown")
    if kind not in RESULT_KINDS:
        diagnostics.append(_diagnostic("RESULT_KIND_UNSUPPORTED", "blocking", result_id))
    values = [_value_record(result_id, value, diagnostics) for value in _list(item.get("values"))]
    if not values:
        diagnostics.append(_diagnostic("RESULT_VALUES_MISSING", "blocking", result_id))
    return {
        "result_id": result_id,
        "result_kind": kind,
        "source_ref": deepcopy(item.get("source_ref")),
        "analysis_status": _text(item.get("analysis_status"), "not_provided"),
        "diagnostic_refs": deepcopy(_list(item.get("diagnostic_refs"))),
        "table_columns": _columns(values),
        "values": values,
        "overlay_descriptor": {
            "enabled": bool(item.get("overlay_enabled", False)),
            "target_ref": deepcopy(item.get("overlay_target_ref")),
            "style_token": _text(item.get("style_token"), "TBD"),
        },
        "availability": "available" if values and not _has_tbd(values) else "unresolved_TBD",
    }


def _value_record(result_id: str, value: Any, diagnostics: list[dict[str, str]]) -> dict[str, Any]:
    if not isinstance(value, Mapping):
        diagnostics.append(_diagnostic("RESULT_VALUE_NOT_OBJECT", "blocking", result_id))
        return {"value_id": "value:TBD", "numeric_value": "TBD", "unit": "TBD"}
    numeric = value.get("numeric_value", "TBD")
    unit = value.get("unit", "TBD")
    if numeric == "TBD" or unit in (None, "TBD"):
        diagnostics.append(_diagnostic("RESULT_VALUE_OR_UNIT_UNRESOLVED", "warning", result_id))
    if isinstance(numeric, (int, float)) and not isfinite(float(numeric)):
        diagnostics.append(_diagnostic("RESULT_VALUE_NONFINITE", "blocking", result_id))
    return {
        "value_id": _text(value.get("value_id"), "value:TBD"),
        "component": _text(value.get("component"), "scalar"),
        "numeric_value": numeric,
        "unit": unit,
        "location_ref": deepcopy(value.get("location_ref")),
        "provenance_ref": deepcopy(value.get("provenance_ref")),
    }


def _columns(values: list[Mapping[str, Any]]) -> list[str]:
    columns = {"value_id", "component", "numeric_value", "unit"}
    if any(item.get("location_ref") for item in values):
        columns.add("location_ref")
    return sorted(columns)


def _has_tbd(values: list[Mapping[str, Any]]) -> bool:
    return any(item.get("numeric_value") == "TBD" or item.get("unit") in (None, "TBD") for item in values)


def _diagnostic(code: str, severity: str, target_ref: str) -> dict[str, str]:
    return {"code": code, "severity": severity, "target_ref": target_ref}


def _list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _text(value: Any, fallback: str) -> str:
    text = str(value).strip() if value is not None else ""
    return text or fallback
