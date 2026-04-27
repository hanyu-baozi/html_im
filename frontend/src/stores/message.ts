import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Message } from '@/types'

export const useMessageStore = defineStore('message', () => {
  const messages = ref<Message[]>([])
  const currentContactId = ref<string | null>(null)

  const addMessage = (message: Message) => {
    messages.value.push(message)
  }

  const setMessages = (newMessages: Message[]) => {
    messages.value = newMessages
  }

  const setCurrentContact = (contactId: string) => {
    currentContactId.value = contactId
  }

  const getMessagesByContact = (contactId: string) => {
    return messages.value.filter(
      (m: Message) => m.sender_id === contactId || m.receiver_id === contactId
    )
  }

  const clearMessages = () => {
    messages.value = []
  }

  return {
    messages,
    currentContactId,
    addMessage,
    setMessages,
    setCurrentContact,
    getMessagesByContact,
    clearMessages,
  }
})
