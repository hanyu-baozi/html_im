import { createRouter, createWebHistory } from 'vue-router'
import Login from '../views/Login.vue'
import Register from '../views/Register.vue'
import Chat from '../views/Chat.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/login',
      name: 'Login',
      component: Login
    },
    {
      path: '/register',
      name: 'Register',
      component: Register
    },
    {
      path: '/',
      name: 'Chat',
      component: Chat,
      meta: { requiresAuth: true }
    }
  ]
})

// Navigation guard
router.beforeEach((to) => {
  const token = localStorage.getItem('token')
  
  if (to.meta.requiresAuth && !token) {
    return '/login'
  } else if ((to.path === '/login' || to.path === '/register') && token) {
    return '/'
  }
})

export default router
