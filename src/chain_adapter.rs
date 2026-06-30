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
        let value = address.as_str();

        if value.len() != 42 {
            return false;
        }

        if !value.starts_with("0x") {
            return false;
        }

        value[2..]
            .chars()
            .all(|character| character.is_ascii_hexdigit())
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
        let value = address.as_str();

        if value.len() < 32 || value.len() > 44 {
            return false;
        }

        value.chars().all(|character| {
            character.is_ascii_alphanumeric()
                && character != '0'
                && character != 'O'
                && character != 'I'
                && character != 'l'
        })
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
        let value = address.as_str();

        if value.len() < 26 || value.len() > 90 {
            return false;
        }

        value.starts_with('m')
            || value.starts_with('n')
            || value.starts_with('2')
            || value.starts_with("tb1")
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

    // #[test]
    // fn ethereum_adapter_rejects_blank_addresses() {
    //     assert_basic_address_validation(EthereumAdapter);
    // }

    #[test]
    fn ethereum_adapter_rejects_blank_address() {
        let adapter = EthereumAdapter;
        let address = Address::new("".to_string());

        assert!(!adapter.is_valid_address(&address));
    }

    #[test]
    fn ethereum_adapter_rejects_whitespace_address() {
        let adapter = EthereumAdapter;
        let address = Address::new("   ".to_string());

        assert!(!adapter.is_valid_address(&address));
    }

    // #[test]
    // fn solana_adapter_rejects_blank_addresses() {
    //     assert_basic_address_validation(SolanaAdapter);
    // }

    #[test]
    fn solana_adapter_accepts_base58_like_address() {
        let adapter = SolanaAdapter;
        let address = Address::new("11111111111111111111111111111111".to_string());

        assert!(adapter.is_valid_address(&address));
    }

    #[test]
    fn solana_adapter_rejects_short_address() {
        let adapter = SolanaAdapter;
        let address = Address::new("abc123".to_string());

        assert!(!adapter.is_valid_address(&address));
    }

    #[test]
    fn solana_adapter_rejects_long_address() {
        let adapter = SolanaAdapter;
        let address = Address::new("111111111111111111111111111111111111111111111".to_string());

        assert!(!adapter.is_valid_address(&address));
    }

    #[test]
    fn solana_adapter_rejects_invalid_base58_characters() {
        let adapter = SolanaAdapter;

        let contains_zero = Address::new("11111111111111111111111111111110".to_string());
        let contains_upper_o = Address::new("1111111111111111111111111111111O".to_string());
        let contains_upper_i = Address::new("1111111111111111111111111111111I".to_string());
        let contains_lower_l = Address::new("1111111111111111111111111111111l".to_string());
        let contains_punctuation = Address::new("1111111111111111111111111111111!".to_string());

        assert!(!adapter.is_valid_address(&contains_zero));
        assert!(!adapter.is_valid_address(&contains_upper_o));
        assert!(!adapter.is_valid_address(&contains_upper_i));
        assert!(!adapter.is_valid_address(&contains_lower_l));
        assert!(!adapter.is_valid_address(&contains_punctuation));
    }

    // #[test]
    // fn bitcoin_adapter_rejects_blank_addresses() {
    //     assert_basic_address_validation(BitcoinAdapter);
    // }

    #[test]
    fn bitcoin_adapter_accepts_testnet_legacy_p2pkh_address() {
        let adapter = BitcoinAdapter;
        let address = Address::new("mipcBbFg9gMiCh81Kj8tqqdgoZub1ZJRfn".to_string());

        assert!(adapter.is_valid_address(&address));
    }

    #[test]
    fn bitcoin_adapter_accepts_testnet_legacy_p2pkh_n_address() {
        let adapter = BitcoinAdapter;
        let address = Address::new("n2eMqTT929pb1RDNuqEnxdaLau1rxy3efi".to_string());

        assert!(adapter.is_valid_address(&address));
    }

    #[test]
    fn bitcoin_adapter_accepts_testnet_p2sh_address() {
        let adapter = BitcoinAdapter;
        let address = Address::new("2N2JD6wb56AfK4tfmM6PwdVmoYk2dCKf4Br".to_string());

        assert!(adapter.is_valid_address(&address));
    }

    #[test]
    fn bitcoin_adapter_accepts_testnet_bech32_address() {
        let adapter = BitcoinAdapter;
        let address = Address::new("tb1qfm6q8pewxd6wqvv3um0zj5x8drx0g3gq7x0j3a".to_string());

        assert!(adapter.is_valid_address(&address));
    }

    #[test]
    fn bitcoin_adapter_rejects_mainnet_p2pkh_address() {
        let adapter = BitcoinAdapter;
        let address = Address::new("1BoatSLRHtKNngkdXEeobR76b53LETtpyT".to_string());

        assert!(!adapter.is_valid_address(&address));
    }

    #[test]
    fn bitcoin_adapter_rejects_mainnet_p2sh_address() {
        let adapter = BitcoinAdapter;
        let address = Address::new("3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy".to_string());

        assert!(!adapter.is_valid_address(&address));
    }

    #[test]
    fn bitcoin_adapter_rejects_mainnet_bech32_address() {
        let adapter = BitcoinAdapter;
        let address = Address::new("bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kygt080".to_string());

        assert!(!adapter.is_valid_address(&address));
    }

    #[test]
    fn bitcoin_adapter_rejects_short_address() {
        let adapter = BitcoinAdapter;
        let address = Address::new("tb1short".to_string());

        assert!(!adapter.is_valid_address(&address));
    }

    #[test]
    fn ethereum_adapter_accepts_basic_hex_address() {
        let adapter = EthereumAdapter;
        let address = Address::new("0x0000000000000000000000000000000000000000".to_string());

        assert!(adapter.is_valid_address(&address));
    }

    #[test]
    fn ethereum_adapter_rejects_address_without_prefix() {
        let adapter = EthereumAdapter;
        let address = Address::new("0000000000000000000000000000000000000000".to_string());

        assert!(!adapter.is_valid_address(&address));
    }

    #[test]
    fn ethereum_adapter_rejects_wrong_length_address() {
        let adapter = EthereumAdapter;
        let address = Address::new("0x1234".to_string());

        assert!(!adapter.is_valid_address(&address));
    }

    #[test]
    fn ethereum_adapter_rejects_non_hex_address() {
        let adapter = EthereumAdapter;
        let address = Address::new("0x000000000000000000000000000000000000000g".to_string());

        assert!(!adapter.is_valid_address(&address));
    }
}
