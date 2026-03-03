<template>
  <div class="post-edit">
    <div class="edit-header">
      <router-link to="/admin/posts" class="back-link">← 返回文章列表</router-link>
      <h1 class="page-title">{{ isEditing ? '编辑文章' : '写新文章' }}</h1>
    </div>

    <div v-if="loading" class="loading">
      <div class="spinner"></div>
    </div>
    <form v-else @submit.prevent="handleSubmit" class="edit-form">
      <div class="form-main">
        <!-- 标题 -->
        <div class="form-group">
          <input
            v-model="form.title"
            type="text"
            class="title-input"
            placeholder="输入文章标题..."
            required
          />
        </div>

        <!-- 富文本编辑器 -->
        <div class="editor-wrapper">
          <div class="editor-toolbar" v-if="editor">
            <button
              type="button"
              @click="editor.chain().focus().toggleBold().run()"
              :class="{ active: editor.isActive('bold') }"
              title="粗体"
            >
              <strong>B</strong>
            </button>
            <button
              type="button"
              @click="editor.chain().focus().toggleItalic().run()"
              :class="{ active: editor.isActive('italic') }"
              title="斜体"
            >
              <em>I</em>
            </button>
            <button
              type="button"
              @click="editor.chain().focus().toggleStrike().run()"
              :class="{ active: editor.isActive('strike') }"
              title="删除线"
            >
              <s>S</s>
            </button>
            <span class="toolbar-divider"></span>
            <button
              type="button"
              @click="editor.chain().focus().toggleHeading({ level: 2 }).run()"
              :class="{ active: editor.isActive('heading', { level: 2 }) }"
              title="标题 2"
            >
              H2
            </button>
            <button
              type="button"
              @click="editor.chain().focus().toggleHeading({ level: 3 }).run()"
              :class="{ active: editor.isActive('heading', { level: 3 }) }"
              title="标题 3"
            >
              H3
            </button>
            <span class="toolbar-divider"></span>
            <button
              type="button"
              @click="editor.chain().focus().toggleBulletList().run()"
              :class="{ active: editor.isActive('bulletList') }"
              title="无序列表"
            >
              •
            </button>
            <button
              type="button"
              @click="editor.chain().focus().toggleOrderedList().run()"
              :class="{ active: editor.isActive('orderedList') }"
              title="有序列表"
            >
              1.
            </button>
            <span class="toolbar-divider"></span>
            <button
              type="button"
              @click="editor.chain().focus().toggleBlockquote().run()"
              :class="{ active: editor.isActive('blockquote') }"
              title="引用"
            >
              "
            </button>
            <button
              type="button"
              @click="editor.chain().focus().toggleCodeBlock().run()"
              :class="{ active: editor.isActive('codeBlock') }"
              title="代码块"
            >
              &lt;/&gt;
            </button>
            <span class="toolbar-divider"></span>
            <button type="button" @click="setLink" title="链接">🔗</button>
            <button type="button" @click="triggerImageUpload" title="图片">🖼️</button>
            <input
              ref="imageInput"
              type="file"
              accept="image/*"
              @change="handleImageUpload"
              style="display: none"
            />
          </div>
          <editor-content :editor="editor" class="editor-content" />
        </div>
      </div>

      <!-- 侧边栏 -->
      <div class="form-sidebar">
        <!-- 发布设置 -->
        <div class="sidebar-card">
          <h3 class="card-title">发布设置</h3>
          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="form.published" />
              <span>立即发布</span>
            </label>
          </div>
          <div class="form-actions">
            <button type="submit" class="btn btn-primary btn-block" :disabled="saving">
              {{ saving ? '保存中...' : (isEditing ? '更新文章' : '保存文章') }}
            </button>
          </div>
        </div>

        <!-- 摘要 -->
        <div class="sidebar-card">
          <h3 class="card-title">文章摘要</h3>
          <textarea
            v-model="form.summary"
            placeholder="输入文章摘要（可选）"
            rows="3"
            class="form-textarea"
          ></textarea>
        </div>

        <!-- 分类 -->
        <div class="sidebar-card">
          <h3 class="card-title">分类</h3>
          <select v-model="form.category_id" class="form-select">
            <option value="">无分类</option>
            <option v-for="cat in categories" :key="cat.id" :value="cat.id">
              {{ cat.name }}
            </option>
          </select>
        </div>

        <!-- 标签 -->
        <div class="sidebar-card">
          <h3 class="card-title">标签</h3>
          <div class="tags-input">
            <div class="selected-tags">
              <span v-for="tagId in form.tag_ids" :key="tagId" class="selected-tag">
                {{ getTagName(tagId) }}
                <button type="button" @click="removeTag(tagId)" class="remove-tag">×</button>
              </span>
            </div>
            <select @change="addTag" class="form-select">
              <option value="">添加标签...</option>
              <option
                v-for="tag in availableTags"
                :key="tag.id"
                :value="tag.id"
              >
                {{ tag.name }}
              </option>
            </select>
          </div>
        </div>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, onBeforeUnmount } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Link from '@tiptap/extension-link'
