<template>
  <div class="post-detail-page">
    <div v-if="loading" class="loading-container">
      <div class="spinner"></div>
    </div>
    <template v-else-if="post">
      <article class="post-article">
        <div class="container">
          <header class="post-header">
            <div class="post-meta-top">
              <span v-if="post.category" class="category-badge">
                {{ post.category.name }}
              </span>
              <span class="date">{{ formatDate(post.created_at) }}</span>
            </div>
            <h1 class="post-title">{{ post.title }}</h1>
            <div class="post-author">
              <div class="author-info">
                <span class="author-name">{{ post.author.username }}</span>
                <span class="reading-time">阅读时间约 {{ readingTime }} 分钟</span>
              </div>
            </div>
            <div class="post-tags" v-if="post.tags.length > 0">
              <span v-for="tag in post.tags" :key="tag.id" class="tag">
                #{{ tag.name }}
              </span>
            </div>
          </header>

          <div class="post-content" v-html="post.content"></div>

          <footer class="post-footer">
            <div class="post-actions">
              <router-link to="/posts" class="btn btn-secondary">
                ← 返回文章列表
              </router-link>
            </div>
          </footer>
        </div>
      </article>

      <!-- 评论区 -->
      <section class="comments-section">
        <div class="container">
          <h2 class="comments-title">评论 ({{ comments.length }})</h2>

          <!-- 评论表单 -->
          <div v-if="authStore.isAuthenticated" class="comment-form">
            <textarea
              v-model="newComment"
              placeholder="写下你的评论..."
              class="comment-input"
              rows="3"
            ></textarea>
            <button @click="submitComment" class="btn btn-primary" :disabled="submitting">
              {{ submitting ? '提交中...' : '发表评论' }}
            </button>
          </div>
          <div v-else class="login-prompt">
            <p>
              <router-link :to="`/login?redirect=/posts/${post.id}`">登录</router-link>
              后参与评论
            </p>
          </div>

          <!-- 评论列表 -->
          <div v-if="comments.length === 0" class="no-comments">
            <p>暂无评论，来写第一条吧！</p>
          </div>
          <div v-else class="comments-list">
            <div v-for="comment in comments" :key="comment.id" class="comment-item">
              <div class="comment-header">
                <span class="comment-author">{{ comment.author.username }}</span>
                <span class="comment-date">{{ formatDate(comment.created_at) }}</span>
              </div>
              <p class="comment-content">{{ comment.content }}</p>
            </div>
          </div>
        </div>
      </section>
    </template>
    <div v-else class="error-state">
      <div class="container">
        <h2>文章不存在</h2>
        <p>您访问的文章可能已被删除或不存在</p>
        <router-link to="/posts" class="btn btn-primary">返回文章列表</router-link>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { postsApi, commentsApi } from '@/api/services'
import type { Post, Comment } from '@/types'
import dayjs from 'dayjs'

const route = useRoute()
const authStore = useAuthStore()

const post = ref<Post | null>(null)
const comments = ref<Comment[]>([])
const loading = ref(true)
const newComment = ref('')
const submitting = ref(false)

const readingTime = computed(() => {
  if (!post.value?.content) return 1
  const words = post.value.content.replace(/<[^>]+>/g, '').length
  return Math.max(1, Math.ceil(words / 500))
})

function formatDate(date: string) {
  return dayjs(date).format('YYYY年MM月DD日 HH:mm')
}

async function fetchPost() {
  try {
    const postId = Number(route.params.id)
    const response = await postsApi.getById(postId)
    post.value = response.data
  } catch (error) {
    console.error('Failed to fetch post:', error)
    post.value = null
  }
}

async function fetchComments() {
  try {
    const postId = Number(route.params.id)
    const response = await commentsApi.getByPost(postId)
    comments.value = response.data
  } catch (error) {
    console.error('Failed to fetch comments:', error)
  }
}

async function submitComment() {
  if (!newComment.value.trim()) return

  submitting.value = true
  try {
    const postId = Number(route.params.id)
    const response = await commentsApi.create(postId, newComment.value.trim())
    comments.value.unshift(response.data)
    newComment.value = ''
  } catch (error) {
    console.error('Failed to submit comment:', error)
    alert('评论提交失败，请重试')
  } finally {
    submitting.value = false
  }
}

onMounted(async () => {
  await fetchPost()
  if (post.value) {
    await fetchComments()
  }
  loading.value = false
})
</script>

<style lang="scss" scoped>
.post-detail-page {
  padding: 2rem 0;
}

