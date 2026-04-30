#!/usr/bin/env python3
"""
synthesize_domain_coverage_json.py

Deterministically serializes DOMAIN decomposition coverage telemetry to JSON.

Usage:
  python3 tools/reporting/synthesize_domain_coverage_json.py \\
    --decomposition-root <domain-root>/_Decomposition \\
    --output-json <snapshot>/Post_Change_Coverage.json \\
    [--scope-change-snapshot <snapshot>] \\
    [--missing-manifest-state NOT_REQUIRED|NOT_FORMALIZED|PENDING] \\
    [--note "human-readable note"]

Inputs:
  --decomposition-root: directory containing annex_coverage_telemetry.csv.
  --scope-change-snapshot: optional SCA snapshot used to summarize
    KTY_Remediation_Manifest.csv when present.

Outputs:
  JSON object at --output-json. The tool is idempotent and writes only that
  output path.

Scope boundary:
  Reads only decomposition annex CSVs and optional snapshot manifest artifacts.
  Does not modify decomposition, KTY folders, or SCOPE_CHANGE manifests.

Exit codes:
  0 = coverage JSON written
  1 = fatal input / parsing error
"""

from __future__ import annotations

import argparse
import csv
import json
import sys
from pathlib import Path
from typing import Any


NUMERIC_KEYS = {
    "UnitCount",
    "INUnitCount",
    "TBDUnitCount",
    "OUTUnitCount",
    "CategoryCount",
    "SubjectCount",
    "ObjectiveCount",
    "UnassignedINUnits",
    "UnitsWithoutKnowledgeTypeMapping",
    "UnmappedObjectives",
    "OpenIssuesTotal",
}

COVERAGE_CANDIDATES = [
    "annex_coverage_telemetry.csv",
    "*Coverage_Telemetry*.csv",
    "*Coverage_Telemetry*.json",
]


def read_csv_rows(path: Path) -> list[dict[str, str]]:
    if not path.exists():
        raise FileNotFoundError(str(path))
    with path.open("r", encoding="utf-8-sig", newline="") as f:
        reader = csv.DictReader(f)
        if reader.fieldnames is None:
            raise ValueError(f"CSV has no header: {path}")
        rows: list[dict[str, str]] = []
        for row in reader:
            cleaned: dict[str, str] = {}
            for key, value in row.items():
                if key is None:
                    continue
                cleaned[key.strip()] = "" if value is None else str(value).strip()
            rows.append(cleaned)
        return rows


def parse_scalar(key: str, value: str) -> Any:
    raw = value.strip()
    if key in NUMERIC_KEYS:
        try:
            return int(raw)
        except ValueError:
            return raw
    if raw.lower() in {"true", "false"}:
        return raw.lower() == "true"
    if raw.startswith("{") or raw.startswith("["):
        try:
            return json.loads(raw)
        except json.JSONDecodeError:
            return raw
    return raw


def resolve_coverage_path(decomposition_root: Path) -> Path:
    for pattern in COVERAGE_CANDIDATES:
        exact = decomposition_root / pattern
        if exact.exists():
            return exact
        matches = sorted(decomposition_root.glob(pattern))
        if matches:
            return matches[0]
    return decomposition_root / "annex_coverage_telemetry.csv"


def read_coverage_telemetry(decomposition_root: Path) -> dict[str, Any]:
    path = resolve_coverage_path(decomposition_root)
    if path.suffix.lower() == ".json":
        return json.loads(path.read_text(encoding="utf-8"))
    rows = read_csv_rows(path)
    data: dict[str, Any] = {}
    for row in rows:
        key = row.get("Metric", "").strip()
        if not key:
            continue
        data[key] = parse_scalar(key, row.get("Value", ""))
    return data


def remediation_rollup(rows: list[dict[str, str]]) -> str:
    states = {row.get("CONTENT_DISPOSITION_STATE", "").strip() for row in rows}
    gates = {row.get("FACTUAL_USE_GATE", "").strip() for row in rows}
    if any(not state or state == "PENDING" for state in states):
        return "PENDING"
    if "BLOCKED" in states or "BLOCK_FACTUAL_USE" in gates:
        return "BLOCKED"
    if "DEFERRED" in states:
        return "DEFERRED"
    return "COMPLETE"


def enrich_manifest_state(data: dict[str, Any], snapshot: Path | None, missing_manifest_state: str) -> None:
    if snapshot is None:
        data["KTYRemediationManifestExists"] = False
        data["ContentRemediationState"] = missing_manifest_state
        return

    manifest = snapshot / "KTY_Remediation_Manifest.csv"
    if not manifest.exists():
        data["KTYRemediationManifestExists"] = False
        data["ContentRemediationState"] = missing_manifest_state
        return

    rows = read_csv_rows(manifest)
    data["KTYRemediationManifestExists"] = True
    data["KTYRemediationManifestRows"] = len(rows)
    data["KTYRemediationManifestPending"] = sum(1 for row in rows if row.get("CONTENT_DISPOSITION_STATE", "").strip() == "PENDING")
    data["KTYRemediationManifestBlocked"] = sum(1 for row in rows if row.get("CONTENT_DISPOSITION_STATE", "").strip() == "BLOCKED")
    data["KTYRemediationManifestDeferred"] = sum(1 for row in rows if row.get("CONTENT_DISPOSITION_STATE", "").strip() == "DEFERRED")
    data["ContentRemediationState"] = remediation_rollup(rows) if rows else "NOT_REQUIRED"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Synthesize DOMAIN coverage telemetry JSON")
    parser.add_argument("--decomposition-root", required=True, type=Path, help="Path to DOMAIN _Decomposition directory")
    parser.add_argument("--output-json", required=True, type=Path, help="Coverage JSON output path")
    parser.add_argument("--scope-change-snapshot", type=Path, help="Optional SCA snapshot containing KTY_Remediation_Manifest.csv")
    parser.add_argument(
        "--missing-manifest-state",
        default="NOT_REQUIRED",
        choices=["NOT_REQUIRED", "NOT_FORMALIZED", "PENDING"],
        help="ContentRemediationState when no manifest is present",
    )
    parser.add_argument("--note", default="", help="Optional note to include in the JSON output")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        if not args.decomposition_root.is_dir():
            raise NotADirectoryError(str(args.decomposition_root))
        data = read_coverage_telemetry(args.decomposition_root)
        enrich_manifest_state(data, args.scope_change_snapshot, args.missing_manifest_state)
        if args.note:
            data["Note"] = args.note
        args.output_json.parent.mkdir(parents=True, exist_ok=True)
        args.output_json.write_text(json.dumps(data, indent=2, sort_keys=False) + "\n", encoding="utf-8")
    except (FileNotFoundError, NotADirectoryError, ValueError, OSError, csv.Error) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 1

    print(f"Wrote coverage JSON: {args.output_json}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
