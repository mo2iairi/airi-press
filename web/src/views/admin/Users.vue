<template>
  <div class="admin-users">
    <div class="page-header">
      <h1 class="page-title">用户管理</h1>
      <button @click="openCreateModal" class="btn btn-primary">+ 新建用户</button>
    </div>

    <div v-if="loading" class="loading">
      <div class="spinner"></div>
    </div>
    <div v-else-if="users.length === 0" class="empty-state">
      <p>暂无用户</p>
    </div>
    <div v-else class="users-table-wrapper">
      <table class="users-table">
        <thead>
          <tr>
            <th>用户名</th>
            <th>权限等级</th>
            <th>创建时间</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="user in users" :key="user.id">
            <td class="username-cell">{{ user.username }}</td>
            <td>
              <span :class="['permission-badge', getPermissionClass(user.permission)]">
                {{ getPermissionLabel(user.permission) }}
              </span>
            </td>
            <td class="date-cell">{{ formatDate(user.created_at) }}</td>
            <td class="actions-cell">
              <button @click="openEditModal(user)" class="action-btn edit">编辑</button>
              <button
                v-if="user.id !== authStore.user?.id"
                @click="confirmDelete(user)"
                class="action-btn delete"
              >
                删除
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- 创建/编辑弹窗 -->
    <div v-if="showModal" class="modal-overlay" @click.self="closeModal">
      <div class="modal">
        <h3>{{ isEditing ? '编辑用户' : '新建用户' }}</h3>
        <form @submit.prevent="handleSubmit">
          <div class="form-group">
            <label class="form-label">用户名 *</label>
            <input
              v-model="form.username"
              type="text"
              class="form-input"
              placeholder="用户名"
              required
              :disabled="isEditing"
            />
          </div>
          <div class="form-group" v-if="!isEditing">
            <label class="form-label">密码 *</label>
            <input
              v-model="form.password"
              type="password"
              class="form-input"
              placeholder="密码（至少6位）"
              :required="!isEditing"
              minlength="6"
            />
          </div>
          <div class="form-group">
            <label class="form-label">权限等级</label>
            <select v-model="form.permission" class="form-select">
              <option :value="0">Viewer（仅查看）</option>
              <option :value="1">Commenter（可评论）</option>
              <option :value="2">Author（可写文章）</option>
              <option :value="3">Editor（可编辑所有文章）</option>
              <option :value="4">Admin（管理员）</option>
            </select>
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
        <p>确定要删除用户 "{{ userToDelete?.username }}" 吗？此操作不可撤销。</p>
        <div class="modal-actions">
          <button @click="showDeleteModal = false" class="btn btn-secondary">取消</button>
          <button @click="deleteUser" class="btn btn-danger" :disabled="deleting">
            {{ deleting ? '删除中...' : '确认删除' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { usersApi } from '@/api/services'
import type { User } from '@/types'
import dayjs from 'dayjs'

const authStore = useAuthStore()
const users = ref<User[]>([])
const loading = ref(true)
const saving = ref(false)
const deleting = ref(false)

const showModal = ref(false)
const isEditing = ref(false)
const editingId = ref<string | null>(null)
const form = reactive({
  username: '',
  password: '',
  permission: 1
})

const showDeleteModal = ref(false)
const userToDelete = ref<User | null>(null)

function formatDate(date: string) {
  return dayjs(date).format('YYYY-MM-DD')
}

function getPermissionLabel(level: number): string {
  const labels: Record<number, string> = {
    0: 'Viewer',
    1: 'Commenter',
    2: 'Author',
    3: 'Editor',
    4: 'Admin'
  }
  return labels[level] || 'Unknown'
}

function getPermissionClass(level: number): string {
  const classes: Record<number, string> = {
    0: 'viewer',
    1: 'commenter',
    2: 'author',
    3: 'editor',
    4: 'admin'
  }
  return classes[level] || 'viewer'
}

async function fetchUsers() {
  try {
    const response = await usersApi.getAll()
    users.value = response.data
  } catch (error) {
    console.error('Failed to fetch users:', error)
  } finally {
    loading.value = false
  }
}

function openCreateModal() {
  isEditing.value = false
  editingId.value = null
  form.username = ''
  form.password = ''
  form.permission = 1
  showModal.value = true
}

function openEditModal(user: User) {
  isEditing.value = true
  editingId.value = user.id
  form.username = user.username
  form.password = ''
  form.permission = user.permission
  showModal.value = true
}

function closeModal() {
  showModal.value = false
}

async function handleSubmit() {
  if (!form.username.trim()) return
  if (!isEditing.value && !form.password) return

  saving.value = true
  try {
    if (isEditing.value && editingId.value) {
      const payload: Record<string, any> = {
        permission: form.permission
      }
      const response = await usersApi.update(editingId.value, payload)
      const index = users.value.findIndex(u => u.id === editingId.value)
      if (index !== -1) {
        users.value[index] = response.data
      }
    } else {
      const response = await usersApi.create({
        username: form.username,
        password: form.password,
        permission: form.permission
      })
      users.value.push(response.data)
    }

    closeModal()
  } catch (error) {
    console.error('Failed to save user:', error)
    alert('保存失败，请重试')
  } finally {
    saving.value = false
  }
}

function confirmDelete(user: User) {
  userToDelete.value = user
  showDeleteModal.value = true
}

async function deleteUser() {
  if (!userToDelete.value) return

  deleting.value = true
  try {
    await usersApi.delete(userToDelete.value.id)
    users.value = users.value.filter(u => u.id !== userToDelete.value?.id)
    showDeleteModal.value = false
    userToDelete.value = null
  } catch (error) {
    console.error('Failed to delete user:', error)
    alert('删除失败，请重试')
  } finally {
    deleting.value = false
  }
}

onMounted(() => {
  fetchUsers()
})
</script>

<style lang="scss" scoped>
.admin-users {
  max-width: 1000px;
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

.users-table-wrapper {
  background: white;
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.users-table {
  width: 100%;
  border-collapse: collapse;

  th, td {
    padding: 1rem;
    text-align: left;
    border-bottom: 1px solid #e5e7eb;
  }

  th {
    background: #f9fafb;
    font-weight: 500;
    color: #6b7280;
    font-size: 0.875rem;
  }

  tbody tr:hover {
    background: #f9fafb;
  }
}

.username-cell {
  font-weight: 500;
  color: #1f2937;
}

.date-cell {
  color: #6b7280;
  font-size: 0.875rem;
}

.actions-cell {
  white-space: nowrap;
}

.permission-badge {
  display: inline-block;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.75rem;
  font-weight: 500;

  &.viewer {
    background: #e5e7eb;
    color: #4b5563;
  }

  &.commenter {
    background: #dbeafe;
    color: #1d4ed8;
  }

  &.author {
    background: #d1fae5;
    color: #059669;
  }

  &.editor {
    background: #fef3c7;
    color: #d97706;
  }

  &.admin {
    background: linear-gradient(135deg, #8b5cf6, #ec4899);
    color: white;
  }
}

.action-btn {
  padding: 0.25rem 0.5rem;
  font-size: 0.75rem;
  font-weight: 500;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  margin-right: 0.5rem;

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
.form-select {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #d1d5db;
  border-radius: 8px;
  font-size: 0.875rem;

  &:focus {
    outline: none;
    border-color: #8b5cf6;
  }

  &:disabled {
    background: #f3f4f6;
    cursor: not-allowed;
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
