<template>
  <div class="admin-categories">
    <div class="page-header">
      <h1 class="page-title">分类管理</h1>
      <button @click="openCreateModal" class="btn btn-primary">+ 新建分类</button>
    </div>

    <div v-if="loading" class="loading">
      <div class="spinner"></div>
    </div>
    <div v-else-if="categories.length === 0" class="empty-state">
      <p>暂无分类</p>
      <button @click="openCreateModal" class="btn btn-primary btn-sm">创建第一个分类</button>
    </div>
    <div v-else class="categories-list">
      <div v-for="category in categories" :key="category.id" class="category-item">
        <div class="category-info">
          <h3 class="category-name">{{ category.name }}</h3>
          <p class="category-slug">{{ category.slug }}</p>
          <p v-if="category.description" class="category-description">{{ category.description }}</p>
        </div>
        <div class="category-actions">
          <button @click="openEditModal(category)" class="action-btn edit">编辑</button>
          <button @click="confirmDelete(category)" class="action-btn delete">删除</button>
        </div>
      </div>
    </div>

    <!-- 创建/编辑弹窗 -->
    <div v-if="showModal" class="modal-overlay" @click.self="closeModal">
      <div class="modal">
        <h3>{{ isEditing ? '编辑分类' : '新建分类' }}</h3>
        <form @submit.prevent="handleSubmit">
          <div class="form-group">
            <label class="form-label">名称 *</label>
            <input
              v-model="form.name"
              type="text"
              class="form-input"
              placeholder="分类名称"
              required
            />
          </div>
          <div class="form-group">
            <label class="form-label">别名</label>
            <input
              v-model="form.slug"
              type="text"
              class="form-input"
              placeholder="分类别名（用于 URL）"
            />
          </div>
          <div class="form-group">
            <label class="form-label">描述</label>
            <textarea
              v-model="form.description"
              class="form-textarea"
              placeholder="分类描述（可选）"
              rows="3"
            ></textarea>
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
        <p>确定要删除分类 "{{ categoryToDelete?.name }}" 吗？</p>
        <div class="modal-actions">
          <button @click="showDeleteModal = false" class="btn btn-secondary">取消</button>
          <button @click="deleteCategory" class="btn btn-danger" :disabled="deleting">
            {{ deleting ? '删除中...' : '确认删除' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { categoriesApi } from '@/api/services'
import type { Category } from '@/types'

const categories = ref<Category[]>([])
const loading = ref(true)
const saving = ref(false)
const deleting = ref(false)

const showModal = ref(false)
const isEditing = ref(false)
const editingId = ref<string | null>(null)
const form = reactive({
  name: '',
  slug: '',
  description: ''
})

const showDeleteModal = ref(false)
const categoryToDelete = ref<Category | null>(null)

async function fetchCategories() {
  try {
    const response = await categoriesApi.getAll()
    categories.value = response.data
  } catch (error) {
    console.error('Failed to fetch categories:', error)
  } finally {
    loading.value = false
  }
}

function openCreateModal() {
  isEditing.value = false
  editingId.value = null
  form.name = ''
  form.slug = ''
  form.description = ''
  showModal.value = true
}

function openEditModal(category: Category) {
  isEditing.value = true
  editingId.value = category.id
  form.name = category.name
  form.slug = category.slug
  form.description = category.description || ''
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
      slug: form.slug || undefined,
      description: form.description || undefined
    }

    if (isEditing.value && editingId.value) {
      const response = await categoriesApi.update(editingId.value, payload)
      const index = categories.value.findIndex(c => c.id === editingId.value)
      if (index !== -1) {
        categories.value[index] = response.data
      }
    } else {
      const response = await categoriesApi.create(payload)
      categories.value.push(response.data)
    }

    closeModal()
  } catch (error) {
    console.error('Failed to save category:', error)
    alert('保存失败，请重试')
  } finally {
    saving.value = false
  }
}

function confirmDelete(category: Category) {
  categoryToDelete.value = category
  showDeleteModal.value = true
}

async function deleteCategory() {
  if (!categoryToDelete.value) return

  deleting.value = true
  try {
    await categoriesApi.delete(categoryToDelete.value.id)
    categories.value = categories.value.filter(c => c.id !== categoryToDelete.value?.id)
    showDeleteModal.value = false
    categoryToDelete.value = null
  } catch (error) {
    console.error('Failed to delete category:', error)
    alert('删除失败，请重试')
  } finally {
    deleting.value = false
  }
}

onMounted(() => {
  fetchCategories()
})
</script>

<style lang="scss" scoped>
.admin-categories {
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

.categories-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.category-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.25rem;
  background: white;
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.category-info {
  flex: 1;
}

.category-name {
  font-size: 1rem;
  font-weight: 600;
  color: #1f2937;
  margin-bottom: 0.25rem;
}

.category-slug {
  font-size: 0.75rem;
  color: #9ca3af;
  margin-bottom: 0.25rem;
}

.category-description {
  font-size: 0.875rem;
  color: #6b7280;
}

.category-actions {
  display: flex;
  gap: 0.5rem;
}

.action-btn {
  padding: 0.375rem 0.75rem;
  font-size: 0.75rem;
  font-weight: 500;
  border: none;
  border-radius: 6px;
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
  max-width: 450px;
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

.form-input,
.form-textarea {
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

.form-textarea {
  resize: vertical;
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
