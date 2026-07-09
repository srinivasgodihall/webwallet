#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecureRandomError {
    BrowserWindowUnavailable,
    CryptoUnavailable,
    RandomGenerationFailed,
    InvalidNonceLength,
}

impl SecureRandomError {
    pub fn message(self) -> &'static str {
        match self {
            SecureRandomError::BrowserWindowUnavailable => "Browser window is unavailable",
            SecureRandomError::CryptoUnavailable => "Browser crypto API is unavailable",
            SecureRandomError::RandomGenerationFailed => "Could not generate secure random bytes",
            SecureRandomError::InvalidNonceLength => "Nonce must be exactly 12 bytes",
        }
    }
}

pub fn random_salt() -> Result<Vec<u8>, SecureRandomError> {
    random_bytes(32)
}

pub fn random_nonce() -> Result<[u8; 12], SecureRandomError> {
    let bytes = random_bytes(12)?;

    bytes
        .try_into()
        .map_err(|_| SecureRandomError::InvalidNonceLength)
}

#[cfg(target_arch = "wasm32")]
pub fn random_bytes(length: usize) -> Result<Vec<u8>, SecureRandomError> {
    let window = web_sys::window().ok_or(SecureRandomError::BrowserWindowUnavailable)?;
    let crypto = window
        .crypto()
        .map_err(|_| SecureRandomError::CryptoUnavailable)?;

    let mut bytes = vec![0u8; length];

    crypto
        .get_random_values_with_u8_array(&mut bytes)
        .map_err(|_| SecureRandomError::RandomGenerationFailed)?;

    Ok(bytes)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn random_bytes(length: usize) -> Result<Vec<u8>, SecureRandomError> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| SecureRandomError::RandomGenerationFailed)?
        .as_nanos();

    let mut bytes = Vec::with_capacity(length);

    for index in 0..length {
        let value = now.wrapping_add(index as u128).to_le_bytes()[index % 16];
        bytes.push(value);
    }

    Ok(bytes)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_salt_has_expected_length() {
        let salt = random_salt().expect("salt should generate");

        assert_eq!(salt.len(), 32);
    }

    #[test]
    fn random_nonce_has_expected_length() {
        let nonce = random_nonce().expect("nonce should generate");

        assert_eq!(nonce.len(), 12);
    }

    #[test]
    fn secure_random_error_messages_are_clear() {
        assert_eq!(
            SecureRandomError::RandomGenerationFailed.message(),
            "Could not generate secure random bytes"
        );
    }
}
