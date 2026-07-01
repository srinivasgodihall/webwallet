use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct SecretBytes {
    bytes: Vec<u8>,
}

impl SecretBytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl fmt::Debug for SecretBytes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SecretBytes(<redacted>)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_bytes_reports_length() {
        let secret = SecretBytes::new(vec![1, 2, 3, 4]);

        assert_eq!(secret.len(), 4);
        assert!(!secret.is_empty());
    }

    #[test]
    fn secret_bytes_debug_output_is_redacted() {
        let secret = SecretBytes::new(vec![1, 2, 3, 4]);
        let debug_output = format!("{secret:?}");

        assert_eq!(debug_output, "SecretBytes(<redacted>)");
        assert!(!debug_output.contains("1"));
        assert!(!debug_output.contains("2"));
        assert!(!debug_output.contains("3"));
        assert!(!debug_output.contains("4"));
        assert!(debug_output.contains("redacted"));
    }

    #[test]
    fn secret_bytes_can_expose_bytes_for_internal_use() {
        let secret = SecretBytes::new(vec![1, 2, 3, 4]);

        assert_eq!(secret.as_bytes(), &[1, 2, 3, 4]);
    }
}
