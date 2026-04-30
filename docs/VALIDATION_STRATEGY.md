---
doc_id: OPS-VALIDATION-STRATEGY
doc_kind: governance.validation_strategy
status: draft
created: 2026-04-30
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: implements
    to: SOW-026
  - rel: implements
    to: SOW-027
---

# Verification and Validation Strategy

## 1. Principle

Verification proves that the software solves the stated mechanics problem correctly within declared tolerances. Validation evaluates whether the software workflow is fit for its intended engineering support role. Neither replaces project-specific professional judgment.

## 2. Benchmark families

| Family | Examples | Required evidence |
|---|---|---|
| Units and schemas | dimensional conversions, invalid unit rejection, schema completeness | automated tests |
| Frame mechanics | cantilever, fixed-fixed thermal growth, portal frame, inclined member transforms | hand calculations and automated comparisons |
| Piping loads | weight, pressure thrust where modeled, imposed displacement, thermal expansion | hand calculations and regression outputs |
| Stress recovery | axial, bending, torsion, pressure membrane, stress range | hand calculations |
| Nonlinear supports | one-way support, gap, lift-off, friction active state | convergence and state trace |
| Rule packs | invented demo values, missing required inputs, unit mismatch, unsafe expression | evaluator tests |
| Reports | reproducibility, checksum stability, warning inclusion, protected-content lint | report tests |

## 3. Validation manual structure

The validation manual shall include:

1. product scope and limitations;
2. solver theory summary;
3. unit and schema verification;
4. element verification;
5. load and stress recovery verification;
6. nonlinear support verification;
7. rule-pack evaluator verification;
8. GUI workflow validation;
9. report reproducibility validation;
10. known limitations and open issues.

## 4. Release gate

A release candidate must not be labeled engineering beta unless:

- all required mechanics benchmarks pass;
- stress-recovery benchmarks pass;
- rule-pack missing-input tests pass;
- report reproducibility tests pass;
- protected-content lint passes for public examples/templates;
- open risks are listed and accepted by human maintainers.

## 5. Benchmark source rule

Validation examples must be original, public-domain, or permissively licensed. Protected code examples and commercial software examples are not acceptable unless written permission is documented.
