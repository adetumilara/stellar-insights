use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::{IntoResponse, Response},
    Json,
};
use dashmap::DashMap;
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use tokio::sync::broadcast;
use tracing::{error, info, warn};
use uuid::Uuid;

/// WebSocket connection state with metrics
pub struct WsState {
    /// Map of connection ID to broadcast sender
    pub connections: DashMap<Uuid, tokio::sync::mpsc::Sender<WsMessage>>,
    /// Map of connection ID to list of subscribed channels
    pub subscriptions: DashMap<Uuid, Vec<String>>,
    /// Broadcast channel for sending messages to all connections
    pub tx: broadcast::Sender<WsMessage>,
    /// Connection metrics
    pub metrics: WsMetrics,
}

/// WebSocket connection metrics
#[derive(Debug, Default)]
pub struct WsMetrics {
    /// Total connections ever established
    pub total_connections: AtomicU64,
    /// Current active connections
    pub active_connections: AtomicU64,
    /// Total messages sent
    pub messages_sent: AtomicU64,
    /// Total messages received
    pub messages_received: AtomicU64,
    /// Connection errors
    pub connection_errors: AtomicU64,
    /// Start time for uptime calculation
    pub start_time: Instant,
}

impl WsState {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(100);
        Self {
            connections: DashMap::new(),
            subscriptions: DashMap::new(),
            tx,
            metrics: WsMetrics {
                total_connections: AtomicU64::new(0),
                active_connections: AtomicU64::new(0),
                messages_sent: AtomicU64::new(0),
                messages_received: AtomicU64::new(0),
                connection_errors: AtomicU64::new(0),
                start_time: Instant::now(),
            },
        }
    }

    /// Broadcast a message to all connected clients
    pub fn broadcast(&self, message: WsMessage) {
        self.metrics.messages_sent.fetch_add(1, Ordering::Relaxed);
        if let Err(e) = self.tx.send(message) {
            warn!("Failed to broadcast message: {}", e);
        }
    }

    /// Get the number of active connections
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    /// Get WebSocket metrics
    pub fn get_metrics(&self) -> WsMetricsSnapshot {
        WsMetricsSnapshot {
            total_connections: self.metrics.total_connections.load(Ordering::Relaxed),
            active_connections: self.metrics.active_connections.load(Ordering::Relaxed),
            messages_sent: self.metrics.messages_sent.load(Ordering::Relaxed),
            messages_received: self.metrics.messages_received.load(Ordering::Relaxed),
            connection_errors: self.metrics.connection_errors.load(Ordering::Relaxed),
            uptime_seconds: self.metrics.start_time.elapsed().as_secs(),
        }
    }

    /// Subscribe connection to channels
    pub fn subscribe(&self, id: &Uuid, channels: Vec<String>) {
        self.subscriptions.insert(*id, channels);
    }

    /// Unsubscribe connection from given channels (remove those channels)
    pub fn unsubscribe(&self, id: &Uuid, channels: Vec<String>) {
        if let Some(mut entry) = self.subscriptions.get_mut(id) {
            entry.retain(|c| !channels.contains(c));
        }
    }
}

/// WebSocket metrics snapshot for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMetricsSnapshot {
    pub total_connections: u64,
    pub active_connections: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub connection_errors: u64,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    /// New snapshot available
    SnapshotUpdate {
        snapshot_id: String,
        epoch: i64,
        timestamp: String,
        hash: String,
    },
    /// Corridor metrics updated
    CorridorUpdate {
        corridor_key: String,
        asset_a_code: String,
        asset_a_issuer: String,
        asset_b_code: String,
        asset_b_issuer: String,
    },
    /// Anchor metrics updated
    AnchorUpdate {
        anchor_id: String,
        name: String,
        reliability_score: f64,
        status: String,
    },
    /// Heartbeat/Ping message
    Ping { timestamp: i64 },
    /// Pong response
    Pong { timestamp: i64 },
    /// Connection established
    Connected { connection_id: String },
    /// Error message
    Error { message: String },
    /// Client subscribe to channels
    Subscribe { channels: Vec<String> },
    /// Client unsubscribe from channels
    Unsubscribe { channels: Vec<String> },
    /// New payment event
    NewPayment { payment: crate::models::PaymentRecord },
    /// Health alert
    HealthAlert { corridor_id: String, severity: String, message: String },
    /// Connection status notification
    ConnectionStatus { status: String },
}

/// WebSocket metrics endpoint
pub async fn ws_metrics(State(state): State<Arc<WsState>>) -> Json<WsMetricsSnapshot> {
    Json(state.get_metrics())
}

#[derive(Debug, Deserialize)]
pub struct WsQueryParams {
    /// Optional authentication token
    pub token: Option<String>,
}

/// WebSocket handler endpoint
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WsQueryParams>,
    State(state): State<Arc<WsState>>,
) -> Response {
    // Validate authentication token if provided
    if let Some(token) = params.token {
        if !validate_token(&token) {
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Unauthorized"}))
            ).into_response();

        }
    }

    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Validate authentication token
fn validate_token(token: &str) -> bool {
    // For now, implement basic token validation
    // In production, use JWT or other robust auth mechanism
    
    // If WS_AUTH_TOKEN env var is set, validate against it
    // Otherwise, accept all tokens (for development)
    match std::env::var("WS_AUTH_TOKEN") {
        Ok(expected_token) => token == expected_token,
        Err(_) => {
            // No token configured, allow all connections
            warn!("WS_AUTH_TOKEN not configured, allowing all WebSocket connections");
            true
        }
    }
}

