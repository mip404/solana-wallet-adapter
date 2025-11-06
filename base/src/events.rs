#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum WindowEvent {
    /// Standard App Ready Wallet Event Identifier
    AppReady,
    /// Standard Register Wallet Event Identifier
    Register,
}

impl WindowEvent {
    pub fn event_identifier(&self) -> &str {
        match self {
            Self::AppReady => "wallet-standard:app-ready",
            Self::Register => "wallet-standard:register-wallet",
        }
    }
}

pub trait StandardFeatures {
    fn namespace(&self) -> &str;

    fn connect(&self) -> &str {
        "standard:connect"
    }

    fn disconnect(&self) -> &str {
        "standard:disconnect"
    }

    fn events(&self) -> &str {
        "standard:events"
    }

    fn on(&self) -> &str {
        "standard:on"
    }

    fn sign_in(&self) -> &str;

    fn sign_message(&self) -> &str;

    fn sign_transaction(&self) -> &str;

    fn sign_and_send_transaction(&self) -> &str;
}