.loading-container {
  display: flex;
  justify-content: center;
  padding: 4rem;
}

.post-article {
  background: white;
  border-radius: 16px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.post-header {
  padding: 2.5rem 2.5rem 2rem;
  border-bottom: 1px solid #e5e7eb;
}

.post-meta-top {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1rem;
}

.category-badge {
  display: inline-block;
  padding: 0.25rem 0.75rem;
  background: linear-gradient(135deg, #8b5cf6, #ec4899);
  color: white;
  border-radius: 4px;
  font-size: 0.75rem;
  font-weight: 500;
}

.date {
  font-size: 0.875rem;
  color: #9ca3af;
}

.post-title {
  font-size: 2rem;
  font-weight: 700;
  color: #1f2937;
  line-height: 1.3;
  margin-bottom: 1rem;
}

.post-author {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1rem;
}

.author-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.author-name {
  font-weight: 500;
  color: #374151;
}

.reading-time {
  font-size: 0.75rem;
  color: #9ca3af;
}

.post-tags {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.post-tags .tag {
  color: #8b5cf6;
  font-size: 0.875rem;
}

.post-content {
  padding: 2.5rem;
  font-size: 1rem;
  line-height: 1.8;
  color: #374151;

  :deep(h1), :deep(h2), :deep(h3), :deep(h4), :deep(h5), :deep(h6) {
    margin-top: 2rem;
    margin-bottom: 1rem;
    color: #1f2937;
    font-weight: 600;
  }

  :deep(h2) {
    font-size: 1.5rem;
  }

  :deep(h3) {
    font-size: 1.25rem;
  }

  :deep(p) {
    margin-bottom: 1rem;
  }

  :deep(a) {
    color: #8b5cf6;
    text-decoration: underline;
  }

  :deep(code) {
    background: #f3f4f6;
    padding: 0.2rem 0.4rem;
    border-radius: 4px;
    font-size: 0.875em;
  }

  :deep(pre) {
    background: #1f2937;
    color: #e5e7eb;
    padding: 1rem;
    border-radius: 8px;
    overflow-x: auto;
    margin: 1rem 0;

    code {
      background: none;
      padding: 0;
    }
  }

  :deep(blockquote) {
    border-left: 4px solid #8b5cf6;
    padding-left: 1rem;
    margin: 1rem 0;
    color: #6b7280;
    font-style: italic;
  }

  :deep(img) {
    max-width: 100%;
    height: auto;
    border-radius: 8px;
  }

  :deep(ul), :deep(ol) {
    margin: 1rem 0;
    padding-left: 1.5rem;
  }

  :deep(li) {
    margin-bottom: 0.5rem;
  }
}

.post-footer {
  padding: 1.5rem 2.5rem;
  border-top: 1px solid #e5e7eb;
}

.post-actions {
  display: flex;
  gap: 1rem;
}

.comments-section {
  margin-top: 2rem;
  background: white;
  border-radius: 16px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  padding: 2rem;
}

.comments-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: #1f2937;
  margin-bottom: 1.5rem;
}

.comment-form {
  margin-bottom: 2rem;
}

.comment-input {
  width: 100%;
  padding: 1rem;
  border: 1px solid #d1d5db;
  border-radius: 8px;
  font-size: 0.875rem;
  resize: vertical;
  margin-bottom: 1rem;

  &:focus {
    outline: none;
    border-color: #8b5cf6;
  }
}

.login-prompt {
  padding: 1.5rem;
  background: #f9fafb;
  border-radius: 8px;
  text-align: center;
  margin-bottom: 2rem;

  a {
    color: #8b5cf6;
    font-weight: 500;
  }
}

.no-comments {
  text-align: center;
  padding: 2rem;
  color: #6b7280;
}

.comments-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.comment-item {
  padding: 1rem;
  background: #f9fafb;
  border-radius: 8px;
}

.comment-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 0.5rem;
}

.comment-author {
  font-weight: 500;
  color: #374151;
}

.comment-date {
  font-size: 0.75rem;
  color: #9ca3af;
}

.comment-content {
  color: #4b5563;
  line-height: 1.6;
}

.error-state {
  text-align: center;
  padding: 4rem;
  background: white;
  border-radius: 16px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);

  h2 {
    font-size: 1.5rem;
    color: #1f2937;
    margin-bottom: 0.5rem;
  }

  p {
    color: #6b7280;
    margin-bottom: 1.5rem;
  }
}
</style>
