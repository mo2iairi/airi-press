<template>
  <div class="editor-view page-container">
    <header class="editor-header">
      <button class="icon-btn" @click="goBack">
        <ArrowLeft />
      </button>
      <div class="mode-toggle">
        <button :class="{ active: !isPreview }" @click="isPreview = false">{{ t('edit_mode') }}</button>
        <button :class="{ active: isPreview }" @click="isPreview = true">{{ t('preview_mode') }}</button>
      </div>
      <button class="icon-btn primary" @click="savePost">
        <Save />
      </button>
    </header>

    <div class="editor-content" v-if="!isPreview">
      <input 
        v-model="form.title" 
        class="title-input" 
        :placeholder="t('post_title')"
      />
      
      <div class="meta-grid">
        <div class="meta-item">
          <label>{{ t('post_id') }}</label>
          <input 
            v-model="form.id" 
            class="meta-input" 
            placeholder="auto-generated"
            :readonly="!isNewPost"
            :class="{ 'readonly': !isNewPost }"
          />
        </div>
        <div class="meta-item">
          <label>{{ t('post_date') }}</label>
          <input v-model="form.date" type="date" class="meta-input" />
        </div>
        <div class="meta-item">
          <label>{{ t('post_category') }}</label>
          <input v-model="form.category" class="meta-input" placeholder="Category" />
        </div>
        <div class="meta-item">
          <label>{{ t('post_tags') }}</label>
          <input v-model="form.tags" class="meta-input" :placeholder="t('post_tags')" />
        </div>
        <div class="meta-item full-width">
          <label>{{ t('post_cover') }}</label>
          <input v-model="form.imageUrl" class="meta-input" placeholder="https://example.com/cover.jpg" />
        </div>
        <div class="meta-item full-width">
          <label>{{ t('post_description') }}</label>
          <textarea v-model="form.description" class="meta-input" :placeholder="t('post_description')" rows="2"></textarea>
        </div>
        <div class="meta-item">
          <label>{{ t('post_pinned_weight') }}</label>
          <input v-model.number="form.pinnedWeight" type="number" class="meta-input" min="0" max="100" />
        </div>
      </div>

      <textarea 
        v-model="content" 
        class="markdown-input"
        :placeholder="t('start_writing')"
      ></textarea>
    </div>

    <div class="preview-content markdown-body" v-else v-html="renderedContent"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { ArrowLeft, Save } from 'lucide-vue-next';
import MarkdownIt from 'markdown-it';
// @ts-ignore
import MarkdownItKatex from 'markdown-it-katex';
import { parseFrontMatter } from '../utils/frontmatter';
import { useI18n } from '../composables/useI18n';

const route = useRoute();
const router = useRouter();
const md = new MarkdownIt();
md.use(MarkdownItKatex);

const { t } = useI18n();

const isPreview = ref(false);
const postId = ref(route.params.id as string); // Renamed `id` to `postId` to avoid conflict with `form.id`
const isNewPost = computed(() => postId.value === 'new');

const form = ref({
  id: '', // Unique ID for the post, might be auto-generated
  title: '',
  date: new Date().toISOString().split('T')[0],
  tags: '',
  category: 'General',
  imageUrl: '',
  description: '',
  pinnedWeight: 0
});

const content = ref('');

// Auto-generate ID from title if new post and title changes
watch(() => form.value.title, (newTitle) => {
  if (isNewPost.value && !form.value.id) {
    form.value.id = newTitle.toLowerCase().replace(/\s+/g, '-').replace(/[^a-z0-9-]/g, '');
  }
});

const renderedContent = computed(() => {
  return md.render(content.value);
});

onMounted(async () => {
  if (!isNewPost.value) {
    form.value.id = postId.value; // Set form.id for existing posts
    try {
      const res = await fetch(`${import.meta.env.BASE_URL}posts/${postId.value}.md`);
      if (res.ok) {
        const text = await res.text();
        const parsed = parseFrontMatter(text);
        const attrs = parsed.attributes;
        
        form.value.title = attrs.title || '';
        form.value.date = attrs.date ? new Date(attrs.date).toISOString().split('T')[0] : '';
        form.value.tags = Array.isArray(attrs.tags) ? attrs.tags.join(', ') : (attrs.tags || '');
        form.value.category = attrs.category || 'General';
        form.value.imageUrl = attrs.imageUrl || attrs.cover || '';
        form.value.description = attrs.description || '';
        form.value.pinnedWeight = typeof attrs.pinnedWeight === 'number' ? attrs.pinnedWeight : 0;
        content.value = parsed.body;
      }
    } catch (e) {
      console.error("Failed to load post", e);
    }
  } else {
    // For new post, pre-fill ID based on current timestamp to make it unique quickly
    form.value.id = `post-${Date.now()}`;
  }
});

