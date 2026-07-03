use crate::solana_balance::SolanaBalance;
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

pub fn parse_get_balance_response(response: &Value) -> Option<SolanaBalance> {
    let lamports = response.get("result")?.get("value")?.as_u64()?;

    Some(SolanaBalance::new(lamports))
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

    #[test]
    fn parses_get_balance_response_json() {
        let response = json!({
            "jsonrpc": "2.0",
            "result": {
                "context": { "slot": 1 },
                "value": 5_000_000_000u64
            },
            "id": 1

        });

        let balance =
            parse_get_balance_response(&response).expect("response should include balance");

        assert_eq!(balance.lamports(), 5_000_000_000);
        assert_eq!(balance.sol(), 5.0);
    }

    #[test]
    fn returns_none_when_balance_value_is_missing() {
        let response = json!({
            "jsonrpc": "2.0",
            "result": {
                "context": { "slot": 1 }
            },
            "id": 1
        });

        assert_eq!(parse_get_balance_response(&response), None);
    }

    #[test]
    fn returns_none_when_response_contains_error() {
        let response = json!({
            "jsonrpc": "2.0",
            "error": {
                "code": -32602,
                "message": "Invalid params"
            },
            "id": 1
        });

        assert_eq!(parse_get_balance_response(&response), None);
    }
}
