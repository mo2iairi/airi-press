import api from './index'
import type {
  LoginRequest,
  LoginResponse,
  User,
  Post,
  PostListItem,
  PaginatedResponse,
  CreatePostRequest,
  UpdatePostRequest,
  Category,
  CategoryWithChildren,
  Tag,
  Comment,
  Image,
  PostQuery,
  CreateUserRequest,
  UpdateUserRequest,
  CreateCategoryRequest,
  CreateTagRequest,
} from '@/types'

// Auth
export const authApi = {
  login: (data: LoginRequest) => api.post<LoginResponse>('/auth/login', data),
}

// Users
export const usersApi = {
  getMe: () => api.get<User>('/users/me'),
  getAll: () => api.get<User[]>('/users'),
  getById: (id: string) => api.get<User>(`/users/${id}`),
  create: (data: CreateUserRequest) => api.post<User>('/users', data),
  update: (id: string, data: UpdateUserRequest) => api.put<User>(`/users/${id}`, data),
  delete: (id: string) => api.delete(`/users/${id}`),
}

// Posts
export const postsApi = {
  getAll: (params?: PostQuery) => api.get<PaginatedResponse<PostListItem>>('/posts', { params }),
  getById: (id: number) => api.get<Post>(`/posts/${id}`),
  create: (data: CreatePostRequest) => api.post<Post>('/posts', data),
  update: (id: number, data: UpdatePostRequest) => api.put<Post>(`/posts/${id}`, data),
  delete: (id: number) => api.delete(`/posts/${id}`),
}

// Categories
export const categoriesApi = {
  getAll: () => api.get<Category[]>('/categories'),
  getTree: () => api.get<CategoryWithChildren[]>('/categories/tree'),
  getById: (id: number) => api.get<Category>(`/categories/${id}`),
  create: (data: CreateCategoryRequest) => api.post<Category>('/categories', data),
  update: (id: number, data: Partial<CreateCategoryRequest>) => api.put<Category>(`/categories/${id}`, data),
  delete: (id: number) => api.delete(`/categories/${id}`),
}

// Tags
export const tagsApi = {
  getAll: () => api.get<Tag[]>('/tags'),
  getById: (id: number) => api.get<Tag>(`/tags/${id}`),
  create: (data: CreateTagRequest) => api.post<Tag>('/tags', data),
  update: (id: number, data: CreateTagRequest) => api.put<Tag>(`/tags/${id}`, data),
  delete: (id: number) => api.delete(`/tags/${id}`),
}

// Comments
export const commentsApi = {
  getByPost: (postId: number) => api.get<Comment[]>(`/posts/${postId}/comments`),
  create: (postId: number, content: string) =>
    api.post<Comment>(`/posts/${postId}/comments`, { content }),
  update: (id: number, content: string) => api.put<Comment>(`/comments/${id}`, { content }),
  delete: (id: number) => api.delete(`/comments/${id}`),
}

// Images
export const imagesApi = {
  getAll: () => api.get<Image[]>('/images'),
  getById: (id: number) => api.get<Image>(`/images/${id}`),
  upload: (file: File) => {
    const formData = new FormData()
    formData.append('file', file)
    return api.post<{ id: number; url: string; relative_path: string }>('/images', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    })
  },
  delete: (id: number) => api.delete(`/images/${id}`),
}
