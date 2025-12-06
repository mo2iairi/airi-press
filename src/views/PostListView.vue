<template>
  <div class="post-list-view page-container">
    <header class="page-header">
      <button class="back-btn" @click="router.push('/')">
        <ArrowLeft />
      </button>
      <h1>{{ t('posts') }}</h1>
      <button class="add-btn" @click="createNew">
        <Plus />
      </button>
    </header>

    <!-- Search & Filter Section -->
    <div class="filter-section">
      <div class="search-bar">
        <Search :size="18" class="search-icon" />
        <input 
          type="text" 
          v-model="searchQuery" 
          :placeholder="t('search_placeholder')" 
          class="search-input"
        />
      </div>
      
      <div class="filter-tools">
        <!-- Category Filter -->
        <div class="filter-group">
          <label>{{ t('filter_category') }}</label>
          <select v-model="selectedCategory">
            <option value="">{{ t('all_categories') }}</option>
            <option v-for="cat in uniqueCategories" :key="cat" :value="cat">{{ cat }}</option>
          </select>
        </div>

        <!-- Tag Filter -->
        <div class="filter-group">
          <label>{{ t('filter_tag') }}</label>
          <select v-model="selectedTag">
            <option value="">{{ t('all_tags') }}</option>
            <option v-for="tag in uniqueTags" :key="tag" :value="tag">#{{ tag }}</option>
          </select>
        </div>

        <!-- Date Filter -->
        <div class="filter-group date-range">
          <label>{{ t('filter_date') }}</label>
          <div class="date-inputs">
            <input type="date" v-model="startDate" :placeholder="t('start_date')" />
            <span class="separator">-</span>
            <input type="date" v-model="endDate" :placeholder="t('end_date')" />
          </div>
        </div>
      </div>
    </div>

    <!-- Post List -->
    <div class="posts-grid">
      <div 
        v-for="post in filteredPosts" 
        :key="post.id" 
        class="post-card"
        @click="openPost(post.id)"
      >
        <img v-if="post.imageUrl" :src="getImageUrl(post.imageUrl)" alt="Post thumbnail" class="post-image" />
        
        <div class="post-content">
          <div class="post-meta-top">
            <div class="post-date-badge">
              <span class="day">{{ getDay(post.date) }}</span>
              <span class="month">{{ getMonth(post.date) }}</span>
            </div>
            <span v-if="post.category" class="category-tag">{{ post.category }}</span>
          </div>
          
          <h2>{{ post.title }}</h2>
          <p class="desc">{{ post.description }}</p>
          <div class="tags-row">
            <span v-for="tag in post.tags" :key="tag" class="tag">#{{ tag }}</span>
          </div>
        </div>
      </div>
      
      <div v-if="filteredPosts.length === 0" class="empty-state">
        {{ t('no_posts') }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useRouter } from 'vue-router';
import { ArrowLeft, Plus, Search } from 'lucide-vue-next';
import { useI18n } from '../composables/useI18n';

interface PostMeta {
  id: string;
  title: string;
  date: string;
  description: string;
  tags: string[];
  category?: string; // Optional for now
  imageUrl?: string; // New: Optional image URL
}

const router = useRouter();
const { t } = useI18n();
const posts = ref<PostMeta[]>([]);

// Filter States
const searchQuery = ref('');
const selectedCategory = ref('');
const selectedTag = ref('');
const startDate = ref('');
const endDate = ref('');

onMounted(async () => {
  try {
    const res = await fetch(`${import.meta.env.BASE_URL}posts/index.json`);
    if (res.ok) {
      const data = await res.json();
      // Mock category for demo if missing
      posts.value = data.map((p: PostMeta) => ({
        ...p,
        category: p.category || 'General' // Default category
      }));
    }
  } catch (e) {
    console.error("Failed to load posts", e);
  }
});

// Helpers
const getDay = (dateStr: string) => {
  const date = new Date(dateStr);
  return date.getDate().toString().padStart(2, '0');
};

const getMonth = (dateStr: string) => {
  const date = new Date(dateStr);
  return date.toLocaleDateString('en-US', { month: 'short' }).toUpperCase();
};

const getImageUrl = (url: string) => {
  if (!url) return '';
  if (url.startsWith('http')) return url;
  // Remove leading slash if base url ends with one to avoid double slash
  const cleanPath = url.startsWith('/') ? url.slice(1) : url;
  return `${import.meta.env.BASE_URL}${cleanPath}`;
};

// Computed
const uniqueCategories = computed(() => {
  const cats = new Set(posts.value.map(p => p.category).filter(Boolean));
  return Array.from(cats) as string[];
});

const uniqueTags = computed(() => {
  const tags = new Set(posts.value.flatMap(p => p.tags));
  return Array.from(tags);
});

