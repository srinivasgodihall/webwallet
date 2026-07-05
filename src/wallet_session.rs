use crate::solana_wallet::{GeneratedSolanaWallet, SolanaPublicAddress};
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct NamedWallet{
    name:String,
    wallet: GeneratedSolanaWallet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletSession {
    wallet: Vec<NamedWallet>,
    selected_wallet_index: Option<usize>,
}


impl NamedWallet {
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

impl WalletSession {
    pub fn new_empty()-> Self {
        Self {
            wallet: Vec::new(),
            selected_wallet_index: None,
        }
    }

    pub fn wallet_count(&self) -> usize {
        self.wallet.len()
    }

    pub fn selected_wallet_index(&self) -> Option<usize> {
        self.selected_wallet_index
    }

    pub fn add_wallet(&mut self, wallet: NamedWallet) {
        self.wallet.push(wallet);

        if self.selected_wallet_index.is_none() {
            self.selected_wallet_index = Some(0);
        }

    }

    pub fn select_wallet(&mut self, index: usize) -> bool {
        if index < self.wallet.len() {
            self.selected_wallet_index = Some(index);
            true
        }else{
            false
        }
    }

    pub fn selected_wallet(&self) -> Option<&NamedWallet> {
        self.selected_wallet_index
            .and_then(|index| self.wallet.get(index))
    }
}

impl fmt::Debug for NamedWallet {
    fn fmt (&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NamedWallet")
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
        let wallet = NamedWallet::new("Main Wallet".to_string(), generate_solana_wallet());
        assert_eq!(wallet.name(), "Main Wallet");
     }

     #[test]
    fn named_wallet_exposes_public_address_for_display() {
        let wallet = NamedWallet::new("Main wallet".to_string(), generate_solana_wallet());

        assert!(!wallet.public_address().as_str().is_empty());
    }

    #[test]
    fn named_wallet_debug_output_does_not_expose_secret_data() {
        let wallet = NamedWallet::new("Main Wallet".to_string(), generate_solana_wallet());
        let debug_output = format!("{wallet:?}");

        assert!(debug_output.contains("NamedWallet"));
        assert!(debug_output.contains("Main Wallet"));
        assert!(debug_output.contains("public_address"));
        assert!(!debug_output.contains("secret"));
        assert!(!debug_output.contains("private"));
        assert!(!debug_output.contains("mnemonic"));
        assert!(!debug_output.contains("password"));
    }

    #[test]
    fn new_empty_session_has_no_wallets_and_no_selection() {
        let session = WalletSession::new_empty();

        assert_eq!(session.wallet_count(), 0);
        assert_eq!(session.selected_wallet_index(), None);
    }

    #[test]
    fn adding_first_wallet_selects_it() {
        let mut session = WalletSession::new_empty();
        let wallet = NamedWallet::new("Main Wallet".to_string(), generate_solana_wallet());

        session.add_wallet(wallet);

        assert_eq!(session.wallet_count(), 1);
        assert_eq!(session.selected_wallet_index(), Some(0));
    }

    #[test]
    fn adding_second_wallet_keeps_existing_selection() {
        let mut session = WalletSession::new_empty();

        session.add_wallet(NamedWallet::new(
            "Main Wallet".to_string(),
            generate_solana_wallet(),
        ));

        session.add_wallet(NamedWallet::new(
            "Savings Wallet".to_string(),
            generate_solana_wallet(),
        ));

        assert_eq!(session.wallet_count(), 2);
        assert_eq!(session.selected_wallet_index(), Some(0));
    }

    #[test]
    fn selecting_existing_wallet_changes_selection() {
        let mut session = WalletSession::new_empty();

        session.add_wallet(NamedWallet::new(
            "Main Wallet".to_string(),
            generate_solana_wallet(),
        ));

        session.add_wallet(NamedWallet::new(
            "Savings Wallet".to_string(),
            generate_solana_wallet(),
        ));

        let selected = session.select_wallet(1);

        assert!(selected);
        assert_eq!(session.selected_wallet_index(), Some(1));
    }

    #[test]
    fn selecting_invalid_wallet_keeps_existing_selection() {
        let mut session = WalletSession::new_empty();

        session.add_wallet(NamedWallet::new(
            "Main Wallet".to_string(),
            generate_solana_wallet(),
        ));

        let selected = session.select_wallet(99);

        assert!(!selected);
        assert_eq!(session.selected_wallet_index(), Some(0));
    }

    #[test]
    fn selected_wallet_returns_current_wallet() {
        let mut session = WalletSession::new_empty();

        session.add_wallet(NamedWallet::new(
            "Main Wallet".to_string(),
            generate_solana_wallet(),
        ));

        session.add_wallet(NamedWallet::new(
            "Savings Wallet".to_string(),
            generate_solana_wallet(),
        ));

        session.select_wallet(1);

        let selected_wallet  = session
            .selected_wallet()
            .expect("wallet should be selected");

        assert_eq!(selected_wallet.name(), "Savings Wallet");        

    }
    #[test]
    fn selected_wallet_returns_none_for_empty_session() {
        let session = WalletSession::new_empty();

        assert!(session.selected_wallet().is_none());
    }


}