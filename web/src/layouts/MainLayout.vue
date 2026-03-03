<template>
  <div class="main-layout">
    <header class="header">
      <div class="container">
        <router-link to="/" class="logo">
          <span class="logo-text">AiriPress</span>
        </router-link>
        <nav class="nav">
          <router-link to="/" class="nav-link">首页</router-link>
          <router-link to="/posts" class="nav-link">文章</router-link>
          <template v-if="authStore.isAuthenticated">
            <router-link to="/admin" class="nav-link">管理</router-link>
            <button @click="handleLogout" class="nav-link logout-btn">退出</button>
          </template>
          <router-link v-else to="/login" class="nav-link">登录</router-link>
        </nav>
      </div>
    </header>
    <main class="main">
      <router-view />
    </main>
    <footer class="footer">
      <div class="container">
        <p>&copy; {{ new Date().getFullYear() }} AiriPress. All rights reserved.</p>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const authStore = useAuthStore()

function handleLogout() {
  authStore.logout()
  router.push('/')
}
</script>

<style lang="scss" scoped>
.main-layout {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.header {
  background: white;
  border-bottom: 1px solid #e5e7eb;
  position: sticky;
  top: 0;
  z-index: 40;

  .container {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 64px;
  }
}

.logo {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 1.25rem;
  font-weight: 700;
  color: #1f2937;

  .logo-text {
    background: linear-gradient(135deg, #8b5cf6, #ec4899);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }
}

.nav {
  display: flex;
  align-items: center;
  gap: 1.5rem;
}

.nav-link {
  font-size: 0.875rem;
  font-weight: 500;
  color: #6b7280;
  transition: color 0.2s;

  &:hover,
  &.router-link-active {
    color: #8b5cf6;
  }
}

.logout-btn {
  cursor: pointer;
}

.main {
  flex: 1;
  padding: 2rem 0;
}

.footer {
  background: #1f2937;
  color: #9ca3af;
  padding: 2rem 0;
  text-align: center;
  font-size: 0.875rem;
}
</style>
