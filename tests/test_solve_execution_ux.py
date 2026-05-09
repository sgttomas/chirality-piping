#!/usr/bin/env python3
"""Focused tests for DEL-07-07 solve execution UX contracts."""

from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.gui.solve_execution import build_solve_execution_ux, canonical_json  # noqa: E402


def main():
    record = build_solve_execution_ux(
        run_panel_id="invented-run-panel",
        events=[
            {"event_id": "queued", "state": "queued", "progress_percent": 0},
            {
                "event_id": "running",
                "state": "running",
                "progress_percent": 40,
                "diagnostic_refs": [{"ref_type": "diagnostic", "ref_id": "diag-1"}],
            },
            {
                "event_id": "cancel-request",
                "state": "cancelling",
                "progress_percent": 40,
                "cancellation_requested": True,
            },
            {"event_id": "cancelled", "state": "cancelled", "progress_percent": 40},
        ],
    )
    assert record["deliverable_id"] == "DEL-07-07"
    assert record["solver_execution"] == "not_performed_by_gui_contract"
    assert record["job_orchestration"] == "invented_state_transitions_only"
    assert record["final_state"] == "cancelled"
    assert record["cancellation"]["requested"] is True
    assert record["cancellation"]["mutates_solver_process_directly"] is False
    assert not record["diagnostics"]
    assert record["software_makes_professional_acceptance_claim"] is False
    assert "professional acceptance" not in canonical_json(record).lower()


if __name__ == "__main__":
    main()
