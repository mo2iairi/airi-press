export interface User {
  id: string
  username: string
  permission: number
  created_at: string
  updated_at: string
}

export interface Post {
  id: number
  title: string
  summary: string | null
  content: string
  author: User
  published: boolean
  category?: Category | null
  categories: Category[]
  tags: Tag[]
  created_at: string
  updated_at: string
}

export interface PostListItem {
  id: number
  title: string
  summary: string | null
  author: User
  published: boolean
  category?: Category | null
  categories: Category[]
  tags: Tag[]
  created_at: string
  updated_at: string
}

export interface PostDetail extends Post {
  comment_count?: number
}

export interface Category {
  id: number
  name: string
  slug?: string
  description?: string
  parent_id: number | null
}

export interface CategoryWithChildren extends Category {
  children: CategoryWithChildren[]
}

export interface Tag {
  id: number
  name: string
  slug?: string
}

export interface Comment {
  id: number
  post_id: number
  user?: User
  author: User
  content: string
  created_at: string
}

export interface Image {
  id: number
  relative_path: string
  url: string
  filename?: string
  original_name: string | null
  alt?: string
  mime_type: string | null
  size: number | null
  created_at: string
}

export interface Pagination {
  page: number
  per_page: number
  total: number
  total_pages: number
}

export interface PaginatedResponse<T> {
  data: T[]
  pagination: Pagination
}

export interface LoginRequest {
  username: string
  password: string
}

export interface LoginResponse {
  token: string
  user: User
}

export interface CreatePostRequest {
  title: string
  summary?: string
  content: string
  published?: boolean
  category_ids?: number[]
  tag_ids?: number[]
}

export interface UpdatePostRequest {
  title?: string
  summary?: string
  content?: string
  published?: boolean
  category_ids?: number[]
  tag_ids?: number[]
}

export interface CreateUserRequest {
  username: string
  password: string
  permission?: number
}

export interface UpdateUserRequest {
  username?: string
  password?: string
  permission?: number
}

export interface CreateCategoryRequest {
  name: string
  parent_id?: number
}

export interface CreateTagRequest {
  name: string
}

export interface PostQuery {
  page?: number
  per_page?: number
  published?: boolean
  author_id?: string
  category_id?: number
  tag_id?: number
  search?: string
}

// Permission levels
export const Permission = {
  VIEWER: 0,
  COMMENTER: 1,
  AUTHOR: 2,
  EDITOR: 3,
  ADMIN: 4,
} as const

export type PermissionLevel = typeof Permission[keyof typeof Permission]

export const getPermissionLabel = (permission: number): string => {
  switch (permission) {
    case Permission.VIEWER:
      return '访客'
    case Permission.COMMENTER:
      return '评论者'
    case Permission.AUTHOR:
      return '作者'
    case Permission.EDITOR:
      return '编辑'
    case Permission.ADMIN:
      return '管理员'
    default:
      return '未知'
  }
}