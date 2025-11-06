#![forbid(unsafe_code)]
#![forbid(missing_docs)]

//! Network configuration for the Solana Network based on the `wallet-standard`.
//!
mod clusters;
pub use clusters::*;

mod events;
pub use events::*;
