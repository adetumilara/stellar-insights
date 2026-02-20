use futures::{SinkExt, StreamExt};
use std::sync::Arc;

#[tokio::test]
async fn websocket_integration_connects() {
    use axum::{routing::get, Router};
    use stellar_insights_backend::websocket::{ws_handler, WsState};

    // Start a small server with the ws route
    let ws_state = Arc::new(WsState::new());
    let app = Router::new().route("/ws", get(ws_handler)).with_state(ws_state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server = axum::Server::from_tcp(listener).unwrap().serve(app.into_make_service());
    let server_handle = tokio::spawn(server);

    // Connect a websocket client to the server
    let url = format!("ws://{}/ws", addr);
    let (mut ws_stream, _resp) = tokio_tungstenite::connect_async(url).await.expect("Failed to connect");

    // Expect at least one text message (Connected confirmation or Ping)
    if let Some(Ok(msg)) = ws_stream.next().await {
        match msg {
            tokio_tungstenite::tungstenite::Message::Text(t) => {
                assert!(t.contains("connection_id") || t.contains("connected") || t.contains("ping"));
            }
            _ => panic!("Expected text message from server"),
        }
    } else {
        panic!("No message received from server");
    }

    // Clean up server
    server_handle.abort();
}
