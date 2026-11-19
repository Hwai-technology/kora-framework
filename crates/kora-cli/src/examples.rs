//! Centralized, Dynamic Example Discovery Engine for Kora CLI & Toolchain Ecosystem.
//! Automatically scans, parses, categorizes, and validates example projects across `examples/`.

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleInfo {
    pub slug: String,
    pub name: String,
    pub category: String,
    pub difficulty: String,
    pub description: String,
    pub path: PathBuf,
    pub file_count: usize,
    pub has_tests: bool,
    pub has_docker: bool,
    pub entrypoint: Option<String>,
}

/// Locate the root `examples` directory by inspecting current directory, environment, and parent repositories.
pub fn find_examples_dir() -> Option<PathBuf> {
    if let Ok(dir) = env::var("KORA_EXAMPLES_DIR") {
        let p = PathBuf::from(dir);
        if p.exists() && p.is_dir() {
            return Some(p);
        }
    }

    let candidates = [
        "examples",
        "../examples",
        "../../examples",
        "../../../examples",
    ];

    for c in &candidates {
        let p = Path::new(c);
        if p.exists() && p.is_dir() {
            if let Ok(canon) = p.canonicalize() {
                return Some(canon);
            }
            return Some(p.to_path_buf());
        }
    }

    // Traverse upwards looking for repo root containing `examples/`
    if let Ok(mut current) = env::current_dir() {
        loop {
            let candidate = current.join("examples");
            if candidate.exists() && candidate.is_dir() {
                return Some(candidate);
            }
            if !current.pop() {
                break;
            }
        }
    }

    // Check user cache directory ~/.kora/cache/examples
    if let Ok(home) = env::var("HOME") {
        let cache_p = PathBuf::from(home).join(".kora/cache/examples");
        if cache_p.exists() && cache_p.is_dir() {
            return Some(cache_p);
        }
    }

    None
}

/// Recursively count `.kora` and `.kr` files in a directory
fn count_kora_files(dir: &Path) -> usize {
    let mut count = 0;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                let name = entry.file_name();
                let s = name.to_string_lossy();
                if !s.starts_with('.') && s != "target" && s != "node_modules" {
                    count += count_kora_files(&p);
                }
            } else if let Some(ext) = p.extension() {
                if ext == "kora" || ext == "kr" {
                    count += 1;
                }
            }
        }
    }
    count
}

/// Infer category from slug or keywords
fn infer_category(slug: &str, content: &str) -> String {
    let s = slug.to_lowercase();
    let c = content.to_lowercase();

    if s.contains("fintech")
        || s.contains("banking")
        || s.contains("hft")
        || s.contains("order_book")
        || s.contains("payment")
        || s.contains("fraud")
        || s.contains("derivatives")
        || s.contains("market_maker")
        || s.contains("credit_scoring")
        || s.contains("reconciliation")
        || s.contains("aml")
        || s.contains("treasury")
        || c.contains("banking")
        || c.contains("fintech")
    {
        return "Fintech & High-Frequency Trading".to_string();
    }

    if s.contains("health")
        || s.contains("ehr")
        || s.contains("hipaa")
        || s.contains("dicom")
        || s.contains("hl7")
        || s.contains("fhir")
        || s.contains("telehealth")
        || s.contains("pharmacovigilance")
        || s.contains("clinical")
        || c.contains("hipaa")
        || c.contains("clinical")
    {
        return "HealthTech & Clinical Systems".to_string();
    }

    if s.contains("ai")
        || s.contains("llm")
        || s.contains("vector")
        || s.contains("embedding")
        || s.contains("rag")
        || s.contains("transformer")
        || s.contains("neural")
    {
        return "AI & Machine Learning Systems".to_string();
    }

    if s.contains("auth")
        || s.contains("jwt")
        || s.contains("argon2")
        || s.contains("security")
        || s.contains("zero_trust")
        || s.contains("crypto")
        || s.contains("siem")
        || s.contains("vault")
    {
        return "Security & Cryptography".to_string();
    }

    if s.contains("iot")
        || s.contains("telemetry")
        || s.contains("sensor")
        || s.contains("mqtt")
        || s.contains("scada")
    {
        return "IoT & Embedded Streaming".to_string();
    }

    if s.contains("realtime") || s.contains("websocket") || s.contains("chat") {
        return "Realtime & WebSockets".to_string();
    }

    if s.contains("actor") || s.contains("concurrency") || s.contains("stream") {
        return "Concurrency & Actor Systems".to_string();
    }

    if s.contains("data_structures")
        || s.contains("tree")
        || s.contains("graph")
        || s.contains("filter")
        || s.contains("heap")
        || s.contains("indexer")
    {
        return "Data Structures & High-Performance Algorithms".to_string();
    }

    if s.contains("mysql")
        || s.contains("postgres")
        || s.contains("database")
        || s.contains("orm")
        || s.contains("kv_store")
        || s.contains("nosql")
        || s.contains("caching")
        || s.contains("storage")
        || s.contains("lsm")
    {
        return "Databases, ORM & Storage Engines".to_string();
    }

    if s.contains("clean_architecture") || s.contains("modular") || s.contains("monolith") {
        return "Clean Architecture & Design Patterns".to_string();
    }

    if s.contains("microservice") || s.contains("event_driven") || s.contains("enterprise") {
        return "Enterprise & Distributed Systems".to_string();
    }

    if s.contains("hello") || s.contains("crud") || s.contains("operator") || s.contains("type") {
        return "Language Fundamentals & Web APIs".to_string();
    }

    "Production Applications".to_string()
}

