# 🎮 KoraPlay — Enterprise Realtime Interactive Learning & Quiz Platform
### المبنية بالكامل باستخدام لغة وبرمجيات كورا (Kora Programming Language & Framework)

---

## 🌟 نظرة عامة ورؤية المشروع (Project Vision)

**KoraPlay** هي منصة تعليمية وتنافسية تفاعلية متكاملة في الوقت الفعلي (Real-time Interactive Learning & Quiz Platform) مستوحاة من منصات عالمية مثل *Kahoot!* و *Quizizz*، ولكن بهوية تقنية فريدة ومعمارية برمجية صارمة مبنية **بالكامل بنسبة 100% باستخدام لغة Kora**.

الهدف الأساسي ليس مجرد تطبيق أسئلة واختبارات بسيط، بل هو **الإثبات الواقعي الأكبر (Definitive Real-World Benchmark)** لقدرة لغة Kora على مواجهة التحديات الهندسية المتقدمة في الخوادم:
- **تزامن كثيف (High Concurrency):** إدارة آلاف المتسابقين في نفس اللحظة عبر الـ WebSockets.
- **محرك قرارات حاسم بالخادم (Server-Authoritative Engine):** احتساب النقاط، قياس الفروق الزمنية بالملي ثانية، ومنع أي تلاعب أو غش من جهة العميل.
- **معمارية معيارية نظيفة (Strict Clean Architecture):** فصل صارم بين الطبقات (Controllers, Services, Repositories, Domain Models, Enums) مفروض آلياً من مترجم Kora.
- **تخزين هجين موثوق:** PostgreSQL للبيانات الدائمة غير القابلة للضياع، مع Redis لإدارة الحالات اللحظية السريعة.

---

## 🏛️ الهيكل المعماري للنظام (System Architecture)

يعتمد مشروع KoraPlay على معمارية الـ **Modular Clean Monolith**، حيث تُقسّم النطاقات الوظيفية (Domain Features) إلى وحدات مستقلة تماماً يمكن تحويل أي منها مستقبلاً إلى Microservice عند الحاجة:

