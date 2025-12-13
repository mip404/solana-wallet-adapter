use wallet_standard_base::Commitment;

/// A Solana network commitment
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub enum SolanaNetworkCommitment {
    /// The cluster has processed the transaction
    Processed,
    /// The transaction has been confirmed by the cluster
    Confirmed,
    /// The transaction has been finalized
    #[default]
    Finalized,
}

impl Commitment for SolanaNetworkCommitment {
    fn processed(&self) -> Self {
        Self::Processed
    }

    fn confirmed(&self) -> Self {
        Self::Confirmed
    }

    fn finalized(&self) -> Self {
        Self::Finalized
    }

    fn into(commitment_str: &str) -> Self {
        match commitment_str {
            val if val.as_bytes() == Self::Processed.as_str().as_bytes() => Self::Processed,
            val if val.as_bytes() == Self::Confirmed.as_str().as_bytes() => Self::Confirmed,
            _ => Self::Finalized,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Self::Processed => "processed",
            Self::Confirmed => "confirmed",
            Self::Finalized => "finalized",
        }
    }
}
