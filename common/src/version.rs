use core::fmt;

use wallet_standard_base::Version;

/// The Solana wallet-adapter version supported
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct SolanaWalletAdapterVersion;

impl Version for SolanaWalletAdapterVersion {}

impl fmt::Display for SolanaWalletAdapterVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version())
    }
}
