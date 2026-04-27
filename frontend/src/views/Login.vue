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
        
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">验证码</label>
          <div class="flex gap-3">
            <input
              v-model="captcha"
              type="text"
              placeholder="请输入验证码"
              class="flex-1 px-4 py-3 rounded-xl border-2 border-pink-100 focus:border-pink-300 focus:outline-none transition-colors"
              maxlength="4"
            />
            <img
              v-if="captchaImage"
              :src="captchaImage"
              alt="验证码"
              class="h-12 rounded-xl cursor-pointer border-2 border-pink-100"
              @click="fetchCaptcha"
            />
          </div>
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
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import api from '../services/api'

const router = useRouter()
const email = ref('')
const password = ref('')
const captcha = ref('')
const captchaId = ref('')
const captchaImage = ref('')
const loading = ref(false)
const error = ref('')

onMounted(() => {
  fetchCaptcha()
})

const fetchCaptcha = async () => {
  try {
    const response = await api.get('/captcha')
    captchaId.value = response.data.captcha_id
    captchaImage.value = response.data.image
    captcha.value = ''
  } catch (err) {
    console.error('Failed to fetch captcha:', err)
  }
}

const handleLogin = async () => {
  if (!email.value || !password.value || !captcha.value) {
    error.value = '请填写所有字段'
    return
  }
  
  loading.value = true
  error.value = ''
  
  try {
    const response = await api.post('/auth/login', {
      email: email.value,
      password: password.value,
      captcha_id: captchaId.value,
      captcha: captcha.value
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
    if (err.response?.status === 400) {
      fetchCaptcha()
    }
  } finally {
    loading.value = false
  }
}
</script>
