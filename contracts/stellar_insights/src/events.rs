use soroban_sdk::{contracttype, symbol_short, BytesN, Env, Symbol};

/// Event emitted when an analytics snapshot is successfully submitted
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnalyticsSnapshotSubmitted {
    /// Epoch identifier for this snapshot
    pub epoch: u64,
    /// SHA-256 hash of the analytics snapshot
    pub hash: BytesN<32>,
    /// Ledger timestamp when the snapshot was recorded
    pub timestamp: u64,
}

/// Topics for contract events
pub const SNAPSHOT_SUBMITTED: Symbol = symbol_short!("SNAP_SUB");

impl AnalyticsSnapshotSubmitted {
    /// Publish this event to the blockchain
    pub fn publish(env: &Env, epoch: u64, hash: BytesN<32>, timestamp: u64) {
        let event = AnalyticsSnapshotSubmitted {
            epoch,
            hash: hash.clone(),
            timestamp,
        };
        
        env.events().publish((SNAPSHOT_SUBMITTED,), event);
    }
}