import Image from '@tiptap/extension-image'
import Placeholder from '@tiptap/extension-placeholder'
import { postsApi, categoriesApi, tagsApi, imagesApi } from '@/api/services'
import type { Category, Tag } from '@/types'

const route = useRoute()
const router = useRouter()

const isEditing = computed(() => !!route.params.id)
const loading = ref(true)
const saving = ref(false)
const categories = ref<Category[]>([])
const tags = ref<Tag[]>([])
const imageInput = ref<HTMLInputElement | null>(null)

const form = reactive({
  title: '',
  content: '',
  summary: '',
  category_id: '',
  tag_ids: [] as string[],
  published: false
})

const editor = useEditor({
  extensions: [
    StarterKit,
    Link.configure({
      openOnClick: false
    }),
    Image.configure({
      HTMLAttributes: {
        class: 'editor-image'
      }
    }),
    Placeholder.configure({
      placeholder: '开始写作...'
    })
  ],
  content: '',
  onUpdate: ({ editor }) => {
    form.content = editor.getHTML()
  }
})

const availableTags = computed(() => {
  return tags.value.filter(tag => !form.tag_ids.includes(tag.id))
})

function getTagName(tagId: string) {
  return tags.value.find(t => t.id === tagId)?.name || ''
}

function addTag(event: Event) {
  const select = event.target as HTMLSelectElement
  const tagId = select.value
  if (tagId && !form.tag_ids.includes(tagId)) {
    form.tag_ids.push(tagId)
  }
  select.value = ''
}

function removeTag(tagId: string) {
  form.tag_ids = form.tag_ids.filter(id => id !== tagId)
}

function setLink() {
  const url = window.prompt('输入链接地址:')
  if (url) {
    editor.value?.chain().focus().setLink({ href: url }).run()
  }
}

function triggerImageUpload() {
  imageInput.value?.click()
}

async function handleImageUpload(event: Event) {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return

  try {
    const formData = new FormData()
    formData.append('file', file)
    const response = await imagesApi.upload(formData)
    const imageUrl = response.data.url
    editor.value?.chain().focus().setImage({ src: imageUrl }).run()
  } catch (error) {
    console.error('Failed to upload image:', error)
    alert('图片上传失败，请重试')
  }

  input.value = ''
}

async function fetchPost() {
  if (!isEditing.value) {
    loading.value = false
    return
  }

  try {
    const response = await postsApi.getById(route.params.id as string)
    const post = response.data
    form.title = post.title
    form.content = post.content
    form.summary = post.summary || ''
    form.category_id = post.category?.id || ''
    form.tag_ids = post.tags.map(t => t.id)
    form.published = post.published
    editor.value?.commands.setContent(post.content)
  } catch (error) {
    console.error('Failed to fetch post:', error)
    router.push('/admin/posts')
  } finally {
    loading.value = false
  }
}

async function fetchMetadata() {
  try {
    const [catsRes, tagsRes] = await Promise.all([
      categoriesApi.getAll(),
      tagsApi.getAll()
    ])
    categories.value = catsRes.data
    tags.value = tagsRes.data
  } catch (error) {
    console.error('Failed to fetch metadata:', error)
  }
}

async function handleSubmit() {
  if (!form.title.trim()) {
    alert('请输入文章标题')
    return
  }

  saving.value = true
  try {
    const payload = {
      title: form.title,
      content: form.content,
      summary: form.summary || undefined,
      category_id: form.category_id || undefined,
      tag_ids: form.tag_ids.length > 0 ? form.tag_ids : undefined,
      published: form.published
    }

    if (isEditing.value) {
      await postsApi.update(route.params.id as string, payload)
    } else {
      await postsApi.create(payload)
    }

    router.push('/admin/posts')
  } catch (error) {
    console.error('Failed to save post:', error)
    alert('保存失败，请重试')
  } finally {
    saving.value = false
  }
}

onMounted(() => {
  fetchMetadata()
  fetchPost()
})

onBeforeUnmount(() => {
  editor.value?.destroy()
})
</script>

<style lang="scss" scoped>
.post-edit {
  max-width: 1400px;
}

