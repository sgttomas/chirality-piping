# MEMORY: DEL-11-04 Invented Educational Example Models

## Scope

Implemented DEV-001 revision 0.5 Tranche A `DEL-11-04` only. Work was limited to invented public educational example models, a scoped example smoke test, and this deliverable-local memory record.

## Artifacts

- `examples/models/invented/mechanics_only_toy_span.json`
- `examples/models/invented/fake_rule_pack_toy_model.json`
- `tests/test_invented_example_models.py`

## Boundary Notes

- Example data uses synthetic `INV_*` IDs, invented geometry, invented loads, invented material labels, invented section labels, and invented sample outputs.
- Both examples state that they are invented, non-code, non-project educational fixtures and are not suitable for engineering reliance.
- The fake-rule example references the existing invented public demonstration rule pack by safe identity only and uses fictional user-rule result data.
- No restricted-source content, restricted benchmark file, private project data, credential material, third-party application example, or professional authority claim was intentionally introduced.
- Retired candidate `DAG-002-E0621` and candidate edges/DAG-001 were not used as authority.

## Validation

- Added `tests/test_invented_example_models.py` to load the invented model JSON files, check canonical model-contract shape expected by `schemas/model.schema.yaml`, require invented/non-reliance notices, and scan for protected/private/professional-claim markers.
- Full JSON Schema validation was not added because the current repository uses stdlib schema tests and the local Python environment does not provide `jsonschema`.
- `python3 -m pytest tests/test_invented_example_models.py` passed with 4 tests.
- `python3 tests/test_model_schema.py`, `python3 tests/test_rule_pack_schema.py`, and `python3 tests/test_report_protected_content_linter.py` passed.
- `cargo test --manifest-path core/reporting/protected_content_linter/Cargo.toml` passed with 4 unit tests.
- Focused `rg` scans over the DEL-11-04 changed files found no matches for standards identifiers, protected-table/formula markers, private secret markers, named commercial tool examples, or prohibited professional/compliance claim phrases.
- `git diff --check` passed. New-file `git diff --no-index --check /dev/null <file>` checks produced no whitespace-error output.

## Remaining TBDs

- Concrete checksum values remain `TBD` pending checksum lifecycle tooling.
- Runtime integration with a solver, rule evaluator, report renderer, or tutorial flow remains outside this deliverable.
