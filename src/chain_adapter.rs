use crate::chain::Chain;
use crate::network::Network;

pub trait ChainAdapter {
    fn chain(&self) -> Chain;
    fn default_network(&self) -> Network;
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
}

impl ChainAdapter for SolanaAdapter {
    fn chain(&self) -> Chain {
        Chain::Solana
    }

    fn default_network(&self) -> Network {
        Chain::Solana.default_network()
    }
}

impl ChainAdapter for BitcoinAdapter {
    fn chain(&self) -> Chain {
        Chain::Bitcoin
    }

    fn default_network(&self) -> Network {
        Chain::Bitcoin.default_network()
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
}
