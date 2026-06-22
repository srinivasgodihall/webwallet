use crate::address::Address;
use crate::chain::Chain;
use crate::network::Network;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    network: Network,
    address: Address,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountError {
    BlankAddress,
}
impl Account {
    pub fn chain(&self) -> Chain {
        self.network.chain()
    }

    pub fn new(network: Network, address: Address) -> Result<Self, AccountError> {
        if address.is_blank() {
            Err(AccountError::BlankAddress)
        } else {
            Ok(Self { network, address })
        }
    }

    pub fn network(&self) -> Network {
        self.network
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}

impl AccountError {
    pub fn message(self) -> &'static str {
        match self {
            AccountError::BlankAddress => "Address cannot be blank",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_exposes_public_network_and_address() {
        let account = Account::new(Network::EthereumSepolia, Address::new("0x1234".to_string()))
            .expect("test address should be valid");

        assert_eq!(account.network(), Network::EthereumSepolia);
        assert_eq!(account.address().as_str(), "0x1234");
    }

    #[test]
    fn account_chain_comes_from_network() {
        let account = Account::new(Network::EthereumSepolia, Address::new("0x1234".to_string()))
            .expect("test address should be valid");

        assert_eq!(account.chain(), Chain::Ethereum);
    }

    #[test]
    fn account_debug_output_does_not_include_secret_field_names() {
        let account = Account::new(Network::EthereumSepolia, Address::new("0x1234".to_string()))
            .expect("test address should be valid");

        let debug_output = format!("{account:?}");

        assert!(!debug_output.contains("private_key"));
        assert!(!debug_output.contains("mnemonic"));
        assert!(!debug_output.contains("password"));
        assert!(!debug_output.contains("secret"));
    }

    #[test]
    fn new_account_accepts_non_blank_address() {
        let account = Account::new(Network::EthereumSepolia, Address::new("0x1234".to_string()));

        assert!(account.is_ok());
    }

    #[test]
    fn new_account_rejects_empty_address() {
        let account = Account::new(Network::EthereumSepolia, Address::new("".to_string()));

        assert_eq!(account, Err(AccountError::BlankAddress));
    }

    #[test]
    fn new_account_rejects_whitespace_only_address() {
        let account = Account::new(Network::EthereumSepolia, Address::new("   ".to_string()));

        assert_eq!(account, Err(AccountError::BlankAddress));
    }

    #[test]
    fn blank_address_error_has_clear_message() {
        assert_eq!(
            AccountError::BlankAddress.message(),
            "Address cannot be blank"
        );
    }
}
