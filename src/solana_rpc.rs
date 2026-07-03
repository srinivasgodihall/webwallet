use crate::solana_wallet::SolanaPublicAddress;
use serde_json::{Value, json};

pub fn build_get_balance_request(address: &SolanaPublicAddress) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBalance",
        "params": [address.as_str()],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::Address;

    #[test]
    fn builds_get_balance_request_json() {
        let address =
            SolanaPublicAddress::new(Address::new("11111111111111111111111111111111".to_string()));

        let request = build_get_balance_request(&address);

        assert_eq!(request["jsonrpc"], "2.0");
        assert_eq!(request["id"], 1);
        assert_eq!(request["method"], "getBalance");
        assert_eq!(request["params"][0], address.as_str());
    }
}
