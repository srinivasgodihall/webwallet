use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionKeyError {
    InvalidLength,
}

#[derive(Clone, PartialEq, Eq)]
pub struct EncryptionKey {
    bytes: Vec<u8>,
}

impl EncryptionKey {
    pub fn new(bytes: Vec<u8>) -> Result<Self, EncryptionKeyError> {
        if bytes.len() == 32 {
            Ok(Self { bytes })
        } else {
            Err(EncryptionKeyError::InvalidLength)
        }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn expose_key_bytes_for_encryption_only(&self) -> &[u8] {
        &self.bytes
    }
}


impl EncryptionKeyError {
    pub fn message(self) -> &'static str {
        match self {
            EncryptionKeyError::InvalidLength => "Encryption key must be exactly 32 bytes",
        }
    }
}

impl fmt::Debug for EncryptionKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("EncryptionKey(<redacted>)")
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_length_error_has_clear_message() {
        assert_eq!(
            EncryptionKeyError::InvalidLength.message(),
            "Encryption key must be exactly 32 bytes"
        );
    }
    #[test]
    fn encryption_key_accepts_32_bytes() {
        let key = EncryptionKey::new(vec![7; 32]).expect("key should be valid");

        assert_eq!(key.len(), 32);
    }

    #[test]
    fn encryption_key_rejects_wrong_length() {
        let key = EncryptionKey::new(vec![7; 31]);

        assert_eq!(key, Err(EncryptionKeyError::InvalidLength));
    }

    #[test]
    fn encryption_key_debug_output_is_redacted() {
        let key = EncryptionKey::new(vec![7; 32]).expect("key should be valid");
        let debug_output = format!("{key:?}");

        assert_eq!(debug_output, "EncryptionKey(<redacted>)");
        assert!(!debug_output.contains("7"));
        assert!(debug_output.contains("redacted"));
    }

    #[test]
    fn encryption_key_exposes_bytes_for_encryption_only() {
        let key = EncryptionKey::new(vec![7; 32]).expect("key should be valid");

        assert_eq!(key.expose_key_bytes_for_encryption_only(), &[7; 32]);
    }
}
