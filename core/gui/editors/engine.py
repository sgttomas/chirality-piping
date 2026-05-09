"""Deterministic editor contracts for DEL-07-03."""

from __future__ import annotations

from copy import deepcopy
import json
from typing import Any, Mapping


PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
}

EDITOR_KINDS = {"material", "component", "rule_pack_reference"}


def build_editor_contract(*, editor_set_id: str, editors: list[Mapping[str, Any]]) -> dict[str, Any]:
    """Build deterministic material/component/rule-pack editor records."""

    diagnostics: list[dict[str, str]] = []
    records = [_editor_record(item, diagnostics) for item in editors]
    return {
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-07-03",
        "package_id": "PKG-07",
        "scope_item": "SOW-021",
        "objectives": ["OBJ-006"],
        "editor_set_id": str(editor_set_id),
        "editors": sorted(records, key=canonical_json),
        "diagnostics": sorted(diagnostics, key=canonical_json),
        "command_policy": "bounded_apply_cancel_intents_only",
        "private_payload_policy": "references_and_checksums_only_no_private_payload_copy",
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
    }


def canonical_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _editor_record(editor: Mapping[str, Any], diagnostics: list[dict[str, str]]) -> dict[str, Any]:
    editor_id = _text(editor.get("editor_id"), "editor:TBD")
    kind = _text(editor.get("editor_kind"), "unknown")
    if kind not in EDITOR_KINDS:
        diagnostics.append(_diagnostic("EDITOR_KIND_UNSUPPORTED", "blocking", editor_id))
    fields = [_field_record(editor_id, item, diagnostics) for item in _list(editor.get("fields"))]
    lifecycle = deepcopy(editor.get("rule_pack_lifecycle")) if kind == "rule_pack_reference" else None
    if lifecycle and not lifecycle.get("checksum"):
        diagnostics.append(_diagnostic("RULE_PACK_CHECKSUM_MISSING", "blocking", editor_id))
    return {
        "editor_id": editor_id,
        "editor_kind": kind,
        "target_ref": deepcopy(editor.get("target_ref")),
        "library_classification": _text(editor.get("library_classification"), "invented_public_example"),
        "source_provenance": deepcopy(editor.get("source_provenance")),
        "fields": sorted(fields, key=canonical_json),
        "rule_pack_lifecycle": lifecycle,
        "validation_state": _validation_state(fields, diagnostics, editor_id),
        "save_intent": {
            "status": "pending_service_validation",
            "mutates_persistent_project": False,
        },
    }


def _field_record(editor_id: str, field: Any, diagnostics: list[dict[str, str]]) -> dict[str, Any]:
    if not isinstance(field, Mapping):
        diagnostics.append(_diagnostic("EDITOR_FIELD_NOT_OBJECT", "blocking", editor_id))
        return {"field_id": "field:TBD", "value": "TBD", "validation_state": "invalid"}
    field_id = _text(field.get("field_id"), "field:TBD")
    value = deepcopy(field.get("value", "TBD"))
    if value in (None, "TBD"):
        diagnostics.append(_diagnostic("EDITOR_FIELD_VALUE_UNRESOLVED", "warning", f"{editor_id}:{field_id}"))
    return {
        "field_id": field_id,
        "label": _text(field.get("label"), field_id),
        "value": value,
        "unit": deepcopy(field.get("unit")),
        "editable": bool(field.get("editable", True)),
        "source_ref": deepcopy(field.get("source_ref")),
        "validation_state": _text(field.get("validation_state"), "unresolved_TBD" if value in (None, "TBD") else "ready_for_service_validation"),
    }


def _validation_state(fields: list[Mapping[str, Any]], diagnostics: list[dict[str, str]], editor_id: str) -> str:
    if not fields:
        diagnostics.append(_diagnostic("EDITOR_FIELDS_MISSING", "blocking", editor_id))
        return "blocked"
    if any(item.get("validation_state") == "unresolved_TBD" for item in fields):
        return "blocked_by_unresolved_fields"
    return "ready_for_service_validation"


def _diagnostic(code: str, severity: str, target_ref: str) -> dict[str, str]:
    return {"code": code, "severity": severity, "target_ref": target_ref}


def _list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _text(value: Any, fallback: str) -> str:
    text = str(value).strip() if value is not None else ""
    return text or fallback
