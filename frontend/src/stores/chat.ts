import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { User, Session } from '@/types'

export const useChatStore = defineStore('chat', () => {
  const contacts = ref<User[]>([])
  const sessions = ref<Session[]>([])
  const activeSessionId = ref<string | null>(null)

  const setContacts = (newContacts: User[]) => {
    contacts.value = newContacts
  }

  const addContact = (contact: User) => {
    contacts.value.push(contact)
  }

  const setSessions = (newSessions: Session[]) => {
    sessions.value = newSessions
  }

  const setActiveSession = (sessionId: string) => {
    activeSessionId.value = sessionId
  }

  const clearActiveSession = () => {
    activeSessionId.value = null
  }

  return {
    contacts,
    sessions,
    activeSessionId,
    setContacts,
    addContact,
    setSessions,
    setActiveSession,
    clearActiveSession,
  }
})
