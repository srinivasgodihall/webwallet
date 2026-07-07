use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptedWalletError {
    BlankCiphertext,
}

impl EncryptedWalletError {
    pub fn message(self) -> &'static str {
        match self {
            EncryptedWalletError::BlankCiphertext => "Encrypted wallet data cannot be blank",
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct EncryptedWalletData {
    ciphertext: String,
}

impl EncryptedWalletData {
    pub fn new(ciphertext: String) -> Result<Self, EncryptedWalletError> {
        if ciphertext.trim().is_empty() {
            Err(EncryptedWalletError::BlankCiphertext)
        } else {
            Ok(Self { ciphertext })
        }
    }

    pub fn ciphertext(&self) -> &str {
        &self.ciphertext
    }
}

impl fmt::Debug for EncryptedWalletData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("EncryptedWalletData(<encrypted>)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_ciphertext_error_has_clear_message() {
        assert_eq!(
            EncryptedWalletError::BlankCiphertext.message(),
            "Encrypted wallet data cannot be blank"
        );
    }

    #[test]
    fn encrypted_wallet_data_accepts_ciphertext() {
        let data = EncryptedWalletData::new("encrypted-data".to_string())
            .expect("ciphertext should be valid");

        assert_eq!(data.ciphertext(), "encrypted-data");
    }

    #[test]
    fn encrypted_wallet_data_rejects_blank_ciphertext() {
        let data = EncryptedWalletData::new("".to_string());

        assert_eq!(data, Err(EncryptedWalletError::BlankCiphertext));
    }

    #[test]
    fn encrypted_wallet_data_rejects_whitespace_ciphertext() {
        let data = EncryptedWalletData::new("   ".to_string());

        assert_eq!(data, Err(EncryptedWalletError::BlankCiphertext));
    }

    #[test]
    fn encrypted_wallet_debug_output_does_not_include_ciphertext_or_secret_words() {
        let data = EncryptedWalletData::new("actual-encrypted-wallet-blob".to_string())
            .expect("ciphertext should be valid");

        let debug_output = format!("{data:?}");

        assert!(debug_output.contains("EncryptedWalletData"));
        assert!(debug_output.contains("encrypted"));
        assert!(!debug_output.contains("actual-encrypted-wallet-blob"));
        assert!(!debug_output.contains("secret"));
        assert!(!debug_output.contains("private"));
        assert!(!debug_output.contains("mnemonic"));
        assert!(!debug_output.contains("password"));
    }
}