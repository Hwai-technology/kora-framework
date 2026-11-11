# Example 10: DTO Validation, Defense-in-Depth & Comprehensive API Types

> **"DTO Protects You" (الـ DTO هو درع الحماية)**  
> In modern production APIs, untrusted external input must **never** reach your database models or business domain entities directly. A Data Transfer Object (DTO) acts as a strict boundary firewall at the edge of your service, guaranteeing that only cryptographically valid, schema-compliant, and well-typed data enters your application.

---

## 🛡️ Why DTO Validation is Essential ("DTO سيحميك")

1. **Immunity to Mass Assignment & Over-Posting**:
   - Attackers often inject unauthorized fields (e.g., `isAdmin: true`, `role: "superuser"`, `balance: 999999`) into raw request payloads.
   - A strict DTO whitelist ensures that **only declared properties** are parsed and processed. Any unexpected fields are rejected at the gate.

2. **Zero Value Guessing & Type Certainty**:
   - No silent coercion or dangerous defaults (`|| ''`, `|| 0`).
   - Every field has an explicit type: `String`, `Int`, `Float`, `Bool`, `Option<T>`, `List<T>`, or nested composite models.

3. **Standardized RFC 7807 Error Responses**:
   - When validation fails, the API returns a structured `ValidationEnvelope` with machine-readable error details (`field`, `rule`, `message`, `rejectedValue`), returning HTTP `422 Unprocessable Entity`.

---

## 🧩 Comprehensive Matrix of API Types

This example implements and demonstrates all fundamental types and data structures essential in high-performance web APIs:

| Category | Kora Type | Validation Rules / Attributes | API Use Case |
|:---|:---|:---|:---|
| **Textual Primitive** | `String` | `@required`, `@min(3)`, `@max(30)` | Usernames, identifiers, titles |
| **Email Address** | `String` | `@required`, `@email` | Account verification, notifications |
| **Integral Number** | `Int` | `@min(18)`, `@max(120)` | User age, ports, counts, timestamps |
| **Floating Point** | `Float` | `@min(0.0)` | Currency rates, coordinates, ratings |
| **Boolean Flag** | `Bool` | `@required` (must be `true`) | Terms of service consent, active flags |
| **File / Image Upload** | `FileUploadDto` | `@max(10485760)`, `@mime(...)` | Avatars, documents, receipts |
| **Image Metadata** | `ImageMetadataDto` | `width: Int`, `height: Int` | Dimensions, aspect ratio, alpha channel |
| **Structured JSON / Object** | `AddressDto` | Composite sub-model | Physical billing/shipping address |
| **User Preferences** | `UserPreferencesDto` | Composite sub-model | Theme (`dark`/`light`), locale, toggles |
| **Homogeneous Array** | `List<String>` | `@min(1)`, `@max(10)` | User skills, search tags, categories |
| **Array of Files** | `List<FileUploadDto>` | Bound checks | Batch document attachments |
| **Optional / Nullable** | `Option<T>` | `Some(value)` or `null` | Optional avatar, secondary website URL |
| **Algebraic Enum** | `AccountTier` | `Free`, `Pro`, `Enterprise` | Type-safe subscription tiers |

---

## 🏛️ Clean Architecture Structure

Adhering strictly to Kora's compiler-enforced architectural guard rules:

```text
examples/10_dto_validation_and_types/
├── enums/
│   ├── account_tier.kora           # Pure enum variants (Free, Pro, Enterprise) - E5001
│   └── file_classification.kora    # File categories (Avatar, Document, InvoicePdf) - E5001
├── models/
│   ├── file_dto.kora               # File & image upload DTOs (FileUploadDto, ImageMetadataDto)
│   ├── nested_dto.kora             # Sub-models (AddressDto, GeoLocationDto, UserPreferencesDto)
│   ├── user_registration_dto.kora  # Master DTO combining all data types
│   └── validation_result_dto.kora  # RFC 7807 validation error envelope
├── interfaces/
│   └── validator_interface.kora    # Behavioral contract (RequestValidatorContract) - E5002
├── services/
│   └── dto_validator_service.kora  # Validation implementation & boundary enforcement - E5003
├── main.kora                       # Lean route definitions & dependency wiring - E5004
└── README.md                       # Comprehensive guide & documentation
```

---

## 🚀 Running & Verifying

### Type Check & Verify Compilation:
```bash
cargo run -p kora-cli -- check examples/10_dto_validation_and_types/main.kora
```

### Run the Service:
```bash
cargo run -p kora-cli -- run examples/10_dto_validation_and_types/main.kora
```

### Test API Endpoints:

#### 1. Validate File Upload:
```bash
curl -X POST http://127.0.0.1:7582/api/v1/files/upload \
  -H "Content-Type: application/json"
```

#### 2. Validate Address:
```bash
curl -X POST http://127.0.0.1:7582/api/v1/addresses/validate \
  -H "Content-Type: application/json"
```

#### 3. Full Registration DTO:
```bash
curl -X POST http://127.0.0.1:7582/api/v1/users/register \
  -H "Content-Type: application/json"
```
