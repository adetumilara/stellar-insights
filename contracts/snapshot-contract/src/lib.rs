#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Bytes, Env, Map
    
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Snapshot {
    pub hash: Bytes,       // usually 32 bytes — SHA-256 digest
    pub epoch: u64,
    pub timestamp: u64,
}

#[contracttype]
pub enum DataKey {
    Snapshots,
}

#[contract]
pub struct SnapshotContract;

#[contractimpl]
impl SnapshotContract {

    /// Submit a snapshot hash for an epoch (already implemented)
    pub fn submit_snapshot(env: Env, hash: Bytes, epoch: u64) -> u64 {
        let timestamp = env.ledger().timestamp();

        let snapshot = Snapshot {
            hash: hash.clone(),
            epoch,
            timestamp,
        };

        let mut snapshots: Map<u64, Snapshot> = env
            .storage()
            .persistent()
            .get(&DataKey::Snapshots)
            .unwrap_or_else(|| Map::new(&env));

        // Optional: prevent overwriting existing snapshots
        if snapshots.contains_key(epoch) {
            panic!("Snapshot for epoch {} already exists", epoch);
        }

        snapshots.set(epoch, snapshot);

        env.storage()
            .persistent()
            .set(&DataKey::Snapshots, &snapshots);

        env.events()
            .publish(
                (symbol_short!("SNAP_SUB"),),
                (hash, epoch, timestamp)
            );

        timestamp
    }

    /// Get snapshot data for a specific epoch
    pub fn get_snapshot(env: Env, epoch: u64) -> Bytes {
        let snapshots: Map<u64, Snapshot> = env
            .storage()
            .persistent()
            .get(&DataKey::Snapshots)
            .unwrap_or_else(|| Map::new(&env));

        match snapshots.get(epoch) {
            Some(snapshot) => snapshot.hash,           // return raw hash bytes
            None => panic!("No snapshot found for epoch {}", epoch),
        }
    }

    /// Return the most recent snapshot (by epoch number)
    /// Returns: (hash, epoch, timestamp)
    pub fn latest_snapshot(env: Env) -> (Bytes, u64, u64) {
        let snapshots: Map<u64, Snapshot> = env
            .storage()
            .persistent()
            .get(&DataKey::Snapshots)
            .unwrap_or_else(|| Map::new(&env));

        if snapshots.is_empty() {
            panic!("No snapshots exist");
        }

        let mut max_epoch: u64 = 0;
        let mut latest: Option<Snapshot> = None;

        // We have to iterate — Soroban maps don't have max_by_key yet
        for (epoch, snap) in snapshots.iter() {
            if epoch > max_epoch {
                max_epoch = epoch;
                latest = Some(snap);
            }
        }

        let snap = latest.unwrap(); // safe because we checked !is_empty()

        (snap.hash, snap.epoch, snap.timestamp)
    }

    /// Verify whether a provided hash matches the stored snapshot for that epoch
    pub fn verify_snapshot(env: Env, hash: Bytes) -> bool {
        // We assume the client knows which epoch → hash pair to verify.
        // If you want epoch-specific verification, change signature to:
        // verify_snapshot(env: Env, epoch: u64, hash: Bytes) -> bool

        let snapshots: Map<u64, Snapshot> = env
            .storage()
            .persistent()
            .get(&DataKey::Snapshots)
            .unwrap_or_else(|| Map::new(&env));

        // Naive version: check if this exact hash exists anywhere
        // (works if hashes are unique — common when including merkle root + epoch)
        for (_, snap) in snapshots.iter() {
            if snap.hash == hash {
                return true;
            }
        }

        false

        // Alternative (recommended) — require epoch:
        // match snapshots.get(epoch) {
        //     Some(snap) => snap.hash == hash,
        //     None => false,
        // }
    }
}

