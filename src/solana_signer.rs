use crate::solana_wallet::GeneratedSolanaWallet;
use ed25519_dalek::{Signer, SigningKey};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SolanaSignature {
    bytes: [u8; 64],
}

impl SolanaSignature {
    pub fn new(bytes: [u8; 64]) -> Self {
        Self { bytes }
    }

    pub fn as_bytes(&self) -> &[u8; 64] {
        &self.bytes
    }
}

impl fmt::Debug for SolanaSignature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SolanaSignature(<64 bytes>)")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolanaSigningError {
    InvalidSecretKeyLength,
}

pub fn sign_message(
    wallet: &GeneratedSolanaWallet,
    message: &[u8],
) -> Result<SolanaSignature, SolanaSigningError> {
    let secret_bytes = wallet.secret_key().expose_secret_bytes_for_signing_only();

    let secret_array: [u8; 32] = secret_bytes
        .try_into()
        .map_err(|_| SolanaSigningError::InvalidSecretKeyLength)?;

    let signing_key = SigningKey::from_bytes(&secret_array);

    let signature = signing_key.sign(message);

    Ok(SolanaSignature::new(signature.to_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solana_wallet::generate_solana_wallet;

    #[test]
    fn signing_message_returns_64_byte_signature() {
        let wallet = generate_solana_wallet();
        let signature = sign_message(&wallet, b"hello").expect("message should sign");

        assert_eq!(signature.as_bytes().len(), 64);
    }

    #[test]
    fn signature_debug_output_does_not_include_secret_words() {
        let wallet = generate_solana_wallet();
        let signature = sign_message(&wallet, b"hello").expect("message should sign");
        let debug_output = format!("{signature:?}");

        assert!(debug_output.contains("SolanaSignature"));
        assert!(!debug_output.contains("secret"));
        assert!(!debug_output.contains("private"));
        assert!(!debug_output.contains("mnemonic"));
        assert!(!debug_output.contains("password"));
    }

    #[test]
    fn signing_does_not_modify_wallet_public_address() {
        let wallet = generate_solana_wallet();
        let before = wallet.public_address().as_str().to_string();

        let _signature = sign_message(&wallet, b"hello").expect("message should sign");

        assert_eq!(wallet.public_address().as_str(), before);
    }
}
