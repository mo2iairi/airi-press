# Airi Press

A modern personal blog / CMS system built with **Rust (Axum)** backend and **Vue 3** frontend, featuring a beautiful glassmorphism design.

## Architecture

- **Backend**: Rust + Axum + SQLx + PostgreSQL
- **Frontend**: Vue 3 + TypeScript + Vite + Pinia
- **Content Storage**: GitHub API (Markdown files stored in a GitHub repository)
- **Authentication**: JWT + bcrypt

## Features

- 📝 Markdown-based article management
- 🏷️ Tags and categories
- 🖼️ Image storage via GitHub repository
- 👥 User management with permission system
- 🔐 JWT authentication
- 🎨 Glassmorphism UI design
- 📱 Responsive layout
- 🌙 Dark theme

## Permission System

| Permission | Value | Description |
|------------|-------|-------------|
| Visitor    | 0     | Can only view published articles |
| Author     | 1     | Can manage articles |
| Admin      | 255   | Can manage users and articles |

## Getting Started

### Prerequisites

- Rust (latest stable)
- Node.js 18+
- PostgreSQL 15+
- GitHub Personal Access Token

### Backend Setup

```bash
cd server
cp .env.example .env
# Edit .env with your configuration
cargo run
```

### Frontend Setup

```bash
cd web
cp .env.example .env
# Edit .env with your API base URL
npm install
npm run dev
```

### Docker Deployment (Backend)

```bash
cd server
docker build -t airi-press-server .
docker run -p 3000:3000 --env-file .env airi-press-server
```

## API Endpoints

### Health
- `GET /api/v1/health`

### Authentication
- `POST /api/v1/auth/login`
- `POST /api/v1/auth/register`
- `GET /api/v1/auth/me`

### Users (Admin only)
- `GET /api/v1/users`
- `PUT /api/v1/users/:id/permission`
- `DELETE /api/v1/users/:id`

### Posts
- `GET /api/v1/posts`
- `GET /api/v1/posts/:id`
- `POST /api/v1/posts` (Author+)
- `PUT /api/v1/posts/:id` (Author+)
- `DELETE /api/v1/posts/:id` (Author+)

### Categories
- `GET /api/v1/categories`
- `GET /api/v1/categories/:id`
- `POST /api/v1/categories` (Author+)
- `PUT /api/v1/categories/:id` (Author+)
- `DELETE /api/v1/categories/:id` (Author+)

### Tags
- `GET /api/v1/tags`
- `GET /api/v1/tags/:id`
- `POST /api/v1/tags` (Author+)
- `PUT /api/v1/tags/:id` (Author+)
- `DELETE /api/v1/tags/:id` (Author+)

## License

MIT License
