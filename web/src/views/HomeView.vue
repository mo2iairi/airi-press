<template>
  <div class="home">
    <header class="home-header">
      <h1 class="home-title">
        <span class="title-accent">✦</span> Airi Press
      </h1>
      <p class="home-subtitle">探索思想，记录灵感</p>
    </header>

    <div v-if="postStore.loading" class="spinner"></div>

    <div v-else-if="postStore.posts.length === 0" class="empty-state glass-card">
      <p>暂无文章</p>
    </div>

    <div v-else class="posts-grid">
      <PostCard
        v-for="post in postStore.posts"
        :key="post.id"
        :post="post"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { usePostStore } from '@/stores/posts'
import PostCard from '@/components/PostCard.vue'

const postStore = usePostStore()

onMounted(() => {
  postStore.fetchPosts()
})
</script>

<style scoped>
.home-header {
  text-align: center;
  padding: 3rem 0;
}

.home-title {
  font-size: 2.5rem;
  font-weight: 700;
  letter-spacing: -0.03em;
  margin-bottom: 0.5rem;
}

.title-accent {
  color: var(--accent);
}

.home-subtitle {
  color: var(--text-secondary);
  font-size: 1.1rem;
}

.posts-grid {
  display: grid;
  gap: 1.5rem;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
}

.empty-state {
  text-align: center;
  padding: 3rem;
  color: var(--text-muted);
}

.empty-state:hover {
  transform: none;
}

@media (max-width: 768px) {
  .home-title {
    font-size: 1.8rem;
  }

  .posts-grid {
    grid-template-columns: 1fr;
  }
}
</style>
