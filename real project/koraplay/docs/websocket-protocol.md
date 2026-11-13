# ⚡ مواصفات بروتوكول وأحداث الـ Realtime WebSockets

---

## 1. غلاف الرسالة المعياري (Standardized Event Envelope)

تتبع جميع الرسائل المتبادلة بين العميل وخادم KoraPlay غلافاً موحداً (Standard Envelope):

```json
{
  "eventId": "evt_98fbc12a-3e91",
  "eventType": "SUBMIT_ANSWER",
  "gameSessionId": "sess_102938",
  "participantId": "part_847291",
  "timestamp": 1773004500123,
  "payload": {},
  "correlationId": "req_001a"
}
```

---

## 2. الأحداث المرسلة من العميل إلى الخادم (Client ➔ Server)

| اسم الحدث | الوصف | الحقول الأساسية في `payload` |
|---|---|---|
| `JOIN_GAME` | طلب انضمام لاعب عبر الـ PIN | `{ "pin": "849201", "nickname": "Tariq", "avatar": "falcon" }` |
| `LEAVE_GAME` | مغادرة اللاعب للجلسة طواعية | `{ "reason": "user_exit" }` |
| `SUBMIT_ANSWER` | إرسال إجابة المتسابق للسؤال النشط | `{ "questionId": "q_42", "selectedAnswerIds": ["ans_1"], "clientTimeMs": 4200 }` |
| `REQUEST_RECONNECT` | طلب استعادة الاتصال بعد انقطاع الشبكة | `{ "participantToken": "tok_sec_991823" }` |
| `HOST_START_GAME` | أمر المعلم ببدء المنافسة رسمياً | `{ "countdownSeconds": 5 }` |
| `HOST_NEXT_QUESTION` | أمر المعلم بالانتقال للسؤال التالي | `{ "delaySeconds": 3 }` |
| `HOST_PAUSE_GAME` | إيقاف الجلسة مؤقتاً | `{ "reason": "host_break" }` |
| `HOST_RESUME_GAME` | استئناف اللعبة بعد الإيقاف المؤقت | `{}` |
| `HOST_KICK_PLAYER` | طرد متسابق مسيء أو مخالف | `{ "targetParticipantId": "part_847291" }` |
| `HOST_END_GAME` | إنهاء المسابقة فوراً وإعلان النتائج | `{}` |

---

## 3. الأحداث المبثوثة من الخادم إلى العملاء (Server ➔ Client)

| اسم الحدث | قناة البث | الوصف ومحتوى الـ `payload` |
|---|---|---|
| `GAME_JOINED` | Unicast (فردي) | تأكيد قبول انضمام اللاعب وإعطاؤه الـ `participantToken`. |
| `PLAYER_JOINED` | Room Broadcast | إشعار غرفة الانتظار بدخول لاعب جديد لتحديث شاشة المعلم. |
| `PLAYER_LEFT` | Room Broadcast | إشعار بخروج أو طرد لاعب. |
| `GAME_STARTED` | Room Broadcast | بدء العد التنازلي التمهيدي. |
| `QUESTION_STARTED` | Room Broadcast | إطلاق السؤال مع الخيارات العشوائية والعداد (بدون كشف الإجابة). |
| `QUESTION_TIMER_TICK` | Room Broadcast | تحديث مزامنة الثواني المتبقية (كل ثانية). |
| `ANSWER_ACCEPTED` | Unicast (فردي) | تأكيد استلام إجابة المتسابق بنجاح وحساب بونص السرعة. |
| `QUESTION_ENDED` | Room Broadcast | إغلاق باب الإجابة، كشف الإجابة الصحيحة، وعرض إحصائيات الإجابات. |
| `LEADERBOARD_UPDATED` | Room Broadcast | بث قائمة أفضل 5 متسابقين وترتيبهم والنقاط المكتسبة. |
| `GAME_FINISHED` | Room Broadcast | إعلان الفائزين الثلاثة الأوائل (Podium) وتوزيع الإحصائيات الكاملة. |
| `ERROR` | Unicast (فردي) | إشعار بحدوث خطأ (مثل: انتهاء وقت السؤال، تكرار الإجابة، إلخ). |

---

## 4. رموز الأخطاء اللحظية (Realtime Error Codes)

- `ERR_PIN_NOT_FOUND`: رمز اللعبة غير موجود أو منتهي.
- `ERR_NICKNAME_TAKEN`: الاسم المستعار مستخدم بالفعل في هذه الجلسة.
- `ERR_ROOM_FULL`: تجاوز الحد الأقصى للمشاركين في هذه الجلسة.
- `ERR_QUESTION_EXPIRED`: وصول الإجابة بعد انتهاء مهلة السؤال المحددة.
- `ERR_DUPLICATE_SUBMISSION`: تم تقديم إجابة مسبقة لهذا السؤال بالفعل.
- `ERR_UNAUTHORIZED_HOST`: العملية مخصصة للمعلم/المستضيف فقط.
