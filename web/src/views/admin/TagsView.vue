<template>
  <div class="tags-view">
    <h1 class="page-title">标签管理</h1>

    <form @submit.prevent="handleCreate" class="create-form glass-card mb-3">
      <div class="form-row">
        <input
          v-model="newName"
          type="text"
          class="form-input"
          placeholder="标签名称"
          required
        />
        <button type="submit" class="btn btn-primary">添加</button>
      </div>
    </form>

    <div class="tags-list">
      <div v-for="tag in tags" :key="tag.id" class="tag-item glass-card">
        <span class="tag">{{ tag.name }}</span>
        <button class="btn btn-sm btn-danger" @click="handleDelete(tag.id)">删除</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import api from '@/api'
import type { Tag, ApiResponse } from '@/types'

const tags = ref<Tag[]>([])
const newName = ref('')

async function fetchTags() {
  const { data } = await api.get<ApiResponse<Tag[]>>('/manage/tags')
  tags.value = data.data || []
}

async function handleCreate() {
  await api.post('/manage/tags', { name: newName.value })
  newName.value = ''
  await fetchTags()
}

async function handleDelete(id: string) {
  if (!confirm('确定要删除这个标签吗？')) return
  await api.delete(`/manage/tags/${id}`)
  await fetchTags()
}

onMounted(fetchTags)
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

.tags-list {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
}

.tag-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.625rem 1rem;
}

.tag-item:hover {
  transform: none;
}
</style>
