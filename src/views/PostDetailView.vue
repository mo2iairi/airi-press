<template>
  <div class="post-detail-view page-container">
    <header class="detail-header">
      <button class="icon-btn" @click="goBack">
        <ArrowLeft />
      </button>
      <div class="header-content">
        <h1 class="post-title">{{ form.title }}</h1>
        <p class="post-date">{{ formattedDate }}</p>
      </div>
      <button class="icon-btn primary" @click="editPost" v-if="postId !== 'new'">
        <Edit2 />
      </button>
    </header>

    <div class="post-content markdown-body" v-html="renderedContent"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, nextTick, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { ArrowLeft, Edit2 } from 'lucide-vue-next';
import MarkdownIt from 'markdown-it';
// @ts-ignore
import MarkdownItKatex from 'markdown-it-katex';
import hljs from 'highlight.js';
import { useI18n } from '../composables/useI18n';
import { parseFrontMatter } from '../utils/frontmatter';

const route = useRoute();
const router = useRouter();
const md = new MarkdownIt({
  html: true, // Enable HTML tags in source
  linkify: true, // Autoconvert URL-like text to links
  typographer: true // Enable some language-neutral replacement + quotes beautification
});
md.use(MarkdownItKatex);

// Override fence renderer

md.renderer.rules.fence = (tokens, idx, _options, _env, _self) => {
  const token = tokens[idx];
  const info = token!.info ? md.utils.unescapeAll(token!.info).trim() : '';
  let langName = '';

  if (info) {
    langName = (info.split(/\s+/g)[0] || '');
  }

  let highlighted = '';
  if (langName && hljs.getLanguage(langName)) {
    try {
      highlighted = hljs.highlight(token!.content, { language: langName, ignoreIllegals: true }).value;
    } catch (__) {
      highlighted = md.utils.escapeHtml(token!.content);
    }
  } else {
    highlighted = md.utils.escapeHtml(token!.content);
  }

  const headerHtml = `
    <div class="code-block-header">
      <span class="code-lang">${langName || 'text'}</span>
      <div class="code-actions">
        <button class="code-btn copy-btn" data-code="${encodeURIComponent(token!.content)}">Copy</button>
        <button class="code-btn download-btn" data-lang="${langName || 'txt'}" data-code="${encodeURIComponent(token!.content)}">Download</button>
      </div>
    </div>`;

  return `<div class="custom-code-block">${headerHtml}<pre><code class="hljs">${highlighted}</code></pre></div>`;
};

const { t } = useI18n();

const postId = ref(route.params.id as string);

const form = ref({
  id: '',
  title: '',
  date: '',
  tags: '',
  category: '',
  imageUrl: '',
  description: '',
  pinnedWeight: 0
});

const content = ref('');

const renderedContent = computed(() => {
  return md.render(content.value);
});

const formattedDate = computed(() => {
  if (form.value.date) {
    const date = new Date(form.value.date);
    return date.toLocaleDateString(undefined, { year: 'numeric', month: 'long', day: 'numeric' });
  }
  return '';
});

const goBack = () => {
  router.push('/post');
};

const editPost = () => {
  router.push(`/post/editor/${postId.value}`);
};

// New functions for copy/download
const setupCodeBlockActions = () => {
  // Setup Copy buttons
  document.querySelectorAll('.custom-code-block .copy-btn').forEach(button => {
    // Remove previous listeners if any to avoid duplicates
    (button as HTMLButtonElement).onclick = null; 
    (button as HTMLButtonElement).onclick = async () => {
      const codeToCopy = decodeURIComponent((button as HTMLButtonElement).dataset.code || '');
      try {
        await navigator.clipboard.writeText(codeToCopy);
        const originalText = button.textContent;
        button.textContent = 'Copied!';
        setTimeout(() => {
          button.textContent = originalText;
        }, 2000);
      } catch (err) {
        console.error('Failed to copy text: ', err);
      }
    };
  });

  // Setup Download buttons
  document.querySelectorAll('.custom-code-block .download-btn').forEach(button => {
    // Remove previous listeners if any to avoid duplicates
    (button as HTMLButtonElement).onclick = null;
    (button as HTMLButtonElement).onclick = () => {
      const codeToDownload = decodeURIComponent((button as HTMLButtonElement).dataset.code || '');
      const lang = (button as HTMLButtonElement).dataset.lang || 'txt';
      const blob = new Blob([codeToDownload], { type: 'text/plain' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `code.${lang}`;
      document.body.appendChild(a); // Required for Firefox
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
    };
  });
};

onMounted(async () => {
  if (postId.value) {
    try {
      const res = await fetch(`${import.meta.env.BASE_URL}posts/${postId.value}.md`);
      if (res.ok) {
        const text = await res.text();
        const parsed = parseFrontMatter(text);
        const attrs = parsed.attributes;
        
        form.value.id = postId.value;
        form.value.title = attrs.title || 'Untitled Post';
        form.value.date = attrs.date || '';
        form.value.tags = Array.isArray(attrs.tags) ? attrs.tags.join(', ') : (attrs.tags || '');
        form.value.category = attrs.category || 'General';
        form.value.imageUrl = attrs.imageUrl || attrs.cover || '';
        form.value.description = attrs.description || '';
        form.value.pinnedWeight = typeof attrs.pinnedWeight === 'number' ? attrs.pinnedWeight : 0;
        content.value = parsed.body;
        nextTick(() => { // Ensure DOM is updated before setting up buttons
          setupCodeBlockActions();
        });
      } else {
        // Handle post not found
        console.error(`Post with ID ${postId.value} not found.`);
        form.value.title = t('post_not_found');
        content.value = t('post_not_found_message');
                  nextTick(() => { // Ensure DOM is updated before setting up buttons          setupCodeBlockActions();
        });
      }
    } catch (e) {
      console.error("Failed to load post", e);
      form.value.title = t('error_loading_post');
      content.value = t('error_loading_post_message');
      nextTick(() => { // Ensure DOM is updated before setting up buttons
        setupCodeBlockActions();
      });
    }
  } else {
    form.value.title = t('no_post_id');
    content.value = t('no_post_id_message');
    nextTick(() => { // Ensure DOM is updated before setting up buttons
      setupCodeBlockActions();
    });
  }
});

// Watch for changes in renderedContent to re-setup listeners
watch(renderedContent, () => {
  nextTick(() => {
    setupCodeBlockActions();
  });
});
</script>

<style scoped>
.page-container {
  background: var(--window-bg);
  height: 100%;
  display: flex;
  flex-direction: column;
  color: var(--text-color, white);
}

.detail-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 15px;
  border-bottom: 1px solid var(--border-color-soft);
  padding-top: 35px; /* Consistent safe area */
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

.header-content {
  flex: 1;
  text-align: center;
  margin: 0 10px;
}

.post-title {
  margin: 0;
  font-size: 1.8rem;
  font-weight: 600;
  line-height: 1.2;
}

.post-date {
  font-size: 0.9rem;
  opacity: 0.7;
  margin-top: 5px;
}

.post-content {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
  user-select: text; /* Enable text selection for content */
}
</style>