.edit-header {
  margin-bottom: 1.5rem;
}

.back-link {
  color: #6b7280;
  font-size: 0.875rem;
  display: inline-block;
  margin-bottom: 0.5rem;

  &:hover {
    color: #8b5cf6;
  }
}

.page-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: #1f2937;
}

.edit-form {
  display: grid;
  grid-template-columns: 1fr 320px;
  gap: 1.5rem;
  align-items: start;

  @media (max-width: 1024px) {
    grid-template-columns: 1fr;
  }
}

.form-main {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.title-input {
  width: 100%;
  padding: 1rem;
  border: 1px solid #e5e7eb;
  border-radius: 12px;
  font-size: 1.5rem;
  font-weight: 600;
  background: white;

  &:focus {
    outline: none;
    border-color: #8b5cf6;
  }

  &::placeholder {
    color: #9ca3af;
  }
}

.editor-wrapper {
  background: white;
  border: 1px solid #e5e7eb;
  border-radius: 12px;
  overflow: hidden;
}

.editor-toolbar {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
  padding: 0.75rem;
  border-bottom: 1px solid #e5e7eb;
  background: #f9fafb;

  button {
    padding: 0.5rem 0.75rem;
    border: none;
    border-radius: 6px;
    background: transparent;
    cursor: pointer;
    font-size: 0.875rem;
    color: #4b5563;
    transition: all 0.2s;

    &:hover {
      background: #e5e7eb;
    }

    &.active {
      background: #8b5cf6;
      color: white;
    }
  }
}

.toolbar-divider {
  width: 1px;
  height: 24px;
  background: #d1d5db;
  margin: 0 0.5rem;
  align-self: center;
}

.editor-content {
  min-height: 400px;
  padding: 1rem;

  :deep(.ProseMirror) {
    min-height: 380px;
    outline: none;

    p.is-editor-empty:first-child::before {
      content: attr(data-placeholder);
      float: left;
      color: #9ca3af;
      pointer-events: none;
      height: 0;
    }

    h1, h2, h3 {
      margin-top: 1.5rem;
      margin-bottom: 0.5rem;
    }

    p {
      margin-bottom: 0.75rem;
    }

    ul, ol {
      padding-left: 1.5rem;
      margin-bottom: 0.75rem;
    }

    blockquote {
      border-left: 4px solid #8b5cf6;
      padding-left: 1rem;
      margin: 1rem 0;
      color: #6b7280;
      font-style: italic;
    }

    pre {
      background: #1f2937;
      color: #e5e7eb;
      padding: 1rem;
      border-radius: 8px;
      overflow-x: auto;
      margin: 1rem 0;
    }

    code {
      background: #f3f4f6;
      padding: 0.2rem 0.4rem;
      border-radius: 4px;
      font-size: 0.875em;
    }

    pre code {
      background: none;
      padding: 0;
    }

    a {
      color: #8b5cf6;
      text-decoration: underline;
    }

    img.editor-image {
      max-width: 100%;
      height: auto;
      border-radius: 8px;
      margin: 1rem 0;
    }
  }
}

.form-sidebar {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.sidebar-card {
  background: white;
  border-radius: 12px;
  padding: 1.25rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.card-title {
  font-size: 0.875rem;
  font-weight: 600;
  color: #374151;
  margin-bottom: 1rem;
}

.form-group {
  margin-bottom: 1rem;

  &:last-child {
    margin-bottom: 0;
  }
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;

  input[type="checkbox"] {
    width: 18px;
    height: 18px;
    accent-color: #8b5cf6;
  }

  span {
    font-size: 0.875rem;
    color: #374151;
  }
}

.form-textarea {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #d1d5db;
  border-radius: 8px;
  font-size: 0.875rem;
  resize: vertical;

  &:focus {
    outline: none;
    border-color: #8b5cf6;
  }
}

.form-select {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #d1d5db;
  border-radius: 8px;
  font-size: 0.875rem;
  background: white;

  &:focus {
    outline: none;
    border-color: #8b5cf6;
  }
}

.form-actions {
  margin-top: 1rem;
}

.btn-block {
  width: 100%;
}

.tags-input {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.selected-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.selected-tag {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.5rem;
  background: #e0e7ff;
  color: #4f46e5;
  border-radius: 4px;
  font-size: 0.75rem;
}

.remove-tag {
  border: none;
  background: none;
  color: #4f46e5;
  cursor: pointer;
  font-size: 1rem;
  line-height: 1;
  padding: 0;
  margin-left: 0.25rem;

  &:hover {
    color: #dc2626;
  }
}
</style>
