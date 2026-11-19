# Example 41: Genomic Variant Annotation & Pathogenicity

VCF variant parsing, ClinVar pathogenic classification, chromosome position hashing, and ACMG clinical significance scoring.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/41_genomic_variant_annotator/
├── enums/                                # Domain enums
├── events/                               # Domain events
├── models/                               # DTOs & payloads
├── entities/                             # Database tables
├── interfaces/                           # Contracts
├── repositories/                         # Data access
├── services/                             # Clinical domain logic
├── controllers/                          # Presentation endpoints
├── routes/                               # Route dispatcher
├── main.kora                             # Bootstrap
└── tests/                                # Unit & load test suites
```

---

## 2. CLI Execution & Verification

```bash
kora check examples/41_genomic_variant_annotator/main.kora
kora test examples/41_genomic_variant_annotator/tests/
```
