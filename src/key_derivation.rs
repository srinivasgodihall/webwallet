use crate::encryption_key::EncryptionKey;
use crate::wallet_password::WalletPassword;
use argon2::Argon2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyDerivationError {
    Failed,
}

impl KeyDerivationError {
    pub fn message(self) -> &'static str {
        match self {
            KeyDerivationError::Failed => "Could not derive encryption key",
        }
    }
}

pub fn derive_encryption_key(
    password: &WalletPassword,
    salt: &[u8],
) -> Result<EncryptionKey, KeyDerivationError> {
    let mut key_bytes = [0u8; 32];

    Argon2::default()
        .hash_password_into(
            password.expose_password_bytes_for_key_derivation_only(),
            salt,
            &mut key_bytes,
        )
        .map_err(|_| KeyDerivationError::Failed)?;

    EncryptionKey::new(key_bytes.to_vec()).map_err(|_| KeyDerivationError::Failed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_derivation_error_has_clear_message() {
        assert_eq!(
            KeyDerivationError::Failed.message(),
            "Could not derive encryption key"
        );
    }

    #[test]
    fn derive_encryption_key_returns_32_byte_key() {
        let password = WalletPassword::new("strong password".to_string())
            .expect("password should be valid");

        let key = derive_encryption_key(&password, b"test-salt-123456")
            .expect("key should derive");

        assert_eq!(key.expose_key_bytes_for_encryption_only().len(), 32);
    }

    #[test]
    fn same_password_and_salt_derives_same_key() {
        let password = WalletPassword::new("strong password".to_string())
            .expect("password should be valid");

        let first = derive_encryption_key(&password, b"test-salt-123456")
            .expect("first key should derive");

        let second = derive_encryption_key(&password, b"test-salt-123456")
            .expect("second key should derive");

        assert_eq!(
            first.expose_key_bytes_for_encryption_only(),
            second.expose_key_bytes_for_encryption_only()
        );
    }

    #[test]
    fn different_salt_derives_different_key() {
        let password = WalletPassword::new("strong password".to_string())
            .expect("password should be valid");

        let first = derive_encryption_key(&password, b"test-salt-123456")
            .expect("first key should derive");

        let second = derive_encryption_key(&password, b"different-salt-1")
            .expect("second key should derive");

        assert_ne!(
            first.expose_key_bytes_for_encryption_only(),
            second.expose_key_bytes_for_encryption_only()
        );
    }
}

