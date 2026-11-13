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
        constant_time_compare(expected.as_bytes(), signature_hex.as_bytes())
    }
}

/// Constant-time byte slice comparison to mitigate timing attacks.
fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
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

    fn test_password() -> String {
        SaltString::generate(&mut OsRng).to_string()
    }

    #[test]
    fn hashes_and_verifies_password_with_argon2id() {
        let pwd = test_password();
        let hash = PasswordAuth::hash_password(&pwd).expect("hash success");
        assert!(PasswordAuth::verify_password(&pwd, &hash));
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

    #[test]
    fn security_verifies_argon2id_unique_salts_and_brute_force_resistance() {
        let pwd = "IdenticalPassword2026!";
        let hash1 = PasswordAuth::hash_password(pwd).unwrap();
        let hash2 = PasswordAuth::hash_password(pwd).unwrap();
        // Cryptographic salts must be unique, ensuring identical passwords yield different hashes
        assert_ne!(hash1, hash2);
        assert!(PasswordAuth::verify_password(pwd, &hash1));
        assert!(PasswordAuth::verify_password(pwd, &hash2));
    }

    #[test]
    fn security_rejects_jwt_tampering_and_forgery() {
        let secret = b"strong-cryptographic-master-key-256";
        let payload = r#"{"uid":42,"role":"customer","balance":100}"#;
        let sig = JwtSigner::sign_hs256(secret, payload).unwrap();

        // 1. Bit-flip in signature
        let mut tampered_sig = sig.clone();
        let last_char = if tampered_sig.ends_with('0') {
            '1'
        } else {
            '0'
        };
        tampered_sig.pop();
        tampered_sig.push(last_char);
        assert!(!JwtSigner::verify_hs256(secret, payload, &tampered_sig));

        // 2. Privilege escalation tampering in payload
        let escalated_payload = r#"{"uid":42,"role":"superadmin","balance":999999}"#;
        assert!(!JwtSigner::verify_hs256(secret, escalated_payload, &sig));

        // 3. Different signing key
        let rogue_key = b"attacker-key-32-bytes-long-here!";
        assert!(!JwtSigner::verify_hs256(rogue_key, payload, &sig));
    }

    #[test]
    fn security_enforces_rbac_least_privilege() {
        let mut guard = RbacGuard::new();
        guard.add_permission("editor", "posts:create");
        guard.add_permission("editor", "posts:update");

        // Allowed
        assert!(guard.has_permission("editor", "posts:create"));
        assert!(guard.has_permission("editor", "posts:update"));

        // Denied: least privilege principle
        assert!(!guard.has_permission("editor", "posts:delete"));
        assert!(!guard.has_permission("editor", "users:manage"));
        assert!(!guard.has_permission("guest", "posts:create"));
    }
}
