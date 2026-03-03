<template>
  <div class="dashboard">
    <h1 class="page-title">仪表盘</h1>
    <p class="welcome-text">欢迎回来，{{ authStore.user?.username }}！</p>

    <!-- 统计卡片 -->
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-icon posts">
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/>
            <polyline points="14 2 14 8 20 8"/>
            <line x1="16" y1="13" x2="8" y2="13"/>
            <line x1="16" y1="17" x2="8" y2="17"/>
            <line x1="10" y1="9" x2="8" y2="9"/>
          </svg>
        </div>
        <div class="stat-info">
          <span class="stat-value">{{ stats.posts }}</span>
          <span class="stat-label">文章总数</span>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon categories">
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
          </svg>
        </div>
        <div class="stat-info">
          <span class="stat-value">{{ stats.categories }}</span>
          <span class="stat-label">分类数量</span>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon tags">
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"/>
            <line x1="7" y1="7" x2="7.01" y2="7"/>
          </svg>
        </div>
        <div class="stat-info">
          <span class="stat-value">{{ stats.tags }}</span>
          <span class="stat-label">标签数量</span>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon comments">
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
          </svg>
        </div>
        <div class="stat-info">
          <span class="stat-value">{{ stats.comments }}</span>
          <span class="stat-label">评论数量</span>
        </div>
      </div>
    </div>

    <!-- 快捷操作 -->
    <div class="quick-actions">
      <h2 class="section-title">快捷操作</h2>
      <div class="actions-grid">
        <router-link to="/admin/posts/new" class="action-card">
          <div class="action-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="12" y1="5" x2="12" y2="19"/>
              <line x1="5" y1="12" x2="19" y2="12"/>
            </svg>
          </div>
          <span class="action-text">写新文章</span>
        </router-link>

        <router-link to="/admin/categories" class="action-card">
          <div class="action-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
            </svg>
          </div>
          <span class="action-text">管理分类</span>
        </router-link>

        <router-link to="/admin/tags" class="action-card">
          <div class="action-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"/>
              <line x1="7" y1="7" x2="7.01" y2="7"/>
            </svg>
          </div>
          <span class="action-text">管理标签</span>
        </router-link>

        <router-link to="/admin/images" class="action-card">
          <div class="action-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
              <circle cx="8.5" cy="8.5" r="1.5"/>
              <polyline points="21 15 16 10 5 21"/>
            </svg>
          </div>
          <span class="action-text">图片管理</span>
        </router-link>
      </div>
    </div>

    <!-- 最近文章 -->
    <div class="recent-posts">
      <h2 class="section-title">最近文章</h2>
      <div v-if="loading" class="loading">
        <div class="spinner"></div>
      </div>
      <div v-else-if="recentPosts.length === 0" class="empty-state">
        <p>暂无文章</p>
        <router-link to="/admin/posts/new" class="btn btn-primary btn-sm">
          写第一篇文章
        </router-link>
      </div>
      <div v-else class="posts-table">
        <table>
          <thead>
            <tr>
              <th>标题</th>
              <th>状态</th>
              <th>创建时间</th>
              <th>操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="post in recentPosts" :key="post.id">
              <td class="post-title">{{ post.title }}</td>
              <td>
                <span :class="['status-badge', post.published ? 'published' : 'draft']">
                  {{ post.published ? '已发布' : '草稿' }}
                </span>
              </td>
              <td class="date">{{ formatDate(post.created_at) }}</td>
              <td>
                <router-link :to="`/admin/posts/${post.id}/edit`" class="action-link">
                  编辑
                </router-link>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { postsApi, categoriesApi, tagsApi } from '@/api/services'
import type { PostListItem } from '@/types'
import dayjs from 'dayjs'

const authStore = useAuthStore()
const loading = ref(true)
const recentPosts = ref<PostListItem[]>([])
const stats = reactive({
  posts: 0,
  categories: 0,
  tags: 0,
  comments: 0
})

function formatDate(date: string) {
  return dayjs(date).format('YYYY-MM-DD')
}

async function fetchStats() {
  try {
    const [postsRes, catsRes, tagsRes] = await Promise.all([
      postsApi.getAll({ per_page: 1 }),
      categoriesApi.getAll(),
      tagsApi.getAll()
    ])

    stats.posts = postsRes.data.pagination?.total || 0
    stats.categories = catsRes.data.length
    stats.tags = tagsRes.data.length
    // 评论统计暂时不可用，因为没有全局获取评论的 API
    stats.comments = 0
  } catch (error) {
    console.error('Failed to fetch stats:', error)
  }
}

async function fetchRecentPosts() {
  try {
    const response = await postsApi.getAll({ per_page: 5 })
    recentPosts.value = response.data.data
  } catch (error) {
    console.error('Failed to fetch recent posts:', error)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchStats()
  fetchRecentPosts()
})
</script>

<style lang="scss" scoped>
.dashboard {
  max-width: 1200px;
}

.page-title {
  font-size: 1.75rem;
  font-weight: 700;
  color: #1f2937;
  margin-bottom: 0.5rem;
}

.welcome-text {
  color: #6b7280;
  margin-bottom: 2rem;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.stat-card {
  background: white;
  border-radius: 12px;
  padding: 1.5rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;

  &.posts {
    background: linear-gradient(135deg, #8b5cf6, #7c3aed);
  }

  &.categories {
    background: linear-gradient(135deg, #06b6d4, #0891b2);
  }

  &.tags {
    background: linear-gradient(135deg, #f59e0b, #d97706);
  }

  &.comments {
    background: linear-gradient(135deg, #10b981, #059669);
  }
}

.stat-info {
  display: flex;
  flex-direction: column;
}

.stat-value {
  font-size: 1.5rem;
  font-weight: 700;
  color: #1f2937;
}

.stat-label {
  font-size: 0.875rem;
  color: #6b7280;
}

.section-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: #1f2937;
  margin-bottom: 1rem;
}

.quick-actions {
  margin-bottom: 2rem;
}

.actions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 1rem;
}

.action-card {
  background: white;
  border-radius: 12px;
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.75rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: all 0.2s;
  color: #374151;

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    color: #8b5cf6;
  }
}

.action-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  background: #f3f4f6;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.2s;

  .action-card:hover & {
    background: linear-gradient(135deg, #8b5cf6, #ec4899);
    color: white;
  }
}

.action-text {
  font-weight: 500;
}

.recent-posts {
  background: white;
  border-radius: 12px;
  padding: 1.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.posts-table {
  overflow-x: auto;

  table {
    width: 100%;
    border-collapse: collapse;
  }

  th, td {
    padding: 0.75rem 1rem;
    text-align: left;
    border-bottom: 1px solid #e5e7eb;
  }

  th {
    font-weight: 500;
    color: #6b7280;
    font-size: 0.875rem;
  }

  td {
    color: #374151;
  }

  .post-title {
    font-weight: 500;
    max-width: 300px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .date {
    color: #9ca3af;
    font-size: 0.875rem;
  }
}

.status-badge {
  display: inline-block;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.75rem;
  font-weight: 500;

  &.published {
    background: #d1fae5;
    color: #059669;
  }

  &.draft {
    background: #fef3c7;
    color: #d97706;
  }
}

.action-link {
  color: #8b5cf6;
  font-size: 0.875rem;
  font-weight: 500;

  &:hover {
    text-decoration: underline;
  }
}

.empty-state {
  text-align: center;
  padding: 2rem;
  color: #6b7280;

  .btn {
    margin-top: 1rem;
  }
}
</style>
