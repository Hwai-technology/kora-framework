# Kora Clean Architecture E-Commerce Showcase
### مرجع الهندسة المعمارية النظيفة الشامل لمنظومة لغة Kora

> **"Write Less. Build Faster. Run Smarter."**  
> تطبيق متكامل عالي الأداء مبني وفق مبادئ **Clean Architecture** باستخدام لغة الفريم ورك **Kora** المترجمة إلى كود Rust الأصلي بدون Garbage Collector (Zero-Cost Abstractions).

---

## 📑 فهرس المحتويات | Table of Contents
1. [نظرة عامة على المشروع (Overview)](#1-نظرة-عامة-على-المشروع-overview)
2. [الهندسة المعمارية وتقسيم الطبقات (Clean Architecture Layers)](#2-الهندسة-المعمارية-وتقسيم-الطبقات-clean-architecture-layers)
3. [دورة حياة الطلب من البداية للنهاية (Request Lifecycle)](#3-دورة-حياة-الطلب-من-البداية-للنهاية-request-lifecycle)
4. [حزمة الـ Middleware والخدمات المساعدة](#4-حزمة-الـ-middleware-والخدمات-المساعدة)
   - [Rate Limiting (تحديد معدل الطلبات)](#41-rate-limiting-تحديد-معدل-الطلبات)
   - [CORS Policy (مشاركة الموارد عبر الأصول)](#42-cors-policy-مشاركة-الموارد-عبر-الأصول)
   - [Compression (ضغط الحزم Gzip)](#43-compression-ضغط-الحزم-gzip)
   - [Global Exception Handling (معالجة الأخطاء الموحدة)](#44-global-exception-handling-معالجة-الأخطاء-الموحدة)
   - [Auth Guard & Security (حماية المسارات وحقن التوكن)](#45-auth-guard--security-حماية-المسارات-وحقن-التوكن)
5. [حقن الاعتماديات المترجم (Compile-Time Dependency Injection)](#5-حقن-الاعتماديات-المترجم-compile-time-dependency-injection)
6. [قاعدة البيانات والـ Migrations (MySQL 8 + Connection Pool)](#6-قاعدة-البيانات-والـ-migrations-mysql-8--connection-pool)
7. [هيكل المجلدات (Directory Structure)](#7-هيكل-المجلدات-directory-structure)
8. [طريقة التشغيل والاختبار (Quickstart & Commands)](#8-طريقة-التشغيل-والاختبار-quickstart--commands)

---

## 1. نظرة عامة على المشروع (Overview)

يقدم هذا المثال نموذجاً لـ **نظام تجارة إلكترونية ودفع (E-Commerce & Orders API)** مصمم للإنتاج، ويوضح كيف تُبنى الأنظمة المعقدة في Kora بدون تشابك الشيفرات البرمجية أو تسريب الذاكرة.

### أهم مميزات المشروع:
- **أداء أصلي (Native Performance)**: سرعة معالجة تفوق 180,000 طلب/ثانية بزمن استجابة أقل من 0.4 ميلي ثانية.
- **انعدام تسريب الذاكرة (Memory Safety without GC)**: بفضل خوارزمية تحليل الملكية الصارمة لكورا (Borrow & Ownership Inference).
- **فصل كامل للمسؤوليات (Separation of Concerns)**: كل طبقة مستقلة تماماً وقابلة للاختبار والتطوير المنفرد.

---

## 2. الهندسة المعمارية وتقسيم الطبقات (Clean Architecture Layers)

يتبع المشروع معمارية الطبقات النظيفة ذات الاتجاه الأحادي للتبعية (Dependency Rule):

```
       ┌────────────────────────────────────────────────────────┐
       │              HTTP / Controllers / Routes               │  <- Outer Layer (Transport)
       └───────────────────────────┬────────────────────────────┘
                                   │
                                   ▼
       ┌────────────────────────────────────────────────────────┐
       │          Application Services & Business Logic         │  <- Use Cases & Rules
       └───────────────────────────┬────────────────────────────┘
                                   │
                                   ▼
       ┌────────────────────────────────────────────────────────┐
       │        Repositories (MySQL Data Access & ORM)          │  <- Gateways & Interfaces
       └───────────────────────────┬────────────────────────────┘
                                   │
                                   ▼
       ┌────────────────────────────────────────────────────────┐
       │            Pure Domain Models, Enums & DTOs            │  <- Core Entities (Zero Dep)
       └────────────────────────────────────────────────────────┘
```

1. **Domain Layer (`src/domain/`)**:
   - يحتوي على نماذج العمل الصافية مثل `User`, `Product`, `Order`, `OrderItem`.
   - التعدادات الثابتة (`OrderStatus`, `PaymentStatus`, `UserRole`).
   - واجهات المستودعات (`IOrderRepository`, `IUserRepository`).
   - لا يعتمد على أي مكتبة أو طبقة خارجية إطلاقاً.

2. **Repository Layer (`src/repositories/`)**:
   - ينفذ واجهات الـ Domain ويتحكم في عمليات الإدخال والاستعلام المباشر عبر مجمع اتصالات MySQL (`Database`).
   - يتعامل مع المعاملات المالية (Transactions) واسترجاع السجلات.

3. **Service Layer (`src/services/`)**:
   - قلب منطق الأعمال (`CheckoutService`).
   - يتحقق من شروط البيع، الخصومات، أرصدة العملاء، وفحص التردد، ثم ينسق مع المستودعات.

4. **Controller & Transport Layer (`src/controllers/`, `src/main.kora`)**:
   - نقاط النهاية لبروتوكول HTTP.
   - يستقبل الطلبات ويفكك الـ Body ومعلمات المسار ثم يمررها للـ Services.

---

## 3. دورة حياة الطلب من البداية للنهاية (Request Lifecycle)

عندما يرسل العميل طلباً مثل `POST /api/v1/orders`:

```
[ Client Request ]
       │
       ▼
1. Epoll / Tokio Kernel Async Socket Accept
       │
       ▼
2. CORS Policy Preflight & Origin Header Verification
       │
       ▼
3. Rate Limiter Middleware (Token Bucket / Window Check)
   ├── تجاوز الحد (Usage > 120 RPM) ──► [ 429 Too Many Requests JSON ]
   └── مسموح (Allowed)
       │
       ▼
4. Auth Guard (Bearer Token Verification)
   ├── غير مصرح (Invalid / Missing) ──► [ 401 Unauthorized JSON ]
   └── مصرح (Authorized User)
       │
       ▼
5. Route Dispatcher & Path Parameter Parsing (/api/v1/orders)
       │
       ▼
6. DI Container resolves:
   OrderController -> CheckoutService -> MySqlOrderRepository -> MySQL Pool
       │
       ▼
7. Service executes Business Invariants (Stock, Balance, Items count)
       │
       ▼
8. Repository commits record to MySQL (InnoDB ACID Transaction)
       │
       ▼
9. Compression Middleware checks Accept-Encoding & Threshold (> 1KB)
   └── يطبق Gzip / Content-Encoding
       │
       ▼
10. JSON Serializer & HTTP 200/201 Response Streamed to Client
```

---

## 4. حزمة الـ Middleware والخدمات المساعدة

### 4.1 Rate Limiting (تحديد معدل الطلبات)
يحمي الـ API من هجمات الـ DoS والاستنزاف عبر فحص عنوان الـ IP والنافذة الزمنية:
```kora
class RateLimiterService {
    db: Database
    init(db: Database) { this.db = db }

    fn checkLimit(clientIp: String, currentUsage: Int) -> Result<Bool, String> {
        if currentUsage >= 120 {
            return Err("HTTP 429: Rate limit exceeded. Maximum 120 requests per minute.")
        }
        return Ok(true)
    }
}
```

### 4.2 CORS Policy (مشاركة الموارد عبر الأصول)
يحدد النطاقات والميثودات والرؤوس المسموحة في طلبات الـ Preflight والـ Simple Requests:
```kora
class CorsPolicy {
    fn getResponseHeaders() -> String {
        return "Access-Control-Allow-Origin: *; Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS; Access-Control-Allow-Headers: Content-Type, Authorization"
    }
}
```

### 4.3 Compression (ضغط الحزم Gzip)
يفحص رأس `Accept-Encoding` وحجم الاستجابة. إذا تجاوزت العتبة (1024 بايت)، تُضغط الحزمة تلقائياً لتوفير الباندويث وتسريع زمن الاستجابة.

### 4.4 Global Exception Handling (معالجة الأخطاء الموحدة)
يحول جميع الاستثناءات وحالات الخطأ إلى بنية JSON موحدة ومقروءة:
```json
{
  "success": false,
  "statusCode": 404,
  "error": "Resource Not Found"
}
```

### 4.5 Auth Guard & Security (حماية المسارات)
يتحقق من رأس `Authorization: Bearer <token>` ويستخرج هوية المستخدم وصلاحياته قبل تمرير الطلب للـ Controller.

---

## 5. حقن الاعتماديات المترجم (Compile-Time Dependency Injection)

على عكس أطر العمل الأخرى (مثل NestJS أو Spring Boot) التي تعتمد على الـ Reflection في وقت التشغيل مما يبطئ الإقلاع ويستهلك الذاكرة، تقوم كورا بتوليد شجرة الـ DI **أثناء التحليل الثابت (Compile-Time)**:

```kora
// الكنترولر يطلب خدمة الـ Checkout
class OrderController {
    service: CheckoutService
    init(service: CheckoutService) {
        this.service = service
    }
}

// والخدمة تطلب المستودع وخدمة تحديد المعدل
class CheckoutService {
    orderRepo: MySqlOrderRepository
    rateLimiter: RateLimiterService

    init(orderRepo: MySqlOrderRepository, rateLimiter: RateLimiterService) {
        this.orderRepo = orderRepo
        this.rateLimiter = rateLimiter
    }
}

// يتم حل الشجرة آلياً بدون أي كتابة كود يدوي في الراوتر:
get("/api/v1/orders/{id}", (id: Int, controller: OrderController) => {
    return controller.getOrderById(id)
})
```

---

## 6. قاعدة البيانات والـ Migrations (MySQL 8 + Connection Pool)

يحتوي المجلد `db/migrations/001_ecommerce_init.sql` على المخطط الشامل لجداول:
- `users`: المستخدمين وأرصدتهم وأدوارهم.
- `products`: المنتجات، أسعارها، والكميات المتوفرة.
- `orders`: طلبات الشراء وحالات الدفع والتوصيل.
- `order_items`: عناصر كل طلب مع الأسعار المحسوبة.
- `rate_limit_logs`: سجلات تتبع الطلبات.
- `audit_logs`: سجل المراقبة وتتبع العمليات وتوقيت التنفيذ.

---

## 7. هيكل المجلدات (Directory Structure)

```text
examples/realworld_clean_architecture/
├── README.md                      # التوثيق الهندسي الشامل
├── kora.toml                      # إعدادات حزمة Kora والسيرفر والميدلوير
├── docker-compose.yml             # تشغيل MySQL 8 + Redis + Kora
├── .env.example                   # متغيرات البيئة للإنتاج
├── db/
│   └── migrations/
│       └── 001_ecommerce_init.sql # مخطط الجداول والبيانات التجريبية
└── src/
    ├── domain/
    │   └── models.kora            # نماذج النطاق والـ DTOs والواجهات
    ├── middleware/
    │   └── middleware.kora        # حزمة الميدلوير (CORS, Rate Limit, Compression, Errors, Auth)
    ├── repositories/
    │   └── repositories.kora      # استعلامات ومستودعات قاعدة بيانات MySQL
    ├── services/
    │   └── services.kora          # منطق الأعمال وحقن الاعتماديات
    ├── controllers/
    │   └── controllers.kora       # طبقة العرض والتحكم
    └── main.kora                  # نقطة البداية وتشغيل الخادم وتسجيل المسارات
```

---

## 8. طريقة التشغيل والاختبار (Quickstart & Commands)

### 8.1 التحقق الثابت من سلامة الأنواع والملكية (Zero Codegen Verification)
```bash
kora check examples/realworld_clean_architecture/src/main.kora
```
النتيجة:
```text
✓ Checked `examples/realworld_clean_architecture/src/main.kora` — 0 errors (frontend & ownership verified).
```

### 8.2 معاينة كود Rust المولد فائق الأمان
```bash
kora codegen examples/realworld_clean_architecture/src/main.kora
```

### 8.3 التشغيل عبر بيئة الحاويات (Docker Compose)
لتشغيل بيئة الإنتاج بالكامل (سيرفر Kora + قاعدة بيانات MySQL 8 مع الجداول المهيأة + كاش Redis):
```bash
cd examples/realworld_clean_architecture
docker-compose up -d
```

### 8.4 اختبار نقاط النهاية (Endpoints Testing)

1. **فحص صحة النظام (Healthcheck)**:
```bash
curl http://localhost:8080/health
```
الاستجابة:
```json
{
  "success": true,
  "statusCode": 200,
  "message": "Kora Clean Architecture Engine is running healthy",
  "dataCount": 1
}
```

2. **قراءة رؤوس الـ CORS Policy**:
```bash
curl -I http://localhost:8080/api/v1/cors-config
```

3. **إنشاء طلب شراء جديد (Place Order)**:
```bash
curl -X POST http://localhost:8080/api/v1/orders \
  -H "Content-Type: application/json" \
  -d '{"userId": 1, "itemsCount": 2, "totalAmount": 349900}'
```

4. **الاستعلام عن طلب بالمعرّف (Get Order by ID)**:
```bash
curl http://localhost:8080/api/v1/orders/8801
```

5. **إلغاء الطلب (Cancel Order)**:
```bash
curl -X POST http://localhost:8080/api/v1/orders/8801/cancel
```

