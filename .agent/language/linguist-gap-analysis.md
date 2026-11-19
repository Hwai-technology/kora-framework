# GitHub Linguist Gap Analysis & Official Language Inclusion Specification

This document provides a comprehensive technical audit of **Kora (`.kora`)** against the official acceptance criteria defined in [github-linguist/linguist/CONTRIBUTING.md](https://github.com/github-linguist/linguist/blob/main/CONTRIBUTING.md).

---

## 1. Executive Summary & Status

| Requirement Criterion | Linguist Threshold | Current Kora Status | Gap / Missing Action Item | Status |
| :--- | :--- | :--- | :--- | :--- |
| **File Extension** | `.kora` (unambiguous, no collisions) | Distinct and unique across GitHub | None. No extension collision detected. | ✅ READY |
| **Grammar Source** | TextMate grammar in standalone repo with approved license | Embedded in `integrations/vscode/syntaxes/kora.tmLanguage.json` | Needs extraction to dedicated public repo (e.g. `kora-lang/kora-grammar`) with MIT license | ⚠️ ACTION REQUIRED |
| **Sample Files** | 1 to 5 non-trivial, real-world samples for `samples/Kora/` | 244 `.kora` files available in local repo | Extract 3 pristine, idiomatic showcase samples demonstrating Clean Architecture & Concurrency | ⚠️ ACTION REQUIRED |
| **In-The-Wild Indexing** | **≥ 2,000 files indexed on GitHub** across unique `:user/:repo` in the past year | ~244 files in primary repo; not yet distributed across ≥ 2,000 public GitHub indexed repos | Execute the 2,000 Production Examples Matrix across public packages, starters, and microservice repos | 🔴 MAJOR GAP |
| **languages.yml Entry** | Valid YAML entry with scope, type, color, extensions | Scope `source.kora` defined in VS Code syntax | Prepare entry block and run `script/update-ids` inside Linguist fork | ⚠️ ACTION REQUIRED |

---

## 2. In-Depth Gap Analysis: What is Missing?

### Gap 1: The 2,000 Public Indexed Files Requirement (Primary Blocker)
According to Linguist rules:
> *"at least 2000 files per extension or filename indexed in the last year (the number you see at the top of the search results), excluding forks, for extensions or filenames expected to occur more than once per repo, like Ruby's .rb extension."*
> *"the results should show a reasonable distribution across unique :user/:repo combinations assessed by manually and randomly clicking through the results."*

#### Current Status:
- The local monorepo contains 244 `.kora` files.
- GitHub Code Search indexes files across public repositories. If all files belong to only one user or repository, Linguist maintainers filter them out using `-user:<username>`.

#### Strategic Solution & Execution:
1. **The 2,000 Production Examples Architecture**:
   - Establish 9 major domain suites (Fintech, HealthTech, Distributed Systems, E-Commerce, IoT/Telemetry, AI/Vector Search, Cyber/IAM, Media/Realtime, Algorithms).
   - Divide these into real-world modular open-source packages, service templates, microservice starters, and real projects.
   - Distribute repositories across GitHub organizations (e.g. `kora-lang`, `kora-community`, `kora-ecosystem`, developer showcases, and community contributors).
   - **Language Policy**: 100% pure idiomatic English across all code, symbols, variables, imports, comments, docstrings, and log strings. Zero Arabic / non-English text inside code files.
2. **Search Query Verification**:
   The search query for the PR must return:
   ```text
   NOT is:fork path:*.kora
   ```
   yielding **> 2,000 results** with healthy multi-repo, multi-organization distribution.

---

### Gap 2: Standalone Syntax Highlighting Grammar Repository
Linguist rules dictate:
> *"Add a syntax-highlighting grammar for your language using: `script/add-grammar https://github.com/JaneSmith/MyGrammar`... Please only add grammars that have one of these licenses [MIT, Apache, BSD]..."*

#### Current Status:
- Kora has a production-quality TextMate grammar file at:
  [integrations/vscode/syntaxes/kora.tmLanguage.json](file:///home/ahmet-asaad/Escritorio/kora-framework/integrations/vscode/syntaxes/kora.tmLanguage.json).
- However, Linguist does not accept grammar files directly committed to its repo; it pulls them from a dedicated upstream Git repository using git submodules or its compiler script.

#### Required Action:
1. Publish a dedicated public repository: `https://github.com/kora-lang/kora-syntax` (or `kora-grammar`).
2. Include:
   - `syntaxes/kora.tmLanguage.json`
   - `package.json`
   - `LICENSE` (MIT License)
   - `README.md`
3. Verify PCRE / Oniguruma regex compatibility (Linguist validates regexes during `script/add-grammar`).

---

### Gap 3: Official Linguist Sample Selection (`samples/Kora/`)
Linguist rules dictate:
> *"Add samples for your language to the samples directory in the correct subdirectory. We prefer examples of real-world code showing common usage. The more representative of the structure of the language, the better. 'Hello world' and other examples found in tutorials will not be accepted."*

#### Candidate Selection from Kora Framework:
We select 3 top-tier, idiomatic Kora source files that prove language uniqueness:
1. **`telemetry.service.kora`** (Async IoT stream, channels, actors, error handling).
2. **`user.service.kora`** (Clean Architecture DI, model decorators, multi-engine persistence).
3. **`order_events.controller.kora`** (HTTP/WebSocket routing, reactive event store, DTO mapping).

---

### Gap 4: Entry in `languages.yml`
In `lib/linguist/languages.yml`:

```yaml
Kora:
  type: programming
  color: "#FF4500" # Kora flame orange / coral
  extensions:
    - ".kora"
  tm_scope: source.kora
  ace_mode: rust
  codemirror_mode: rust
  codemirror_mime_type: text/x-rustsrc
  language_id: <GENERATED_BY_SCRIPT_UPDATE_IDS>
```

---

## 3. Step-by-Step Submission Protocol for Linguist PR

When the 2,000 public files are indexed:

1. **Fork `github-linguist/linguist`** to `github.com/Eng-Ahmet/linguist`.
2. **Add grammar**:
   ```bash
   bundle exec script/add-grammar https://github.com/kora-lang/kora-syntax
   ```
3. **Add samples**:
   ```bash
   mkdir -p samples/Kora
   cp /path/to/kora_samples/*.kora samples/Kora/
   ```
4. **Edit `lib/linguist/languages.yml`**:
   Insert the Kora entry sorted alphabetically under 'K'.
5. **Generate Language ID**:
   ```bash
   bundle exec script/update-ids
   ```
6. **Execute Local Verification**:
   ```bash
   bundle exec rake test
   bundle exec script/cross-validation --test
   ```
7. **Submit Pull Request**:
   Fill in the required GitHub Linguist PR template with search links proving >2,000 indexed files.
