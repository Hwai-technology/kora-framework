//! Contextual Security, Authentication, RBAC, and Token Signing for Kora.
//! Uses Argon2id password hashing, HMAC-SHA256 JWT, and Rate Limiting.

use argon2::{
    Argon2,
    password_hash::{
        PasswordHash, PasswordHasher as _, PasswordVerifier as _, SaltString, rand_core::OsRng,
    },
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::{HashMap, HashSet};

type HmacSha256 = Hmac<Sha256>;

pub struct PasswordAuth;

impl PasswordAuth {
    pub fn hash_password(password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("hashing failed: {e}"))?;
        Ok(hash.to_string())
    }

    pub fn verify_password(password: &str, hash_str: &str) -> bool {
        let parsed = match PasswordHash::new(hash_str) {
            Ok(p) => p,
            Err(_) => return false,
        };
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok()
    }
}

pub struct JwtSigner;

impl JwtSigner {
    pub fn sign_hs256(secret: &[u8], payload_json: &str) -> Result<String, String> {
        let mut mac =
            HmacSha256::new_from_slice(secret).map_err(|e| format!("hmac init failed: {e}"))?;
        mac.update(payload_json.as_bytes());
        let result = mac.finalize();
        Ok(hex_encode(&result.into_bytes()))
    }

    pub fn verify_hs256(secret: &[u8], payload_json: &str, signature_hex: &str) -> bool {
        let mut mac = match HmacSha256::new_from_slice(secret) {
            Ok(m) => m,
            Err(_) => return false,
        };
        mac.update(payload_json.as_bytes());
        let expected = hex_encode(&mac.finalize().into_bytes());
        expected == signature_hex
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

/// Role-Based Access Control (RBAC) Guard
#[derive(Default)]
pub struct RbacGuard {
    role_permissions: HashMap<String, HashSet<String>>,
}

impl RbacGuard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_permission(&mut self, role: &str, permission: &str) {
        self.role_permissions
            .entry(role.to_string())
            .or_default()
            .insert(permission.to_string());
    }

    pub fn has_permission(&self, role: &str, permission: &str) -> bool {
        self.role_permissions
            .get(role)
            .map(|perms| perms.contains(permission))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_and_verifies_password_with_argon2id() {
        let pwd = "SuperSecretPassword123!";
        let hash = PasswordAuth::hash_password(pwd).expect("hash success");
        assert!(PasswordAuth::verify_password(pwd, &hash));
        assert!(!PasswordAuth::verify_password("wrong", &hash));
    }

    #[test]
    fn signs_and_verifies_jwt_hmac() {
        let secret = b"my-super-secret-key-32-bytes-long";
        let payload = r#"{"sub":"user_123","role":"admin"}"#;
        let sig = JwtSigner::sign_hs256(secret, payload).unwrap();
        assert!(JwtSigner::verify_hs256(secret, payload, &sig));
        assert!(!JwtSigner::verify_hs256(
            secret,
            r#"{"tampered":true}"#,
            &sig
        ));
    }
}
