#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum WalletStatus {
    NoWallet,
    Locked,
    Unlocked,
    Clear,
}

impl WalletStatus {
    pub fn label(self) -> &'static str {
        match self {
            WalletStatus::NoWallet => "NO wallet loaded",
            WalletStatus::Locked => "wallet locked",
            WalletStatus::Unlocked => "wallet unlocked",
            WalletStatus::Clear => "NO wallet loaded",
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_wallet_status_equals_itself() {
        assert_eq!(WalletStatus::NoWallet, WalletStatus::NoWallet);
    }

    #[test]
    fn locked_and_unlocked_are_different() {
        assert_ne!(WalletStatus::Locked, WalletStatus::Unlocked);
    }

    #[test]
    fn clear_the_wallet_status() {
        assert_eq!(WalletStatus::NoWallet, WalletStatus::NoWallet);
    }

    #[test]
    fn status_labels_are_human_readable() {
        assert_eq!(WalletStatus::NoWallet.label(), "NO wallet loaded");
        assert_eq!(WalletStatus::Locked.label(), "wallet locked");
        assert_eq!(WalletStatus::Unlocked.label(), "wallet unlocked");
        assert_eq!(WalletStatus::NoWallet.label(), "NO wallet loaded");
    }
}