/// Handle individual WebSocket connection
async fn handle_socket(socket: WebSocket, state: Arc<WsState>) {
    let connection_id = Uuid::new_v4();
    info!("New WebSocket connection: {}", connection_id);

    // Update metrics
    state.metrics.total_connections.fetch_add(1, Ordering::Relaxed);
    state.metrics.active_connections.fetch_add(1, Ordering::Relaxed);

    let (sender, receiver) = socket.split();
    let sender = Arc::new(tokio::sync::Mutex::new(sender));

    // Create a channel for this specific connection
    let (tx, mut rx) = tokio::sync::mpsc::channel::<WsMessage>(32);

    // Register the connection
    state.connections.insert(connection_id, tx);
    // Initialize empty subscription list
    state.subscriptions.insert(connection_id, Vec::new());

    // Subscribe to broadcast messages
    let mut broadcast_rx = state.tx.subscribe();

    // Send connection confirmation
    let connected_msg = WsMessage::Connected {
        connection_id: connection_id.to_string(),
    };
    if let Ok(json) = serde_json::to_string(&connected_msg) {
        let mut sender_guard = sender.lock().await;
        let _ = sender_guard.send(Message::Text(json)).await;
    }

    // Clone sender for tasks
    let send_sender = Arc::clone(&sender);
    let recv_sender = Arc::clone(&sender);

    // Task for receiving messages from client
    let recv_task = {
        let connection_id = connection_id;
        let state_clone = Arc::clone(&state);
        tokio::spawn(async move {
            let mut receiver = receiver;
            while let Some(Ok(msg)) = receiver.next().await {
                match msg {
                    Message::Text(text) => {
                        // Track received message
                        state_clone.metrics.messages_received.fetch_add(1, Ordering::Relaxed);
                        
                        if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                            match ws_msg {
                                WsMessage::Ping { timestamp } => {
                                    info!("Received ping from {}", connection_id);
                                    let pong = WsMessage::Pong { timestamp };
                                    if let Ok(json) = serde_json::to_string(&pong) {
                                        let mut sender_guard = recv_sender.lock().await;
                                        let _ = sender_guard.send(Message::Text(json)).await;
                                    }
                                }
                                WsMessage::Subscribe { channels } => {
                                    info!("Connection {} subscribe: {:?}", connection_id, channels);
                                    state_clone.subscribe(&connection_id, channels);
                                }
                                WsMessage::Unsubscribe { channels } => {
                                    info!("Connection {} unsubscribe: {:?}", connection_id, channels);
                                    state_clone.unsubscribe(&connection_id, channels);
                                }
                                WsMessage::Pong { .. } => {
                                    // ignore
                                }
                                _ => {
                                    warn!("Unexpected message type from client: {:?}", ws_msg);
                                }
                            }
                        }
                    }
                    Message::Ping(data) => {
                        info!("Received WebSocket ping from {}", connection_id);
                        let mut sender_guard = recv_sender.lock().await;
                        let _ = sender_guard.send(Message::Pong(data)).await;
                    }
                    Message::Close(_) => {
                        info!("Client {} requested close", connection_id);
                        break;
                    }
                    _ => {}
                }
            }
        })
    };

    // Task for sending messages to client
    let send_task = {
        let connection_id = connection_id;
        tokio::spawn(async move {
            let mut ping_interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
            
            loop {
                tokio::select! {
                    // Send ping every 30 seconds
                    _ = ping_interval.tick() => {
                        let ping = WsMessage::Ping {
                            timestamp: chrono::Utc::now().timestamp(),
                        };
                        if let Ok(json) = serde_json::to_string(&ping) {
                            let mut sender_guard = send_sender.lock().await;
                            if sender_guard.send(Message::Text(json)).await.is_err() {
                                error!("Failed to send ping to {}", connection_id);
                                break;
                            }
                        }
                    }
                    // Receive from broadcast channel
                    Ok(msg) = broadcast_rx.recv() => {
                        if let Ok(json) = serde_json::to_string(&msg) {
                            let mut sender_guard = send_sender.lock().await;
                            if sender_guard.send(Message::Text(json)).await.is_err() {
                                error!("Failed to send broadcast message to {}", connection_id);
                                break;
                            }
                        }
                    }
                    // Receive from connection-specific channel
                    Some(msg) = rx.recv() => {
                        if let Ok(json) = serde_json::to_string(&msg) {
                            let mut sender_guard = send_sender.lock().await;
                            if sender_guard.send(Message::Text(json)).await.is_err() {
                                error!("Failed to send message to {}", connection_id);
                                break;
                            }
                        }
                    }
                }
            }
        })
    };

    // Wait for either task to finish
    tokio::select! {
        _ = recv_task => {
            info!("Receive task finished for {}", connection_id);
        }
        _ = send_task => {
            info!("Send task finished for {}", connection_id);
        }
    }

    // Clean up connection
    state.connections.remove(&connection_id);
    state.subscriptions.remove(&connection_id);
    
    // Update metrics
    state.metrics.active_connections.fetch_sub(1, Ordering::Relaxed);
    
    info!(
        "WebSocket connection {} closed. Active connections: {}",
        connection_id,
        state.connection_count()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_state_creation() {
        let state = WsState::new();
        assert_eq!(state.connection_count(), 0);
    }

    #[test]
    fn test_validate_token_no_env() {
        // Without WS_AUTH_TOKEN env var, should accept any token
        assert!(validate_token("any_token"));
    }

    #[test]
    fn test_ws_message_serialization() {
        let msg = WsMessage::SnapshotUpdate {
            snapshot_id: "test-id".to_string(),
            epoch: 1,
            timestamp: "2024-01-01".to_string(),
            hash: "abc123".to_string(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("snapshot_update"));
        assert!(json.contains("test-id"));
    }
}
