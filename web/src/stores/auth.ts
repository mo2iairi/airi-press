import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { User, LoginRequest } from '@/types'
import { authApi, usersApi } from '@/api/services'
import { Permission } from '@/types'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const token = ref<string | null>(null)
  const loading = ref(false)
  const initialized = ref(false)

  const isAuthenticated = computed(() => !!token.value)
  const isAdmin = computed(() => user.value?.permission === Permission.ADMIN)
  const canCreatePost = computed(() => (user.value?.permission ?? 0) >= Permission.AUTHOR)
  const canEditAnyPost = computed(() => (user.value?.permission ?? 0) >= Permission.EDITOR)

  async function init() {
    if (initialized.value) return

    const savedToken = localStorage.getItem('token')
    const savedUser = localStorage.getItem('user')

    if (savedToken && savedUser) {
      token.value = savedToken
      user.value = JSON.parse(savedUser)
    }

    initialized.value = true
  }

  async function login(credentials: LoginRequest) {
    loading.value = true
    try {
      const response = await authApi.login(credentials)
      token.value = response.data.token
      user.value = response.data.user

      localStorage.setItem('token', response.data.token)
      localStorage.setItem('user', JSON.stringify(response.data.user))

      return response.data
    } finally {
      loading.value = false
    }
  }

  function logout() {
    token.value = null
    user.value = null
    localStorage.removeItem('token')
    localStorage.removeItem('user')
  }

  async function refreshUser() {
    if (!token.value) return
    try {
      const response = await usersApi.getMe()
      user.value = response.data
      localStorage.setItem('user', JSON.stringify(response.data))
    } catch {
      logout()
    }
  }

  return {
    user,
    token,
    loading,
    initialized,
    isAuthenticated,
    isAdmin,
    canCreatePost,
    canEditAnyPost,
    init,
    login,
    logout,
    refreshUser,
  }
})
