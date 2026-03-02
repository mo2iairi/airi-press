export interface User {
  id: string
  username: string
  permission: number
  created_at: string
  updated_at: string
}

export interface AuthResponse {
  token: string
  user: User
}

export interface Post {
  id: string
  title: string
  summary: string
  author_id: string
  author_name: string
  category: Category | null
  tags: Tag[]
  published: boolean
  content?: string
  created_at: string
  updated_at: string
}

export interface CreatePostRequest {
  title: string
  summary: string
  content: string
  category_id?: string
  tag_ids?: string[]
  published?: boolean
}

export interface UpdatePostRequest {
  title?: string
  summary?: string
  content?: string
  category_id?: string
  tag_ids?: string[]
  published?: boolean
}

export interface Category {
  id: string
  name: string
  description: string
  created_at: string
}

export interface Tag {
  id: string
  name: string
  created_at: string
}

export interface ApiResponse<T> {
  success: boolean
  data: T | null
  message: string | null
}

export interface LoginRequest {
  username: string
  password: string
}

export interface RegisterRequest {
  username: string
  password: string
}
