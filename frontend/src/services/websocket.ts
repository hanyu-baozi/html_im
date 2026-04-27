import { useWebSocket } from '@/composables/useWebSocket'

const WS_URL = import.meta.env.VITE_WS_URL || 'ws://localhost:8080/ws'

export const wsClient = useWebSocket(WS_URL)

export default wsClient
