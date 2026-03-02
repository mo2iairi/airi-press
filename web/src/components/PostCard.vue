<template>
  <router-link :to="`/posts/${post.id}`" class="post-card glass-card">
    <div class="post-card-body">
      <div class="post-meta">
        <span v-if="post.category" class="post-category">{{ post.category.name }}</span>
        <span class="post-date">{{ formatDate(post.created_at) }}</span>
      </div>
      <h2 class="post-title">{{ post.title }}</h2>
      <p class="post-summary">{{ post.summary }}</p>
      <div class="post-footer">
        <div class="post-tags">
          <span v-for="tag in post.tags" :key="tag.id" class="tag">{{ tag.name }}</span>
        </div>
        <span class="post-author">{{ post.author_name }}</span>
      </div>
    </div>
  </router-link>
</template>

<script setup lang="ts">
import type { Post } from '@/types'

defineProps<{
  post: Post
}>()

function formatDate(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}
</script>

<style scoped>
.post-card {
  display: block;
  padding: 1.5rem;
  color: var(--text-primary);
  cursor: pointer;
}

.post-meta {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 0.75rem;
}

.post-category {
  color: var(--accent);
  font-size: 0.8rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.post-date {
  color: var(--text-muted);
  font-size: 0.8rem;
}

.post-title {
  font-size: 1.3rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
  line-height: 1.4;
}

.post-summary {
  color: var(--text-secondary);
  font-size: 0.9rem;
  line-height: 1.6;
  margin-bottom: 1rem;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.post-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.post-tags {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.post-author {
  color: var(--text-muted);
  font-size: 0.8rem;
}
</style>
