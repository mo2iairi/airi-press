import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import api from '@/api'
import type { User, ApiResponse, AuthResponse } from '@/types'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const token = ref<string | null>(localStorage.getItem('token'))

  const isLoggedIn = computed(() => !!token.value && !!user.value)
  const isAuthor = computed(() => (user.value?.permission ?? 0) >= 1)
  const isAdmin = computed(() => user.value?.permission === 255)

  async function login(username: string, password: string) {
    const { data } = await api.post<ApiResponse<AuthResponse>>('/auth/login', {
      username,
      password,
    })
    if (data.data) {
      token.value = data.data.token
      user.value = data.data.user
      localStorage.setItem('token', data.data.token)
    }
  }

  async function register(username: string, password: string) {
    const { data } = await api.post<ApiResponse<AuthResponse>>('/auth/register', {
      username,
      password,
    })
    if (data.data) {
      token.value = data.data.token
      user.value = data.data.user
      localStorage.setItem('token', data.data.token)
    }
  }

  async function fetchMe() {
    if (!token.value) return
    try {
      const { data } = await api.get<ApiResponse<User>>('/auth/me')
      if (data.data) {
        user.value = data.data
      }
    } catch {
      logout()
    }
  }

  function logout() {
    token.value = null
    user.value = null
    localStorage.removeItem('token')
  }

  return {
    user,
    token,
    isLoggedIn,
    isAuthor,
    isAdmin,
    login,
    register,
    fetchMe,
    logout,
  }
})
