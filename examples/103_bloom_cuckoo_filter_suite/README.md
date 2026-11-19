# Example 103: Probabilistic Bloom & Cuckoo Filter Suite

Standard bitset Bloom filter with Murmur3 hashing, Counting Bloom filter with decrement, and Cuckoo filter with victim kick-out evictions.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/103_bloom_cuckoo_filter_suite/
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
kora check examples/103_bloom_cuckoo_filter_suite/main.kora
kora test examples/103_bloom_cuckoo_filter_suite/tests/
```
