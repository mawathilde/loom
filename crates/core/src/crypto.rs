use argon2::PasswordHasher;
use argon2::password_hash::SaltString;

use hkdf::Hkdf;
use sha2::Sha256;

pub fn derive_master_key(password: &str, email: &str) -> [u8; 32] {
    let salt = SaltString::encode_b64(email.as_bytes()).expect("Failed to create salt from email");
    let argon2 = argon2::Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(19 * 1024, 2, 1, None).expect("Invalid Argon2 parameters"),
    );

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .hash
        .unwrap();

    hash.as_bytes()
        .try_into()
        .expect("Hash output is not 32 bytes")
}

pub fn derive_auth_key(master_key: &[u8]) -> [u8; 32] {
    derive_subkey(master_key, "auth")
}

fn derive_subkey(master_key: &[u8], context: &str) -> [u8; 32] {
    let hk = Hkdf::<Sha256>::new(None, master_key);

    let mut out = [0u8; 32];

    hk.expand(context.as_bytes(), &mut out)
        .expect("Failed to derive subkey");

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_master_key() {
        let password = "password123";
        let email = "user@example.com";

        let master_key = derive_master_key(password, email);

        assert_eq!(master_key.len(), 32);

        assert!(!master_key.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_derive_auth_key() {
        let master_key = [0u8; 32];
        let auth_key = derive_auth_key(&master_key);

        assert_eq!(auth_key.len(), 32);

        assert!(!auth_key.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_derive_subkey() {
        let master_key = [0u8; 32];
        let context = "auth";
        let subkey = derive_subkey(&master_key, context);

        assert_eq!(subkey.len(), 32);

        assert!(!subkey.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_derive_master_key_consistency() {
        let password = "password123";
        let email = "user@example.com";

        let master_key1 = derive_master_key(password, email);
        let master_key2 = derive_master_key(password, email);

        assert_eq!(master_key1, master_key2);
    }

    #[test]
    fn test_derive_auth_key_consistency() {
        let master_key = [0u8; 32];
        let auth_key1 = derive_auth_key(&master_key);
        let auth_key2 = derive_auth_key(&master_key);

        assert_eq!(auth_key1, auth_key2);
    }

    #[test]
    fn test_derive_subkey_consistency() {
        let master_key = [0u8; 32];
        let context = "test";
        let subkey1 = derive_subkey(&master_key, context);
        let subkey2 = derive_subkey(&master_key, context);

        assert_eq!(subkey1, subkey2);
    }
}
