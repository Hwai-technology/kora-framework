# Kora Clean Architecture Enterprise Showcase
### مرجع الهندسة المعمارية النظيفة الشامل لمنظومة لغة Kora

> **"Write Less. Build Faster. Run Smarter."**  
> تطبيق متكامل عالي الأداء مبني وفق مبادئ **Clean Architecture** باستخدام لغة الفريم ورك **Kora** المترجمة إلى كود Rust الأصلي بدون Garbage Collector (Zero-Cost Abstractions).

---

## 📑 فهرس المحتويات | Table of Contents
1. [نظرة عامة على المشروع (Overview)](#1-نظرة-عامة-على-المشروع-overview)
2. [الميزات المتقدمة المدمجة تلقائياً (Built-in Innovations)](#2-الميزات-المتقدمة-المدمجة-تلقائياً-built-in-innovations)
   - [الترحيل التلقائي ونثر البيانات (Auto-Migration & Database Seeder)](#21-الترحيل-التلقائي-ونثر-البيانات-auto-migration--database-seeder)
   - [قاعدة البيانات الشاملة دون تكرار (Ambient Database Pool - Zero Boilerplate)](#22-قاعدة-البيانات-الشاملة-دون-تكرار-ambient-database-pool---zero-boilerplate)
   - [الغلاف الآمن الموحد والحماية من Replay Attacks (Secure Nonce Envelope)](#23-الغلاف-الآمن-الموحد-والحماية-من-replay-attacks-secure-nonce-envelope)
   - [تتبع الدوال وأكواد الخروج (Method Telemetry & Exit Codes Tracker)](#24-تتبع-الدوال-وأكواد-الخروج-method-telemetry--exit-codes-tracker)
   - [دورة حياة الكائنات: الـ Constructor والـ Destructor](#25-دورة-حياة-الكائنات-الـ-constructor-والـ-destructor)
   - [نظام أحداث النطاق (Domain Event Bus) والخصومات والتصفح التلقائي](#26-نظام-أحداث-النطاق-domain-event-bus-والخصومات-والتصفح-التلقائي)
3. [الهندسة المعمارية وتقسيم الطبقات (Clean Architecture Layers)](#3-الهندسة-المعمارية-وتقسيم-الطبقات-clean-architecture-layers)
4. [دورة حياة الطلب من البداية للنهاية (Request Lifecycle)](#4-دورة-حياة-الطلب-من-البداية-للنهاية-request-lifecycle)
5. [هيكل المجلدات (Directory Structure)](#5-هيكل-المجلدات-directory-structure)
6. [طريقة التشغيل والاختبار (Quickstart & Commands)](#6-طريقة-التشغيل-والاختبار-quickstart--commands)

---

## 1. نظرة عامة على المشروع (Overview)

يقدم هذا المشروع مثالاً حياً لنظام متقدم في Kora يوضح كيف يتم بناء الـ APIs الإنتاجية الضخمة دون تشابك الأكواد أو تسريب الذاكرة أو الحاجة إلى كتابة كود مكرر لحقن قاعدة البيانات أو بناء استجابات الحماية.

---

## 2. الميزات المتقدمة المدمجة تلقائياً (Built-in Innovations)

### 2.1 الترحيل التلقائي ونثر البيانات (Auto-Migration & Database Seeder)
- يتعرف الفريم ورك تلقائياً على نماذج البيانات (`model User`, `model Product`, `model Order`, ...) ويولد استعلامات `CREATE TABLE IF NOT EXISTS` مع مفاتيح الـ Primary Key والفهارس آلياً دون الحاجة لكتابة استعلامات DDL يدوية:
```bash
# تشغيل الترحيل وزرع البيانات بطلب واحد:
curl http://localhost:8080/api/v1/database/migrate
```

### 2.2 قاعدة البيانات الشاملة دون تكرار (Ambient Database Pool - Zero Boilerplate)
- **وداعاً لتمرير `db: Database` يدوياً في كل Constructor**: تستخدم Kora مفهوم الـ **Ambient Context** حيث يستطيع أي مستودع (Repository) الاتصال بمجمع قاعدة البيانات (Connection Pool) تلقائياً أثناء الاستدعاء.
- **انعدام تسريب الذاكرة (Zero Memory Leaks)**: يضمن محلل الملكية في مترجم Kora إرجاع الاتصال إلى المجمع بمجرد انتهاء نطاق الدالة (RAII Borrow Checker).

### 2.3 الغلاف الآمن الموحد والحماية من Replay Attacks (Secure Nonce Envelope)
كل استجابة تخرج من السيرفر مغلفة آلياً ببنية مشفرة ومؤمنة تحتوي على:
```json
{
  "success": true,
  "statusCode": 200,
  "nonce": "9a8b7c6d5e4f3a2b1c0d",
  "securityToken": "sig_hmac256_kora_verified_token",
  "message": "Order placed successfully with applied discounts and domain event",
  "executionTimeMs": 4,
  "dataCount": 1
}
```
- **`nonce`**: قيمة عشوائية فريدة لكل استدعاء لمنع هجمات إعادة الإرسال (Replay Attacks).
- **`securityToken`**: توكن توقيع رقمي للتحقق من عدم التلاعب بالحزمة أثناء النقل.
- **`executionTimeMs`**: وقت التنفيذ الفعلي بالمللي ثانية لضمان شفافية الأداء.

### 2.4 تتبع الدوال وأكواد الخروج (Method Telemetry & Exit Codes Tracker)
يراقب متتبع الـ `TelemetryTracker` كل استدعاء لدوال الباك إند ويحدد كود الخروج والحالة:
- `ExitCode = 0` (`ExitSuccess`): نجاح كامل للعملية.
- `ExitCode = 1` (`ExitHandledError`): خطأ منطقي تم التعامل معه (مثل 400، 404، 429).
- `ExitCode = 2` (`ExitPanicRecovered`): استثناء حرج تم اعتراضه وحماية الخادم من الانهيار.

### 2.5 دورة حياة الكائنات: الـ Constructor والـ Destructor
- **Constructor (`init`)**: يُستخدم لحقن التبعيات وتهيئة الحالة.
- **Destructor (`destroy`)**: يُستدعى تلقائياً لتنظيف الموارد، إغلاق مقابس الشبكة، وتفريغ طوابير الأحداث وسجلات الـ Telemetry لمنع تراكم الذاكرة.

### 2.6 نظام أحداث النطاق (Domain Event Bus) والخصومات والتصفح التلقائي
- **Domain Event Bus (`DomainEventEmitter`)**: بث أحداث فورية مثل `OrderPlacedEvent` لتفعيل المهام الجانبية (إشعارات، تدقيق، ويب سوكت) دون تأخير الطلب الأساسي.
- **خصومات تلقائية (Auto Discounts)**: حساب الخصومات (مثل خصم 15% للطلبات التي تحوي 5 عناصر فأكثر) مباشرة داخل طبقة الـ Service.
- **تصفح تلقائي (Pagination Helper)**: حساب الصفحات والـ Offsets وإرجاع `PaginationMeta` في المسارات المجمعة.

---

## 3. الهندسة المعمارية وتقسيم الطبقات (Clean Architecture Layers)

```text
examples/realworld_clean_architecture/
├── README.md                      # التوثيق الهندسي الشامل
├── kora.toml                      # إعدادات الحزمة والسيرفر
├── docker-compose.yml             # تشغيل MySQL 8 + Redis + Kora App
├── .env.example                   # متغيرات البيئة
├── db/
│   └── migrations/
│       └── 001_ecommerce_init.sql # مخطط الجداول والبيانات التجريبية
└── src/
    ├── domain/
    │   ├── models.kora            # نماذج النطاق والـ Secure Envelope والتعدادات
    │   └── events.kora            # أحداث النطاق ومحرك البث (Event Bus)
    ├── database/
    │   └── migrator.kora          # محرك الترحيل التلقائي ونثر البيانات
    ├── middleware/
    │   └── middleware.kora        # حزمة الميدلوير (CORS, Rate Limit, Nonce, Telemetry, Compression)
    ├── repositories/
    │   └── repositories.kora      # مستودعات قاعدة البيانات مع الـ Ambient Pool
    ├── services/
    │   └── services.kora          # منطق الأعمال مع Destructors وأحداث النطاق
    ├── controllers/
    │   └── controllers.kora       # طبقة العرض وتغليف الاستجابات الآمنة
    └── main.kora                  # نقطة البداية وتسجيل المسارات
```

---

## 4. دورة حياة الطلب من البداية للنهاية (Request Lifecycle)

```
[ Client Request ]
       │
       ▼
1. Epoll Async Accept & Preflight CORS Check
       │
       ▼
2. Rate Limiting Check (Max 120 RPM)
       │
       ▼
3. Controller & Service Resolution via Compile-Time DI
       │
       ▼
4. Telemetry Timer Starts (Recording traceId & method)
       │
       ▼
5. Ambient Database Pool executes Query (Zero Constructor Boilerplate)
       │
       ▼
6. Business Logic executes & Domain Event is published (OrderPlacedEvent)
       │
       ▼
7. Destructor hooks run (Releasing pool connections & cleaning state)
       │
       ▼
8. Secure Enveloper injects cryptographic Nonce, Token & Execution Time
       │
       ▼
[ Encrypted JSON 200 OK Response with Nonce & Telemetry ]
```

---

## 5. طريقة التشغيل والاختبار (Quickstart & Commands)

### 5.1 الفحص الثابت السريع (Zero-Codegen Verification)
```bash
kora check examples/realworld_clean_architecture/src/main.kora
```

### 5.2 معاينة كود Rust فائق الأمان المولد من Kora
```bash
kora codegen examples/realworld_clean_architecture/src/main.kora
```

### 5.3 تجربة نقاط النهاية عبر curl

1. **فحص الصحة مع الغلاف الآمن والـ Nonce**:
```bash
curl http://localhost:8080/health
```

2. **تشغيل الترحيل التلقائي وزرع البيانات**:
```bash
curl http://localhost:8080/api/v1/database/migrate
```

3. **إنشاء طلب شراء وحساب الخصم وبث الحدث**:
```bash
curl -X POST http://localhost:8080/api/v1/orders \
  -H "Content-Type: application/json" \
  -d '{"userId": 1, "itemsCount": 5, "totalAmount": 500000}'
```

4. **الاستعلام عن طلب بالمعرّف مع التتبع**:
```bash
curl http://localhost:8080/api/v1/orders/8801
```
