import { ref, onUnmounted } from 'vue'
import type { WSMessage } from '@/types'

export function useWebSocket(url: string) {
  const ws = ref<WebSocket | null>(null)
  const connected = ref(false)
  const reconnectTimer = ref<number | null>(null)

  const connect = () => {
    ws.value = new WebSocket(url)
    
    ws.value.onopen = () => {
      connected.value = true
      startHeartbeat()
    }
    
    ws.value.onclose = () => {
      connected.value = false
      stopHeartbeat()
      scheduleReconnect()
    }
    
    ws.value.onerror = (error) => {
      console.error('WebSocket error:', error)
    }
  }

  const send = (data: WSMessage) => {
    if (ws.value?.readyState === WebSocket.OPEN) {
      ws.value.send(JSON.stringify(data))
    }
  }

  const onMessage = (callback: (data: any) => void) => {
    if (ws.value) {
      ws.value.onmessage = (event) => {
        callback(JSON.parse(event.data))
      }
    }
  }

  // 心跳机制
  let heartbeatInterval: number
  const startHeartbeat = () => {
    heartbeatInterval = window.setInterval(() => {
      send({ type: 'ping' })
    }, 30000) // 30秒一次
  }

  const stopHeartbeat = () => {
    clearInterval(heartbeatInterval)
  }

  // 断线重连
  const scheduleReconnect = () => {
    reconnectTimer.value = window.setTimeout(() => {
      connect()
    }, 3000) // 3秒后重连
  }

  onUnmounted(() => {
    if (reconnectTimer.value) {
      clearTimeout(reconnectTimer.value)
    }
    stopHeartbeat()
    ws.value?.close()
  })

  return {
    ws,
    connected,
    connect,
    send,
    onMessage,
  }
}
