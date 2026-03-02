<template>
  <div class="admin-layout">
    <aside class="admin-sidebar glass-card">
      <h2 class="sidebar-title">管理面板</h2>
      <nav class="sidebar-nav">
        <router-link to="/admin" class="sidebar-link" exact-active-class="active">
          📊 仪表盘
        </router-link>
        <router-link to="/admin/posts" class="sidebar-link" active-class="active">
          📝 文章管理
        </router-link>
        <router-link to="/admin/categories" class="sidebar-link" active-class="active">
          📁 分类管理
        </router-link>
        <router-link to="/admin/tags" class="sidebar-link" active-class="active">
          🏷️ 标签管理
        </router-link>
        <router-link
          v-if="auth.isAdmin"
          to="/admin/users"
          class="sidebar-link"
          active-class="active"
        >
          👥 用户管理
        </router-link>
      </nav>
    </aside>

    <div class="admin-content">
      <router-view />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'

const auth = useAuthStore()
</script>

<style scoped>
.admin-layout {
  display: flex;
  gap: 1.5rem;
  min-height: 70vh;
}

.admin-sidebar {
  width: 240px;
  flex-shrink: 0;
  padding: 1.5rem;
  height: fit-content;
  position: sticky;
  top: 80px;
}

.admin-sidebar:hover {
  transform: none;
}

.sidebar-title {
  font-size: 1.1rem;
  font-weight: 600;
  margin-bottom: 1.5rem;
  color: var(--text-primary);
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.sidebar-link {
  display: block;
  padding: 0.625rem 0.75rem;
  border-radius: 8px;
  color: var(--text-secondary);
  font-size: 0.9rem;
  transition: var(--transition);
}

.sidebar-link:hover {
  background: var(--bg-glass-hover);
  color: var(--text-primary);
}

.sidebar-link.active {
  background: rgba(124, 91, 245, 0.15);
  color: var(--accent);
  font-weight: 500;
}

.admin-content {
  flex: 1;
  min-width: 0;
}

@media (max-width: 768px) {
  .admin-layout {
    flex-direction: column;
  }

  .admin-sidebar {
    width: 100%;
    position: static;
  }

  .sidebar-nav {
    flex-direction: row;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
}
</style>
