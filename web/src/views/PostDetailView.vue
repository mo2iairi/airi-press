<template>
  <div class="post-detail">
    <div v-if="postStore.loading" class="spinner"></div>

    <template v-else-if="postStore.currentPost">
      <article class="article glass-card">
        <header class="article-header">
          <div class="article-meta">
            <span v-if="postStore.currentPost.category" class="post-category">
              {{ postStore.currentPost.category.name }}
            </span>
            <span class="post-date">{{ formatDate(postStore.currentPost.created_at) }}</span>
            <span class="post-author">by {{ postStore.currentPost.author_name }}</span>
          </div>
          <h1 class="article-title">{{ postStore.currentPost.title }}</h1>
          <p class="article-summary">{{ postStore.currentPost.summary }}</p>
          <div class="article-tags">
            <span v-for="tag in postStore.currentPost.tags" :key="tag.id" class="tag">
              {{ tag.name }}
            </span>
          </div>
        </header>

        <div class="article-content markdown-body" v-html="renderedContent"></div>
      </article>

      <div class="back-link mt-3">
        <router-link to="/" class="btn">← 返回首页</router-link>
      </div>
    </template>

    <div v-else class="empty-state glass-card">
      <p>文章不存在</p>
      <router-link to="/" class="btn mt-2">返回首页</router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { usePostStore } from '@/stores/posts'
import { marked } from 'marked'

const route = useRoute()
const postStore = usePostStore()

const renderedContent = computed(() => {
  if (!postStore.currentPost?.content) return ''
  return marked(postStore.currentPost.content) as string
})

function formatDate(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

onMounted(() => {
  const id = route.params.id as string
  postStore.fetchPost(id)
})
</script>

<style scoped>
.article {
  padding: 2.5rem;
}

.article:hover {
  transform: none;
}

.article-header {
  margin-bottom: 2rem;
  padding-bottom: 2rem;
  border-bottom: 1px solid var(--border-glass);
}

.article-meta {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1rem;
}

.post-category {
  color: var(--accent);
  font-size: 0.8rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.post-date,
.post-author {
  color: var(--text-muted);
  font-size: 0.85rem;
}

.article-title {
  font-size: 2rem;
  font-weight: 700;
  line-height: 1.3;
  margin-bottom: 0.75rem;
}

.article-summary {
  color: var(--text-secondary);
  font-size: 1.05rem;
  line-height: 1.6;
  margin-bottom: 1rem;
}

.article-tags {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.empty-state {
  text-align: center;
  padding: 3rem;
  color: var(--text-muted);
}

.empty-state:hover {
  transform: none;
}

@media (max-width: 768px) {
  .article {
    padding: 1.5rem;
  }

  .article-title {
    font-size: 1.5rem;
  }
}
</style>
