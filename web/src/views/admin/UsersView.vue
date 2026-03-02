<template>
  <div class="users-view">
    <h1 class="page-title">用户管理</h1>

    <div class="users-list">
      <div v-for="user in users" :key="user.id" class="user-item glass-card">
        <div class="user-info">
          <strong>{{ user.username }}</strong>
          <span class="permission-badge" :class="permissionClass(user.permission)">
            {{ permissionLabel(user.permission) }}
          </span>
          <span class="text-muted">{{ formatDate(user.created_at) }}</span>
        </div>
        <div class="user-actions">
          <select
            :value="user.permission"
            @change="handlePermissionChange(user.id, $event)"
            class="form-select permission-select"
          >
            <option :value="0">访客 (0)</option>
            <option :value="1">作者 (1)</option>
            <option :value="255">管理员 (255)</option>
          </select>
          <button
            class="btn btn-sm btn-danger"
            @click="handleDelete(user.id)"
            :disabled="user.permission === 255"
          >
            删除
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import api from '@/api'
import type { User, ApiResponse } from '@/types'

const users = ref<User[]>([])

async function fetchUsers() {
  const { data } = await api.get<ApiResponse<User[]>>('/users')
  users.value = data.data || []
}

function permissionLabel(perm: number): string {
  if (perm === 255) return '管理员'
  if (perm >= 1) return '作者'
  return '访客'
}

function permissionClass(perm: number): string {
  if (perm === 255) return 'admin'
  if (perm >= 1) return 'author'
  return 'visitor'
}

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleDateString('zh-CN')
}

async function handlePermissionChange(id: string, event: Event) {
  const target = event.target as HTMLSelectElement
  const permission = parseInt(target.value)
  await api.put(`/users/${id}/permission`, { permission })
  await fetchUsers()
}

async function handleDelete(id: string) {
  if (!confirm('确定要删除这个用户吗？')) return
  await api.delete(`/users/${id}`)
  await fetchUsers()
}

onMounted(fetchUsers)
</script>

<style scoped>
.page-title {
  font-size: 1.5rem;
  font-weight: 700;
  margin-bottom: 1.5rem;
}

.users-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.user-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.25rem;
}

.user-item:hover {
  transform: none;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.permission-badge {
  padding: 0.15rem 0.5rem;
  border-radius: 6px;
  font-size: 0.75rem;
  font-weight: 500;
}

.permission-badge.admin {
  background: rgba(245, 91, 122, 0.12);
  color: var(--danger);
}

.permission-badge.author {
  background: rgba(91, 245, 160, 0.12);
  color: var(--success);
}

.permission-badge.visitor {
  background: rgba(255, 255, 255, 0.08);
  color: var(--text-muted);
}

.user-actions {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.permission-select {
  width: auto;
  padding: 0.375rem 0.75rem;
  font-size: 0.8rem;
}
</style>
