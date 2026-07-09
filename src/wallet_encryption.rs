use crate::encryption_key::EncryptionKey;
use base64::{engine::general_purpose, Engine as _};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletEncryptionError {
    EncryptionFailed,
    DecryptionFailed,
}

impl WalletEncryptionError {
    pub fn message(self) -> &'static str {
        match self {
            WalletEncryptionError::EncryptionFailed => "Could not encrypt wallet data",
            WalletEncryptionError::DecryptionFailed => "Could not decrypt wallet data",
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SerializableEncryptedWallet {
    version: u8,
    nonce: String,
    ciphertext: String,
}

impl SerializableEncryptedWallet {
    pub fn new(version: u8, nonce: String, ciphertext: String) -> Self {
        Self {
            version,
            nonce,
            ciphertext,
        }
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn nonce(&self) -> &str {
        &self.nonce
    }

    pub fn ciphertext(&self) -> &str {
        &self.ciphertext
    }
}

impl fmt::Debug for SerializableEncryptedWallet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SerializableEncryptedWallet(<encrypted>)")
    }
}

pub fn encrypt_wallet_data(
    plaintext: &[u8],
    key: &EncryptionKey,
    nonce_bytes: [u8; 12],
) -> Result<SerializableEncryptedWallet, WalletEncryptionError> {
    let cipher_key = Key::from_slice(key.expose_key_bytes_for_encryption_only());
    let cipher = ChaCha20Poly1305::new(cipher_key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| WalletEncryptionError::EncryptionFailed)?;

    Ok(SerializableEncryptedWallet::new(
        1,
        general_purpose::STANDARD.encode(nonce_bytes),
        general_purpose::STANDARD.encode(ciphertext),
    ))
}

pub fn decrypt_wallet_data(
    encrypted: &SerializableEncryptedWallet,
    key: &EncryptionKey,
) -> Result<Vec<u8>, WalletEncryptionError> {
    let nonce_bytes = general_purpose::STANDARD
        .decode(encrypted.nonce())
        .map_err(|_| WalletEncryptionError::DecryptionFailed)?;

    let ciphertext = general_purpose::STANDARD
        .decode(encrypted.ciphertext())
        .map_err(|_| WalletEncryptionError::DecryptionFailed)?;

    let cipher_key = Key::from_slice(key.expose_key_bytes_for_encryption_only());
    let cipher = ChaCha20Poly1305::new(cipher_key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|_| WalletEncryptionError::DecryptionFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> EncryptionKey {
        EncryptionKey::new(vec![7u8; 32]).expect("test key should be valid")
    }

    fn wrong_key() -> EncryptionKey {
        EncryptionKey::new(vec![8u8; 32]).expect("test key should be valid")
    }

    fn test_nonce() -> [u8; 12] {
        [3u8; 12]
    }

    fn test_plaintext() -> &'static [u8] {
        b"private_key=test-secret-password mnemonic=test words"
    }

    #[test]
    fn wallet_encryption_error_messages_are_clear() {
        assert_eq!(
            WalletEncryptionError::EncryptionFailed.message(),
            "Could not encrypt wallet data"
        );

        assert_eq!(
            WalletEncryptionError::DecryptionFailed.message(),
            "Could not decrypt wallet data"
        );
    }

    #[test]
    fn encrypt_wallet_data_does_not_store_plaintext() {
        let encrypted =
            encrypt_wallet_data(test_plaintext(), &test_key(), test_nonce())
                .expect("wallet data should encrypt");

        assert_ne!(encrypted.ciphertext(), "private_key=test-secret-password");
        assert!(!encrypted.ciphertext().contains("test-secret-password"));
        assert!(!encrypted.ciphertext().contains("private_key"));
        assert!(!encrypted.ciphertext().contains("mnemonic"));
        assert!(!encrypted.ciphertext().contains("password"));
    }

    #[test]
    fn decrypt_wallet_data_returns_original_plaintext() {
        let encrypted =
            encrypt_wallet_data(test_plaintext(), &test_key(), test_nonce())
                .expect("wallet data should encrypt");

        let decrypted =
            decrypt_wallet_data(&encrypted, &test_key()).expect("wallet data should decrypt");

        assert_eq!(decrypted, test_plaintext());
    }

    #[test]
    fn encrypted_wallet_debug_does_not_expose_plaintext() {
        let encrypted =
            encrypt_wallet_data(test_plaintext(), &test_key(), test_nonce())
                .expect("wallet data should encrypt");

        let debug_output = format!("{encrypted:?}");

        assert!(debug_output.contains("SerializableEncryptedWallet"));
        assert!(debug_output.contains("encrypted"));
        assert!(!debug_output.contains("test-secret-password"));
        assert!(!debug_output.contains("private_key"));
        assert!(!debug_output.contains("mnemonic"));
        assert!(!debug_output.contains("password"));
    }

    #[test]
    fn encrypted_wallet_serializes_without_raw_password_or_secret_words() {
        let encrypted =
            encrypt_wallet_data(test_plaintext(), &test_key(), test_nonce())
                .expect("wallet data should encrypt");

        let serialized =
            serde_json::to_string(&encrypted).expect("encrypted wallet should serialize");

        assert!(serialized.contains("ciphertext"));
        assert!(serialized.contains("nonce"));
        assert!(!serialized.contains("test-secret-password"));
        assert!(!serialized.contains("private_key"));
        assert!(!serialized.contains("mnemonic"));
        assert!(!serialized.contains("password"));
    }

    #[test]
    fn decrypt_rejects_wrong_key() {
        let encrypted =
            encrypt_wallet_data(test_plaintext(), &test_key(), test_nonce())
                .expect("wallet data should encrypt");

        let decrypted = decrypt_wallet_data(&encrypted, &wrong_key());

        assert_eq!(decrypted, Err(WalletEncryptionError::DecryptionFailed));
    }
}