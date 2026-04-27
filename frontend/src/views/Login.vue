<template>
  <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-pink-50 to-pink-100">
    <div class="bg-white/80 backdrop-blur-sm rounded-3xl shadow-xl p-8 w-full max-w-md">
      <div class="text-center mb-8">
        <div class="w-20 h-20 rounded-full bg-gradient-to-br from-pink-400 to-pink-600 flex items-center justify-center text-white font-bold text-3xl mx-auto mb-4">
          IM
        </div>
        <h1 class="text-2xl font-bold text-gray-700">欢迎回来</h1>
        <p class="text-gray-500 mt-2">登录您的账户</p>
      </div>
      
      <form @submit.prevent="handleLogin" class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">邮箱</label>
          <input
            v-model="email"
            type="email"
            placeholder="请输入邮箱"
            class="w-full px-4 py-3 rounded-xl border-2 border-pink-100 focus:border-pink-300 focus:outline-none transition-colors"
          />
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">密码</label>
          <input
            v-model="password"
            type="password"
            placeholder="请输入密码"
            class="w-full px-4 py-3 rounded-xl border-2 border-pink-100 focus:border-pink-300 focus:outline-none transition-colors"
          />
        </div>
        
        <button
          type="submit"
          :disabled="loading"
          class="w-full py-3 rounded-xl bg-gradient-to-r from-pink-500 to-pink-600 text-white font-medium hover:from-pink-600 hover:to-pink-700 transition-all hover:shadow-lg disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {{ loading ? '登录中...' : '登录' }}
        </button>
      </form>
      
      <div class="mt-6 text-center">
        <p class="text-gray-500">
          还没有账户？
          <router-link to="/register" class="text-pink-500 hover:text-pink-600 font-medium">
            立即注册
          </router-link>
        </p>
      </div>
      
      <div v-if="error" class="mt-4 p-3 bg-red-50 border border-red-200 rounded-xl text-red-600 text-sm">
        {{ error }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import api from '../services/api'

const router = useRouter()
const email = ref('')
const password = ref('')
const loading = ref(false)
const error = ref('')

const handleLogin = async () => {
  if (!email.value || !password.value) {
    error.value = '请填写所有字段'
    return
  }
  
  loading.value = true
  error.value = ''
  
  try {
    const response = await api.post('/auth/login', {
      email: email.value,
      password: password.value
    })
    
    localStorage.setItem('token', response.data.token)
    localStorage.setItem('user', JSON.stringify(response.data.user))
    
    // Set user status to online
    try {
      await api.put('/users/status', { status: 'online' })
    } catch (statusError) {
      console.error('Failed to update status:', statusError)
    }
    
    router.push('/')
  } catch (err: any) {
    error.value = err.response?.data?.error || '登录失败，请检查邮箱和密码'
  } finally {
    loading.value = false
  }
}
</script>
