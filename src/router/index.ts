import { createRouter, createWebHistory } from 'vue-router';
import DesktopView from '../views/DesktopView.vue';
import SettingsView from '../views/SettingsView.vue';
import PostListView from '../views/PostListView.vue';
import PostEditorView from '../views/PostEditorView.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', component: DesktopView },
    { path: '/setting', component: SettingsView },
    { path: '/post', component: PostListView },
    { path: '/post/editor/:id?', component: PostEditorView },
  ]
});

export default router;