/// Parse metadata from a single example directory
pub fn parse_example_dir(dir: &Path, examples_root: &Path) -> Option<ExampleInfo> {
    if !dir.is_dir() {
        return None;
    }

    let file_count = count_kora_files(dir);
    if file_count == 0 {
        return None;
    }

    let rel_path = dir.strip_prefix(examples_root).unwrap_or(dir);
    let slug = rel_path.to_string_lossy().to_string();

    let mut name = slug.clone();
    let mut description = String::new();
    let mut category = String::new();
    let mut difficulty = String::new();

    let readme_path = dir.join("README.md");
    let mut readme_content = String::new();

    if readme_path.exists() {
        if let Ok(content) = fs::read_to_string(&readme_path) {
            readme_content = content;
            for line in readme_content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("# ") && name == slug {
                    let raw = &trimmed[2..].trim();
                    if let Some(pos) = raw.find(':') {
                        if raw[..pos].to_lowercase().contains("example") {
                            name = raw[pos + 1..].trim().to_string();
                        } else {
                            name = raw.to_string();
                        }
                    } else {
                        name = raw.to_string();
                    }
                }

                if (trimmed.starts_with("Category:") || trimmed.starts_with("**Category**:"))
                    && category.is_empty()
                {
                    if let Some(pos) = trimmed.find(':') {
                        category = trimmed[pos + 1..].trim().replace('*', "");
                    }
                }

                if (trimmed.starts_with("Difficulty:") || trimmed.starts_with("**Difficulty**:"))
                    && difficulty.is_empty()
                {
                    if let Some(pos) = trimmed.find(':') {
                        difficulty = trimmed[pos + 1..].trim().replace('*', "");
                    }
                }

                if description.is_empty()
                    && !trimmed.is_empty()
                    && !trimmed.starts_with('#')
                    && !trimmed.starts_with("---")
                    && !trimmed.starts_with("```")
                    && !trimmed.starts_with('>')
                {
                    description = trimmed.to_string();
                }
            }
        }
    }

    if name == slug {
        // Synthesize readable name from slug
        let without_num = if let Some(pos) = slug.find('_') {
            if slug[..pos].chars().all(|c| c.is_ascii_digit()) {
                &slug[pos + 1..]
            } else {
                &slug
            }
        } else {
            &slug
        };
        name = without_num
            .split('_')
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().chain(c).collect(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
    }

    if category.is_empty() {
        category = infer_category(&slug, &readme_content);
    }

    if difficulty.is_empty() {
        difficulty = if file_count >= 15
            || category.contains("Fintech")
            || category.contains("HealthTech")
            || category.contains("AI")
            || category.contains("Concurrency")
        {
            "Advanced".to_string()
        } else if file_count >= 6 {
            "Intermediate".to_string()
        } else {
            "Beginner".to_string()
        };
    }

    if description.is_empty() {
        description = format!(
            "Production-grade Kora implementation demonstrating Clean Architecture in {category}."
        );
    }

    let has_tests = dir.join("tests").exists() && count_kora_files(&dir.join("tests")) > 0;
    let has_docker = dir.join("docker-compose.yml").exists() || dir.join("Dockerfile").exists();

    let entrypoint = if dir.join("main.kora").exists() {
        Some("main.kora".to_string())
    } else if dir.join("src/main.kora").exists() {
        Some("src/main.kora".to_string())
    } else {
        None
    };

    Some(ExampleInfo {
        slug,
        name,
        category,
        difficulty,
        description,
        path: dir.to_path_buf(),
        file_count,
        has_tests,
        has_docker,
        entrypoint,
    })
}

/// Dynamically discover and parse all examples in the repository
pub fn discover_all_examples() -> Vec<ExampleInfo> {
    let mut examples = Vec::new();

    let root = match find_examples_dir() {
        Some(r) => r,
        None => return examples,
    };

    if let Ok(entries) = fs::read_dir(&root) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                let name = entry.file_name();
                let s = name.to_string_lossy();
                if s.starts_with('.') || s == "target" || s == "node_modules" {
                    continue;
                }

                if let Some(ex) = parse_example_dir(&p, &root) {
                    examples.push(ex);
                } else {
                    // Check for nested subdirectories (e.g. domain/project)
                    if let Ok(sub_entries) = fs::read_dir(&p) {
                        for sub in sub_entries.flatten() {
                            let sub_p = sub.path();
                            if sub_p.is_dir() {
                                if let Some(sub_ex) = parse_example_dir(&sub_p, &root) {
                                    examples.push(sub_ex);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Natural sorting: leading numbers first, then alphanumeric
    examples.sort_by(|a, b| {
        let num_a = extract_leading_number(&a.slug);
        let num_b = extract_leading_number(&b.slug);
        match (num_a, num_b) {
            (Some(na), Some(nb)) => na.cmp(&nb).then_with(|| a.slug.cmp(&b.slug)),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.slug.cmp(&b.slug),
        }
    });

    examples
}

fn extract_leading_number(slug: &str) -> Option<usize> {
    let prefix: String = slug.chars().take_while(|c| c.is_ascii_digit()).collect();
    prefix.parse::<usize>().ok()
}

/// Synchronize the dynamically discovered examples into JSON catalog for VS Code & Tooling
pub fn sync_catalog_to_file(target_file: &Path) -> std::io::Result<usize> {
    let examples = discover_all_examples();
    if let Some(parent) = target_file.parent() {
        fs::create_dir_all(parent)?;
    }

    #[derive(Serialize)]
    struct CatalogOutput<'a> {
        total: usize,
        repository: &'static str,
        examples: &'a [ExampleInfo],
    }

    let catalog = CatalogOutput {
        total: examples.len(),
        repository: "https://github.com/Hwai-technology/kora-framework.git",
        examples: &examples,
    };

    let json = serde_json::to_string_pretty(&catalog)?;
    fs::write(target_file, json)?;
    Ok(examples.len())
}
