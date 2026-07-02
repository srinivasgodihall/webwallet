use std::fmt;

use crate::address::Address;
use crate::secret::SecretBytes;

#[derive(Clone, PartialEq, Eq)]
pub struct SolanaSecretKey {
    secret: SecretBytes,
}

impl SolanaSecretKey {
    pub fn new(secret: SecretBytes) -> Self {
        Self { secret }
    }

    pub fn len(&self) -> usize {
        self.secret.len()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.secret.as_bytes()
    }
}

impl fmt::Debug for SolanaSecretKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SolanaSecretKey(<redacted>)")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolanaPublicAddress {
    address: Address,
}

impl SolanaPublicAddress {
    pub fn new(address: Address) -> Self {
        Self { address }
    }

    pub fn as_str(&self) -> &str {
        self.address.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solana_secret_key_reports_length() {
        let secret = SolanaSecretKey::new(SecretBytes::new(vec![1, 2, 3, 4]));

        assert_eq!(secret.len(), 4);
    }

    #[test]
    fn solana_secret_key_debug_output_is_redacted() {
        let secret = SolanaSecretKey::new(SecretBytes::new(vec![1, 2, 3, 4]));
        let debug_output = format!("{secret:?}");

        assert_eq!(debug_output, "SolanaSecretKey(<redacted>)");
        assert!(!debug_output.contains("1"));
        assert!(!debug_output.contains("2"));
        assert!(!debug_output.contains("3"));
        assert!(!debug_output.contains("4"));
        assert!(debug_output.contains("redacted"));
    }

    #[test]
    fn solana_secret_key_exposes_bytes_for_internal_use() {
        let secret = SolanaSecretKey::new(SecretBytes::new(vec![1, 2, 3, 4]));

        assert_eq!(secret.as_bytes(), &[1, 2, 3, 4]);
    }

    #[test]
    fn solana_public_address_exposes_string() {
        let address =
            SolanaPublicAddress::new(Address::new("11111111111111111111111111111111".to_string()));

        assert_eq!(address.as_str(), "11111111111111111111111111111111");
    }

    #[test]
    fn solana_public_address_debug_output_is_allowed() {
        let address =
            SolanaPublicAddress::new(Address::new("11111111111111111111111111111111".to_string()));

        let debug_output = format!("{address:?}");

        assert!(debug_output.contains("11111111111111111111111111111111"));
    }
}
