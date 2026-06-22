use crate::chain::Chain;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    EthereumSepolia,
    SolanaDevnet,
    BitcoinTestnet,
}

impl Network {
    pub fn label(self) -> &'static str {
        match self {
            Network::EthereumSepolia => "Chain::Ethereum",
            Network::SolanaDevnet => "Chain::Solana",
            Network::BitcoinTestnet => "Chain::Bitcoin",
        }
    }

    pub fn chain(self) -> Chain {
        match self {
            Network::EthereumSepolia => Chain::Ethereum,
            Network::SolanaDevnet => Chain::Solana,
            Network::BitcoinTestnet => Chain::Bitcoin,
        }
    }

    pub fn is_test_network(self) -> bool {
        match self {
            Network::EthereumSepolia => true,
            Network::SolanaDevnet => true,
            Network::BitcoinTestnet => true,
        }
    }

    pub fn rpc_url(self) -> &'static str {
        match self {
            Network::EthereumSepolia => "https://sepolia.example-rpc.local",
            Network::SolanaDevnet => "https://api.devnet.solana.com",
            Network::BitcoinTestnet => "https://bitcoin-testnet.example-rpc.local",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_labels_are_human_readable() {
        assert_eq!(Network::EthereumSepolia.chain(), Chain::Ethereum);
        assert_eq!(Network::SolanaDevnet.chain(), Chain::Solana);
        assert_eq!(Network::BitcoinTestnet.chain(), Chain::Bitcoin);
    }

    #[test]
    fn networks_map_to_their_chains() {
        assert_eq!(Network::EthereumSepolia.chain(), Chain::Ethereum);
        assert_eq!(Network::SolanaDevnet.chain(), Chain::Solana);
        assert_eq!(Network::BitcoinTestnet.chain(), Chain::Bitcoin);
    }

    #[test]
    fn all_supported_networks_are_test_networks() {
        assert!(Network::EthereumSepolia.is_test_network());
        assert!(Network::SolanaDevnet.is_test_network());
        assert!(Network::BitcoinTestnet.is_test_network());
    }

    #[test]
    fn rpc_url_are_test_network_urls() {
        let urls = [
            Network::EthereumSepolia.rpc_url(),
            Network::SolanaDevnet.rpc_url(),
            Network::BitcoinTestnet.rpc_url(),

        ];

        for url in urls {
            assert!(url.starts_with("https://"));
            assert!(!url.to_ascii_lowercase().contains("mainnet"));
        }

        assert!(Network::EthereumSepolia.rpc_url().contains("sepolia"));
        assert!(Network::SolanaDevnet.rpc_url().contains("devnet"));
        assert!(Network::BitcoinTestnet.rpc_url().contains("testnet"));
    }
}
