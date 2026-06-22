use crate::network::Network;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Chain {
    Ethereum,
    Solana,
    Bitcoin,
}

impl Chain {
    pub fn label(self) -> &'static str {
        match self {
            Chain::Ethereum => "Ethereum",
            Chain::Solana => "Solana",
            Chain::Bitcoin => "Bitcoin",
        }
    }

    pub fn default_network(self) -> Network {
        match self {
            Chain::Ethereum => Network::EthereumSepolia,
            Chain::Solana => Network::SolanaDevnet,
            Chain::Bitcoin => Network::BitcoinTestnet,
            
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn chain_labels_are_human_readable() {
        assert_eq!(Chain::Ethereum.label(), "Ethereum");
        assert_eq!(Chain::Solana.label(), "Solana");
        assert_eq!(Chain::Bitcoin.label(), "Bitcoin");
    }

    #[test]
    fn chian_default_to_networks() {
        assert_eq!(Chain::Ethereum.default_network(), Network::EthereumSepolia);
        assert_eq!(Chain::Solana.default_network(), Network::SolanaDevnet);
        assert_eq!(Chain::Bitcoin.default_network(), Network::BitcoinTestnet);
    }
}
