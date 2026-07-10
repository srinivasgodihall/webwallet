use crate::wallet_encryption::SerializableEncryptedWallet;
use gloo_storage::{LocalStorage, Storage};

pub const ENCRYPTED_WALLET_STORAGE_KEY: &str = "webwallet.encrypted_wallet";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletStorageError {
    SerializeFailed,
    SaveFailed,
    LoadFailed,
    DeserializeFailed,
}

impl WalletStorageError {
    pub fn message(self) -> &'static str {
        match self {
            WalletStorageError::SerializeFailed => "Could not serialize encrypted wallet",
            WalletStorageError::SaveFailed => "Could not save encrypted wallet",
            WalletStorageError::LoadFailed => "Could not load encrypted wallet",
            WalletStorageError::DeserializeFailed => "Could not deserialize encrypted wallet",
        }
    }
}

pub fn save_encrypted_wallet(
    wallet: &SerializableEncryptedWallet,
) -> Result<(), WalletStorageError> {
    let serialized =
        serde_json::to_string(wallet).map_err(|_| WalletStorageError::SerializeFailed)?;

    LocalStorage::set(ENCRYPTED_WALLET_STORAGE_KEY, serialized)
        .map_err(|_| WalletStorageError::SaveFailed)
}

pub fn load_encrypted_wallet() -> Result<Option<SerializableEncryptedWallet>, WalletStorageError> {
    let serialized = LocalStorage::get::<String>(ENCRYPTED_WALLET_STORAGE_KEY).ok();
    match serialized {
        Some(value) => {
            let wallet = serde_json::from_str(&value)
                .map_err(|_| WalletStorageError::DeserializeFailed)?;

            Ok(Some(wallet))
        }
        None => Ok(None),
    }
}

pub fn clear_encrypted_wallet() -> Result<(), WalletStorageError> {
    LocalStorage::delete(ENCRYPTED_WALLET_STORAGE_KEY);
    Ok(())
}

pub fn has_encrypted_wallet() -> bool {
    LocalStorage::get::<String>(ENCRYPTED_WALLET_STORAGE_KEY).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encryption_key::EncryptionKey;
    use crate::wallet_encryption::encrypt_wallet_data;

    fn test_encrypted_wallet() -> SerializableEncryptedWallet {
        let key = EncryptionKey::new(vec![7u8; 32]).expect("test key should be valid");
        let nonce = [3u8; 12];
        let plaintext = b"private_key=test-secret-password mnemonic=test words";

        encrypt_wallet_data(plaintext, &key, nonce).expect("wallet should encrypt")
    }

    #[test]
    fn wallet_storage_error_messages_are_clear() {
        assert_eq!(
            WalletStorageError::SerializeFailed.message(),
            "Could not serialize encrypted wallet"
        );

        assert_eq!(
            WalletStorageError::SaveFailed.message(),
            "Could not save encrypted wallet"
        );

        assert_eq!(
            WalletStorageError::LoadFailed.message(),
            "Could not load encrypted wallet"
        );

        assert_eq!(
            WalletStorageError::DeserializeFailed.message(),
            "Could not deserialize encrypted wallet"
        );
    }

    #[test]
    fn encrypted_wallet_storage_key_is_stable() {
        assert_eq!(ENCRYPTED_WALLET_STORAGE_KEY, "webwallet.encrypted_wallet");
    }

    #[test]
    fn encrypted_wallet_serializes_for_storage() {
        let wallet = test_encrypted_wallet();

        let serialized =
            serde_json::to_string(&wallet).expect("encrypted wallet should serialize");

        assert!(serialized.contains("nonce"));
        assert!(serialized.contains("ciphertext"));
    }

    #[test]
    fn encrypted_wallet_serialization_does_not_store_raw_secret_words() {
        let wallet = test_encrypted_wallet();

        let serialized =
            serde_json::to_string(&wallet).expect("encrypted wallet should serialize");

        assert!(!serialized.contains("test-secret-password"));
        assert!(!serialized.contains("private_key"));
        assert!(!serialized.contains("mnemonic"));
        assert!(!serialized.contains("password"));
    }
}