---
doc_id: OPS-IP-DATA-BOUNDARY
doc_kind: governance.policy
status: draft
created: 2026-04-30
refs:
  - rel: governed_by
    to: OPS-CONTRACT
---

# IP and Data Boundary Policy

## 1. Purpose

This policy keeps the public OpenPipeStress repository aligned with the project intent: public open mechanics, private/user-supplied code and proprietary data.

## 2. Public repository may contain

- solver algorithms based on open mechanics;
- geometry, unit, load, stress-recovery, and report schemas;
- blank templates and private-library import mechanisms;
- invented example data with clear notices;
- public-domain or permissively licensed data with documented provenance;
- validation benchmarks from original, public-domain, or permissively licensed sources.

## 3. Public repository must not contain

- ASME or other standards-body code text, tables, figures, examples, or commentary;
- material allowable tables copied from standards;
- B31J stress-intensification or flexibility-factor content copied from standards;
- B16/B36/MSS/ISO/EN/CSA dimensional or rating tables copied from standards;
- proprietary vendor catalogs without redistribution rights;
- commercial software examples, report templates, or benchmark files without permission;
- user private rule packs, owner standards, or company design bases.

## 4. Required provenance fields

Every public data record must include:

| Field | Meaning |
|---|---|
| `source_name` | Human-readable source. |
| `source_location` | URL/path/document identifier or `TBD`. |
| `source_license` | Redistribution basis or `TBD`. |
| `contributor` | Person/org submitting. |
| `contributor_certification` | Statement that data is not copied from protected source unless licensed. |
| `redistribution_status` | `public_permissive`, `private_only`, `unknown`, `protected_suspected`. |
| `review_status` | `pending`, `accepted`, `rejected`, `quarantined`. |

## 5. Quarantine rule

If data appears to derive from a protected standard or proprietary source, agents and maintainers must:

1. stop ingestion;
2. mark the record `protected_suspected`;
3. move the artifact to a quarantine path outside public examples;
4. record the issue;
5. request human/legal review.

## 6. Private user data

Private material libraries, rule packs, component catalogs, and owner design bases belong in user-controlled private paths. The public project may provide schemas and importers, but must not assume those files are redistributable.

## 7. Report boundary

Generated reports may reference a user rule-pack ID, version, checksum, and source note. Public report templates must not embed protected formulas or tables. Users are responsible for private report templates that quote licensed standards.
