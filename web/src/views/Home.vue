<template>
  <div class="home">
    <section class="hero">
      <div class="container">
        <h1 class="hero-title">
          欢迎来到 <span class="gradient-text">AiriPress</span>
        </h1>
        <p class="hero-subtitle">一个现代化的博客系统，分享你的想法与故事</p>
        <div class="hero-actions">
          <router-link to="/posts" class="btn btn-primary btn-lg">浏览文章</router-link>
          <router-link v-if="!authStore.isAuthenticated" to="/login" class="btn btn-secondary btn-lg">
            登录
          </router-link>
        </div>
      </div>
    </section>

    <section class="latest-posts">
      <div class="container">
        <h2 class="section-title">最新文章</h2>
        <div v-if="loading" class="loading">
          <div class="spinner"></div>
        </div>
        <div v-else-if="posts.length === 0" class="empty-state">
          <p>暂无文章</p>
        </div>
        <div v-else class="posts-grid">
          <article v-for="post in posts" :key="post.id" class="post-card">
            <router-link :to="`/posts/${post.id}`" class="post-link">
              <h3 class="post-title">{{ post.title }}</h3>
              <p class="post-summary">{{ post.summary || '暂无摘要' }}</p>
              <div class="post-meta">
                <span class="post-author">{{ post.author.username }}</span>
                <span class="post-date">{{ formatDate(post.created_at) }}</span>
              </div>
              <div class="post-tags">
                <span v-for="tag in post.tags.slice(0, 3)" :key="tag.id" class="tag">
                  {{ tag.name }}
                </span>
              </div>
            </router-link>
          </article>
        </div>
        <div v-if="posts.length > 0" class="view-all">
          <router-link to="/posts" class="btn btn-secondary">查看全部文章</router-link>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { postsApi } from '@/api/services'
import type { PostListItem } from '@/types'
import dayjs from 'dayjs'

const authStore = useAuthStore()
const posts = ref<PostListItem[]>([])
const loading = ref(true)

function formatDate(date: string) {
  return dayjs(date).format('YYYY年MM月DD日')
}

onMounted(async () => {
  try {
    const response = await postsApi.getAll({ per_page: 6, published: true })
    posts.value = response.data.data
  } catch (error) {
    console.error('Failed to fetch posts:', error)
  } finally {
    loading.value = false
  }
})
</script>

<style lang="scss" scoped>
.hero {
  background: linear-gradient(135deg, #1f2937 0%, #111827 100%);
  color: white;
  padding: 6rem 0;
  text-align: center;
}

.hero-title {
  font-size: 3rem;
  font-weight: 700;
  margin-bottom: 1rem;
}

.gradient-text {
  background: linear-gradient(135deg, #8b5cf6, #ec4899);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.hero-subtitle {
  font-size: 1.25rem;
  color: #9ca3af;
  margin-bottom: 2rem;
}

.hero-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
}

.latest-posts {
  padding: 4rem 0;
}

.section-title {
  font-size: 1.5rem;
  font-weight: 600;
  margin-bottom: 2rem;
  text-align: center;
}

.posts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 1.5rem;
}

.post-card {
  background: white;
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
  overflow: hidden;

  &:hover {
    transform: translateY(-4px);
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
  }
}

.post-link {
  display: block;
  padding: 1.5rem;
  color: inherit;
}

.post-title {
  font-size: 1.125rem;
  font-weight: 600;
  margin-bottom: 0.75rem;
  color: #1f2937;
  line-height: 1.4;
}

.post-summary {
  font-size: 0.875rem;
  color: #6b7280;
  margin-bottom: 1rem;
  line-height: 1.6;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.post-meta {
  display: flex;
  gap: 1rem;
  font-size: 0.75rem;
  color: #9ca3af;
  margin-bottom: 0.75rem;
}

.post-tags {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.empty-state {
  text-align: center;
  padding: 3rem;
  color: #6b7280;
}

.view-all {
  text-align: center;
  margin-top: 2rem;
}
</style>
