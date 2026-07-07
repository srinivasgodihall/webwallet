use crate::solana_wallet::SolanaPublicAddress;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolanaTransfer {
    recipient: SolanaPublicAddress,
    lamports: u64,
}

impl SolanaTransfer {
    pub fn new(recipient: SolanaPublicAddress, lamports: u64) -> Self {
        Self {
            recipient,
            lamports,
        }
    }

    pub fn recipient(&self) -> &SolanaPublicAddress {
        &self.recipient
    }

    pub fn lamports(&self) -> u64 {
        self.lamports
    }

    pub fn validate(&self) -> Result<(), SolanaTransferError> {
        if self.recipient.as_str().trim().is_empty() {
            return Err(SolanaTransferError::BlankRecipient);
        }

        if self.lamports == 0 {
            return Err(SolanaTransferError::ZeroLamports);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolanaTransferError {
    BlankRecipient,
    ZeroLamports,
}

impl SolanaTransferError {
    pub fn message(self) -> &'static str {
        match self {
            SolanaTransferError::BlankRecipient => "Recipient address cannot be blank",
            SolanaTransferError::ZeroLamports => "Transfer amount must be greater than zero",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::Address;

    fn test_recipient() -> SolanaPublicAddress {
        SolanaPublicAddress::new(Address::new("11111111111111111111111111111111".to_string()))
    }

    #[test]
    fn solana_transfer_exposes_recipient() {
        let recipient = test_recipient();
        let transfer = SolanaTransfer::new(recipient.clone(), 1_000_000);

        assert_eq!(transfer.recipient(), &recipient);
    }

    #[test]
    fn solana_transfer_exposes_lamports() {
        let transfer = SolanaTransfer::new(test_recipient(), 1_000_000);

        assert_eq!(transfer.lamports(), 1_000_000);
    }

    #[test]
    fn solana_transfer_debug_output_does_not_include_secret_words() {
        let transfer = SolanaTransfer::new(test_recipient(), 1_000_000);
        let debug_output = format!("{transfer:?}");

        assert!(debug_output.contains("SolanaTransfer"));
        assert!(debug_output.contains("recipient"));
        assert!(debug_output.contains("lamports"));
        assert!(!debug_output.contains("secret"));
        assert!(!debug_output.contains("private"));
        assert!(!debug_output.contains("mnemonic"));
        assert!(!debug_output.contains("password"));
    }

    #[test]
    fn transfer_error_messages_are_clear() {
        assert_eq!(
            SolanaTransferError::BlankRecipient.message(),
            "Recipient address cannot be blank"
        );

        assert_eq!(
            SolanaTransferError::ZeroLamports.message(),
            "Transfer amount must be greater than zero"
        );
    }

    #[test]
    fn valid_transfer_passes_validation() {
        let transfer = SolanaTransfer::new(test_recipient(), 1_000_000);

        assert_eq!(transfer.validate(), Ok(()));
    }

    #[test]
    fn transfer_rejects_blank_recipient() {
        let recipient = SolanaPublicAddress::new(Address::new("".to_string()));
        let transfer = SolanaTransfer::new(recipient, 1_000_000);

        assert_eq!(
            transfer.validate(),
            Err(SolanaTransferError::BlankRecipient)
        );
    }

    #[test]
    fn transfer_rejects_zero_lamports() {
        let transfer = SolanaTransfer::new(test_recipient(), 0);

        assert_eq!(transfer.validate(), Err(SolanaTransferError::ZeroLamports));
    }
}
