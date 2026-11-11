# Kora Strict Modular Clean Architecture Enterprise Showcase
### مرجع الهندسة المعمارية النظيفة الصارمة لمنظومة لغة Kora

> **"Write Less. Build Faster. Run Smarter."**  
> تطبيق إنتاجي متكامل عالي الأداء مقسم وفق معايير **Strict Modular Clean Architecture** باستخدام لغة الفريم ورك **Kora** المترجمة إلى كود Rust الأصلي بدون Garbage Collector (Zero-Cost Abstractions).

---

## 📑 فهرس المحتويات | Table of Contents
1. [الهيكل المعماري الصارم وتقسيم المجلدات (Directory Structure)](#1-الهيكل-المعماري-الصارم-وتقسيم-المجلدات-directory-structure)
2. [تقسيم الطبقات والأنواع المستقلة (Domain Segregation)](#2-تقسيم-الطبقات-والأنواع-المستقلة-domain-segregation)
   - [`src/enums/`: مجلد التعدادات المستقل](#21-srcenums-مجلد-التعدادات-المستقل)
   - [`src/constants/`: مجلد الثوابت والتكوينات](#22-srcconstants-مجلد-الثوابت-والتكوينات)
   - [`src/interfaces/`: مجلد واجهات العقود (IoC Interfaces)](#23-srcinterfaces-مجلد-واجهات-العقود-ioc-interfaces)
   - [`src/models/`: مجلد نماذج النطاق والـ DTOs](#24-srcmodels-مجلد-نماذج-النطاق-والـ-dtos)
   - [`src/database/`: إدارة قاعدة البيانات والترحيل التلقائي](#25-srcdatabase-إدارة-قاعدة-البيانات-والترحيل-التلقائي)
   - [`src/middleware/`: حزمة الميدلوير ومصنع الاستجابة الجاهز](#26-srcmiddleware-حزمة-الميدلوير-ومصنع-الاستجابة-الجاهز)
   - [`src/shared/services/`: حزمة الخدمات المؤسسية (Enterprise Services)](#27-srcsharedservices-حزمة-الخدمات-المؤسسية-enterprise-services)
3. [فرض النقاء المعماري من الكومبايلر (Compiler Architectural Purity Diagnostics)](#3-فرض-النقاء-المعماري-من-الكومبايلر-compiler-architectural-purity-diagnostics)
4. [هيكلة الوحدات الموديلار (Feature Modules)](#4-هيكلة-الوحدات-الموديلار-feature-modules)
   - [وحدة المصادقة والأمان (`src/modules/auth/`) — 6 ملفات متخصصة](#41-وحدة-المصادقة-والأمان-srcmodulesauth--6-ملفات-متخصصة)
   - [وحدة التجارة والطلبات (`src/modules/orders/`)](#42-وحدة-التجارة-والطلبات-srcmodulesorders)
5. [طريقة التشغيل والاختبار السريع (Quickstart & Commands)](#5-طريقة-التشغيل-والاختبار-السريع-quickstart--commands)

---

## 1. الهيكل المعماري الصارم وتقسيم المجلدات (Directory Structure)

تم تنظيم المشروع ليكون نموذجاً مرجعياً معيارياً للمشاريع الكبرى:

```text
examples/realworld_clean_architecture/src/
├── constants/                            # 1. مجلد الثوابت وإعدادات الأمان
│   └── app_constants.kora                # ثوابت الحدود، الرؤوس، ومفاتيح التشفير
│
├── enums/                                # 2. مجلد التعدادات المستقل
│   ├── http_status.kora                  # رموز الحالة (HttpStatus: Ok, Created, NotFound, ...)
│   ├── order_status.kora                 # حالات الطلب والدفع (OrderStatus, PaymentStatus)
│   └── user_role.kora                    # أدوار المستخدمين (UserRole: Admin, Customer)
│
├── interfaces/                           # 3. مجلد واجهات العقود (Contracts & Interfaces)
│   ├── order_repository_interface.kora   # IOrderRepository
│   ├── user_repository_interface.kora    # IUserRepository
│   └── event_emitter_interface.kora      # IDomainEventEmitter
│
├── models/                               # 4. مجلد نماذج النطاق وغلاف الاستجابة
│   ├── user_model.kora                   # User, UserRegisterInput
│   ├── product_model.kora                # Product model
│   ├── order_model.kora                  # Order, OrderItem, CreateOrderInput
│   └── response_envelope.kora            # SecureApiResponse, RequestContext, TelemetryTrace
│
├── database/                             # 5. مكتبات وإدارة قاعدة البيانات الشاملة
│   ├── database_pool.kora                # مجمع الاتصالات الشامل (Ambient DB Pool)
│   └── auto_migrator.kora                # محرك الترحيل التلقائي وزرع البيانات الأولية
│
├── middleware/                           # 6. حزمة الميدلوير ومصانع الاستجابة
│   ├── api_response_factory.kora         # مصنع ApiResponse (ok, created, notFound, badRequest)
│   ├── cors_middleware.kora              # سياسة CORS Preflight
│   ├── rate_limit_middleware.kora        # حماية التردد (RateLimiterService)
│   ├── compression_middleware.kora       # ضغط البيانات (CompressionService)
│   └── telemetry_middleware.kora         # متتبع الدوال وأكواد الخروج (TelemetryTracker)
│
├── shared/                               # 7. الخدمات والبنى المؤسسية المشتركة (Enterprise Services)
│   └── services/                         # خدمات قياسية جاهزة للإنتاج
│       ├── email_service.kora            # خدمة البريد الإلكتروني (SMTP / SES)
│       ├── sms_service.kora              # خدمة الرسائل النصية القصيرة ورمز OTP
│       ├── jwt_service.kora              # خدمة توليد وفك تشفير توكنات JWT
│       ├── crypto_service.kora           # تشفير AES-256-GCM وتجزئة Argon2id و HMAC
│       ├── storage_service.kora          # تخزين الملفات (Local Disk + Cloudflare R2 / S3)
│       ├── cache_service.kora            # نظام كاش متعدد المستويات (In-Memory + Redis)
│       ├── session_service.kora          # إدارة الجلسات، التجديد التلقائي، والإلغاء
│       ├── logger_service.kora           # سجلات الـ JSON الهيكلية والتتبع السياقي
│       └── validation_service.kora       # محرك التحقق الشامل من قيود الحقول والـ DTOs
│
├── modules/                              # 8. وحدات العمل الموديلار الصارمة (Feature Modules)
│   ├── auth/                             # وحدة المصادقة الكاملة (6 ملفات متخصصة)
│   │   ├── auth_controller.kora          # وحدة التحكم بنقاط دخول الـ Auth
│   │   ├── auth_service.kora             # منطق أعمال التوثيق والتحقق
│   │   ├── auth_types.kora               # الـ DTOs الخاصة بتسجيل الدخول والتوكن
│   │   ├── auth_repository.kora          # مستودع بيانات حسابات المستخدمين
│   │   ├── auth_cache.kora               # كاش الجلسات والتوكن في Redis
│   │   └── auth_guard.kora               # حارس التوثيق وفحص Bearer Token
│   │
│   └── orders/                           # وحدة إدارة الطلبات والتجارة
│       ├── order_controller.kora         # معالجة طلبات الـ HTTP للطلبات
│       ├── order_service.kora            # حساب الخصومات ومنطق الشراء
│       ├── order_repository.kora         # استعلامات أوامر الشراء في MySQL
│       ├── order_types.kora              # الـ DTOs والمدخلات
│       └── order_events.kora             # أحداث الشراء وتفريغ الطوابير
│
└── main.kora                             # ملف الإقلاع الرئيسي النظيف جداً (يربط المسارات فقط)
```

---

## 2. تقسيم الطبقات والأنواع المستقلة (Domain Segregation)

### 2.1 `src/enums/`: مجلد التعدادات المستقل
- تم فصل جميع التعدادات (`HttpStatus`, `OrderStatus`, `PaymentStatus`, `UserRole`) داخل ملفات مخصصة لضمان إعادة استخدامها في كامل طبقات النظام دون تبعيات دائرية.

### 2.2 `src/constants/`: مجلد الثوابت والتكوينات
- ملف [`app_constants.kora`](file:///home/ahmet/Desktop/Projects/Kora/examples/realworld_clean_architecture/src/constants/app_constants.kora) يحتوي على إعدادات الحدود القياسية (Rate limits, Page sizes, Security headers) لتوحيد التكوينات.

### 2.3 `src/interfaces/`: مجلد واجهات العقود (IoC Interfaces)
- يحتوي على عقود المستودعات وبث الأحداث (`IOrderRepository`, `IUserRepository`, `IDomainEventEmitter`) لعزل منطق الأعمال عن تفاصيل التخزين والاتصال الخارجي.

### 2.4 `src/models/`: مجلد نماذج النطاق والـ DTOs
- الكيانات النقية (`User`, `Product`, `Order`, `OrderItem`) ونماذج الاستجابة الموحدة (`SecureApiResponse`, `RequestContext`, `TelemetryTrace`).

### 2.5 `src/database/`: إدارة قاعدة البيانات والترحيل التلقائي
- **Ambient Pool**: الاتصال بقاعدة البيانات دون الحاجة لتمرير `db: Database` يدوياً في كل Constructor، مع انعدام تسريب الذاكرة بفضل فحص الملكية الصارم في Kora.
- **Auto-Migration & Seeding**: التعرف الآلي على الجداول ونثر البيانات التجريبية الأولية بنقرة واحدة.

### 2.6 `src/middleware/`: حزمة الميدلوير ومصنع الاستجابة الجاهز
- مصنع [`ApiResponse`](file:///home/ahmet/Desktop/Projects/Kora/examples/realworld_clean_architecture/src/middleware/api_response_factory.kora) يتيح إرجاع الردود بسطر واحد:
```kora
return this.response.ok("Order fetched successfully", 1)
return this.response.created("Order placed successfully", 1)
return this.response.notFound("Order not found")
return this.response.badRequest("Invalid input")
```
حيث يُحقن الـ `nonce` وتوقيع الـ HMAC ووقت التنفيذ `executionTimeMs` آلياً.

---

### 2.7 `src/shared/services/`: حزمة الخدمات المؤسسية (Enterprise Services)
توفر المنظومة 7 خدمات قياسية مدمجة تغطي كافة متطلبات المشاريع الكبرى:
1. **Email Service (`email_service.kora`)**: إرسال رسائل التفعيل والقوالب عبر خوادم SMTP وخدمة AWS SES مع استراتيجيات إعادة المحاولة.
2. **SMS Service (`sms_service.kora`)**: إرسال وتوليد رموز التحقق الثنائي (OTP) والرسائل النصية عبر مزودي الاتصالات مثل Twilio.
3. **JWT Service (`jwt_service.kora`)**: توقيع والتحقق من رموز الـ Access Token و Refresh Token مع دعم خوارزميات التشفير وفحص فترات الانتهاء.
4. **Crypto Service (`crypto_service.kora`)**: تجزئة كلمات المرور باستخدام خوارزمية Argon2id المقاومة لهجمات القوة الغاشمة، وتشفير الحقول الحساسة بتقنية AES-256-GCM مع توقيع HMAC-SHA256.
5. **Storage Service (`storage_service.kora`)**: رفع وحذف وتوليد الروابط الموقعة (Signed URLs) للملفات عبر التخزين السحابي (Amazon S3 / MinIO) والتخزين المحلي.
6. **Multi-tier Cache Service (`cache_service.kora`)**: إدارة الذاكرة المخبئية ثنائية المستويات (ذاكرة محلية سريعة L1 + كاش موزع Redis L2) مع مدة صلاحية دقيقة (TTL).
7. **Session Service (`session_service.kora`)**: حفظ واسترجاع جلسات المستخدمين، دعم التمديد التلقائي (Sliding Expiration)، وإلغاء الجلسات فورياً عند تسجيل الخروج.
8. **Structured Logger Service (`logger_service.kora`)**: تسجيل أحداث النظام في صيغة JSON القياسية مع مستويات الخطورة (`INFO`, `WARN`, `ERROR`) والتوقيت ومعرّفات التتبع `traceId`.
9. **Universal Validation Service (`validation_service.kora`)**: التحقق الديناميكي عالي السرعة من صحة الحقول (Required, Email, Range, UUID, Phone, URL) مع خرائط أخطاء تفصيلية.
10. **Cloudflare R2 & Local Storage (`storage_service.kora`)**: رفع وحذف وتوليد الروابط الموقعة للملفات عبر Cloudflare R2 والتخزين المحلي.

---

### 2.8 محددات الوصول والبناء التلقائي في الكلاسات (Access Modifiers & Auto-Constructor)
- تدعم كلاسات Kora محددات الوصول الصريحة:
  - `public`, `private`, `protected`, `readonly` للحقول والدوال.
  - **Auto-Constructor (حقن المعاملات)**: في حال عدم تعريف دالة `init`، يقوم المترجم تلقائياً بتوليد Constructor كامل يسند جميع الحقول إلى `this.field = field`.
  - **Parameter Properties**: عند تعريف محدد وصول داخل معاملات `init` (مثل `init(public repo: Repo, private cache: Cache)`)، يتم تحويلها تلقائياً إلى حقول في الكلاس دون الحاجة لتكرار تعريفها!

### 2.9 خصائص وسمات الـ Model المتقدمة (Schema Attributes & Decorators)
- تدعم الـ `model` سمات القيود والـ Indexing مباشرة:
  - `@unique`: تحديد فرادة الحقل.
  - `@index`: إنشاء فهرس لقاعدة البيانات لتسريع الاستعلام.
  - `@required`: حقل إلزامي لا يقبل الفراغ.
  - `@email`: التحقق من نسق البريد الإلكتروني.
  - `@min(n)`, `@max(n)`: قيود الحدود الدنيا والقصوى.
  - `@default(val)`: تعيين القيمة الافتراضية.

### 2.10 تنسيق الردود والبيانات حصراً بأسلوب `kebab-case`
- تفرض المنظومة توليد وتنسيق كافة مفاتيح الـ JSON في الردود والطلبات والـ DTOs بأسلوب **`kebab-case`** تلقائياً عبر `#[serde(rename_all = "kebab-case")]`:
  - `status-code`, `security-token`, `execution-time-ms`, `order-number`, `total-amount`, `user-id`.
  - ترويسات HTTP: `content-type`, `authorization`, `x-request-id`, `x-kora-trace`.

### 2.11 إلغاء `struct` واعتماد `model` و `class` حصراً (Struct-Free Architecture)
- تم إزالة الكلمة المفتاحية `struct` من لغة Kora لصالح البنية النظيفة:
  - **`model`**: يُستخدم لتعريف كائنات البيانات، الجداول، والـ DTOs فقط (مثل `model User`, `model SecureApiResponse`).
  - **`class`**: يُستخدم لتعريف الخدمات، المتحكمات، والحاويات التي تحتوي على منطق أعمال وحقن تبعيات `init(...)`.

### 2.9 نمط الاستيراد المتقدم `import { ... } from "..."`
- تدعم Kora استيراد الرموز المحددة بدقة تامة لتسهيل التنقل البرمجي (Go to Definition) ومعرفة مصدر كل عنصر:
```kora
import { HttpStatus } from "enums/http_status";
import { User, UserProfileDto } from "models/user_model";
import { Order, CreateOrderInput } from "models/order_model";
import { SecureApiResponse, LogEntry } from "models/response_envelope";
import { Logger } from "shared/services/logger_service";
```

### 2.10 هيكل تشغيل المشروع الكائني `class Application`
- لا يعمل المشروع من دوال سائبة، بل يُنظّم بالكامل داخل حاوية تطبيق موحدة `class Application` تحقن كافة المتحكمات والميدلوير والخدمات عبر الـ DI المترجم في وقت البناء (Zero Runtime Overhead).

### 2.11 أنواع الويب الخلفية الأساسية (Backend First-Class Types)
- تتضمن منظومة الأنواع في Kora دعماً أصيلاً لعناصر الـ Backend:
  - **`Request`**: يوفر الحقول `path`, `method`, `body`, `ip`, `url`, `headers`، والدوال `header(name)`, `query(key)`, `param(key)`.
  - **`Headers`**: يوفر `get(name)`, `set(name, value)`, `has(name)`.
  - **`Response`**: يوفر `status(code)`, `json(payload)`, `text(msg)`, `header(k, v)`.

### 2.12 دعم `null` الحصري ومنع `undefined` قطعياً (`E2005`)
- في لغة Kora، تم اعتماد الكلمة المفتاحية **`null`** كقيمة أصيلة أولى لتمثيل غياب القيمة (`Option<T>`).
- يُمنع منعاً باتاً استخدام كلمة `undefined` في أي مكان في الكود؛ وفي حال محاولة كتابة `undefined` يُطلق المترجم خطأ تشخيصياً صريحاً:
  ```text
  error[E2005]: `undefined` is not permitted in Kora; use `null` exclusively to represent absence of value.
  ```
- يمكن استخدام `null` في المقارنات المباشرة ومطابقة الأنماط (Pattern Matching):
  ```kora
  if req.header("authorization") == null {
      return Err("Authorization header is missing")
  }

  match optUser {
      Some(u) => handleUser(u),
      null => handleNotFound(),
  }
  ```

### 2.13 حراس الأمان الديناميكيون المتصلون بالطلب (`AuthGuard`)
- تم بناء حارس الأمان `AuthGuard` ليتعامل مع الطلبات الحقيقية `req: Request` بشكل ديناميكي كامل بدون قيم وهمية ثابتة:
  - استخراج الترويسة ديناميكياً: `req.header("authorization")`.
  - التحقق من `null` وفراغ القيمة.
  - التحقق التشفيري عبر `JwtService` وإصدار توكنات موقعة حقيقية.
  - تسجيل محاولات الوصول والرفض عبر خدمة السجلات المنظمة `Logger`.

### 2.14 مستودعات البيانات المتصلة بالمجمع الديناميكي (`AmbientDatabasePool`)
- تم التخلص بالكامل من البيانات الثابتة في `AuthRepository` و `AuthService`:
  - حجز وإطلاق الاتصالات ديناميكياً عبر مجمع الاتصالات التلقائي `AmbientDatabasePool`.
  - تنفيذ الاستعلامات الحقيقية عبر `pool.executeQuery(...)` ومعالجة نتائج الـ `Result` والـ `Option`.
  - توليد توكنات الجلسات المشفرة ديناميكياً مع ختم زمني حقيقي وتخزينها في الكاش.

---

## 3. فرض النقاء المعماري من الكومبايلر (Compiler Architectural Purity Diagnostics)

لأول مرة في لغات الويب الحديثة، تم تضمين قواعد صارمة داخل مترجم Kora (`kora-cli`) لمنع التداخل المعماري وإجبار المطور على الالتزام بالكود النظيف:

| رمز الخطأ | القاعدة المعمارية المفروضة | السلوك الممنوع |
|:---|:---|:---|
| `E5001` | **Purity of `enums/`** | يُمنع منعاً باتاً وجود دوال أو مسارات أو أصناف برمجية داخل أي ملف ضمن مجلد `enums/`. |
| `E5002` | **Purity of `models/` & `interfaces/`** | يُمنع احتواء ملفات النماذج أو الواجهات على منطق تنفيذي، مسارات HTTP، أو دالة تشغيل الخادم `run()`. |
| `E5003` | **Segregation in Controllers & Services** | يُمنع تعريف `enum` أو `model` مضمّن في ملفات الـ Controllers والـ Services، بل يجب استيرادها حصراً عبر `import`. |
| `E5004` | **Clean Root Entry (`main.kora`)** | يُمنع تعريف أي كائن `model` أو `enum` أو `interface` داخل `main.kora`؛ يجب أن يقتصر الملف على الربط والتشغيل. |

أي خرق لهذه القواعد يُنتج خطأ 컴بايلر فوري يمنع بناء التطبيق أو تشغيله.

---

## 4. هيكلة الوحدات الموديلار (Feature Modules)

### 4.1 وحدة المصادقة والأمان (`src/modules/auth/`) — 6 ملفات متخصصة
تحتوي على بنية متكاملة مغلقة:
1. **`auth_controller.kora`**: استلام طلبات الدخول والتسجيل وإرجاع الاستجابات المغلفة.
2. **`auth_service.kora`**: التحقق من بيانات الدخول وتوليد التوكن واستدعاء الكاش.
3. **`auth_types.kora`**: كائنات الـ DTOs (`LoginInput`, `RegisterInput`, `AuthSessionDto`).
4. **`auth_repository.kora`**: مستودع البحث عن الحسابات وإنشائها.
5. **`auth_cache.kora`**: تخزين جلسات المستخدمين في Redis والتحقق من التوكنات الملغاة.
6. **`auth_guard.kora`**: فحص ترويسة `Authorization: Bearer <token>` وتفويض الصلاحيات.

### 4.2 وحدة التجارة والطلبات (`src/modules/orders/`)
تحتوي على معالجة الطلبات، تطبيق الخصومات التلقائية (15% للطلبات التي تحوي 5 عناصر فأكثر)، وبث أحداث النطاق `OrderPlacedEvent`.

---

## 5. طريقة التشغيل والاختبار السريع (Quickstart & Commands)

### 5.1 الفحص الثابت الشامل لجميع الملفات
```bash
for f in $(find src -name "*.kora"); do kora check "$f"; done
```
جميع ملفات النظام الـ 30 تجتاز الفحص بنجاح:
```text
✓ Checked `src/...` — 0 errors (frontend & ownership verified).
```

### 5.2 توليد كود Rust الأصلي
```bash
kora codegen src/main.kora
```

### 5.3 اختبار نقاط النهاية عبر curl

```bash
# 1. فحص الصحة
curl http://localhost:7582/health

# 2. تشغيل الترحيل التلقائي
curl http://localhost:7582/api/v1/database/migrate

# 3. تسجيل الدخول عبر وحدة الـ Auth
curl -X POST http://localhost:7582/api/v1/auth/login

# 4. إنشاء طلب شراء وحساب الخصم وبث الحدث
curl -X POST http://localhost:7582/api/v1/orders
```
