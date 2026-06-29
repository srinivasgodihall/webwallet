use crate::address::Address;
use crate::chain::Chain;
use crate::network::Network;

pub trait ChainAdapter {
    fn chain(&self) -> Chain;
    fn default_network(&self) -> Network;
    fn is_valid_address(&self, address: &Address) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EthereumAdapter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolanaAdapter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitcoinAdapter;

impl ChainAdapter for EthereumAdapter {
    fn chain(&self) -> Chain {
        Chain::Ethereum
    }

    fn default_network(&self) -> Network {
        Chain::Ethereum.default_network()
    }

    fn is_valid_address(&self, address: &Address) -> bool {
        !address.is_blank()
    }
}

impl ChainAdapter for SolanaAdapter {
    fn chain(&self) -> Chain {
        Chain::Solana
    }

    fn default_network(&self) -> Network {
        Chain::Solana.default_network()
    }
    fn is_valid_address(&self, address: &Address) -> bool {
        !address.is_blank()
    }
}

impl ChainAdapter for BitcoinAdapter {
    fn chain(&self) -> Chain {
        Chain::Bitcoin
    }

    fn default_network(&self) -> Network {
        Chain::Bitcoin.default_network()
    }

    fn is_valid_address(&self, address: &Address) -> bool {
        !address.is_blank()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ethereum_adapter_returns_ethereum_metadata() {
        let adapter = EthereumAdapter;

        assert_eq!(adapter.chain(), Chain::Ethereum);
        assert_eq!(adapter.default_network(), Network::EthereumSepolia);
    }

    #[test]
    fn solana_adapter_returns_solana_metadata() {
        let adapter = SolanaAdapter;

        assert_eq!(adapter.chain(), Chain::Solana);
        assert_eq!(adapter.default_network(), Network::SolanaDevnet);
    }

    #[test]
    fn bitcoin_adapter_returns_bitcoin_metadata() {
        let adapter = BitcoinAdapter;

        assert_eq!(adapter.chain(), Chain::Bitcoin);
        assert_eq!(adapter.default_network(), Network::BitcoinTestnet);
    }

    fn assert_basic_address_validation<A: ChainAdapter>(adapter: A) {
        let valid = Address::new("test-address".to_string());
        let empty = Address::new("".to_string());
        let whitespace = Address::new("   ".to_string());

        assert!(adapter.is_valid_address(&valid));
        assert!(!adapter.is_valid_address(&empty));
        assert!(!adapter.is_valid_address(&whitespace));
    }

    #[test]
    fn ethereum_adapter_rejects_blank_addresses() {
        assert_basic_address_validation(EthereumAdapter);
    }

    #[test]
    fn solana_adapter_rejects_blank_addresses() {
        assert_basic_address_validation(SolanaAdapter);
    }

    #[test]
    fn bitcoin_adapter_rejects_blank_addresses() {
        assert_basic_address_validation(BitcoinAdapter);
    }
}
