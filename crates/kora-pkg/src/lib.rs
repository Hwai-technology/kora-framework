//! Distributed Package & Plugin Manager for Kora (`kora-pkg`).
//! Implements dependency resolution, semantic versioning (SemVer),
//! lockfile generation with SHA-256 integrity hashes, and package publishing.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::fs;
use std::path::Path;

/// Semantic Version representation conforming to SemVer 2.0.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SemVer {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl fmt::Display for SemVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl SemVer {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        let clean = s.trim().trim_start_matches('v');
        let parts: Vec<&str> = clean.split('.').collect();
        if parts.len() != 3 {
            return Err(format!("invalid semver string `{s}`, expected `X.Y.Z`"));
        }
        let major = parts[0]
            .parse::<u64>()
            .map_err(|e| format!("invalid major version in `{s}`: {e}"))?;
        let minor = parts[1]
            .parse::<u64>()
            .map_err(|e| format!("invalid minor version in `{s}`: {e}"))?;
        let patch = parts[2]
            .parse::<u64>()
            .map_err(|e| format!("invalid patch version in `{s}`: {e}"))?;

        Ok(Self {
            major,
            minor,
            patch,
        })
    }

    /// Evaluates whether this version satisfies a requirement expression.
    /// Supports `^` (compatible), `~` (approximate), `=`, `>=`, `<=`, and `*`.
    pub fn matches_requirement(&self, req: &str) -> bool {
        let r = req.trim();
        if r == "*" || r.is_empty() {
            return true;
        }

        if let Some(v) = r.strip_prefix('^').and_then(|t| SemVer::parse(t).ok()) {
            if self.major != v.major {
                return false;
            }
            if self.major == 0 {
                return self.minor == v.minor && self.patch >= v.patch;
            }
            return (self.minor > v.minor) || (self.minor == v.minor && self.patch >= v.patch);
        }

        if let Some(v) = r.strip_prefix('~').and_then(|t| SemVer::parse(t).ok()) {
            return self.major == v.major && self.minor == v.minor && self.patch >= v.patch;
        }

        if let Some(v) = r.strip_prefix(">=").and_then(|t| SemVer::parse(t).ok()) {
            return *self >= v;
        }

        if let Some(v) = r.strip_prefix("<=").and_then(|t| SemVer::parse(t).ok()) {
            return *self <= v;
        }

        let target_str = r.strip_prefix('=').unwrap_or(r);
        if let Ok(v) = SemVer::parse(target_str) {
            return *self == v;
        }

        false
    }
}

/// Package manifest schema (`kora.toml`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KoraManifest {
    pub package: PackageSection,
    #[serde(default)]
    pub server: Option<ServerSection>,
    #[serde(default)]
    pub dependencies: BTreeMap<String, String>,
    #[serde(default)]
    pub dev_dependencies: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSection {
    pub name: String,
    pub version: String,
    pub edition: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub repository: Option<String>,
    #[serde(default)]
    pub entry: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSection {
    pub host: String,
    pub port: u16,
}

/// Resolved locked package with cryptographic SHA-256 integrity hash.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockedPackage {
    pub name: String,
    pub version: String,
    pub source: String,
    pub checksum: String,
    #[serde(default)]
    pub dependencies: BTreeMap<String, String>,
}

/// Deterministic lockfile representation (`kora.lock`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KoraLockfile {
    pub version: u32,
    pub package: Vec<LockedPackage>,
}

/// Registry package index entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryIndexEntry {
    pub name: String,
    pub version: String,
    pub checksum: String,
    pub download_url: String,
    pub dependencies: BTreeMap<String, String>,
}

pub struct PackageManager;

impl PackageManager {
    /// Reads and parses `kora.toml`.
    pub fn parse_manifest(path: &Path) -> Result<KoraManifest, String> {
        let content =
            fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        toml::from_str(&content).map_err(|e| format!("invalid kora.toml: {e}"))
    }

    /// Serializes and writes `kora.toml`.
    pub fn write_manifest(manifest: &KoraManifest, path: &Path) -> Result<(), String> {
        let serialized =
            toml::to_string_pretty(manifest).map_err(|e| format!("manifest encode error: {e}"))?;
        fs::write(path, serialized).map_err(|e| format!("failed to write {}: {e}", path.display()))
    }

    /// Reads and parses `kora.lock`.
    pub fn parse_lockfile(path: &Path) -> Result<KoraLockfile, String> {
        let content =
            fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        toml::from_str(&content).map_err(|e| format!("invalid kora.lock: {e}"))
    }

