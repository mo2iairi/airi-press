<template>
  <div class="dashboard">
    <h1 class="page-title">仪表盘</h1>

    <div class="stats-grid">
      <div class="stat-card glass-card">
        <div class="stat-value">{{ postStore.posts.length }}</div>
        <div class="stat-label">文章数量</div>
      </div>
      <div class="stat-card glass-card">
        <div class="stat-value">{{ categories.length }}</div>
        <div class="stat-label">分类数量</div>
      </div>
      <div class="stat-card glass-card">
        <div class="stat-value">{{ tags.length }}</div>
        <div class="stat-label">标签数量</div>
      </div>
    </div>

    <div class="quick-actions mt-4">
      <h2 class="section-title">快捷操作</h2>
      <div class="actions-grid">
        <router-link to="/admin/posts/new" class="action-card glass-card">
          <span class="action-icon">✍️</span>
          <span class="action-text">写新文章</span>
        </router-link>
        <router-link to="/admin/categories" class="action-card glass-card">
          <span class="action-icon">📁</span>
          <span class="action-text">管理分类</span>
        </router-link>
        <router-link to="/admin/tags" class="action-card glass-card">
          <span class="action-icon">🏷️</span>
          <span class="action-text">管理标签</span>
        </router-link>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { usePostStore } from '@/stores/posts'
import api from '@/api'
import type { Category, Tag, ApiResponse } from '@/types'

const postStore = usePostStore()
const categories = ref<Category[]>([])
const tags = ref<Tag[]>([])

onMounted(async () => {
  postStore.fetchPosts()

  const [catRes, tagRes] = await Promise.all([
    api.get<ApiResponse<Category[]>>('/categories'),
    api.get<ApiResponse<Tag[]>>('/tags'),
  ])

  categories.value = catRes.data.data || []
  tags.value = tagRes.data.data || []
})
</script>

<style scoped>
.page-title {
  font-size: 1.5rem;
  font-weight: 700;
  margin-bottom: 1.5rem;
}

.section-title {
  font-size: 1.1rem;
  font-weight: 600;
  margin-bottom: 1rem;
  color: var(--text-secondary);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 1rem;
}

.stat-card {
  padding: 1.5rem;
  text-align: center;
}

.stat-card:hover {
  transform: none;
}

.stat-value {
  font-size: 2rem;
  font-weight: 700;
  color: var(--accent);
}

.stat-label {
  color: var(--text-secondary);
  font-size: 0.85rem;
  margin-top: 0.25rem;
}

.actions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 1rem;
}

.action-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.75rem;
  padding: 1.5rem;
  color: var(--text-primary);
  cursor: pointer;
}

.action-icon {
  font-size: 2rem;
}

.action-text {
  font-size: 0.9rem;
  font-weight: 500;
}
</style>
