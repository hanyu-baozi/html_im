import { useUserStore } from '@/stores/user'
import type { User } from '@/types'

export function useAuth() {
  const userStore = useUserStore()

  const login = async (email: string, password: string) => {
    // TODO: Implement API call to login
    // const response = await api.post('/auth/login', { email, password })
    // userStore.setToken(response.data.token)
    // userStore.setUser(response.data.user)
  }

  const register = async (username: string, email: string, password: string) => {
    // TODO: Implement API call to register
    // const response = await api.post('/auth/register', { username, email, password })
    // userStore.setToken(response.data.token)
    // userStore.setUser(response.data.user)
  }

  const logout = () => {
    userStore.logout()
  }

  return {
    login,
    register,
    logout,
  }
}
