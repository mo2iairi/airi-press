<template>
  <div class="post-editor">
    <h1 class="page-title">{{ isEdit ? '编辑文章' : '新建文章' }}</h1>

    <form @submit.prevent="handleSubmit" class="editor-form">
      <div class="editor-grid">
        <div class="editor-main">
          <div class="form-group">
            <label class="form-label">标题</label>
            <input v-model="form.title" type="text" class="form-input" placeholder="文章标题" required />
          </div>

          <div class="form-group">
            <label class="form-label">摘要</label>
            <textarea v-model="form.summary" class="form-textarea" placeholder="文章摘要" rows="3"></textarea>
          </div>

          <div class="form-group">
            <label class="form-label">内容 (Markdown)</label>
            <textarea
              v-model="form.content"
              class="form-textarea content-editor"
              placeholder="使用 Markdown 编写文章内容..."
              rows="20"
            ></textarea>
          </div>
        </div>

        <div class="editor-sidebar">
          <div class="sidebar-section glass-card">
            <h3 class="sidebar-section-title">发布设置</h3>

            <div class="form-group">
              <label class="form-label">分类</label>
              <select v-model="form.category_id" class="form-select">
                <option value="">无分类</option>
                <option v-for="cat in categories" :key="cat.id" :value="cat.id">
                  {{ cat.name }}
                </option>
              </select>
            </div>

            <div class="form-group">
              <label class="form-label">标签</label>
              <div class="tags-select">
                <label v-for="tag in tags" :key="tag.id" class="tag-checkbox">
                  <input
                    type="checkbox"
                    :value="tag.id"
                    v-model="form.tag_ids"
                  />
                  <span class="tag">{{ tag.name }}</span>
                </label>
              </div>
            </div>

            <div class="form-group">
              <label class="toggle-label">
                <input type="checkbox" v-model="form.published" />
                <span>{{ form.published ? '已发布' : '草稿' }}</span>
              </label>
            </div>

            <p v-if="error" class="text-danger mb-2">{{ error }}</p>

            <div class="form-actions">
              <button type="submit" class="btn btn-primary" :disabled="saving" style="width: 100%">
                {{ saving ? '保存中...' : '保存' }}
              </button>
              <router-link to="/admin/posts" class="btn" style="width: 100%; text-align: center;">取消</router-link>
            </div>
          </div>
        </div>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { usePostStore } from '@/stores/posts'
import api from '@/api'
import type { Category, Tag, ApiResponse } from '@/types'

const route = useRoute()
const router = useRouter()
const postStore = usePostStore()

const isEdit = computed(() => !!route.params.id)
const saving = ref(false)
const error = ref('')
const categories = ref<Category[]>([])
const tags = ref<Tag[]>([])

const form = reactive({
  title: '',
  summary: '',
  content: '',
  category_id: '',
  tag_ids: [] as string[],
  published: false,
})

onMounted(async () => {
  const [catRes, tagRes] = await Promise.all([
    api.get<ApiResponse<Category[]>>('/manage/categories'),
    api.get<ApiResponse<Tag[]>>('/manage/tags'),
  ])
  categories.value = catRes.data.data || []
  tags.value = tagRes.data.data || []

  if (isEdit.value) {
    await postStore.fetchPost(route.params.id as string, true)
    if (postStore.currentPost) {
      form.title = postStore.currentPost.title
      form.summary = postStore.currentPost.summary
      form.content = postStore.currentPost.content || ''
      form.category_id = postStore.currentPost.category?.id || ''
      form.tag_ids = postStore.currentPost.tags.map((t) => t.id)
      form.published = postStore.currentPost.published
    }
  }
})

async function handleSubmit() {
  error.value = ''
  saving.value = true
  try {
    const payload = {
      title: form.title,
      summary: form.summary,
      content: form.content,
      category_id: form.category_id || undefined,
      tag_ids: form.tag_ids.length > 0 ? form.tag_ids : undefined,
      published: form.published,
    }

    if (isEdit.value) {
      await postStore.updatePost(route.params.id as string, payload)
    } else {
      await postStore.createPost(payload)
    }

    router.push('/admin/posts')
  } catch (e: any) {
    error.value = e.response?.data?.message || '保存失败'
  } finally {
    saving.value = false
  }
}
</script>

<style scoped>
.page-title {
  font-size: 1.5rem;
  font-weight: 700;
  margin-bottom: 1.5rem;
}

.editor-grid {
  display: grid;
  grid-template-columns: 1fr 280px;
  gap: 1.5rem;
  align-items: start;
}

.content-editor {
  min-height: 400px;
  font-family: 'Fira Code', 'Cascadia Code', monospace;
  font-size: 0.9rem;
  line-height: 1.6;
}

.sidebar-section {
  padding: 1.25rem;
  position: sticky;
  top: 80px;
}

.sidebar-section:hover {
  transform: none;
}

.sidebar-section-title {
  font-size: 0.95rem;
  font-weight: 600;
  margin-bottom: 1rem;
}

.tags-select {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.tag-checkbox {
  cursor: pointer;
}

.tag-checkbox input {
  display: none;
}

.tag-checkbox input:checked + .tag {
  background: rgba(124, 91, 245, 0.25);
  border-color: var(--accent);
}

.toggle-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.form-actions {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

@media (max-width: 768px) {
  .editor-grid {
    grid-template-columns: 1fr;
  }
}
</style>
