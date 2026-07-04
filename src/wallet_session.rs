use crate::solana_wallet::{GeneratedSolanaWallet, SolanaPublicAddress};
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct NameWallet{
    name:String,
    wallet: GeneratedSolanaWallet,
}

impl NameWallet {
    pub fn new(name: String, wallet: GeneratedSolanaWallet) -> Self {
        Self {name, wallet}
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn public_address(&self) -> &SolanaPublicAddress {
        self.wallet.public_address()
    }
}

impl fmt::Debug for NameWallet {
    fn fmt (&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NameWallet")
            .field("name", &self.name)
            .field("public_address", self.wallet.public_address())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solana_wallet::generate_solana_wallet;

    #[test]
    fn named_wallet_exposed_name() {
        let wallet = NameWallet::new("Main Wallet".to_string(), generate_solana_wallet());
        assert_eq!(wallet.name(), "Main Wallet");
     }

     #[test]
    fn named_wallet_exposes_public_address_for_display() {
        let wallet = NameWallet::new("Main wallet".to_string(), generate_solana_wallet());

        assert!(!wallet.public_address().as_str().is_empty());
    }

    #[test]
    fn named_wallet_debug_output_does_not_expose_secret_data() {
        let wallet = NameWallet::new("Main Wallet".to_string(), generate_solana_wallet());
        let debug_output = format!("{wallet:?}");

        assert!(debug_output.contains("NameWallet"));
        assert!(debug_output.contains("Main Wallet"));
        assert!(debug_output.contains("public_address"));
        assert!(!debug_output.contains("secret"));
        assert!(!debug_output.contains("private"));
        assert!(!debug_output.contains("mnemonic"));
        assert!(!debug_output.contains("password"));
    }

}