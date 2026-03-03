<template>
  <div class="admin-posts">
    <div class="page-header">
      <h1 class="page-title">文章管理</h1>
      <router-link to="/admin/posts/new" class="btn btn-primary">
        + 写新文章
      </router-link>
    </div>

    <!-- 筛选栏 -->
    <div class="filters">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="搜索文章..."
        class="search-input"
        @keyup.enter="fetchPosts"
      />
      <select v-model="statusFilter" @change="fetchPosts" class="filter-select">
        <option value="">全部状态</option>
        <option value="published">已发布</option>
        <option value="draft">草稿</option>
      </select>
    </div>

    <!-- 文章列表 -->
    <div v-if="loading" class="loading">
      <div class="spinner"></div>
    </div>
    <div v-else-if="posts.length === 0" class="empty-state">
      <p>暂无文章</p>
    </div>
    <div v-else class="posts-table-wrapper">
      <table class="posts-table">
        <thead>
          <tr>
            <th>标题</th>
            <th>分类</th>
            <th>状态</th>
            <th>创建时间</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="post in posts" :key="post.id">
            <td class="title-cell">
              <router-link :to="`/posts/${post.id}`" target="_blank" class="post-link">
                {{ post.title }}
              </router-link>
            </td>
            <td>
              <span v-if="post.categories && post.categories.length > 0" class="category-tag">
                {{ post.categories[0].name }}
              </span>
              <span v-else class="text-muted">无分类</span>
            </td>
            <td>
              <span :class="['status-badge', post.published ? 'published' : 'draft']">
                {{ post.published ? '已发布' : '草稿' }}
              </span>
            </td>
            <td class="date-cell">{{ formatDate(post.created_at) }}</td>
            <td class="actions-cell">
              <router-link :to="`/admin/posts/${post.id}/edit`" class="action-btn edit">
                编辑
              </router-link>
              <button @click="confirmDelete(post)" class="action-btn delete">
                删除
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- 分页 -->
    <div v-if="pagination && pagination.total_pages > 1" class="pagination">
      <button
        @click="changePage(pagination.page - 1)"
        :disabled="pagination.page <= 1"
        class="page-btn"
      >
        上一页
      </button>
      <span class="page-info">
        第 {{ pagination.page }} 页 / 共 {{ pagination.total_pages }} 页
      </span>
      <button
        @click="changePage(pagination.page + 1)"
        :disabled="pagination.page >= pagination.total_pages"
        class="page-btn"
      >
        下一页
      </button>
    </div>

    <!-- 删除确认弹窗 -->
    <div v-if="showDeleteModal" class="modal-overlay" @click.self="showDeleteModal = false">
      <div class="modal">
        <h3>确认删除</h3>
        <p>确定要删除文章 "{{ postToDelete?.title }}" 吗？此操作不可撤销。</p>
        <div class="modal-actions">
          <button @click="showDeleteModal = false" class="btn btn-secondary">取消</button>
          <button @click="deletePost" class="btn btn-danger" :disabled="deleting">
            {{ deleting ? '删除中...' : '确认删除' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { postsApi } from '@/api/services'
import type { PostListItem, Pagination } from '@/types'
import dayjs from 'dayjs'

const posts = ref<PostListItem[]>([])
const loading = ref(true)
const searchQuery = ref('')
const statusFilter = ref('')
const pagination = ref<Pagination>({
  page: 1,
  per_page: 15,
  total: 0,
  total_pages: 0
})

const showDeleteModal = ref(false)
const postToDelete = ref<PostListItem | null>(null)
const deleting = ref(false)

function formatDate(date: string) {
  return dayjs(date).format('YYYY-MM-DD HH:mm')
}

async function fetchPosts() {
  loading.value = true
  try {
    const params: Record<string, any> = {
      page: pagination.value.page,
      per_page: pagination.value.per_page
    }
    if (searchQuery.value) params.search = searchQuery.value
    if (statusFilter.value === 'published') params.published = true
    if (statusFilter.value === 'draft') params.published = false

    const response = await postsApi.getAll(params)
    posts.value = response.data.data
    if (response.data.pagination) {
      pagination.value = response.data.pagination
    }
  } catch (error) {
    console.error('Failed to fetch posts:', error)
  } finally {
    loading.value = false
  }
}

function changePage(page: number) {
  pagination.value.page = page
  fetchPosts()
}

function confirmDelete(post: PostListItem) {
  postToDelete.value = post
  showDeleteModal.value = true
}

async function deletePost() {
  if (!postToDelete.value) return

  deleting.value = true
  try {
    await postsApi.delete(postToDelete.value.id)
    posts.value = posts.value.filter(p => p.id !== postToDelete.value?.id)
    showDeleteModal.value = false
    postToDelete.value = null
  } catch (error) {
    console.error('Failed to delete post:', error)
    alert('删除失败，请重试')
  } finally {
    deleting.value = false
  }
}

onMounted(() => {
  fetchPosts()
})
</script>

<style lang="scss" scoped>
.admin-posts {
  max-width: 1200px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.page-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: #1f2937;
}

.filters {
  display: flex;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.search-input {
  flex: 1;
  max-width: 300px;
  padding: 0.5rem 1rem;
  border: 1px solid #d1d5db;
  border-radius: 8px;
  font-size: 0.875rem;

  &:focus {
    outline: none;
    border-color: #8b5cf6;
  }
}

.filter-select {
  padding: 0.5rem 1rem;
  border: 1px solid #d1d5db;
  border-radius: 8px;
  font-size: 0.875rem;
  background: white;

  &:focus {
    outline: none;
    border-color: #8b5cf6;
  }
}

.posts-table-wrapper {
  background: white;
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.posts-table {
  width: 100%;
  border-collapse: collapse;

  th, td {
    padding: 1rem;
    text-align: left;
    border-bottom: 1px solid #e5e7eb;
  }

  th {
    background: #f9fafb;
    font-weight: 500;
    color: #6b7280;
    font-size: 0.875rem;
  }

  tbody tr:hover {
    background: #f9fafb;
  }
}

.title-cell {
  max-width: 300px;
}

.post-link {
  color: #1f2937;
  font-weight: 500;
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;

  &:hover {
    color: #8b5cf6;
  }
}

.category-tag {
  display: inline-block;
  padding: 0.25rem 0.5rem;
  background: #e0e7ff;
  color: #4f46e5;
  border-radius: 4px;
  font-size: 0.75rem;
  font-weight: 500;
}

.text-muted {
  color: #9ca3af;
  font-size: 0.875rem;
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

.date-cell {
  color: #6b7280;
  font-size: 0.875rem;
  white-space: nowrap;
}

.actions-cell {
  white-space: nowrap;
}

.action-btn {
  padding: 0.25rem 0.5rem;
  font-size: 0.75rem;
  font-weight: 500;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  margin-right: 0.5rem;

  &.edit {
    background: #e0e7ff;
    color: #4f46e5;

    &:hover {
      background: #c7d2fe;
    }
  }

  &.delete {
    background: #fee2e2;
    color: #dc2626;

    &:hover {
      background: #fecaca;
    }
  }
}

.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 1rem;
  margin-top: 1.5rem;
}

.page-btn {
  padding: 0.5rem 1rem;
  border: 1px solid #d1d5db;
  background: white;
  border-radius: 8px;
  cursor: pointer;
  font-size: 0.875rem;

  &:hover:not(:disabled) {
    border-color: #8b5cf6;
    color: #8b5cf6;
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

.page-info {
  font-size: 0.875rem;
  color: #6b7280;
}

.empty-state {
  text-align: center;
  padding: 4rem;
  background: white;
  border-radius: 12px;
  color: #6b7280;
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: white;
  border-radius: 12px;
  padding: 2rem;
  max-width: 400px;
  width: 90%;

  h3 {
    font-size: 1.25rem;
    font-weight: 600;
    margin-bottom: 1rem;
    color: #1f2937;
  }

  p {
    color: #6b7280;
    margin-bottom: 1.5rem;
  }
}

.modal-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
}

.btn-danger {
  background: #dc2626;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 8px;
  cursor: pointer;

  &:hover {
    background: #b91c1c;
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}
</style>
