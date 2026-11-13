# 🗄️ مخطط وهندسة قاعدة البيانات (Relational Database Schema)

يعتمد تطبيق **KoraPlay** على محرك **PostgreSQL** عبر Kora ORM للبيانات الدائمة، مع ضمان القيود التنافسية وحماية النزاهة المرجعية (Referential Integrity).

---

## 1. الكيانات والجداول الرئيسية (Entities & Tables)

### أ. جدول المستخدمين والجلسات (`users` & `auth_sessions`)
- **`users`**:
  - `id`: `BIGSERIAL PRIMARY KEY`
  - `email`: `VARCHAR(255) UNIQUE NOT NULL`
  - `password_hash`: `VARCHAR(255) NOT NULL` (Argon2id)
  - `full_name`: `VARCHAR(128) NOT NULL`
  - `role`: `VARCHAR(32) NOT NULL` (`ADMIN`, `TEACHER`, `STUDENT`)
  - `created_at`: `TIMESTAMP WITH TIME ZONE DEFAULT NOW()`
  - `updated_at`: `TIMESTAMP WITH TIME ZONE DEFAULT NOW()`
- **`refresh_tokens`**:
  - `id`: `BIGSERIAL PRIMARY KEY`
  - `user_id`: `BIGINT REFERENCES users(id) ON DELETE CASCADE`
  - `token_hash`: `VARCHAR(255) UNIQUE NOT NULL`
  - `expires_at`: `TIMESTAMP WITH TIME ZONE NOT NULL`
  - `revoked`: `BOOLEAN DEFAULT FALSE`

---

### ب. جدول الكويزات وبنك الأسئلة (`quizzes` & `questions`)
- **`quizzes`**:
  - `id`: `BIGSERIAL PRIMARY KEY`
  - `owner_id`: `BIGINT REFERENCES users(id) ON DELETE RESTRICT`
  - `title`: `VARCHAR(255) NOT NULL`
  - `description`: `TEXT`
  - `category`: `VARCHAR(64) DEFAULT 'General'`
  - `status`: `VARCHAR(32) DEFAULT 'DRAFT'` (`DRAFT`, `PUBLISHED`, `ARCHIVED`)
  - `is_public`: `BOOLEAN DEFAULT TRUE`
  - `created_at`: `TIMESTAMP WITH TIME ZONE DEFAULT NOW()`
- **`quiz_versions`**:
  - `id`: `BIGSERIAL PRIMARY KEY`
  - `quiz_id`: `BIGINT REFERENCES quizzes(id) ON DELETE CASCADE`
  - `version_number`: `INT NOT NULL`
  - `snapshot_data`: `JSONB NOT NULL` (تجميد كامل لكافة أسئلة وخيارات الاختبار)
  - `published_at`: `TIMESTAMP WITH TIME ZONE DEFAULT NOW()`
- **`questions`**:
  - `id`: `BIGSERIAL PRIMARY KEY`
  - `quiz_id`: `BIGINT REFERENCES quizzes(id) ON DELETE CASCADE`
  - `question_type`: `VARCHAR(32) NOT NULL` (`SINGLE_CHOICE`, `MULTIPLE_CHOICE`, `TRUE_FALSE`, `ORDERING`, ...)
  - `prompt`: `TEXT NOT NULL`
  - `media_url`: `VARCHAR(512)`
  - `time_limit_seconds`: `INT DEFAULT 20`
  - `base_points`: `INT DEFAULT 1000`
  - `sort_order`: `INT NOT NULL`
- **`question_answers`**:
  - `id`: `BIGSERIAL PRIMARY KEY`
  - `question_id`: `BIGINT REFERENCES questions(id) ON DELETE CASCADE`
  - `answer_text`: `TEXT NOT NULL`
  - `is_correct`: `BOOLEAN NOT NULL`
  - `sort_order`: `INT NOT NULL`

---

### ج. محرك الجلسات الحية والتنافس (`game_sessions`, `participants`, `answers`)
- **`game_sessions`**:
  - `id`: `BIGSERIAL PRIMARY KEY`
  - `quiz_version_id`: `BIGINT REFERENCES quiz_versions(id) ON DELETE RESTRICT`
  - `host_id`: `BIGINT REFERENCES users(id) ON DELETE RESTRICT`
  - `game_pin`: `VARCHAR(6) NOT NULL`
  - `status`: `VARCHAR(32) NOT NULL` (`CREATED`, `WAITING`, `ACTIVE`, `FINISHED`, `CANCELLED`)
  - `current_question_index`: `INT DEFAULT 0`
  - `started_at`: `TIMESTAMP WITH TIME ZONE`
  - `finished_at`: `TIMESTAMP WITH TIME ZONE`
  - **قيد فريد مشروط:** `CREATE UNIQUE INDEX idx_unique_active_pin ON game_sessions(game_pin) WHERE status NOT IN ('FINISHED', 'CANCELLED');`
- **`game_participants`**:
  - `id`: `BIGSERIAL PRIMARY KEY`
  - `session_id`: `BIGINT REFERENCES game_sessions(id) ON DELETE CASCADE`
  - `nickname`: `VARCHAR(64) NOT NULL`
  - `participant_token`: `VARCHAR(128) UNIQUE NOT NULL`
  - `total_score`: `INT DEFAULT 0`
  - `current_streak`: `INT DEFAULT 0`
  - `is_connected`: `BOOLEAN DEFAULT TRUE`
  - `joined_at`: `TIMESTAMP WITH TIME ZONE DEFAULT NOW()`
  - **قيد فريد:** `UNIQUE(session_id, nickname)`
- **`participant_answers`**:
  - `id`: `BIGSERIAL PRIMARY KEY`
  - `session_id`: `BIGINT REFERENCES game_sessions(id) ON DELETE CASCADE`
  - `participant_id`: `BIGINT REFERENCES game_participants(id) ON DELETE CASCADE`
  - `question_id`: `BIGINT REFERENCES questions(id) ON DELETE CASCADE`
  - `submitted_answer_ids`: `JSONB NOT NULL`
  - `is_correct`: `BOOLEAN NOT NULL`
  - `points_awarded`: `INT DEFAULT 0`
  - `time_taken_ms`: `INT NOT NULL`
  - `submitted_at`: `TIMESTAMP WITH TIME ZONE DEFAULT NOW()`
  - **قيد فريد صارم لمنع تكرار الإجابة:**
    `UNIQUE(session_id, participant_id, question_id)`

---

## 2. المؤشرات الحيوية لتحسين الأداء (High-Throughput Indexes)

```sql
-- سرعة الاستعلام عن الجلسة بواسطة الـ PIN
CREATE INDEX idx_game_sessions_pin_status ON game_sessions(game_pin, status);

-- سرعة جلب جميع متسابقي الجلسة وترتيب المتصدرين
CREATE INDEX idx_participants_leaderboard ON game_participants(session_id, total_score DESC);

-- سرعة فحص إجابات سؤال معين أثناء انتهاء الوقت
CREATE INDEX idx_answers_session_question ON participant_answers(session_id, question_id);
```
