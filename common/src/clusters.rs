use wallet_standard_base::Cluster;

/// Configures information for the Solana Network cluster based on the traits
/// from [wallet-standard-base](https://crates.io/crates/wallet-standard-base)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub enum SolanaNetworkCluster {
    /// Solana Mainnet cluster,  [https://api.mainnet-beta.solana.com](https://api.mainnet-beta.solana.com)
    MainNet,
    /// Solana Testnet cluster, e.g. [https://api.testnet.solana.com](https://api.testnet.solana.com)
    TestNet,
    /// Solana Devnet cluster, e.g. [https://api.devnet.solana.com](https://api.devnet.solana.com)
    #[default]
    DevNet,
    /// Solana Localnet cluster, e.g. [http://localhost:8899](http://localhost:8899)
    LocalNet,
}

impl Cluster for SolanaNetworkCluster {
    /// The namespace of the network, like "solana"
    fn network_namespace(&self) -> &str {
        "solana"
    }

    fn chains(&self) -> [&str; 4] {
        [
            Self::MainNet.chain(),
            Self::TestNet.chain(),
            Self::DevNet.chain(),
            Self::LocalNet.chain(),
        ]
    }

    fn identifier(&self) -> &str {
        match self {
            Self::MainNet => "mainnet",
            Self::TestNet => "testnet",
            Self::DevNet => "devnet",
            Self::LocalNet => "localnet",
        }
    }

    fn chain(&self) -> &str {
        match self {
            Self::MainNet => "solana:mainnet",
            Self::TestNet => "solana:testnet",
            Self::DevNet => "solana:devnet",
            Self::LocalNet => "solana:localnet",
        }
    }

    fn endpoint(&self) -> &str {
        match self {
            Self::MainNet => "https://api.mainnet-beta.solana.com",
            Self::TestNet => "https://api.testnet.solana.com",
            Self::DevNet => "https://api.devnet.solana.com",
            Self::LocalNet => "http://localhost:8899",
        }
    }
}

impl core::fmt::Display for SolanaNetworkCluster {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.identifier())
    }
}

impl From<&str> for SolanaNetworkCluster {
    fn from(value: &str) -> Self {
        match value {
            val if val == Self::MainNet.identifier() => Self::MainNet,
            val if val == Self::MainNet.chain() => Self::MainNet,
            val if val == Self::MainNet.endpoint() => Self::MainNet,

            val if val == Self::TestNet.identifier() => Self::TestNet,
            val if val == Self::TestNet.chain() => Self::TestNet,
            val if val == Self::TestNet.endpoint() => Self::TestNet,

            val if val == Self::LocalNet.identifier() => Self::LocalNet,
            val if val == Self::LocalNet.chain() => Self::LocalNet,
            val if val == Self::LocalNet.endpoint() => Self::LocalNet,

            _ => Self::DevNet,
        }
    }
}

#[cfg(test)]
mod chain_tests {
    use super::*;

    #[test]
    fn is_valid_uri() {
        assert_eq!(
            SolanaNetworkCluster::MainNet.endpoint(),
            "https://api.mainnet-beta.solana.com"
        );
        assert_eq!(
            SolanaNetworkCluster::DevNet.endpoint(),
            "https://api.devnet.solana.com"
        );
        assert_eq!(
            SolanaNetworkCluster::TestNet.endpoint(),
            "https://api.testnet.solana.com"
        );
        assert_eq!(
            SolanaNetworkCluster::LocalNet.endpoint(),
            "http://localhost:8899"
        );
    }

    #[test]
    fn test_from() {
        assert_eq!(SolanaNetworkCluster::MainNet, "solana:mainnet".into());
        assert_eq!(SolanaNetworkCluster::DevNet, "solana:devnet".into());
        assert_eq!(SolanaNetworkCluster::TestNet, "solana:testnet".into());
        assert_eq!(SolanaNetworkCluster::LocalNet, "solana:localnet".into());
        assert!(SolanaNetworkCluster::DevNet == "solana:localnet2".into());

        assert_eq!(
            SolanaNetworkCluster::MainNet,
            "https://api.mainnet-beta.solana.com".into()
        );
        assert_eq!(
            SolanaNetworkCluster::DevNet,
            "https://api.devnet.solana.com".into()
        );
        assert_eq!(
            SolanaNetworkCluster::TestNet,
            "https://api.testnet.solana.com".into()
        );
        assert_eq!(
            SolanaNetworkCluster::LocalNet,
            "http://localhost:8899".into()
        );
        assert!(SolanaNetworkCluster::DevNet == "https://localhost:8899".into());
        assert!(SolanaNetworkCluster::DevNet == "https://cluster.foo".into());
    }

    #[test]
    fn validate_endpoint() {
        assert_eq!(
            SolanaNetworkCluster::MainNet.endpoint(),
            "https://api.mainnet-beta.solana.com"
        );
        assert_eq!(
            SolanaNetworkCluster::DevNet.endpoint(),
            "https://api.devnet.solana.com"
        );
        assert_eq!(
            SolanaNetworkCluster::TestNet.endpoint(),
            "https://api.testnet.solana.com"
        );
        assert_eq!(
            SolanaNetworkCluster::LocalNet.endpoint(),
            "http://localhost:8899"
        );
    }

    #[test]
    fn validate_chain() {
        assert_eq!(SolanaNetworkCluster::MainNet.chain(), "solana:mainnet");
        assert_eq!(SolanaNetworkCluster::DevNet.chain(), "solana:devnet");
        assert_eq!(SolanaNetworkCluster::TestNet.chain(), "solana:testnet");
        assert_eq!(SolanaNetworkCluster::LocalNet.chain(), "solana:localnet");
    }
}
