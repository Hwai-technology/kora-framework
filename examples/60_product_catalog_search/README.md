# Example 60: Faceted Product Catalog Search & Inverted Index

Attribute inheritance, multi-language localization, price-range filtering, and inverted index token matching.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/60_product_catalog_search/
├── enums/                                # Domain enums
├── events/                               # Domain events
├── models/                               # DTOs & payloads
├── entities/                             # Database tables
├── interfaces/                           # Contracts
├── repositories/                         # Data access
├── services/                             # Domain logic
├── controllers/                          # Presentation endpoints
├── routes/                               # Route dispatcher
├── main.kora                             # Bootstrap
└── tests/                                # Unit & load test suites
```

---

## 2. CLI Execution & Verification

```bash
kora check examples/60_product_catalog_search/main.kora
kora test examples/60_product_catalog_search/tests/
```
