import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { User } from '@/types'

export const useUserStore = defineStore('user', () => {
  const currentUser = ref<User | null>(null)
  const token = ref<string | null>(localStorage.getItem('token'))

  const setUser = (user: User) => {
    currentUser.value = user
  }

  const setToken = (newToken: string) => {
    token.value = newToken
    localStorage.setItem('token', newToken)
  }

  const clearToken = () => {
    token.value = null
    localStorage.removeItem('token')
  }

  const logout = () => {
    currentUser.value = null
    clearToken()
  }

  return {
    currentUser,
    token,
    setUser,
    setToken,
    clearToken,
    logout,
  }
})
