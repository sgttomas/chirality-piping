"""Deterministic solve execution UX records for DEL-07-07."""

from __future__ import annotations

from copy import deepcopy
import json
from typing import Any, Mapping


VALID_STATES = {"queued", "running", "cancelling", "cancelled", "completed", "failed"}
TERMINAL_STATES = {"cancelled", "completed", "failed"}


def build_solve_execution_ux(*, run_panel_id: str, events: list[Mapping[str, Any]]) -> dict[str, Any]:
    diagnostics: list[dict[str, str]] = []
    timeline = [_event_record(index, item, diagnostics) for index, item in enumerate(events)]
    final_state = timeline[-1]["state"] if timeline else "queued"
    if final_state not in TERMINAL_STATES:
        diagnostics.append(_diagnostic("SOLVE_UX_TERMINAL_STATE_PENDING", "warning", run_panel_id))
    return {
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-07-07",
        "package_id": "PKG-07",
        "scope_item": "SOW-055",
        "objectives": ["OBJ-006", "OBJ-007"],
        "run_panel_id": str(run_panel_id),
        "timeline": timeline,
        "final_state": final_state,
        "cancellation": _cancellation(timeline),
        "diagnostics": sorted(diagnostics, key=canonical_json),
        "solver_execution": "not_performed_by_gui_contract",
        "job_orchestration": "invented_state_transitions_only",
        "software_makes_professional_acceptance_claim": False,
    }


def canonical_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _event_record(index: int, event: Mapping[str, Any], diagnostics: list[dict[str, str]]) -> dict[str, Any]:
    event_id = _text(event.get("event_id"), f"event:{index:04d}")
    state = _text(event.get("state"), "queued")
    if state not in VALID_STATES:
        diagnostics.append(_diagnostic("SOLVE_UX_STATE_UNSUPPORTED", "blocking", event_id))
    progress = event.get("progress_percent", 0)
    if not isinstance(progress, (int, float)) or progress < 0 or progress > 100:
        diagnostics.append(_diagnostic("SOLVE_UX_PROGRESS_INVALID", "blocking", event_id))
        progress = "TBD"
    return {
        "event_id": event_id,
        "sequence": index,
        "state": state,
        "progress_percent": progress,
        "message": _text(event.get("message"), state),
        "diagnostic_refs": deepcopy(_list(event.get("diagnostic_refs"))),
        "warning_refs": deepcopy(_list(event.get("warning_refs"))),
        "analysis_status": _text(event.get("analysis_status"), "not_provided"),
        "cancellation_requested": bool(event.get("cancellation_requested", False)),
        "source_ref": deepcopy(event.get("source_ref")),
    }


def _cancellation(timeline: list[Mapping[str, Any]]) -> dict[str, Any]:
    requested = [item for item in timeline if item["cancellation_requested"] or item["state"] in {"cancelling", "cancelled"}]
    return {
        "requested": bool(requested),
        "request_event_ids": [item["event_id"] for item in requested],
        "terminal_state": timeline[-1]["state"] if timeline else "queued",
        "mutates_solver_process_directly": False,
    }


def _diagnostic(code: str, severity: str, target_ref: str) -> dict[str, str]:
    return {"code": code, "severity": severity, "target_ref": target_ref}


def _list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _text(value: Any, fallback: str) -> str:
    text = str(value).strip() if value is not None else ""
    return text or fallback