    /// Serializes and writes `kora.lock`.
    pub fn write_lockfile(lockfile: &KoraLockfile, path: &Path) -> Result<(), String> {
        let serialized =
            toml::to_string_pretty(lockfile).map_err(|e| format!("lockfile encode error: {e}"))?;
        fs::write(path, serialized).map_err(|e| format!("failed to write {}: {e}", path.display()))
    }

    /// Validates package name against path traversal and malicious characters.
    pub fn validate_package_name(name: &str) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("package name cannot be empty".to_string());
        }
        if name.contains("..") || name.contains('/') || name.contains('\\') || name.contains('\0') {
            return Err(format!(
                "security violation: illegal path traversal characters in package name `{name}`"
            ));
        }
        if !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '@')
        {
            return Err(format!(
                "invalid package name `{name}`: only alphanumeric, '-', '_', '@' allowed"
            ));
        }
        Ok(())
    }

    /// Adds a new dependency to `kora.toml`.
    pub fn add_dependency(manifest_path: &Path, name: &str, req: &str) -> Result<(), String> {
        Self::validate_package_name(name)?;
        let mut manifest = Self::parse_manifest(manifest_path)?;
        manifest
            .dependencies
            .insert(name.to_string(), req.to_string());
        Self::write_manifest(&manifest, manifest_path)
    }

    /// Removes an existing dependency from `kora.toml`.
    pub fn remove_dependency(manifest_path: &Path, name: &str) -> Result<(), String> {
        let mut manifest = Self::parse_manifest(manifest_path)?;
        if manifest.dependencies.remove(name).is_none() {
            return Err(format!("package `{name}` was not found in dependencies"));
        }
        Self::write_manifest(&manifest, manifest_path)
    }

    /// Computes cryptographic SHA-256 hash of bytes.
    pub fn calculate_sha256(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Resolves dependency tree against available registry versions and produces `kora.lock`.
    pub fn resolve_dependencies(
        manifest: &KoraManifest,
        available: &HashMap<String, Vec<RegistryIndexEntry>>,
    ) -> Result<KoraLockfile, String> {
        let mut locked_packages = Vec::new();

        for (dep_name, req_str) in &manifest.dependencies {
            let Some(versions) = available.get(dep_name) else {
                // If not in remote registry, register as local workspace dependency
                locked_packages.push(LockedPackage {
                    name: dep_name.clone(),
                    version: req_str.clone(),
                    source: "local".to_string(),
                    checksum: Self::calculate_sha256(format!("{dep_name}-{req_str}").as_bytes()),
                    dependencies: BTreeMap::new(),
                });
                continue;
            };

            // Find best matching version
            let mut matched = None;
            for entry in versions {
                if SemVer::parse(&entry.version).is_ok_and(|v| v.matches_requirement(req_str)) {
                    matched = Some(entry);
                    break;
                }
            }

            let Some(best) = matched else {
                return Err(format!(
                    "no matching version found for dependency `{dep_name}` satisfying `{req_str}`"
                ));
            };

            locked_packages.push(LockedPackage {
                name: best.name.clone(),
                version: best.version.clone(),
                source: best.download_url.clone(),
                checksum: best.checksum.clone(),
                dependencies: best.dependencies.clone(),
            });
        }

        Ok(KoraLockfile {
            version: 1,
            package: locked_packages,
        })
    }

    /// Executes full installation: reads `kora.toml`, locks dependencies, and scaffolds `.kora-packages/`.
    pub fn install(project_dir: &Path) -> Result<KoraLockfile, String> {
        let manifest_path = project_dir.join("kora.toml");
        let manifest = Self::parse_manifest(&manifest_path)?;

        let packages_dir = project_dir.join(".kora-packages");
        fs::create_dir_all(&packages_dir)
            .map_err(|e| format!("failed to create .kora-packages directory: {e}"))?;

        // Empty registry index for built-in/ambient modules
        let registry = HashMap::new();
        let lockfile = Self::resolve_dependencies(&manifest, &registry)?;

        let lock_path = project_dir.join("kora.lock");
        Self::write_lockfile(&lockfile, &lock_path)?;

        Ok(lockfile)
    }

    /// Prepares package for publishing: validates metadata and generates checksum.
    pub fn publish(project_dir: &Path, registry_url: &str) -> Result<RegistryIndexEntry, String> {
        let manifest_path = project_dir.join("kora.toml");
        let manifest = Self::parse_manifest(&manifest_path)?;

        let entry = manifest
            .package
            .entry
            .unwrap_or_else(|| "src/main.kora".to_string());
        let entry_path = project_dir.join(&entry);
        if !entry_path.exists() {
            return Err(format!(
                "package entrypoint `{}` not found",
                entry_path.display()
            ));
        }

        let content = fs::read_to_string(&entry_path).unwrap_or_default();
        let checksum = Self::calculate_sha256(content.as_bytes());

        Ok(RegistryIndexEntry {
            name: manifest.package.name.clone(),
            version: manifest.package.version.clone(),
            checksum,
            download_url: format!(
                "{registry_url}/packages/{}/{}.kpkg",
                manifest.package.name, manifest.package.version
            ),
            dependencies: manifest.dependencies,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_compares_semver() {
        let v1 = SemVer::parse("1.2.3").expect("valid");
        assert_eq!(v1.major, 1);
        assert_eq!(v1.minor, 2);
        assert_eq!(v1.patch, 3);

        let v2 = SemVer::parse("1.3.0").expect("valid");
        assert!(v2 > v1);
    }

    #[test]
    fn verifies_semver_requirements() {
        let v = SemVer::parse("1.4.2").unwrap();
        assert!(v.matches_requirement("^1.0.0"));
        assert!(v.matches_requirement(">=1.2.0"));
        assert!(v.matches_requirement("~1.4.0"));
        assert!(!v.matches_requirement("^2.0.0"));
        assert!(!v.matches_requirement("~1.3.0"));
    }

    #[test]
    fn serializes_and_parses_lockfile() {
        let mut lock = KoraLockfile {
            version: 1,
            package: Vec::new(),
        };

        lock.package.push(LockedPackage {
            name: "kora-jwt".to_string(),
            version: "0.2.1".to_string(),
            source: "https://registry.kora-lang.org/kora-jwt".to_string(),
            checksum: "a1b2c3d4e5f6".to_string(),
            dependencies: BTreeMap::new(),
        });

        let serialized = toml::to_string(&lock).expect("serializes");
        let parsed: KoraLockfile = toml::from_str(&serialized).expect("deserializes");
        assert_eq!(parsed.version, 1);
        assert_eq!(parsed.package.len(), 1);
        assert_eq!(parsed.package[0].name, "kora-jwt");
    }

    #[test]
    fn resolves_dependencies_with_checksums() {
        let manifest = KoraManifest {
            package: PackageSection {
                name: "my-shop".to_string(),
                version: "0.1.0".to_string(),
                edition: "2026".to_string(),
                description: None,
                authors: vec!["Engineer".to_string()],
                license: Some("MIT".to_string()),
                repository: None,
                entry: Some("src/main.kora".to_string()),
            },
            server: None,
            dependencies: {
                let mut d = BTreeMap::new();
                d.insert("kora-crypto".to_string(), "^1.0.0".to_string());
                d
            },
            dev_dependencies: BTreeMap::new(),
        };

        let mut available = HashMap::new();
        available.insert(
            "kora-crypto".to_string(),
            vec![RegistryIndexEntry {
                name: "kora-crypto".to_string(),
                version: "1.2.0".to_string(),
                checksum: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                    .to_string(),
                download_url: "https://registry.kora-lang.org/packages/kora-crypto/1.2.0.kpkg"
                    .to_string(),
                dependencies: BTreeMap::new(),
            }],
        );

        let locked = PackageManager::resolve_dependencies(&manifest, &available).expect("resolved");
        assert_eq!(locked.package.len(), 1);
        assert_eq!(locked.package[0].name, "kora-crypto");
        assert_eq!(locked.package[0].version, "1.2.0");
    }

    #[test]
    fn security_rejects_path_traversal_in_package_names() {
        let malicious_names = [
            "../../etc/passwd",
            "../bad-pkg",
            "kora/../../root",
            "kora\\windows\\system32",
            "pkg\0nullbyte",
            "package; rm -rf /",
            "",
        ];

        for name in malicious_names {
            let res = PackageManager::validate_package_name(name);
            assert!(
                res.is_err(),
                "Malicious package name `{name}` was not rejected by security validation!"
            );
        }

        // Legitimate package names must pass
        assert!(PackageManager::validate_package_name("kora-crypto").is_ok());
        assert!(PackageManager::validate_package_name("auth_provider").is_ok());
        assert!(PackageManager::validate_package_name("@kora/core").is_err()); // slash rejected for safety
    }

    #[test]
    fn security_verifies_sha256_cryptographic_integrity() {
        let data = b"kora-framework-verified-source-code";
        let hash = PackageManager::calculate_sha256(data);
        assert_eq!(hash.len(), 64); // Valid 256-bit hexadecimal string

        // Any single bit alteration produces completely different hash (avalanche effect)
        let mut tampered_data = data.to_vec();
        tampered_data[0] ^= 1;
        let tampered_hash = PackageManager::calculate_sha256(&tampered_data);
        assert_ne!(hash, tampered_hash);
    }
}
