<template>
  <nav class="navbar glass-card">
    <div class="navbar-inner">
      <router-link to="/" class="navbar-brand">
        <span class="brand-icon">✦</span>
        <span class="brand-text">Airi Press</span>
      </router-link>

      <div class="navbar-links">
        <router-link to="/" class="nav-link">首页</router-link>

        <template v-if="auth.isLoggedIn">
          <router-link v-if="auth.isAuthor" to="/admin" class="nav-link">管理</router-link>
          <div class="nav-user">
            <span class="nav-username">{{ auth.user?.username }}</span>
            <button class="btn btn-sm" @click="handleLogout">退出</button>
          </div>
        </template>

        <template v-else>
          <router-link to="/login" class="nav-link">登录</router-link>
          <router-link to="/register" class="btn btn-primary btn-sm">注册</router-link>
        </template>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'
import { useRouter } from 'vue-router'

const auth = useAuthStore()
const router = useRouter()

function handleLogout() {
  auth.logout()
  router.push('/')
}
</script>

<style scoped>
.navbar {
  position: sticky;
  top: 0;
  z-index: 100;
  border-radius: 0;
  border-top: none;
  border-left: none;
  border-right: none;
}

.navbar:hover {
  transform: none;
}

.navbar-inner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  max-width: 1200px;
  margin: 0 auto;
  padding: 0.75rem 2rem;
}

.navbar-brand {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: var(--text-primary);
  font-size: 1.2rem;
  font-weight: 700;
  letter-spacing: -0.02em;
}

.brand-icon {
  color: var(--accent);
  font-size: 1.4rem;
}

.navbar-links {
  display: flex;
  align-items: center;
  gap: 1.5rem;
}

.nav-link {
  color: var(--text-secondary);
  font-size: 0.9rem;
  font-weight: 500;
  padding: 0.375rem 0;
  position: relative;
}

.nav-link:hover,
.nav-link.router-link-active {
  color: var(--text-primary);
}

.nav-link::after {
  content: '';
  position: absolute;
  bottom: -2px;
  left: 0;
  width: 0;
  height: 2px;
  background: var(--accent);
  border-radius: 1px;
  transition: var(--transition);
}

.nav-link:hover::after,
.nav-link.router-link-active::after {
  width: 100%;
}

.nav-user {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.nav-username {
  color: var(--text-secondary);
  font-size: 0.85rem;
}

@media (max-width: 768px) {
  .navbar-inner {
    padding: 0.75rem 1rem;
  }

  .navbar-links {
    gap: 1rem;
  }
}
</style>