const goBack = () => {
  router.back();
};

const savePost = async () => {
  // Ensure ID is set for new posts based on title if not manual
  if (isNewPost.value && form.value.id === `post-${Date.now()}` && form.value.title) {
     form.value.id = form.value.title.toLowerCase().replace(/\s+/g, '-').replace(/[^a-z0-9-]/g, '');
  } else if (isNewPost.value && !form.value.id) {
    alert('Please enter a title to generate an ID, or manually enter an ID.');
    return;
  }
  
  // Construct file content
  const tagsArray = form.value.tags.split(',').map(t => t.trim()).filter(t => t);
  const frontmatter = [
    '---',
    `id: ${form.value.id}`,
    `title: ${form.value.title}`,
    `date: ${form.value.date}`,
    `category: ${form.value.category}`,
    `tags: [${tagsArray.join(', ')}]`,
    `imageUrl: ${form.value.imageUrl}`,
    `description: ${form.value.description}`,
    `pinnedWeight: ${form.value.pinnedWeight}`,
    '---',
    '',
    content.value
  ].join('\n');

  // Trigger Save
  try {
    if (import.meta.env.VITE_LOCAL_MODE === 'true') {
      // Local Dev Mode: Auto-save to disk via Vite middleware
      const filename = `${form.value.id}.md`; // Use form.id for filename
      
      const res = await fetch('/api/save-post', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          filename,
          content: frontmatter
        })
      });

      if (res.ok) {
        alert('Saved to disk & Index updated!');
        if (isNewPost.value) {
          router.replace(`/post/editor/${form.value.id}`); // Replace route to new ID
        }
      } else {
        throw new Error('Server failed to save');
      }

    } else {
      // Production Mode: Redirect to Headless CMS
      alert('In production, please use the CMS Dashboard to edit content.');
      window.location.href = `${import.meta.env.BASE_URL}/admin/`;
    }
  } catch (e) {
    console.error("Save cancelled or failed", e);
    alert('Save failed: ' + e);
  }
};
</script>

<style scoped>
.page-container {
  background: var(--window-bg);
  height: 100%;
  display: flex;
  flex-direction: column;
}

.editor-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 15px;
  border-bottom: 1px solid rgba(255,255,255,0.1);
  padding-top: 35px; /* Safe area */
}

.icon-btn {
  background: none;
  border: none;
  color: inherit;
  cursor: pointer;
  padding: 8px;
}

.icon-btn.primary {
  color: var(--accent-color);
}

.mode-toggle {
  background: rgba(0,0,0,0.3);
  border-radius: 20px;
  padding: 4px;
  display: flex;
}

.mode-toggle button {
  background: none;
  border: none;
  color: rgba(255,255,255,0.5);
  padding: 6px 16px;
  border-radius: 16px;
  font-size: 0.9rem;
  cursor: pointer;
}

.mode-toggle button.active {
  background: rgba(255,255,255,0.1);
  color: white;
}

.editor-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 20px;
  gap: 15px;
  overflow-y: auto;
}

.title-input {
  background: none;
  border: none;
  color: white;
  font-size: 1.8rem;
  font-weight: bold;
  outline: none;
  width: 100%;
}

.meta-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 10px;
}

.meta-item {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.meta-item label {
  font-size: 0.8rem;
  opacity: 0.6;
  padding-left: 4px;
}

.meta-item.full-width {
  grid-column: 1 / -1;
}

.meta-input {
  background: rgba(255,255,255,0.05);
  border: none;
  border-radius: 8px;
  color: white;
  padding: 8px 12px;
  flex: 1;
  outline: none;
  font-size: 0.9rem;
}

.meta-input.readonly {
  opacity: 0.7;
  cursor: not-allowed;
}

.markdown-input {
  flex: 1;
  background: none;
  border: none;
  color: rgba(255,255,255,0.9);
  font-size: 1rem;
  font-family: monospace;
  resize: none;
  outline: none;
  line-height: 1.6;
}

.preview-content {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
  background: white; /* Preview usually needs light mode or specific dark CSS */
  color: #333;
}
</style>
