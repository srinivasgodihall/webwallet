use crate::account::Account;
use crate::address::Address;
use crate::wallet::WalletStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletModel {
    pub status: WalletStatus,
    pub accounts: Vec<Account>,
}

impl WalletModel {
    pub fn new_empty() -> Self {
        Self {
            status: WalletStatus::NoWallet,
            accounts: Vec::new(),
        }
    }

    pub fn account_count(&self) -> usize {
        self.accounts.len()
    }

    pub fn lock(&mut self) {
        self.status = WalletStatus::Locked;
    }

    pub fn unlock(&mut self) {
        self.status = WalletStatus::Unlocked;
    }

    pub fn clear(&mut self) {
        self.status = WalletStatus::NoWallet;
        self.accounts.clear();
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::Network;

    #[test]
    fn new_empty_wallet_has_no_wallet_status_and_no_account() {
        let wallet = WalletModel::new_empty();

        assert_eq!(wallet.status, WalletStatus::NoWallet);
        assert_eq!(wallet.account_count(), 0);
    }
    #[test]
    fn account_count_returns_number_of_accounts() {
        let wallet = WalletModel {
            status: WalletStatus::Locked,
            accounts: vec![
                Account::new(Network::EthereumSepolia, Address::new("0x1234".to_string()))
                    .expect("test address should be valid"),
                Account::new(
                    Network::SolanaDevnet,
                    Address::new("solana-address".to_string()),
                )
                .expect("test address should be valid"),
            ],
        };

        assert_eq!(wallet.account_count(), 2);
    }

    #[test]
    fn wallet_model_debug_output_does_not_include_secret_field_names() {
        let wallet = WalletModel::new_empty();
        let debug_output = format!("{wallet:?}");

        assert!(!debug_output.contains("private_key"));
        assert!(!debug_output.contains("mnemonic"));
        assert!(!debug_output.contains("password"));
        assert!(!debug_output.contains("secret"));
    }

    #[test]
    fn lock_changes_status_to_locked() {
        let mut wallet = WalletModel::new_empty();

        wallet.lock();

        assert_eq!(wallet.status, WalletStatus::Locked);
    }

    #[test]
    fn unlock_changes_status_to_unlocked() {
        let mut wallet = WalletModel::new_empty();

        wallet.unlock();

        assert_eq!(wallet.status, WalletStatus::Unlocked);
    }

    #[test]
    fn clear_resets_wallet_to_no_wallet() {
        let mut wallet = WalletModel::new_empty();

        wallet.clear();

        assert_eq!(wallet.status, WalletStatus::NoWallet);
    }

    #[test]
    fn add_account_stores_public_account_metadata() {
        let mut wallet = WalletModel::new_empty();

        let account = Account::new(Network::EthereumSepolia, Address::new("0x1234".to_string()))
            .expect("test address should be valid");

        wallet.add_account(account);

        assert_eq!(wallet.account_count(), 1);
        assert_eq!(wallet.accounts[0].address().as_str(), "0x1234");
        assert_eq!(wallet.accounts[0].network(), Network::EthereumSepolia);
    }
}
