<template>
  <div class="admin-images">
    <div class="page-header">
      <h1 class="page-title">图片管理</h1>
      <button @click="triggerUpload" class="btn btn-primary">+ 上传图片</button>
      <input
        ref="fileInput"
        type="file"
        accept="image/*"
        multiple
        @change="handleUpload"
        style="display: none"
      />
    </div>

    <!-- 上传进度 -->
    <div v-if="uploading" class="upload-progress">
      <div class="progress-bar">
        <div class="progress-fill" :style="{ width: uploadProgress + '%' }"></div>
      </div>
      <span class="progress-text">上传中... {{ uploadProgress }}%</span>
    </div>

    <div v-if="loading" class="loading">
      <div class="spinner"></div>
    </div>
    <div v-else-if="images.length === 0" class="empty-state">
      <p>暂无图片</p>
      <button @click="triggerUpload" class="btn btn-primary btn-sm">上传第一张图片</button>
    </div>
    <div v-else class="images-grid">
      <div v-for="image in images" :key="image.id" class="image-item">
        <div class="image-preview">
          <img :src="image.url" :alt="image.alt || image.original_name || '图片'" loading="lazy" />
        </div>
        <div class="image-info">
          <span class="image-name" :title="image.original_name || '未命名'">{{ image.original_name || '未命名' }}</span>
          <span class="image-size">{{ formatSize(image.size || 0) }}</span>
        </div>
        <div class="image-actions">
          <button @click="copyUrl(image.url)" class="action-btn copy" title="复制链接">
            📋
          </button>
          <button @click="confirmDelete(image)" class="action-btn delete" title="删除">
            🗑️
          </button>
        </div>
      </div>
    </div>

    <!-- 分页 -->
    <div v-if="pagination.total_pages > 1" class="pagination">
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

    <!-- 删除确认弹窗 -->
    <div v-if="showDeleteModal" class="modal-overlay" @click.self="showDeleteModal = false">
      <div class="modal">
        <h3>确认删除</h3>
        <p>确定要删除这张图片吗？此操作不可撤销。</p>
        <div class="image-preview-modal" v-if="imageToDelete">
          <img :src="imageToDelete.url" :alt="imageToDelete.original_name || '图片'" />
        </div>
        <div class="modal-actions">
          <button @click="showDeleteModal = false" class="btn btn-secondary">取消</button>
          <button @click="deleteImage" class="btn btn-danger" :disabled="deleting">
            {{ deleting ? '删除中...' : '确认删除' }}
          </button>
        </div>
      </div>
    </div>

    <!-- Toast 提示 -->
    <div v-if="toast.show" class="toast" :class="toast.type">
      {{ toast.message }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { imagesApi } from '@/api/services'
import type { Image, Pagination } from '@/types'

const images = ref<Image[]>([])
const loading = ref(true)
const uploading = ref(false)
const uploadProgress = ref(0)
const deleting = ref(false)
const fileInput = ref<HTMLInputElement | null>(null)

const pagination = ref<Pagination>({
  page: 1,
  per_page: 20,
  total: 0,
  total_pages: 0
})

const showDeleteModal = ref(false)
const imageToDelete = ref<Image | null>(null)

const toast = reactive({
  show: false,
  message: '',
  type: 'success' as 'success' | 'error'
})

function showToast(message: string, type: 'success' | 'error' = 'success') {
  toast.message = message
  toast.type = type
  toast.show = true
  setTimeout(() => {
    toast.show = false
  }, 3000)
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
}

async function fetchImages() {
  try {
    const response = await imagesApi.getAll({
      page: pagination.value.page,
      per_page: pagination.value.per_page
    })
    images.value = response.data.data
    pagination.value = response.data.pagination
  } catch (error) {
    console.error('Failed to fetch images:', error)
  } finally {
    loading.value = false
  }
}

function triggerUpload() {
  fileInput.value?.click()
}

async function handleUpload(event: Event) {
  const input = event.target as HTMLInputElement
  const files = input.files
  if (!files || files.length === 0) return

  uploading.value = true
  uploadProgress.value = 0

  try {
    for (let i = 0; i < files.length; i++) {
      const formData = new FormData()
      formData.append('file', files[i])

      const response = await imagesApi.upload(formData)
      // 上传返回的是简化对象，需要重新获取完整图片列表
      // 或者构造一个临时的 Image 对象
      const uploadedImage: Image = {
        id: response.data.id,
        url: response.data.url,
        relative_path: response.data.relative_path,
        original_name: files[i].name,
        alt: undefined,
        filename: files[i].name,
        mime_type: files[i].type,
        size: files[i].size,
        created_at: new Date().toISOString()
      }
      images.value.unshift(uploadedImage)
      uploadProgress.value = Math.round(((i + 1) / files.length) * 100)
    }

    showToast(`成功上传 ${files.length} 张图片`)
  } catch (error) {
    console.error('Failed to upload images:', error)
    showToast('上传失败，请重试', 'error')
  } finally {
    uploading.value = false
    input.value = ''
  }
}

function copyUrl(url: string) {
  navigator.clipboard.writeText(url).then(() => {
    showToast('链接已复制到剪贴板')
  }).catch(() => {
    showToast('复制失败', 'error')
  })
}

function confirmDelete(image: Image) {
  imageToDelete.value = image
  showDeleteModal.value = true
}

async function deleteImage() {
  if (!imageToDelete.value) return

  deleting.value = true
  try {
    await imagesApi.delete(imageToDelete.value.id)
    images.value = images.value.filter(img => img.id !== imageToDelete.value?.id)
    showDeleteModal.value = false
    imageToDelete.value = null
    showToast('图片已删除')
  } catch (error) {
    console.error('Failed to delete image:', error)
    showToast('删除失败，请重试', 'error')
  } finally {
    deleting.value = false
  }
}

function changePage(page: number) {
  pagination.value.page = page
  fetchImages()
}

onMounted(() => {
  fetchImages()
})
</script>

<style lang="scss" scoped>
.admin-images {
  max-width: 1200px;
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

.upload-progress {
  margin-bottom: 1.5rem;
  background: white;
  padding: 1rem;
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.progress-bar {
  height: 8px;
  background: #e5e7eb;
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 0.5rem;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(135deg, #8b5cf6, #ec4899);
  transition: width 0.3s;
}

.progress-text {
  font-size: 0.875rem;
  color: #6b7280;
}

.images-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 1rem;
}

.image-item {
  background: white;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: all 0.2s;

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }
}

.image-preview {
  aspect-ratio: 1;
  overflow: hidden;
  background: #f3f4f6;

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
}

.image-info {
  padding: 0.75rem;
  border-bottom: 1px solid #e5e7eb;
}

.image-name {
  display: block;
  font-size: 0.75rem;
  font-weight: 500;
  color: #374151;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin-bottom: 0.25rem;
}

.image-size {
  font-size: 0.625rem;
  color: #9ca3af;
}

.image-actions {
  display: flex;
  padding: 0.5rem;
  gap: 0.5rem;
}

.action-btn {
  flex: 1;
  padding: 0.5rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1rem;
  transition: background 0.2s;

  &.copy {
    background: #e0e7ff;

    &:hover {
      background: #c7d2fe;
    }
  }

  &.delete {
    background: #fee2e2;

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

.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 1rem;
  margin-top: 1.5rem;
}

.page-btn {
  padding: 0.5rem 1rem;
  border: 1px solid #d1d5db;
  background: white;
  border-radius: 8px;
  cursor: pointer;
  font-size: 0.875rem;

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
    margin-bottom: 1rem;
    color: #1f2937;
  }

  p {
    color: #6b7280;
    margin-bottom: 1rem;
  }
}

.image-preview-modal {
  margin-bottom: 1.5rem;
  border-radius: 8px;
  overflow: hidden;

  img {
    width: 100%;
    max-height: 200px;
    object-fit: contain;
    background: #f3f4f6;
  }
}

.modal-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
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

.toast {
  position: fixed;
  bottom: 2rem;
  left: 50%;
  transform: translateX(-50%);
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  font-size: 0.875rem;
  font-weight: 500;
  z-index: 1100;
  animation: slideUp 0.3s ease;

  &.success {
    background: #059669;
    color: white;
  }

  &.error {
    background: #dc2626;
    color: white;
  }
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateX(-50%) translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateX(-50%) translateY(0);
  }
}
</style>
