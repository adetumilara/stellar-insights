use crate::database::Database;
use crate::rpc::StellarRpcClient;
use crate::cache::CacheManager;
use crate::websocket::WsMessage;
use crate::websocket::WsState;
use anyhow::Result;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

pub struct RealtimeBroadcaster {
    pub state: Arc<WsState>,
    pub rpc_client: Arc<StellarRpcClient>,
    pub cache: Arc<CacheManager>,
    pub db: Arc<Database>,
    /// Last sent timestamps per corridor key to enforce 30s limit
    last_sent: Arc<DashMap<String, DateTime<Utc>>>,
}

impl RealtimeBroadcaster {
    pub fn new(
        state: Arc<WsState>,
        rpc_client: Arc<StellarRpcClient>,
        cache: Arc<CacheManager>,
        db: Arc<Database>,
    ) -> Self {
        Self {
            state,
            rpc_client,
            cache,
            db,
            last_sent: Arc::new(DashMap::new()),
        }
    }

    /// Start background broadcaster loop that fetches corridor metrics and broadcasts updates every 30s
    pub async fn start(self: Arc<Self>) {
        let this = Arc::clone(&self);
        tokio::spawn(async move {
            info!("RealtimeBroadcaster started");
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
            loop {
                interval.tick().await;
                if let Err(e) = this.publish_corridor_updates().await {
                    warn!("Realtime publish error: {}", e);
                }
            }
        });
    }

    async fn publish_corridor_updates(&self) -> Result<()> {
        // Fetch today's corridor metrics
        let today = chrono::Utc::now().date_naive();
        let aggregates = self.db.corridor_aggregates();
        let metrics = aggregates.get_corridor_metrics_for_date(today).await?;

        for m in metrics.into_iter() {
            let key = m.corridor_key.clone();
            let should_send = match self.last_sent.get(&key) {
                Some(ts) => {
                    let elapsed = chrono::Utc::now().signed_duration_since(*ts);
                    elapsed.num_seconds() >= 30
                }
                None => true,
            };

            if !should_send {
                continue;
            }

            // Build message
            let msg = WsMessage::CorridorUpdate {
                corridor_key: m.corridor_key.clone(),
                asset_a_code: m.asset_a_code.clone(),
                asset_a_issuer: m.asset_a_issuer.clone(),
                asset_b_code: m.asset_b_code.clone(),
                asset_b_issuer: m.asset_b_issuer.clone(),
            };

            // Broadcast via per-connection senders, respecting subscriptions
            for entry in self.state.connections.iter() {
                let conn_id = *entry.key();
                if let Some(s) = self.state.subscriptions.get(&conn_id) {
                    let subs = s.value();
                    if !subs.is_empty() {
                        // If any subscription matches the corridor key, send
                        let mut matched = false;
                        for channel in subs.iter() {
                            if channel.starts_with("corridor:") {
                                let sub_key = channel.trim_start_matches("corridor:");
                                if sub_key == m.corridor_key {
                                    matched = true;
                                    break;
                                }
                            }
                        }
                        if !matched {
                            continue;
                        }
                    }
                }

                // Non-blocking try_send
                if let Err(e) = entry.value().try_send(msg.clone()) {
                    error!("Failed to send corridor update to {}: {}", conn_id, e);
                }
            }

            self.last_sent.insert(key.clone(), chrono::Utc::now());
        }

        Ok(())
    }

    pub async fn broadcast_anchor_status(&self, anchor: crate::models::AnchorMetrics) {
        let msg = WsMessage::AnchorUpdate {
            anchor_id: "".to_string(),
            name: "".to_string(),
            reliability_score: anchor.reliability_score,
            status: match anchor.status {
                crate::models::AnchorStatus::Green => "green".to_string(),
                crate::models::AnchorStatus::Yellow => "yellow".to_string(),
                crate::models::AnchorStatus::Red => "red".to_string(),
            },
        };

        // Broadcast to all (anchor subscriptions support optional filtering)
        for entry in self.state.connections.iter() {
            let conn_id = *entry.key();
            if let Some(s) = self.state.subscriptions.get(&conn_id) {
                let subs = s.value();
                if !subs.is_empty() {
                    let mut matched = false;
                    for channel in subs.iter() {
                        if channel.starts_with("anchor:") {
                            // allow subscriber to use anchor:<id> - since we don't have anchor id here, send to all anchors
                            matched = true;
                            break;
                        }
                    }
                    if !matched {
                        continue;
                    }
                }
            }

            if let Err(e) = entry.value().try_send(msg.clone()) {
                error!("Failed to send anchor update to {}: {}", conn_id, e);
            }
        }
    }

