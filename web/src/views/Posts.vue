<template>
  <div class="posts-page">
    <div class="container">
      <header class="page-header">
        <h1 class="page-title">所有文章</h1>
        <p class="page-description">探索我们的文章库，发现更多精彩内容</p>
      </header>

      <!-- 筛选栏 -->
      <div class="filters">
        <div class="search-box">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索文章..."
            class="search-input"
            @keyup.enter="handleSearch"
          />
          <button @click="handleSearch" class="search-btn">搜索</button>
        </div>
        <div class="filter-tags">
          <select v-model="selectedCategory" @change="handleFilter" class="filter-select">
            <option value="">所有分类</option>
            <option v-for="cat in categories" :key="cat.id" :value="cat.id">
              {{ cat.name }}
            </option>
          </select>
          <select v-model="selectedTag" @change="handleFilter" class="filter-select">
            <option value="">所有标签</option>
            <option v-for="tag in tags" :key="tag.id" :value="tag.id">
              {{ tag.name }}
            </option>
          </select>
        </div>
      </div>

      <!-- 文章列表 -->
      <div v-if="loading" class="loading">
        <div class="spinner"></div>
      </div>
      <div v-else-if="posts.length === 0" class="empty-state">
        <p>没有找到相关文章</p>
      </div>
      <div v-else class="posts-list">
        <article v-for="post in posts" :key="post.id" class="post-item">
          <router-link :to="`/posts/${post.id}`" class="post-link">
            <div class="post-content">
              <div class="post-category" v-if="post.category">
                {{ post.category.name }}
              </div>
              <h2 class="post-title">{{ post.title }}</h2>
              <p class="post-summary">{{ post.summary || '暂无摘要' }}</p>
              <div class="post-footer">
                <div class="post-meta">
                  <span class="author">{{ post.author.username }}</span>
                  <span class="date">{{ formatDate(post.created_at) }}</span>
                </div>
                <div class="post-tags">
                  <span v-for="tag in post.tags.slice(0, 3)" :key="tag.id" class="tag">
                    {{ tag.name }}
                  </span>
                </div>
              </div>
            </div>
          </router-link>
        </article>
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
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { postsApi, categoriesApi, tagsApi } from '@/api/services'
import type { PostListItem, Category, Tag, Pagination } from '@/types'
import dayjs from 'dayjs'

const route = useRoute()
const router = useRouter()

const posts = ref<PostListItem[]>([])
const categories = ref<Category[]>([])
const tags = ref<Tag[]>([])
const loading = ref(true)

const searchQuery = ref('')
const selectedCategory = ref('')
const selectedTag = ref('')
const pagination = ref<Pagination>({
  page: 1,
  per_page: 10,
  total: 0,
  total_pages: 0
})

function formatDate(date: string) {
  return dayjs(date).format('YYYY年MM月DD日')
}

async function fetchPosts() {
  loading.value = true
  try {
    const params: Record<string, any> = {
      page: pagination.value.page,
      per_page: pagination.value.per_page,
      published: true
    }
    if (searchQuery.value) params.search = searchQuery.value
    if (selectedCategory.value) params.category_id = selectedCategory.value
    if (selectedTag.value) params.tag_id = selectedTag.value

    const response = await postsApi.getAll(params)
    posts.value = response.data.data
    pagination.value = response.data.pagination
  } catch (error) {
    console.error('Failed to fetch posts:', error)
  } finally {
    loading.value = false
  }
}

async function fetchFilters() {
  try {
    const [catRes, tagRes] = await Promise.all([
      categoriesApi.getAll(),
      tagsApi.getAll()
    ])
    categories.value = catRes.data
    tags.value = tagRes.data
  } catch (error) {
    console.error('Failed to fetch filters:', error)
  }
}

function handleSearch() {
  pagination.value.page = 1
  updateUrl()
  fetchPosts()
}

function handleFilter() {
  pagination.value.page = 1
  updateUrl()
  fetchPosts()
}

function changePage(page: number) {
  pagination.value.page = page
  updateUrl()
  fetchPosts()
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

function updateUrl() {
  const query: Record<string, string> = {}
  if (pagination.value.page > 1) query.page = String(pagination.value.page)
  if (searchQuery.value) query.q = searchQuery.value
  if (selectedCategory.value) query.category = selectedCategory.value
  if (selectedTag.value) query.tag = selectedTag.value
  router.replace({ query })
}

function loadFromUrl() {
  pagination.value.page = Number(route.query.page) || 1
  searchQuery.value = (route.query.q as string) || ''
  selectedCategory.value = (route.query.category as string) || ''
  selectedTag.value = (route.query.tag as string) || ''
}

onMounted(() => {
  loadFromUrl()
  fetchFilters()
  fetchPosts()
})

watch(() => route.query, () => {
  loadFromUrl()
  fetchPosts()
})
</script>

<style lang="scss" scoped>
.posts-page {
  padding: 2rem 0;
}

.page-header {
  text-align: center;
  margin-bottom: 2rem;
}

.page-title {
  font-size: 2rem;
  font-weight: 700;
  color: #1f2937;
  margin-bottom: 0.5rem;
}

.page-description {
  color: #6b7280;
}

.filters {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
  margin-bottom: 2rem;
  padding: 1.5rem;
  background: white;
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.search-box {
  display: flex;
  flex: 1;
  min-width: 250px;
  gap: 0.5rem;
}

.search-input {
  flex: 1;
  padding: 0.75rem 1rem;
  border: 1px solid #d1d5db;
  border-radius: 8px;
  font-size: 0.875rem;

  &:focus {
    outline: none;
    border-color: #8b5cf6;
  }
}

.search-btn {
  padding: 0.75rem 1.5rem;
  background: linear-gradient(135deg, #8b5cf6, #7c3aed);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 500;
  transition: opacity 0.2s;

  &:hover {
    opacity: 0.9;
  }
}

.filter-tags {
  display: flex;
  gap: 0.5rem;
}

.filter-select {
  padding: 0.75rem 1rem;
  border: 1px solid #d1d5db;
  border-radius: 8px;
  font-size: 0.875rem;
  background: white;
  cursor: pointer;
  min-width: 120px;

  &:focus {
    outline: none;
    border-color: #8b5cf6;
  }
}

.posts-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.post-item {
  background: white;
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  transition: all 0.3s ease;

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }
}

.post-link {
  display: block;
  color: inherit;
}

.post-content {
  padding: 1.5rem;
}

.post-category {
  display: inline-block;
  padding: 0.25rem 0.75rem;
  background: linear-gradient(135deg, #8b5cf6, #ec4899);
  color: white;
  border-radius: 4px;
  font-size: 0.75rem;
  font-weight: 500;
  margin-bottom: 0.75rem;
}

.post-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: #1f2937;
  margin-bottom: 0.75rem;
  line-height: 1.4;
}

.post-summary {
  font-size: 0.875rem;
  color: #6b7280;
  line-height: 1.6;
  margin-bottom: 1rem;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.post-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 1rem;
}

.post-meta {
  display: flex;
  gap: 1rem;
  font-size: 0.75rem;
  color: #9ca3af;
}

.post-tags {
  display: flex;
  gap: 0.5rem;
}

.empty-state {
  text-align: center;
  padding: 4rem;
  color: #6b7280;
  background: white;
  border-radius: 12px;
}

.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 1rem;
  margin-top: 2rem;
}

.page-btn {
  padding: 0.5rem 1rem;
  border: 1px solid #d1d5db;
  background: white;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;

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
</style>
