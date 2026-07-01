use crate::chain::Chain;
use crate::network::Network;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    active_chain: Chain,
    active_network: Network,
}

impl AppConfig {
    pub fn new_solana_devnet() -> Self {
        Self {
            active_chain: Chain::Solana,
            active_network: Network::SolanaDevnet,
        }
    }

    pub fn active_chain(&self) -> Chain {
        self.active_chain.clone()
    }

    pub fn active_network(&self) -> Network {
        self.active_network
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solana_devnet_config_uses_solana_chain() {
        let config = AppConfig::new_solana_devnet();

        assert_eq!(config.active_chain(), Chain::Solana);
    }

    #[test]
    fn solana_devnet_config_uses_solana_devnet() {
        let config = AppConfig::new_solana_devnet();

        assert_eq!(config.active_network(), Network::SolanaDevnet);
    }

    #[test]
    fn solana_devnet_config_uses_test_network() {
        let config = AppConfig::new_solana_devnet();

        assert!(config.active_network().is_test_network());
    }
}
