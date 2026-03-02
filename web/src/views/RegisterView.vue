<template>
  <div class="auth-page">
    <div class="auth-card glass-card">
      <h1 class="auth-title">注册</h1>
      <p class="auth-subtitle">加入 Airi Press</p>

      <form @submit.prevent="handleRegister" class="auth-form">
        <div class="form-group">
          <label class="form-label">用户名</label>
          <input
            v-model="username"
            type="text"
            class="form-input"
            placeholder="至少 3 个字符"
            required
            minlength="3"
          />
        </div>

        <div class="form-group">
          <label class="form-label">密码</label>
          <input
            v-model="password"
            type="password"
            class="form-input"
            placeholder="至少 6 个字符"
            required
            minlength="6"
          />
        </div>

        <div class="form-group">
          <label class="form-label">确认密码</label>
          <input
            v-model="confirmPassword"
            type="password"
            class="form-input"
            placeholder="再次输入密码"
            required
          />
        </div>

        <p v-if="error" class="text-danger mb-2">{{ error }}</p>

        <button type="submit" class="btn btn-primary auth-btn" :disabled="loading">
          {{ loading ? '注册中...' : '注册' }}
        </button>
      </form>

      <p class="auth-footer">
        已有账号？
        <router-link to="/login">登录</router-link>
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const auth = useAuthStore()

const username = ref('')
const password = ref('')
const confirmPassword = ref('')
const error = ref('')
const loading = ref(false)

async function handleRegister() {
  error.value = ''

  if (password.value !== confirmPassword.value) {
    error.value = '两次输入的密码不一致'
    return
  }

  loading.value = true
  try {
    await auth.register(username.value, password.value)
    router.push('/')
  } catch (e: any) {
    error.value = e.response?.data?.message || '注册失败'
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.auth-page {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 60vh;
}

.auth-card {
  width: 100%;
  max-width: 420px;
  padding: 2.5rem;
}

.auth-card:hover {
  transform: none;
}

.auth-title {
  font-size: 1.8rem;
  font-weight: 700;
  margin-bottom: 0.25rem;
}

.auth-subtitle {
  color: var(--text-secondary);
  margin-bottom: 2rem;
}

.auth-form {
  margin-bottom: 1.5rem;
}

.auth-btn {
  width: 100%;
  padding: 0.75rem;
  font-size: 1rem;
}

.auth-footer {
  text-align: center;
  color: var(--text-secondary);
  font-size: 0.9rem;
}
</style>