const filteredPosts = computed(() => {
  return posts.value.filter(post => {
    // Search
    const query = searchQuery.value.toLowerCase();
    const matchesSearch = post.title.toLowerCase().includes(query) || 
                          post.description.toLowerCase().includes(query);
    
    // Category
    const matchesCategory = !selectedCategory.value || post.category === selectedCategory.value;

    // Tag
    const matchesTag = !selectedTag.value || post.tags.includes(selectedTag.value);

    // Date
    let matchesDate = true;
    if (startDate.value) {
      matchesDate = matchesDate && new Date(post.date) >= new Date(startDate.value);
    }
    if (endDate.value) {
      matchesDate = matchesDate && new Date(post.date) <= new Date(endDate.value);
    }

    return matchesSearch && matchesCategory && matchesTag && matchesDate;
  });
});

const openPost = (id: string) => {
  router.push(`/post/${id}`);
};

const createNew = () => {
  router.push('/post/editor/new');
};
</script>

<style scoped>
.page-container {
  background: var(--window-bg);
  height: 100%;
  overflow-y: auto;
  padding: 20px;
  box-sizing: border-box;
  color: var(--text-color, white);
  display: flex;
  flex-direction: column;
}

.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
  padding-top: 15px;
  flex-shrink: 0;
}

.back-btn, .add-btn {
  background: none;
  border: none;
  color: inherit;
  cursor: pointer;
  padding: 8px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.back-btn:hover, .add-btn:hover {
  background: rgba(255,255,255,0.1);
}

h1 {
  font-size: 1.5rem;
  flex: 1;
  text-align: center;
  margin: 0;
}

/* Filter Section */
.filter-section {
  display: flex;
  flex-direction: column;
  gap: 15px;
  margin-bottom: 25px;
  flex-shrink: 0;
}

.search-bar {
  background: rgba(255,255,255,0.1);
  border-radius: 12px;
  padding: 10px 15px;
  display: flex;
  align-items: center;
  gap: 10px;
}

.search-icon {
  opacity: 0.6;
}

.search-input {
  background: transparent;
  border: none;
  color: inherit;
  flex: 1;
  font-size: 1rem;
  outline: none;
}

.filter-tools {
  display: flex;
  gap: 15px;
  flex-wrap: wrap;
}

.filter-group {
  display: flex;
  flex-direction: column;
  gap: 5px;
  flex: 1;
  min-width: 120px;
}

.filter-group label {
  font-size: 0.75rem;
  opacity: 0.6;
  text-transform: uppercase;
  font-weight: 600;
  padding-left: 4px;
}

.filter-group select, .date-inputs {
  background: rgba(255,255,255,0.05);
  border: 1px solid rgba(255,255,255,0.1);
  border-radius: 8px;
  padding: 8px;
  color: inherit;
  font-size: 0.9rem;
  outline: none;
  height: 36px;
  box-sizing: border-box;
  display: flex;
  align-items: center;
}

.filter-group select {
  width: 100%;
}

.filter-group select option {
  background: #333;
  color: white;
}

.date-range {
  flex: 2; /* Give date range more space */
}

.date-inputs {
  gap: 8px;
  justify-content: space-between;
}

.date-inputs input {
  background: transparent;
  border: none;
  color: inherit;
  font-family: inherit;
  width: 100%;
  outline: none;
  font-size: 0.85rem;
}

/* Post Grid */
.posts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); /* Responsive grid */
  gap: 20px;
  padding-bottom: 20px;
}

.post-card {
  background: rgba(255,255,255,0.03);
  border-radius: 16px;
  overflow: hidden; /* Ensure image and content respect border-radius */
  cursor: pointer;
  transition: all 0.2s;
  border: 1px solid rgba(255,255,255,0.1); /* Subtle border */
  box-shadow: 0 4px 15px rgba(0,0,0,0.2);
}

.post-card:hover {
  background: rgba(255,255,255,0.08);
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(0,0,0,0.3);
}

.post-image {
  width: 100%;
  height: 180px; /* Fixed height for consistency */
  object-fit: cover; /* Cover the area, crop if necessary */
  display: block;
}

.post-content {
  padding: 15px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.post-meta-top {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
}

.post-date-badge {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background: rgba(255,255,255,0.05);
  border-radius: 8px;
  width: 50px;
  height: 50px;
  flex-shrink: 0;
  color: var(--accent-color, #007aff);
}

.post-date-badge .day {
  font-size: 1.2rem;
  font-weight: 700;
  line-height: 1;
}

.post-date-badge .month {
  font-size: 0.6rem;
  text-transform: uppercase;
  opacity: 0.8;
}

h2 {
  margin: 0;
  font-size: 1.2rem;
  font-weight: 600;
  line-height: 1.3;
}

.category-tag {
  font-size: 0.7rem;
  background: var(--accent-color, #007aff);
  color: white;
  padding: 2px 8px;
  border-radius: 10px;
  opacity: 0.8;
}

.desc {
  margin: 0;
  font-size: 0.9rem;
  opacity: 0.7;
  display: -webkit-box;
  -webkit-line-clamp: 3; /* Show 3 lines of description */
  -webkit-box-orient: vertical;
  overflow: hidden;
  line-height: 1.4;
}

.tags-row {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  margin-top: 10px;
}

.tag {
  font-size: 0.75rem;
  opacity: 0.5;
}

.empty-state {
  text-align: center;
  padding: 40px;
  opacity: 0.5;
  grid-column: 1 / -1; /* Center in grid */
}
</style>