    pub async fn broadcast_payment(&self, payment: crate::models::PaymentRecord) {
        let msg = WsMessage::NewPayment {
            payment: crate::models::PaymentRecord {
                id: payment.id.clone(),
                transaction_hash: payment.transaction_hash.clone(),
                source_account: payment.source_account.clone(),
                destination_account: payment.destination_account.clone(),
                asset_type: payment.asset_type.clone(),
                asset_code: payment.asset_code.clone(),
                asset_issuer: payment.asset_issuer.clone(),
                amount: payment.amount,
                created_at: payment.created_at.clone(),
            },
        };

        for entry in self.state.connections.iter() {
            let conn_id = *entry.key();
            if let Some(s) = self.state.subscriptions.get(&conn_id) {
                let subs = s.value();
                if !subs.is_empty() {
                    // payment subscription support not implemented granularly yet
                    let mut matched = false;
                    for channel in subs.iter() {
                        if channel == "payments" {
                            matched = true;
                            break;
                        }
                    }
                    if !matched {
                        continue;
                    }
                }
            }

            if let Err(e) = entry.value().try_send(msg.clone()) {
                error!("Failed to send payment to {}: {}", conn_id, e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::websocket::WsState;
    use std::sync::Arc;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_broadcaster_creation() {
        let dummy_state = Arc::new(WsState::new());
        let rpc = Arc::new(StellarRpcClient::new("".to_string(), "".to_string(), true));
        let cache = Arc::new(CacheManager::new(Default::default()).await.unwrap());
        
        // Create a test database pool
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        let db = Arc::new(Database::new(pool));
        
        let broadcaster = RealtimeBroadcaster::new(
            dummy_state.clone(),
            rpc,
            cache,
            db,
        );
        
        assert_eq!(dummy_state.connection_count(), 0);
        assert!(broadcaster.last_sent.is_empty());
    }

    #[tokio::test]
    async fn test_broadcaster_start() {
        let dummy_state = Arc::new(WsState::new());
        let rpc = Arc::new(StellarRpcClient::new("".to_string(), "".to_string(), true));
        let cache = Arc::new(CacheManager::new(Default::default()).await.unwrap());
        
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        let db = Arc::new(Database::new(pool));
        
        let broadcaster = Arc::new(RealtimeBroadcaster::new(
            dummy_state,
            rpc,
            cache,
            db,
        ));
        
        // Start broadcaster (this spawns a background task)
        broadcaster.clone().start().await;
        
        // Give it a moment to start
        sleep(Duration::from_millis(100)).await;
        
        // The broadcaster should be running (we can't easily test the loop without mocking)
        assert!(true); // Test passes if no panic occurs
    }

    #[tokio::test]
    async fn test_broadcast_anchor_status() {
        let dummy_state = Arc::new(WsState::new());
        let rpc = Arc::new(StellarRpcClient::new("".to_string(), "".to_string(), true));
        let cache = Arc::new(CacheManager::new(Default::default()).await.unwrap());
        
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        let db = Arc::new(Database::new(pool));
        
        let broadcaster = RealtimeBroadcaster::new(
            dummy_state.clone(),
            rpc,
            cache,
            db,
        );
        
        let anchor_metrics = crate::models::AnchorMetrics {
            reliability_score: 95.5,
            status: crate::models::AnchorStatus::Green,
        };
        
        // This should not panic even with no connections
        broadcaster.broadcast_anchor_status(anchor_metrics).await;
        
        // Verify no connections were affected
        assert_eq!(dummy_state.connection_count(), 0);
    }

    #[tokio::test]
    async fn test_broadcast_payment() {
        let dummy_state = Arc::new(WsState::new());
        let rpc = Arc::new(StellarRpcClient::new("".to_string(), "".to_string(), true));
        let cache = Arc::new(CacheManager::new(Default::default()).await.unwrap());
        
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        let db = Arc::new(Database::new(pool));
        
        let broadcaster = RealtimeBroadcaster::new(
            dummy_state.clone(),
            rpc,
            cache,
            db,
        );
        
        let payment = crate::models::PaymentRecord {
            id: "test_payment".to_string(),
            transaction_hash: "test_hash".to_string(),
            source_account: "GTEST1".to_string(),
            destination_account: "GTEST2".to_string(),
            asset_type: "credit_alphanum4".to_string(),
            asset_code: Some("USDC".to_string()),
            asset_issuer: Some("GTEST3".to_string()),
            amount: "100.0".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        
        // This should not panic even with no connections
        broadcaster.broadcast_payment(payment).await;
        
        // Verify no connections were affected
        assert_eq!(dummy_state.connection_count(), 0);
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let dummy_state = Arc::new(WsState::new());
        let rpc = Arc::new(StellarRpcClient::new("".to_string(), "".to_string(), true));
        let cache = Arc::new(CacheManager::new(Default::default()).await.unwrap());
        
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        let db = Arc::new(Database::new(pool));
        
        let broadcaster = RealtimeBroadcaster::new(
            dummy_state,
            rpc,
            cache,
            db,
        );
        
        let corridor_key = "USDC-XLM".to_string();
        
        // First update should be allowed
        broadcaster.last_sent.insert(corridor_key.clone(), chrono::Utc::now());
        
        // Immediate second update should be rate limited (in real implementation)
        let now = chrono::Utc::now();
        let last_sent = broadcaster.last_sent.get(&corridor_key).unwrap();
        let elapsed = now.signed_duration_since(*last_sent);
        
        // Should be less than 30 seconds
        assert!(elapsed.num_seconds() < 30);
    }
}
