#!/usr/bin/env python3
"""Focused regression checks for invented nonlinear support fixtures."""

import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
BENCHMARK_DIR = ROOT / "validation" / "benchmarks" / "nonlinear"
SOURCE_PATH = BENCHMARK_DIR / "src" / "lib.rs"

REQUIRED_FAMILIES = {
    "ActiveSet",
    "Gap",
    "LiftOff",
    "Friction",
    "NonConvergence",
}

FORBIDDEN_TERMS = {
    "AS" + "ME",
    "B" + "31",
    "B" + "31J",
    "allowable stress " + "table",
    "stress intensification factor " + "table",
    "vendor catalog " + "value",
    "real " + "se" + "cret",
    "cert" + "ified",
    "sea" + "led",
}


def test_nonlinear_benchmark_crate_runs_focused_regressions():
    result = subprocess.run(
        ["cargo", "test", "--quiet"],
        cwd=BENCHMARK_DIR,
        check=False,
        text=True,
        capture_output=True,
    )

    assert result.returncode == 0, result.stdout + result.stderr


def test_nonlinear_fixture_catalog_is_bounded_and_invented():
    source = SOURCE_PATH.read_text(encoding="utf-8")

    for family in REQUIRED_FAMILIES:
        assert family in source

    assert "project-original-public-content" in source
    assert "invented support states" in source
    assert "tolerance_policy: None" in source

    lowered_source = source.lower()
    for term in FORBIDDEN_TERMS:
        assert term.lower() not in lowered_source
