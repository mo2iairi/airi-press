<template>
  <div class="admin-tags">
    <div class="page-header">
      <h1 class="page-title">标签管理</h1>
      <button @click="openCreateModal" class="btn btn-primary">+ 新建标签</button>
    </div>

    <div v-if="loading" class="loading">
      <div class="spinner"></div>
    </div>
    <div v-else-if="tags.length === 0" class="empty-state">
      <p>暂无标签</p>
      <button @click="openCreateModal" class="btn btn-primary btn-sm">创建第一个标签</button>
    </div>
    <div v-else class="tags-grid">
      <div v-for="tag in tags" :key="tag.id" class="tag-item">
        <span class="tag-name">{{ tag.name }}</span>
        <span class="tag-slug">{{ tag.slug }}</span>
        <div class="tag-actions">
          <button @click="openEditModal(tag)" class="action-btn edit">编辑</button>
          <button @click="confirmDelete(tag)" class="action-btn delete">删除</button>
        </div>
      </div>
    </div>

    <!-- 创建/编辑弹窗 -->
    <div v-if="showModal" class="modal-overlay" @click.self="closeModal">
      <div class="modal">
        <h3>{{ isEditing ? '编辑标签' : '新建标签' }}</h3>
        <form @submit.prevent="handleSubmit">
          <div class="form-group">
            <label class="form-label">名称 *</label>
            <input
              v-model="form.name"
              type="text"
              class="form-input"
              placeholder="标签名称"
              required
            />
          </div>
          <div class="form-group">
            <label class="form-label">别名</label>
            <input
              v-model="form.slug"
              type="text"
              class="form-input"
              placeholder="标签别名（用于 URL）"
            />
          </div>
          <div class="modal-actions">
            <button type="button" @click="closeModal" class="btn btn-secondary">取消</button>
            <button type="submit" class="btn btn-primary" :disabled="saving">
              {{ saving ? '保存中...' : '保存' }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- 删除确认弹窗 -->
    <div v-if="showDeleteModal" class="modal-overlay" @click.self="showDeleteModal = false">
      <div class="modal">
        <h3>确认删除</h3>
        <p>确定要删除标签 "{{ tagToDelete?.name }}" 吗？</p>
        <div class="modal-actions">
          <button @click="showDeleteModal = false" class="btn btn-secondary">取消</button>
          <button @click="deleteTag" class="btn btn-danger" :disabled="deleting">
            {{ deleting ? '删除中...' : '确认删除' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { tagsApi } from '@/api/services'
import type { Tag } from '@/types'

const tags = ref<Tag[]>([])
const loading = ref(true)
const saving = ref(false)
const deleting = ref(false)

const showModal = ref(false)
const isEditing = ref(false)
const editingId = ref<string | null>(null)
const form = reactive({
  name: '',
  slug: ''
})

const showDeleteModal = ref(false)
const tagToDelete = ref<Tag | null>(null)

async function fetchTags() {
  try {
    const response = await tagsApi.getAll()
    tags.value = response.data
  } catch (error) {
    console.error('Failed to fetch tags:', error)
  } finally {
    loading.value = false
  }
}

function openCreateModal() {
  isEditing.value = false
  editingId.value = null
  form.name = ''
  form.slug = ''
  showModal.value = true
}

function openEditModal(tag: Tag) {
  isEditing.value = true
  editingId.value = tag.id
  form.name = tag.name
  form.slug = tag.slug
  showModal.value = true
}

function closeModal() {
  showModal.value = false
}

async function handleSubmit() {
  if (!form.name.trim()) return

  saving.value = true
  try {
    const payload = {
      name: form.name,
      slug: form.slug || undefined
    }

    if (isEditing.value && editingId.value) {
      const response = await tagsApi.update(editingId.value, payload)
      const index = tags.value.findIndex(t => t.id === editingId.value)
      if (index !== -1) {
        tags.value[index] = response.data
      }
    } else {
      const response = await tagsApi.create(payload)
      tags.value.push(response.data)
    }

    closeModal()
  } catch (error) {
    console.error('Failed to save tag:', error)
    alert('保存失败，请重试')
  } finally {
    saving.value = false
  }
}

function confirmDelete(tag: Tag) {
  tagToDelete.value = tag
  showDeleteModal.value = true
}

async function deleteTag() {
  if (!tagToDelete.value) return

  deleting.value = true
  try {
    await tagsApi.delete(tagToDelete.value.id)
    tags.value = tags.value.filter(t => t.id !== tagToDelete.value?.id)
    showDeleteModal.value = false
    tagToDelete.value = null
  } catch (error) {
    console.error('Failed to delete tag:', error)
    alert('删除失败，请重试')
  } finally {
    deleting.value = false
  }
}

onMounted(() => {
  fetchTags()
})
</script>

<style lang="scss" scoped>
.admin-tags {
  max-width: 800px;
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

.tags-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 1rem;
}

.tag-item {
  display: flex;
  flex-direction: column;
  padding: 1rem;
  background: white;
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.tag-name {
  font-size: 1rem;
  font-weight: 600;
  color: #1f2937;
  margin-bottom: 0.25rem;
}

.tag-slug {
  font-size: 0.75rem;
  color: #9ca3af;
  margin-bottom: 0.75rem;
}

.tag-actions {
  display: flex;
  gap: 0.5rem;
}

.action-btn {
  padding: 0.25rem 0.5rem;
  font-size: 0.75rem;
  font-weight: 500;
  border: none;
  border-radius: 4px;
  cursor: pointer;

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

.empty-state {
  text-align: center;
  padding: 4rem;
  background: white;
  border-radius: 12px;
  color: #6b7280;

  .btn {
    margin-top: 1rem;
  }
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
    margin-bottom: 1.5rem;
    color: #1f2937;
  }

  p {
    color: #6b7280;
    margin-bottom: 1.5rem;
  }
}

.form-group {
  margin-bottom: 1rem;
}

.form-label {
  display: block;
  font-size: 0.875rem;
  font-weight: 500;
  color: #374151;
  margin-bottom: 0.5rem;
}

.form-input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #d1d5db;
  border-radius: 8px;
  font-size: 0.875rem;

  &:focus {
    outline: none;
    border-color: #8b5cf6;
  }
}

.modal-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 1.5rem;
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
