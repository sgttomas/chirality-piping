from __future__ import annotations

import csv
import json
import subprocess
import sys
from pathlib import Path


TOOL = Path(__file__).resolve().parent / "synthesize_domain_coverage_json.py"


def write_csv(path: Path, rows: list[dict[str, str]], fieldnames: list[str] | None = None) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if fieldnames is None:
        fieldnames = list(rows[0].keys())
    with path.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)


def test_synthesizes_coverage_json_with_manifest_rollup(tmp_path: Path) -> None:
    decomposition = tmp_path / "_Decomposition"
    snapshot = tmp_path / "_ScopeChange" / "SCA-001_2026-04-21_1200"
    output = snapshot / "Post_Change_Coverage.json"

    write_csv(decomposition / "annex_coverage_telemetry.csv", [
        {"Metric": "UnitCount", "Value": "2"},
        {"Metric": "Revision", "Value": "SCA-001"},
        {"Metric": "OpenIssuesByType", "Value": "{\"TBD\": 1}"},
    ])
    write_csv(snapshot / "KTY_Remediation_Manifest.csv", [{
        "CONTENT_DISPOSITION_STATE": "VERIFIED",
        "FACTUAL_USE_GATE": "ALLOW_FACTUAL_USE",
    }])

    result = subprocess.run(
        [
            sys.executable,
            str(TOOL),
            "--decomposition-root",
            str(decomposition),
            "--scope-change-snapshot",
            str(snapshot),
            "--output-json",
            str(output),
        ],
        check=False,
        text=True,
        capture_output=True,
    )

    assert result.returncode == 0, result.stderr
    data = json.loads(output.read_text(encoding="utf-8"))
    assert data["UnitCount"] == 2
    assert data["Revision"] == "SCA-001"
    assert data["OpenIssuesByType"] == {"TBD": 1}
    assert data["KTYRemediationManifestRows"] == 1
    assert data["ContentRemediationState"] == "COMPLETE"


def test_missing_manifest_state_is_explicit(tmp_path: Path) -> None:
    decomposition = tmp_path / "_Decomposition"
    output = tmp_path / "Pre_Change_Coverage.json"
    write_csv(decomposition / "annex_coverage_telemetry.csv", [{"Metric": "UnitCount", "Value": "1"}])

    result = subprocess.run(
        [
            sys.executable,
            str(TOOL),
            "--decomposition-root",
            str(decomposition),
            "--missing-manifest-state",
            "NOT_FORMALIZED",
            "--output-json",
            str(output),
        ],
        check=False,
        text=True,
        capture_output=True,
    )

    assert result.returncode == 0, result.stderr
    data = json.loads(output.read_text(encoding="utf-8"))
    assert data["KTYRemediationManifestExists"] is False
    assert data["ContentRemediationState"] == "NOT_FORMALIZED"


def test_reads_legacy_coverage_telemetry_json(tmp_path: Path) -> None:
    decomposition = tmp_path / "_Decomposition"
    output = tmp_path / "coverage.json"
    decomposition.mkdir(parents=True)
    (decomposition / "DeepCut_Coverage_Telemetry_v4.json").write_text(
        json.dumps({"Revision": "v4", "UnitCount": 7}),
        encoding="utf-8",
    )

    result = subprocess.run(
        [
            sys.executable,
            str(TOOL),
            "--decomposition-root",
            str(decomposition),
            "--output-json",
            str(output),
        ],
        check=False,
        text=True,
        capture_output=True,
    )

    assert result.returncode == 0, result.stderr
    data = json.loads(output.read_text(encoding="utf-8"))
    assert data["Revision"] == "v4"
    assert data["UnitCount"] == 7
