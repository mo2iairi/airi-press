import { defineStore } from 'pinia'
import { ref } from 'vue'
import api from '@/api'
import type { Post, ApiResponse, CreatePostRequest, UpdatePostRequest } from '@/types'

export const usePostStore = defineStore('posts', () => {
  const posts = ref<Post[]>([])
  const currentPost = ref<Post | null>(null)
  const loading = ref(false)

  async function fetchPosts(all: boolean = false) {
    loading.value = true
    try {
      // Admin view uses /manage/posts (requires auth), public uses /posts
      const endpoint = all ? '/manage/posts' : '/posts'
      const { data } = await api.get<ApiResponse<Post[]>>(endpoint, {
        params: all ? { all: true } : {}
      })
      if (data.data) {
        posts.value = data.data
      }
    } finally {
      loading.value = false
    }
  }

  async function fetchPost(id: string, manage: boolean = false) {
    loading.value = true
    try {
      const endpoint = manage ? `/manage/posts/${id}` : `/posts/${id}`
      const { data } = await api.get<ApiResponse<Post>>(endpoint)
      if (data.data) {
        currentPost.value = data.data
      }
    } finally {
      loading.value = false
    }
  }

  async function createPost(payload: CreatePostRequest) {
    const { data } = await api.post<ApiResponse<Post>>('/manage/posts', payload)
    if (data.data) {
      posts.value.unshift(data.data)
    }
    return data.data
  }

  async function updatePost(id: string, payload: UpdatePostRequest) {
    const { data } = await api.put<ApiResponse<Post>>(`/manage/posts/${id}`, payload)
    if (data.data) {
      const index = posts.value.findIndex((p) => p.id === id)
      if (index !== -1) {
        posts.value[index] = data.data
      }
      currentPost.value = data.data
    }
    return data.data
  }

  async function deletePost(id: string) {
    await api.delete(`/manage/posts/${id}`)
    posts.value = posts.value.filter((p) => p.id !== id)
  }

  return {
    posts,
    currentPost,
    loading,
    fetchPosts,
    fetchPost,
    createPost,
    updatePost,
    deletePost,
  }
})