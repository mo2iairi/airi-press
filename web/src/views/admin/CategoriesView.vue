<template>
  <div class="categories-view">
    <h1 class="page-title">分类管理</h1>

    <form @submit.prevent="handleCreate" class="create-form glass-card mb-3">
      <div class="form-row">
        <input
          v-model="newName"
          type="text"
          class="form-input"
          placeholder="分类名称"
          required
        />
        <input
          v-model="newDescription"
          type="text"
          class="form-input"
          placeholder="分类描述（可选）"
        />
        <button type="submit" class="btn btn-primary">添加</button>
      </div>
    </form>

    <div class="list">
      <div v-for="cat in categories" :key="cat.id" class="list-item glass-card">
        <div class="item-info">
          <strong>{{ cat.name }}</strong>
          <span class="text-muted" v-if="cat.description">{{ cat.description }}</span>
        </div>
        <button class="btn btn-sm btn-danger" @click="handleDelete(cat.id)">删除</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import api from '@/api'
import type { Category, ApiResponse } from '@/types'

const categories = ref<Category[]>([])
const newName = ref('')
const newDescription = ref('')

async function fetchCategories() {
  const { data } = await api.get<ApiResponse<Category[]>>('/manage/categories')
  categories.value = data.data || []
}

async function handleCreate() {
  await api.post('/manage/categories', {
    name: newName.value,
    description: newDescription.value || undefined,
  })
  newName.value = ''
  newDescription.value = ''
  await fetchCategories()
}

async function handleDelete(id: string) {
  if (!confirm('确定要删除这个分类吗？')) return
  await api.delete(`/manage/categories/${id}`)
  await fetchCategories()
}

onMounted(fetchCategories)
</script>

<style scoped>
.page-title {
  font-size: 1.5rem;
  font-weight: 700;
  margin-bottom: 1.5rem;
}

.create-form {
  padding: 1.25rem;
}

.create-form:hover {
  transform: none;
}

.form-row {
  display: flex;
  gap: 0.75rem;
  align-items: center;
}

.form-row .form-input {
  flex: 1;
}

.list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.list-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.875rem 1.25rem;
}

.list-item:hover {
  transform: none;
}

.item-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.item-info .text-muted {
  font-size: 0.8rem;
}

@media (max-width: 768px) {
  .form-row {
    flex-direction: column;
  }
}
</style>
