import { useEffect, useState } from 'react'
import { useWebSocket } from './useWebSocket'
// import { WsMessage } from './useWebSocket'

export function useRealtimeCorridors(url: string) {
  const { isConnected, lastMessage, send } = useWebSocket(url)
  const [corridors, setCorridors] = useState<Record<string, unknown>[]>([])

  useEffect(() => {
    if (!isConnected) return
    // Subscribe to corridor updates for dashboard (subscribe to all)
    send({ type: 'subscribe', channels: ['corridors'] })
  }, [isConnected, send])

  useEffect(() => {
    if (!lastMessage) return
    
    // Use a callback to avoid direct setState in effect
    const handleCorridorUpdate = () => {
      if (lastMessage.type === 'corridor_update' && 'data' in lastMessage) {
        const msg = lastMessage as unknown as { data: Record<string, unknown> & { id: string } }
        // update or insert
        setCorridors((prev) => {
          const found = prev.find((c) => (c as { id: string }).id === msg.data.id)
          if (found) {
            return prev.map((c) => ((c as { id: string }).id === msg.data.id ? { ...c, ...msg.data } : c))
          }
          return [msg.data, ...prev]
        })
      }
    }

    // Use setTimeout to avoid synchronous setState in effect
    const timer = setTimeout(handleCorridorUpdate, 0)
    return () => clearTimeout(timer)
  }, [lastMessage])

  return { isConnected, corridors, send, lastMessage }
}
