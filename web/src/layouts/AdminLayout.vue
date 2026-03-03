<template>
  <div class="admin-layout">
    <aside class="sidebar">
      <div class="sidebar-header">
        <router-link to="/" class="logo">
          <span class="logo-text">AiriPress</span>
        </router-link>
      </div>
      <nav class="sidebar-nav">
        <router-link to="/admin" class="nav-item" exact>
          <span class="nav-icon">📊</span>
          <span>仪表盘</span>
        </router-link>
        <router-link to="/admin/posts" class="nav-item">
          <span class="nav-icon">📝</span>
          <span>文章管理</span>
        </router-link>
        <router-link to="/admin/categories" class="nav-item">
          <span class="nav-icon">📁</span>
          <span>分类管理</span>
        </router-link>
        <router-link to="/admin/tags" class="nav-item">
          <span class="nav-icon">🏷️</span>
          <span>标签管理</span>
        </router-link>
        <router-link to="/admin/images" class="nav-item">
          <span class="nav-icon">🖼️</span>
          <span>图片管理</span>
        </router-link>
        <router-link v-if="authStore.isAdmin" to="/admin/users" class="nav-item">
          <span class="nav-icon">👥</span>
          <span>用户管理</span>
        </router-link>
      </nav>
      <div class="sidebar-footer">
        <div class="user-info">
          <span class="user-avatar">{{ authStore.user?.username.charAt(0).toUpperCase() }}</span>
          <div class="user-details">
            <span class="user-name">{{ authStore.user?.username }}</span>
            <span class="user-role">{{ getPermissionLabel(authStore.user?.permission ?? 0) }}</span>
          </div>
        </div>
        <button @click="handleLogout" class="logout-btn">退出</button>
      </div>
    </aside>
    <main class="main-content">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { getPermissionLabel } from '@/types'

const router = useRouter()
const authStore = useAuthStore()

function handleLogout() {
  authStore.logout()
  router.push('/login')
}
</script>

<style lang="scss" scoped>
.admin-layout {
  display: flex;
  min-height: 100vh;
}

.sidebar {
  width: 240px;
  background: linear-gradient(180deg, #1f2937 0%, #111827 100%);
  color: white;
  display: flex;
  flex-direction: column;
  position: fixed;
  top: 0;
  left: 0;
  bottom: 0;
}

.sidebar-header {
  padding: 1.5rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.logo {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.logo-text {
  font-size: 1.25rem;
  font-weight: 700;
  background: linear-gradient(135deg, #8b5cf6, #ec4899);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.sidebar-nav {
  flex: 1;
  padding: 1rem 0;
  overflow-y: auto;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1.5rem;
  color: #9ca3af;
  font-size: 0.875rem;
  transition: all 0.2s;

  &:hover {
    background: rgba(255, 255, 255, 0.05);
    color: white;
  }

  &.router-link-active {
    background: rgba(139, 92, 246, 0.2);
    color: #a78bfa;
    border-right: 3px solid #8b5cf6;
  }
}

.nav-icon {
  font-size: 1rem;
}

.sidebar-footer {
  padding: 1rem 1.5rem;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.user-info {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 1rem;
}

.user-avatar {
  width: 36px;
  height: 36px;
  background: linear-gradient(135deg, #8b5cf6, #ec4899);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 0.875rem;
}

.user-details {
  display: flex;
  flex-direction: column;
}

.user-name {
  font-size: 0.875rem;
  font-weight: 500;
}

.user-role {
  font-size: 0.75rem;
  color: #9ca3af;
}

.logout-btn {
  width: 100%;
  padding: 0.5rem;
  background: rgba(255, 255, 255, 0.1);
  color: #9ca3af;
  border-radius: 6px;
  font-size: 0.875rem;
  transition: all 0.2s;

  &:hover {
    background: rgba(239, 68, 68, 0.2);
    color: #f87171;
  }
}

.main-content {
  flex: 1;
  margin-left: 240px;
  padding: 2rem;
  background: #f9fafb;
  min-height: 100vh;
}
</style>
