use std::fmt;

impl fmt::Debug for WalletPassword {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("WalletPassword(<redacted>)")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletPasswordError {
    BlankPassword,
}

impl WalletPasswordError {
    pub fn message(self) -> &'static str {
        match self {
            WalletPasswordError::BlankPassword => "Password cannot be blank",
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct WalletPassword {
    password: String,
}

impl WalletPassword {
    pub fn new(password: String) -> Result<Self, WalletPasswordError> {
        if password.trim().is_empty() {
            Err(WalletPasswordError::BlankPassword)
        } else {
            Ok(Self { password })
        }
    }

    pub fn len(&self) -> usize {
        self.password.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_password_error_has_clear_message() {
        assert_eq!(
            WalletPasswordError::BlankPassword.message(),
            "Password cannot be blank"
        );
    }

    #[test]
    fn wallet_password_accepts_nonblank_password() {
        let password = WalletPassword::new("correct horse battery staple".to_string());

        assert!(password.is_ok());
    }

    #[test]
    fn wallet_password_rejects_blank_password() {
        let password = WalletPassword::new("".to_string());

        assert_eq!(password, Err(WalletPasswordError::BlankPassword));
    }

    #[test]
    fn wallet_password_rejects_whitespace_password() {
        let password = WalletPassword::new("   ".to_string());

        assert_eq!(password, Err(WalletPasswordError::BlankPassword));
    }

    #[test]
    fn wallet_password_reports_length() {
        let password = WalletPassword::new("password123".to_string())
            .expect("password should be valid");

        assert_eq!(password.len(), 11);
    }

    #[test]
    fn wallet_password_debug_output_redacts_password() {
        let password = WalletPassword::new("super-secret-password".to_string())
            .expect("password should be valid");

        let debug_output = format!("{password:?}");

        assert!(debug_output.contains("WalletPassword"));
        assert!(debug_output.contains("redacted"));
        assert!(!debug_output.contains("super-secret-password"));
        assert!(!debug_output.contains("secret"));
        assert!(!debug_output.contains("password123"));
    }
}
