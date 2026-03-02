<template>
  <div class="posts-manage">
    <div class="page-header flex justify-between items-center mb-3">
      <h1 class="page-title">文章管理</h1>
      <router-link to="/admin/posts/new" class="btn btn-primary">+ 新建文章</router-link>
    </div>

    <div v-if="postStore.loading" class="spinner"></div>

    <div v-else-if="postStore.posts.length === 0" class="empty-state glass-card">
      <p>暂无文章，点击上方按钮创建第一篇文章吧</p>
    </div>

    <div v-else class="posts-list">
      <div v-for="post in postStore.posts" :key="post.id" class="post-item glass-card">
        <div class="post-info">
          <h3 class="post-title">{{ post.title }}</h3>
          <div class="post-meta">
            <span :class="['status', post.published ? 'published' : 'draft']">
              {{ post.published ? '已发布' : '草稿' }}
            </span>
            <span class="text-muted">{{ formatDate(post.created_at) }}</span>
          </div>
        </div>
        <div class="post-actions">
          <router-link :to="`/admin/posts/${post.id}/edit`" class="btn btn-sm">编辑</router-link>
          <button class="btn btn-sm btn-danger" @click="handleDelete(post.id)">删除</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { usePostStore } from '@/stores/posts'

const postStore = usePostStore()

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleDateString('zh-CN')
}

async function handleDelete(id: string) {
  if (!confirm('确定要删除这篇文章吗？')) return
  await postStore.deletePost(id)
}

onMounted(() => {
  postStore.fetchPosts(true)
})
</script>

<style scoped>
.page-title {
  font-size: 1.5rem;
  font-weight: 700;
}

.posts-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.post-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.25rem;
}

.post-item:hover {
  transform: none;
}

.post-title {
  font-size: 1rem;
  font-weight: 600;
  margin-bottom: 0.25rem;
}

.post-meta {
  display: flex;
  gap: 0.75rem;
  align-items: center;
  font-size: 0.8rem;
}

.status {
  padding: 0.15rem 0.5rem;
  border-radius: 6px;
  font-size: 0.75rem;
  font-weight: 500;
}

.status.published {
  background: rgba(91, 245, 160, 0.12);
  color: var(--success);
}

.status.draft {
  background: rgba(245, 200, 91, 0.12);
  color: var(--warning);
}

.post-actions {
  display: flex;
  gap: 0.5rem;
}

.empty-state {
  text-align: center;
  padding: 3rem;
  color: var(--text-muted);
}

.empty-state:hover {
  transform: none;
}
</style>
