//! Package & Plugin Manager for Kora.
//! Parses kora.toml, manages dependencies and locks versions into kora.lock.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KoraManifest {
    pub package: PackageSection,
    #[serde(default)]
    pub server: Option<ServerSection>,
    #[serde(default)]
    pub dependencies: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSection {
    pub name: String,
    pub version: String,
    pub edition: String,
    #[serde(default)]
    pub entry: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSection {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KoraLockfile {
    pub packages: BTreeMap<String, String>,
}

pub struct PackageManager;

impl PackageManager {
    pub fn parse_manifest(path: &Path) -> Result<KoraManifest, String> {
        let content =
            fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        toml::from_str(&content).map_err(|e| format!("invalid kora.toml: {e}"))
    }

    pub fn generate_lockfile(manifest: &KoraManifest, path: &Path) -> Result<(), String> {
        let mut lock = KoraLockfile::default();
        for (dep, ver) in &manifest.dependencies {
            lock.packages.insert(dep.clone(), ver.clone());
        }
        let serialized =
            toml::to_string_pretty(&lock).map_err(|e| format!("lockfile error: {e}"))?;
        fs::write(path, serialized).map_err(|e| format!("failed to write {}: {e}", path.display()))
    }
}
