from __future__ import annotations

import csv
import json
import sys
from pathlib import Path


VALIDATION_DIR = Path(__file__).resolve().parent
if str(VALIDATION_DIR) not in sys.path:
    sys.path.insert(0, str(VALIDATION_DIR))

from validate_domain_decomposition_integrity import (  # noqa: E402
    REQUIRED_SNAPSHOT_ARTIFACTS,
    load_required,
    validate_coverage,
    validate_domain_rows,
    validate_snapshot,
)


def write_csv(path: Path, rows: list[dict[str, str]], fieldnames: list[str] | None = None) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if fieldnames is None:
        fieldnames = list(rows[0].keys()) if rows else []
    with path.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)


def write_valid_decomposition(root: Path) -> None:
    write_csv(root / "annex_categories.csv", [{"CategoryID": "CAT-001", "CategoryName": "Process"}])
    write_csv(root / "annex_knowledge_types.csv", [{
        "KnowledgeTypeID": "KTY-01",
        "ParentCategoryID": "CAT-001",
        "Name": "Example KTY",
    }])
    write_csv(root / "annex_knowledge_subjects.csv", [{
        "SubjectID": "SUB-01",
        "ParentKnowledgeTypeID": "KTY-01",
        "CategoryID": "CAT-001",
        "UnitStatus": "IN",
    }])
    write_csv(root / "annex_domain_ledger.csv", [{
        "UnitID": "HBK-0001",
        "InOutStatus": "IN",
        "CategoryID": "CAT-001",
        "KnowledgeTypeID(s)": "KTY-01",
        "SubjectID(s)": "SUB-01",
        "ObjectiveID(s)": "OBJ-001",
    }])
    write_csv(root / "annex_objectives.csv", [{
        "ObjectiveID": "OBJ-001",
        "MappedKnowledgeTypes": "KTY-01",
    }])
    write_csv(root / "annex_coverage_telemetry.csv", [
        {"Metric": "UnitCount", "Value": "1"},
        {"Metric": "INUnitCount", "Value": "1"},
        {"Metric": "TBDUnitCount", "Value": "0"},
        {"Metric": "OUTUnitCount", "Value": "0"},
        {"Metric": "CategoryCount", "Value": "1"},
        {"Metric": "KnowledgeTypeCount", "Value": "1 active / 1 total"},
        {"Metric": "SubjectCount", "Value": "1"},
        {"Metric": "ObjectiveCount", "Value": "1"},
        {"Metric": "UnassignedINUnits", "Value": "0"},
        {"Metric": "UnitsWithoutKnowledgeTypeMapping", "Value": "0"},
        {"Metric": "UnmappedObjectives", "Value": "0"},
    ])


def write_snapshot(snapshot: Path, supersession_binding_present: str) -> None:
    snapshot.mkdir(parents=True, exist_ok=True)
    for name in REQUIRED_SNAPSHOT_ARTIFACTS:
        if name == "Amendment_Actions.csv":
            continue
        (snapshot / name).write_text("ok\n", encoding="utf-8")
    write_csv(snapshot / "Amendment_Actions.csv", [{
        "AmendmentID": "SCA-001",
        "ActionSeq": "1",
        "ActionType": "MODIFY",
        "EntityType": "KNOWLEDGE_TYPE",
        "EntityID": "KTY-01",
        "SupersessionBindingPresent": supersession_binding_present,
    }])
    (snapshot.parent / "_LATEST.md").write_text(snapshot.name + "\n", encoding="utf-8")


def test_valid_domain_decomposition_has_no_findings(tmp_path: Path) -> None:
    root = tmp_path / "_Decomposition"
    write_valid_decomposition(root)

    paths, rows = load_required(root)
    findings = validate_domain_rows(paths, rows) + validate_coverage(paths, rows)

    assert findings == []


def test_legacy_domain_register_names_are_resolved(tmp_path: Path) -> None:
    root = tmp_path / "_Decomposition"
    write_csv(root / "DeepCut_Category_Register_v4.csv", [{"CategoryID": "CAT-001"}])
    write_csv(root / "DeepCut_Knowledge_Type_Register_v4.csv", [{
        "KnowledgeTypeID": "KTY-01",
        "ParentCategoryID": "CAT-001",
        "InOutStatus": "IN",
    }])
    write_csv(root / "DeepCut_Knowledge_Subject_Register_v4.csv", [{
        "SubjectID": "SUB-01",
        "ParentKnowledgeTypeID": "KTY-01",
        "InOutStatus": "IN",
    }])
    write_csv(root / "DeepCut_Domain_Ledger_v4.csv", [{
        "UnitID": "HBK-0001",
        "InOutStatus": "IN",
        "CategoryID": "CAT-001",
        "KnowledgeTypeID": "KTY-01",
        "SubjectID": "SUB-01",
        "ObjectiveIDs": "OBJ-001",
    }])
    write_csv(root / "DeepCut_Objective_Register_v4.csv", [{
        "ObjectiveID": "OBJ-001",
        "MappedKnowledgeTypes": "KTY-01",
    }])
    (root / "DeepCut_Coverage_Telemetry_v4.json").write_text(
        json.dumps({
            "UnitCount": 1,
            "INUnitCount": 1,
            "TBDUnitCount": 0,
            "OUTUnitCount": 0,
            "CategoryCount": 1,
            "KnowledgeTypeCount": 1,
            "SubjectCount": 1,
            "ObjectiveCount": 1,
            "UnassignedINUnits": 0,
            "UnitsWithoutKnowledgeTypeMapping": 0,
            "UnmappedObjectives": 0,
        }),
        encoding="utf-8",
    )

    paths, rows = load_required(root)
    findings = validate_domain_rows(paths, rows) + validate_coverage(paths, rows)

    assert paths["coverage"].suffix == ".json"
    assert findings == []


def test_active_kty_without_active_subject_is_blocking(tmp_path: Path) -> None:
    root = tmp_path / "_Decomposition"
    write_valid_decomposition(root)
    write_csv(
        root / "annex_knowledge_subjects.csv",
        [],
        ["SubjectID", "ParentKnowledgeTypeID", "CategoryID", "UnitStatus"],
    )

    paths, rows = load_required(root)
    findings = validate_domain_rows(paths, rows)

    assert any(f.category == "ACTIVE_KTY_WITHOUT_ACTIVE_SUBJECT" for f in findings)


def test_supersession_delta_required_only_when_action_declares_binding(tmp_path: Path) -> None:
    no_binding = tmp_path / "_ScopeChange" / "SCA-001_2026-04-21_1200"
    with_binding = tmp_path / "_ScopeChange" / "SCA-002_2026-04-21_1300"
    write_snapshot(no_binding, "NO")
    write_snapshot(with_binding, "YES")

    no_binding_findings = validate_snapshot(no_binding)
    with_binding_findings = validate_snapshot(with_binding)

    assert not any(f.category == "SUPERSESSION_DELTA_MISSING" for f in no_binding_findings)
    assert any(f.category == "SUPERSESSION_DELTA_MISSING" for f in with_binding_findings)
