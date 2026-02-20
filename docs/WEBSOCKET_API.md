# WebSocket API Documentation

## Overview

The Stellar Insights WebSocket API provides real-time updates for corridor metrics, anchor status changes, and payment events. The WebSocket server supports subscription-based messaging, allowing clients to receive only the data they're interested in.

## Connection

### Endpoint
```
ws://localhost:8080/ws
wss://api.stellarinsights.io/ws
```

### Authentication (Optional)
```
ws://localhost:8080/ws?token=your_auth_token
```

## Connection Lifecycle

### 1. Connection Established
Upon successful connection, the server sends a confirmation message:

```json
{
  "type": "connected",
  "connection_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### 2. Heartbeat/Ping-Pong
The client can send ping messages to keep the connection alive:

**Client → Server (Ping):**
```json
{
  "type": "ping",
  "timestamp": 1640995200000
}
```

**Server → Client (Pong):**
```json
{
  "type": "pong",
  "timestamp": 1640995200000
}
```

## Subscription Management

### Subscribe to Channels
Subscribe to specific data channels to receive targeted updates:

```json
{
  "type": "subscribe",
  "channels": [
    "corridor:USDC-XLM",
    "anchor:GCKFBEIYTKP33UHQUKZ6IXDW",
    "payments"
  ]
}
```

### Unsubscribe from Channels
```json
{
  "type": "unsubscribe",
  "channels": ["corridor:USDC-XLM"]
}
```

### Available Channel Types

#### 1. Corridor Updates
- **Pattern:** `corridor:{corridor_key}`
- **Example:** `corridor:USDC-XLM`
- **Description:** Receive updates for specific payment corridors

#### 2. Anchor Updates
- **Pattern:** `anchor:{anchor_id}`
- **Example:** `anchor:GCKFBEIYTKP33UHQUKZ6IXDW`
- **Description:** Receive status changes for specific anchors

#### 3. Payment Stream
- **Pattern:** `payments`
- **Description:** Receive live payment events

#### 4. All Corridors
- **Pattern:** `corridors`
- **Description:** Receive updates for all corridors (dashboard view)

## Message Types

### 1. Corridor Update
Sent every 30 seconds for subscribed corridors or when significant changes occur:

```json
{
  "type": "corridor_update",
  "corridor_key": "USDC-XLM",
  "asset_a_code": "USDC",
  "asset_a_issuer": "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
  "asset_b_code": "XLM",
  "asset_b_issuer": null
}
```

### 2. Anchor Status Change
Sent immediately when anchor status changes:

```json
{
  "type": "anchor_update",
  "anchor_id": "GCKFBEIYTKP33UHQUKZ6IXDW",
  "name": "Example Anchor",
  "reliability_score": 95.5,
  "status": "green"
}
```

**Status Values:**
- `green`: >98% success rate, <1% failures
- `yellow`: 95-98% success rate, 1-5% failures  
- `red`: <95% success rate, >5% failures

### 3. New Payment
Sent for live payment events (requires `payments` subscription):

```json
{
  "type": "new_payment",
  "payment": {
    "id": "payment_123",
    "transaction_hash": "abc123...",
    "source_account": "GCKFBEIYTKP33UHQUKZ6IXDW",
    "destination_account": "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
    "asset_type": "credit_alphanum4",
    "asset_code": "USDC",
    "asset_issuer": "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
    "amount": "100.0000000",
    "created_at": "2024-01-01T12:00:00Z"
  }
}
```

### 4. Health Alert
Sent when corridor health issues are detected:

```json
{
  "type": "health_alert",
  "corridor_id": "USDC-XLM",
  "severity": "warning",
  "message": "Success rate dropped below 95%"
}
```

**Severity Levels:**
- `info`: Informational updates
- `warning`: Performance degradation
- `critical`: Service disruption

### 5. Connection Status
Sent for connection state changes:

```json
{
  "type": "connection_status",
  "status": "connected"
}
```

### 6. Error Messages
Sent when errors occur:

```json
{
  "type": "error",
  "message": "Invalid subscription channel format"
}
```

## Rate Limiting

- **Corridor Updates:** Maximum once every 30 seconds per corridor
- **Anchor Updates:** Immediate (no rate limiting)
- **Payment Events:** Real-time (subject to network conditions)
- **Connection Limit:** 1000 concurrent connections per server

## Error Handling

### Connection Errors
- **1000:** Normal closure
- **1001:** Going away (server shutdown)
- **1002:** Protocol error
- **1003:** Unsupported data type
- **1011:** Server error

### Automatic Reconnection
Clients should implement exponential backoff for reconnection:

```javascript
let reconnectDelay = 1000; // Start with 1 second
const maxDelay = 30000; // Max 30 seconds

function reconnect() {
  setTimeout(() => {
    connect();
    reconnectDelay = Math.min(reconnectDelay * 2, maxDelay);
  }, reconnectDelay);
}
```

## WebSocket Metrics

Monitor WebSocket performance via the metrics endpoint:

```
GET /api/ws/metrics
```

**Response:**
```json
{
  "total_connections": 1250,
  "active_connections": 45,
  "messages_sent": 125000,
  "messages_received": 8500,
  "connection_errors": 12,
  "uptime_seconds": 86400
}
```

## Example Client Implementation

### JavaScript/TypeScript
```typescript
class StellarInsightsWebSocket {
  private ws: WebSocket | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;

  connect(url: string) {
    this.ws = new WebSocket(url);
    
    this.ws.onopen = () => {
      console.log('Connected to Stellar Insights WebSocket');
      this.reconnectAttempts = 0;
      
      // Subscribe to corridors
      this.subscribe(['corridors']);
    };
    
    this.ws.onmessage = (event) => {
      const message = JSON.parse(event.data);
      this.handleMessage(message);
    };
    
    this.ws.onclose = () => {
      console.log('WebSocket connection closed');
      this.reconnect();
    };
    
    this.ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };
  }
  
  subscribe(channels: string[]) {
    this.send({
      type: 'subscribe',
      channels
    });
  }
  
  private send(message: any) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message));
    }
  }
  
  private handleMessage(message: any) {
    switch (message.type) {
      case 'corridor_update':
        this.onCorridorUpdate(message);
        break;
      case 'anchor_update':
        this.onAnchorUpdate(message);
        break;
      case 'new_payment':
        this.onNewPayment(message);
        break;
    }
  }
  
  private reconnect() {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      const delay = Math.pow(2, this.reconnectAttempts) * 1000;
      setTimeout(() => {
        this.reconnectAttempts++;
        this.connect(this.url);
      }, delay);
    }
  }
}
```

## Best Practices

### 1. Subscription Management
- Subscribe only to channels you need
- Unsubscribe when components unmount
- Use specific corridor/anchor IDs rather than broad subscriptions

### 2. Message Handling
- Implement proper error handling for malformed messages
- Use message timestamps to handle out-of-order delivery
- Debounce rapid updates to prevent UI flickering

### 3. Connection Management
- Implement exponential backoff for reconnection
- Handle network state changes (online/offline)
- Show connection status to users

### 4. Performance
- Batch UI updates when receiving multiple messages
- Use efficient data structures for storing real-time data
- Implement proper cleanup to prevent memory leaks

## Security Considerations

- Use WSS (WebSocket Secure) in production
- Validate all incoming messages
- Implement proper authentication for sensitive data
- Rate limit subscription requests
- Monitor for suspicious connection patterns

## Support

For technical support or questions about the WebSocket API:
- Email: support@stellarinsights.io
- Documentation: https://docs.stellarinsights.io
- GitHub Issues: https://github.com/stellar-insights/issues