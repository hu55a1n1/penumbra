//! The Penumbra shielded graph [`Component`] and [`ActionHandler`] implementations.

mod action_handler;
mod assets;
mod fmd;
mod ics20_withdrawal_with_handler;
mod metrics;
mod note_manager;
mod shielded_graph;
mod transfer;

pub use self::metrics::register_metrics;
pub use assets::{AssetRegistry, AssetRegistryRead};
pub use fmd::ClueManager;
pub use ics20_withdrawal_with_handler::Ics20WithdrawalWithHandler;
pub use note_manager::NoteManager;
pub use shielded_graph::{ShieldedGraph, StateReadExt, StateWriteExt};
pub use transfer::Ics20Transfer;

pub mod rpc;
