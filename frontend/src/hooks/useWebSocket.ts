import { useEffect, useRef, useState, useCallback } from 'react'

export type WsMessage =
  | { type: 'corridor_update'; data: Record<string, unknown> }
  | { type: 'anchor_update'; data: Record<string, unknown> }
  | { type: 'new_payment'; data: Record<string, unknown> }
  | { type: 'health_alert'; data: Record<string, unknown> }
  | { type: 'connection_status'; status: string }
  | { type: 'pong'; timestamp?: number }
  | { type: 'subscribe' }

export function useWebSocket(url: string) {
  const [isConnected, setIsConnected] = useState(false)
  const [lastMessage, setLastMessage] = useState<WsMessage | null>(null)
  const wsRef = useRef<WebSocket | null>(null)
  const reconnectRef = useRef(0)

  useEffect(() => {
    const connect = () => {
      const ws = new WebSocket(url)
      wsRef.current = ws

      ws.onopen = () => {
        reconnectRef.current = 0
        setIsConnected(true)
        setLastMessage({ type: 'connection_status', status: 'connected' })
      }

      ws.onmessage = (ev) => {
        try {
          const msg = JSON.parse(ev.data)
          setLastMessage(msg)
        } catch (e) {
          console.warn('Failed to parse WS message', e)
        }
      }

      ws.onclose = () => {
        setIsConnected(false)
        setLastMessage({ type: 'connection_status', status: 'closed' })
        // reconnect with exponential backoff
        const timeout = Math.min(30000, 1000 * 2 ** reconnectRef.current)
        reconnectRef.current += 1
        setTimeout(() => {
          if (wsRef.current?.readyState === WebSocket.CLOSED) {
            connect()
          }
        }, timeout)
      }

      ws.onerror = () => {
        setIsConnected(false)
        setLastMessage({ type: 'connection_status', status: 'error' })
        try {
          ws.close()
        } catch {}
      }
    }

    connect()

    return () => {
      try {
        wsRef.current?.close()
      } catch {}
    }
  }, [url])

  const send = useCallback((obj: Record<string, unknown>) => {
    try {
      wsRef.current?.send(JSON.stringify(obj))
    } catch (e) {
      console.warn('Failed to send ws message', e)
    }
  }, [])

  return { isConnected, lastMessage, send }
}
