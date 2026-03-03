# AiriPress 博客系统

一个现代化、易维护的生产级博客系统，采用 Rust (Axum) 后端和 Vue 3 前端构建。

## 🚀 特性

- **现代技术栈**: Rust/Axum 后端 + Vue 3/Vite 前端
- **富文本编辑器**: 基于 Tiptap 的所见即所得编辑器
- **图片存储**: 使用 GitHub Repository 作为图片存储后端
- **权限系统**: 5级用户权限（Viewer, Commenter, Author, Editor, Admin）
- **完整功能**: 文章、分类、标签、评论管理
- **响应式设计**: 适配桌面和移动设备
- **生产就绪**: Docker 支持，Railway 部署配置

## 📁 项目结构

```
airipress/
├── server/                 # Rust 后端
│   ├── src/
│   │   ├── handlers/       # API 处理器
│   │   ├── models/         # 数据模型
│   │   ├── services/       # 业务逻辑
│   │   ├── middleware/     # 中间件
│   │   └── main.rs         # 入口文件
│   ├── migrations/         # 数据库迁移
│   ├── Cargo.toml
│   └── Dockerfile
│
├── web/                    # Vue 3 前端
│   ├── src/
│   │   ├── views/          # 页面组件
│   │   ├── layouts/        # 布局组件
│   │   ├── stores/         # Pinia 状态
│   │   ├── api/            # API 服务
│   │   └── types/          # TypeScript 类型
│   ├── package.json
│   └── vite.config.ts
│
└── .github/workflows/      # CI/CD 配置
```

## 🛠 本地开发

### 后端

1. 安装 Rust 和 PostgreSQL
2. 复制环境变量配置：
   ```bash
   cd server
   cp .env.example .env
   # 编辑 .env 填入数据库和 GitHub 配置
   ```
3. 运行数据库迁移：
   ```bash
   sqlx migrate run
   ```
4. 启动服务器：
   ```bash
   cargo run
   ```

### 前端

1. 安装依赖：
   ```bash
   cd web
   npm install
   ```
2. 启动开发服务器：
   ```bash
   npm run dev
   ```

## 🌐 部署

### 后端部署 (Railway)

1. 在 Railway 创建新项目
2. 连接 GitHub 仓库
3. 添加 PostgreSQL 数据库服务
4. 设置环境变量：
   - `DATABASE_URL`
   - `JWT_SECRET`
   - `GITHUB_TOKEN`
   - `GITHUB_REPO`
   - `GITHUB_OWNER`

### 前端部署 (GitHub Pages)

1. 在仓库设置中启用 GitHub Pages
2. 设置 Secret `API_BASE_URL` 为后端 API 地址
3. 推送到 main 分支自动部署

## 📝 API 端点

### 认证
- `POST /api/v1/auth/login` - 登录
- `POST /api/v1/auth/register` - 注册
- `GET /api/v1/auth/me` - 获取当前用户

### 文章
- `GET /api/v1/posts` - 获取文章列表
- `GET /api/v1/posts/:id` - 获取文章详情
- `POST /api/v1/posts` - 创建文章
- `PUT /api/v1/posts/:id` - 更新文章
- `DELETE /api/v1/posts/:id` - 删除文章

### 分类
- `GET /api/v1/categories` - 获取分类列表
- `POST /api/v1/categories` - 创建分类
- `PUT /api/v1/categories/:id` - 更新分类
- `DELETE /api/v1/categories/:id` - 删除分类

### 标签
- `GET /api/v1/tags` - 获取标签列表
- `POST /api/v1/tags` - 创建标签
- `PUT /api/v1/tags/:id` - 更新标签
- `DELETE /api/v1/tags/:id` - 删除标签

### 评论
- `GET /api/v1/posts/:id/comments` - 获取文章评论
- `POST /api/v1/comments` - 创建评论
- `DELETE /api/v1/comments/:id` - 删除评论

### 图片
- `GET /api/v1/images` - 获取图片列表
- `POST /api/v1/images` - 上传图片
- `DELETE /api/v1/images/:id` - 删除图片

### 用户管理 (Admin)
- `GET /api/v1/users` - 获取用户列表
- `POST /api/v1/users` - 创建用户
- `PUT /api/v1/users/:id` - 更新用户
- `DELETE /api/v1/users/:id` - 删除用户

## 🔐 权限等级

| 等级 | 名称 | 权限 |
|------|------|------|
| 0 | Viewer | 仅浏览 |
| 1 | Commenter | 可发表评论 |
| 2 | Author | 可写文章 |
| 3 | Editor | 可编辑所有文章、管理分类标签 |
| 4 | Admin | 完全管理权限 |

## 📄 License

MIT License
