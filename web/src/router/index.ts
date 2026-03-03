import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import MainLayout from '@/layouts/MainLayout.vue'
import AdminLayout from '@/layouts/AdminLayout.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: MainLayout,
      children: [
        {
          path: '',
          name: 'home',
          component: () => import('@/views/Home.vue')
        },
        {
          path: 'posts',
          name: 'posts',
          component: () => import('@/views/Posts.vue')
        },
        {
          path: 'posts/:id',
          name: 'post-detail',
          component: () => import('@/views/PostDetail.vue')
        }
      ]
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('@/views/Login.vue')
    },
    {
      path: '/admin',
      component: AdminLayout,
      meta: { requiresAuth: true },
      children: [
        {
          path: '',
          name: 'admin-dashboard',
          component: () => import('@/views/admin/Dashboard.vue')
        },
        {
          path: 'posts',
          name: 'admin-posts',
          component: () => import('@/views/admin/Posts.vue'),
          meta: { minPermission: 2 }
        },
        {
          path: 'posts/new',
          name: 'admin-post-new',
          component: () => import('@/views/admin/PostEdit.vue'),
          meta: { minPermission: 2 }
        },
        {
          path: 'posts/:id/edit',
          name: 'admin-post-edit',
          component: () => import('@/views/admin/PostEdit.vue'),
          meta: { minPermission: 2 }
        },
        {
          path: 'categories',
          name: 'admin-categories',
          component: () => import('@/views/admin/Categories.vue'),
          meta: { minPermission: 3 }
        },
        {
          path: 'tags',
          name: 'admin-tags',
          component: () => import('@/views/admin/Tags.vue'),
          meta: { minPermission: 3 }
        },
        {
          path: 'images',
          name: 'admin-images',
          component: () => import('@/views/admin/Images.vue'),
          meta: { minPermission: 2 }
        },
        {
          path: 'users',
          name: 'admin-users',
          component: () => import('@/views/admin/Users.vue'),
          meta: { minPermission: 4 }
        }
      ]
    },
    {
      path: '/:pathMatch(.*)*',
      name: 'not-found',
      redirect: '/'
    }
  ],
  scrollBehavior(_to, _from, savedPosition) {
    if (savedPosition) {
      return savedPosition
    } else {
      return { top: 0 }
    }
  }
})

// Navigation guard
router.beforeEach(async (to, _from, next) => {
  const authStore = useAuthStore()

  // Initialize auth state if not done
  if (!authStore.initialized) {
    await authStore.init()
  }

  // Check if route requires auth
  if (to.meta.requiresAuth) {
    if (!authStore.isAuthenticated) {
      return next({ name: 'login', query: { redirect: to.fullPath } })
    }

    // Check permission level
    const minPermission = (to.meta.minPermission as number) || 0
    if (authStore.user && authStore.user.permission < minPermission) {
      return next({ name: 'admin-dashboard' })
    }
  }

  // Redirect to admin if already logged in and going to login
  if (to.name === 'login' && authStore.isAuthenticated) {
    return next({ name: 'admin-dashboard' })
  }

  next()
})

export default router