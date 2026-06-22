use crate::chain::Chain;
use crate::network::Network;
use crate::address::Address;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    network: Network,
    address: Address,
}

impl Account {
    pub fn chain(&self) -> Chain {
        self.network.chain()
    }

    pub fn new(network: Network, address: Address) -> Option<Self> {
        if address.is_blank() {
            None
        }else {
            Some(Self{network, address})
        }
    }

    pub fn network(&self) -> Network {
        self.network
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_exposes_public_network_and_address() {
        let account = Account {
            network: Network::EthereumSepolia,
            address: Address::new("0x1234".to_string()),
        };

        assert_eq!(account.network(), Network::EthereumSepolia);
        assert_eq!(account.address.as_str(), "0x1234");
    }

    #[test]
    fn account_chain_comes_from_network() {
        let account = Account {
            network: Network::EthereumSepolia,
            address: Address::new("0x1234".to_string()),
        };

        assert_eq!(account.chain(), Chain::Ethereum);
    }

    #[test]
    fn account_debug_output_does_not_include_secret_field_names() {
        let account = Account {
            network: Network::EthereumSepolia,
            address: Address::new("0x1234".to_string()),
        };

        let debug_output = format!("{account:?}");

        assert!(!debug_output.contains("private_key"));
        assert!(!debug_output.contains("mnemonic"));
        assert!(!debug_output.contains("password"));
        assert!(!debug_output.contains("secret"));
    }

    #[test]
    fn new_account_accepts_non_blank_address() {
        let account = Account::new(
             Network::EthereumSepolia,
             Address::new("0x1234".to_string()),
        );

         assert!(account.is_some());
    }

    #[test]
    fn new_account_rejects_empty_address() {
        let account = Account::new(
            Network::EthereumSepolia,
            Address::new("".to_string()),
        );

        assert!(account.is_none());

    }

    #[test]
    fn new_account_rejects_whitespace_only_address() {
        let account = Account::new(
            Network::EthereumSepolia,
            Address::new("   ".to_string()),
        );

        assert!(account.is_none());
    }
        
}


