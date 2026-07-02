use ed25519_dalek::SigningKey;
use rand_core::OsRng;
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

#[derive(Clone, PartialEq, Eq)]
pub struct GeneratedSolanaWallet {
    public_address: SolanaPublicAddress,
    secret_key: SolanaSecretKey,
}

impl GeneratedSolanaWallet {
    pub fn public_address(&self) -> &SolanaPublicAddress {
        &self.public_address
    }

    pub fn secret_key(&self) -> &SolanaSecretKey {
        &self.secret_key
    }
}

impl fmt::Debug for GeneratedSolanaWallet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GeneratedSolanaWallet")
            .field("public_address", &self.public_address)
            .field("secret_key", &"<redacted>")
            .finish()
    }
}
pub fn generate_solana_wallet() -> GeneratedSolanaWallet {
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();

    let public_address = bs58::encode(verifying_key.as_bytes()).into_string();
    let secret_key = signing_key.to_bytes().to_vec();

    GeneratedSolanaWallet {
        public_address: SolanaPublicAddress::new(Address::new(public_address)),
        secret_key: SolanaSecretKey::new(SecretBytes::new(secret_key)),
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

    #[test]
    fn generated_solana_wallet_has_public_address() {
        let wallet = generate_solana_wallet();

        assert!(!wallet.public_address().as_str().is_empty());
    }

    #[test]
    fn generated_solana_wallet_has_32_byte_secret_key() {
        let wallet = generate_solana_wallet();

        assert_eq!(wallet.secret_key().len(), 32);
    }

    #[test]
    fn generated_solana_wallet_public_address_is_valid_for_solana_adapter() {
        use crate::chain_adapter::{ChainAdapter, SolanaAdapter};

        let wallet = generate_solana_wallet();
        let adapter = SolanaAdapter;
        let address = Address::new(wallet.public_address().as_str().to_string());

        assert!(adapter.is_valid_address(&address));
    }

    #[test]
    fn generated_solana_wallet_debug_output_redacts_secret() {
        let wallet = generate_solana_wallet();
        let debug_output = format!("{wallet:?}");

        assert!(debug_output.contains("GeneratedSolanaWallet"));
        assert!(debug_output.contains("public_address"));
        assert!(debug_output.contains("redacted"));
        assert!(!debug_output.contains("secret_key: SolanaSecretKey"));
    }
}
