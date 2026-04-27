import { useMessageStore } from '@/stores/message'
import { useWebSocket } from '@/composables/useWebSocket'
import type { MessagePayload, WSMessage } from '@/types'

export function useMessage() {
  const messageStore = useMessageStore()
  const { send } = useWebSocket('ws://localhost:8080/ws')

  const sendMessage = (content: string, receiverId: string) => {
    const message: WSMessage = {
      type: 'message',
      payload: {
        id: generateId(),
        sender_id: 'current_user',
        receiver_id: receiverId,
        content,
        message_type: 'text',
        timestamp: Date.now(),
      },
    }
    send(message)
    
    // 乐观更新
    messageStore.addMessage({
      id: message.payload!.id,
      content,
      sender_id: 'current_user',
      receiver_id: receiverId,
      message_type: 'text',
      timestamp: Date.now(),
      is_read: false,
    })
  }

  const generateId = () => {
    return Math.random().toString(36).substring(2, 15)
  }

  return {
    sendMessage,
  }
}
