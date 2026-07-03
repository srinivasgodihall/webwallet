#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct SolanaBalance {
    lamports: u64,
}

impl SolanaBalance {
    pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

    pub fn new(lamports: u64) -> Self {
        Self { lamports }
    }

    pub fn lamports(&self) -> u64 {
        self.lamports
    }

    pub fn sol(&self) -> f64 {
        self.lamports as f64 / Self::LAMPORTS_PER_SOL as f64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stores_lamports() {
        let balance = SolanaBalance::new(500);

        assert_eq!(balance.lamports(), 500);
    }

    #[test]
    fn converts_lamports_to_sol() {
        let balance = SolanaBalance::new(1_500_000_000);

        assert_eq!(balance.sol(), 1.5);
    }

    #[test]
    fn zero_lamports_is_zero_sol() {
        let balance = SolanaBalance::new(0);

        assert_eq!(balance.sol(), 0.0);
    }
}
