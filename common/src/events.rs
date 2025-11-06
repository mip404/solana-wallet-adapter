use wallet_standard_base::StandardFeatures;

/// Implementation of wallet-standard events
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub enum SolanaStandardEvents {
    /// Wallet-adapter "standard:connect"
    Connect,
    /// Wallet-adapter "standard:disconnect"
    #[default]
    Disconnect,
    /// Wallet-adapter "standard:on"
    On,
    /// Wallet-adapter "solana:signIn"
    SignIn,
    /// Wallet-adapter "solana:signMessage"
    SignMessage,
    /// Wallet-adapter "solana:signTransaction"
    SignTransaction,
    /// Wallet-adapter "solana:signAndSendTransaction"
    SignAndSendTransaction,
}

impl StandardFeatures for SolanaStandardEvents {
    /// Identifier for `standard:connect`
    fn connect(&self) -> &str {
        "standard:connect"
    }

    /// Identifier for `standard:disconnect`
    fn disconnect(&self) -> &str {
        "standard:disconnect"
    }

    fn events(&self) -> &str {
        "standard:events"
    }

    fn on(&self) -> &str {
        "standard:on"
    }

    fn supports_sign_in(&self) -> bool {
        self.sign_in().is_some()
    }

    /// Identifier for `standard:events`
    fn namespace(&self) -> &str {
        "standard:events"
    }

    /// Identifier for `solana:signIn`
    fn sign_in(&self) -> Option<&str> {
        Some("solana:signIn")
    }

    /// Identifier for `solana:signMessage`
    fn sign_message(&self) -> &str {
        "solana:signMessage"
    }

    /// Identifier for `solana:signTransaction`
    fn sign_transaction(&self) -> &str {
        "solana:signTransaction"
    }

    /// Identifier for `solana:signAndSendTransaction`
    fn sign_and_send_transaction(&self) -> &str {
        "solana:signAndSendTransaction"
    }
}
