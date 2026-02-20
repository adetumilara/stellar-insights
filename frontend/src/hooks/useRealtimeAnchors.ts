import { useEffect, useState } from 'react'
import { useWebSocket } from './useWebSocket'
// import { WsMessage } from './useWebSocket'

export function useRealtimeAnchors(url: string) {
  const { isConnected, lastMessage, send } = useWebSocket(url)
  const [anchors, setAnchors] = useState<Record<string, unknown>[]>([])

  useEffect(() => {
    if (!isConnected) return
    // Subscribe to anchor updates
    send({ type: 'subscribe', channels: ['anchors'] })
  }, [isConnected, send])

  useEffect(() => {
    if (!lastMessage) return
    
    // Use a callback to avoid direct setState in effect
    const handleAnchorUpdate = () => {
      if (lastMessage.type === 'anchor_update' && 'data' in lastMessage) {
        const msg = lastMessage as unknown as { data: Record<string, unknown> & { id: string } }
        setAnchors((prev) => {
          const found = prev.find((a) => (a as { id: string }).id === msg.data.id)
          if (found) {
            return prev.map((a) => ((a as { id: string }).id === msg.data.id ? { ...a, ...msg.data } : a))
          }
          return [msg.data, ...prev]
        })
      }
    }

    // Use setTimeout to avoid synchronous setState in effect
    const timer = setTimeout(handleAnchorUpdate, 0)
    return () => clearTimeout(timer)
  }, [lastMessage])

  return { isConnected, anchors, send, lastMessage }
}