```text
                                  ┌──────────────────────────┐
                                  │   Participants / Hosts   │
                                  │   (Web, Mobile, Screens) │
                                  └─────────────┬────────────┘
                                                │
                                HTTP REST / WS  │  Events (JSON)
                                                ▼
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                                    KORAPLAY ENGINE                                     │
│                                                                                        │
│   [Presentation Layer]                                                                 │
│     ├── REST Controllers (Auth, Quizzes, Games, Analytics)                            │
│     └── Realtime Gateway (WebSocket Pub/Sub, Room Sessions, Heartbeats)                │
│                                                                                        │
│   [Application & Domain Logic]                                                         │
│     ├── Identity & RBAC (Argon2id, JWT Rotation, Device Sessions)                      │
│     ├── Quiz & Question Domain (Snapshots, Versioning, Dynamic Question Types)         │
│     ├── Game Session State Machine (PIN Gen, Room Lifecycle, Reconnection)             │
│     ├── Scoring & Streak Engine (Deterministic decay curve, Bonus Multipliers)        │
│     └── Anti-Cheat & Rate Limiting Guards                                              │
│                                                                                        │
│   [Infrastructure Layer]                                                               │
│     ├── PostgreSQL Repository (Kora ORM, ACID Transactions, Immutable Results)        │
│     └── Redis Realtime State Hub (Room Presence, Ephemeral Game Caches)                │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 🎯 الميزات الرئيسية للمنصة (Core Capabilities)

### 1. منظومة الأسئلة والاختبارات الفائقة (Quiz & Assessment Engine)
- دعم أنواع أسئلة متعددة: اختيار من متعدد، صح/خطأ، إجابة كتابية، ترتيب تسلسلي، استطلاعات رأي، وأسئلة وسائط (صور/فيديو/معادلات).
- **النسخ المتجمدة (Immutable Quiz Snapshots):** بمجرد بدء لعبة حية، يتم أخذ لقطة غير قابلة للتعديل من الاختبار لمنع تأثر الجلسة بأي تعديل يقوم به المعلم أثناء المسابقة.
- معايير تقييم ذكية وبونص السرعة ومضاعفات السلسلة المتتالية (Streak Multipliers).

### 2. محرك الجلسات المباشرة والألعاب اللحظية (Live Game Sessions)
- توليد رمز لعبة مكون من 6 أرقام (Unique 6-digit Game PIN) مع حماية ضد التخمين والتكرار.
- انضمام فوري للمتسابقين كضيوف (Guest Participants) باسم مستعار دون الحاجة لتسجيل حساب.
- آلة حالات صارمة (Finite State Machine):
  `CREATED` ➔ `WAITING_FOR_PLAYERS` ➔ `STARTING` ➔ `QUESTION_ACTIVE` ➔ `QUESTION_CLOSED` ➔ `LEADERBOARD` ➔ `FINISHED`.
- تسامح عالي مع انقطاع الاتصال (Reconnection Tolerance) للمتسابقين والمضيف دون فقدان النقاط.

### 3. الأمان والحماية الصارمة (Security & Anti-Cheating)
- **منع تسريب الإجابات:** لا يتم إرسال الإجابة الصحيحة أو حتى الإشارة إليها للعملاء إلا بعد إغلاق عداد وقت السؤال رسمياً من الخادم.
- **توقيع وتشفير الطوابع الزمنية:** منع التلاعب بتوقيت الإجابة عبر احتساب وقت وصول الحزمة ومطابقته مع ساعة السيرفر.
- حماية ضد الهجمات المتكررة (Idempotency Keys & Replay Attack Defense).

---

## 📂 هيكلية الملفات وتوزيع النطاقات

```text
real project/koraplay/
├── kora.toml                         # ملف تعريف المشروع وتبعيّات لغة Kora
├── Dockerfile                        # بناء حاوية بيئة التشغيل
├── docker-compose.yml                # إعداد خوادم PostgreSQL و Redis
├── docs/                             # التوثيق الفني الشامل والتصورات المعمارية
│   ├── architecture.md               # التصميم المعماري وحدود الموديولات
│   ├── quiz-system-concept.md        # التصور العميق لمنظومة الأسئلة والاختبارات
│   ├── game-engine-lifecycle.md      # دورة حياة اللعبة وآلة الحالات اللحظية
│   ├── websocket-protocol.md         # مواصفات أحداث ومراسلات الـ WebSockets
│   ├── database-schema.md            # المخطط العلائقي لقواعد البيانات والمؤشرات
│   ├── api-specification.md          # توثيق نقاط النهاية لـ REST API v1
│   └── KORA_LANGUAGE_EVALUATION.md   # سجل تقييم لغة Kora وقدرات المترجم
├── src/
│   ├── main.kora                     # نقطة الدخول وتهيئة الخادم والمسارات
│   ├── enums/                        # الثوابت المعمارية والحالات
│   │   ├── http_status.kora
│   │   ├── user_role.kora
│   │   ├── question_type.kora
│   │   └── session_status.kora
│   ├── models/                       # النماذج العامة وأغلفة الردود
│   │   └── response_envelope.kora
│   ├── shared/services/              # الخدمات العامة (التسجيل والتشفير)
│   │   └── logger_service.kora
│   └── features/                     # موديولات النطاق المعماري (Clean Features)
│       ├── users/                    # إدارة المستخدمين والتوثيق
│       ├── quizzes/                  # إدارة الكويزات وبنك الأسئلة
│       ├── games/                    # محرك الجلسات والتنافس الحي
│       └── scoring/                  # محرك احتساب النقاط والمتصدرين
```

---

## 🛠️ أوامر التشغيل السريع عبر Kora CLI

تم بناء KoraPlay باستخدام أدوات المترجم الرسمية لـ **Kora Toolchain (v0.2.0)**:

```bash
# 1. فحص سلامة النحو والأنواع والملكية (Ownership & Purity)
./target/release/kora check "real project/koraplay/src/main.kora"

# 2. تشغيل جناح الاختبارات الآلية
./target/release/kora test "real project/koraplay"

# 3. تشغيل الخادم في وضع التطوير المحلي
./target/release/kora run "real project/koraplay/src/main.kora"

# 4. تشغيل خادم التطوير مع إعادة التحميل التلقائي (Hot-Reload)
./target/release/kora watch "real project/koraplay/src/main.kora"

# 5. توليد توثيق OpenAPI / Swagger UI تلقائياً
./target/release/kora swagger "real project/koraplay/src/main.kora"
```

---

## 📈 خارطة الطريق ومراحل التطوير (Roadmap)

- [x] **المرحلة 1:** تأسيس المشروع، تهيئة Clean Architecture، وإعداد وثائق التصور الفني الشاملة.
- [ ] **المرحلة 2:** بناء نظام المصادقة، هوية المستخدمين، وتأمين كلمات المرور عبر Argon2id.
- [ ] **المرحلة 3:** موديول بنك الأسئلة، أنواع الأسئلة المتعددة، ونظام الـ Snapshots.
- [ ] **المرحلة 4:** محرك الجلسات الحية، توليد الـ PIN، وغرف التنافس المباشرة.
- [ ] **المرحلة 5:** بوابة الـ WebSockets اللحظية وبث أحداث الأسئلة والعدادات.
- [ ] **المرحلة 6:** محرك احتساب النقاط، احتساب الفروق الزمنية، وبث الـ Leaderboard المباشر.
- [ ] **المرحلة 7:** التحليلات الإحصائية المتقدمة، نظام الأوسمة والبطولات (Tournaments).
- [ ] **المرحلة 8:** اختبارات الحمل العالي (Load Testing)، تعزيز الأمان، ونشر بيئة الإنتاج.
