#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Address(String);

impl Address {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_blank(&self) -> bool {
        self.0.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_stores_string_value() {
        let address = Address::new("0x1234".to_string());
        assert_eq!(address.as_str(), "0x1234");
    }

    #[test]
    fn non_empty_address_is_not_blank() {
        let address = Address::new("0x1234".to_string());

        assert!(!address.is_blank());
    }

    #[test]
    fn empty_address_is_blank() {
        let address = Address::new("".to_string());
        assert!(address.is_blank());
    }

    #[test]
    fn whitespace_only_address_is_blank() {
        let address = Address::new("   ".to_string());
        assert!(address.is_blank());
    }
}
