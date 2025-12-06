import { createRouter, createWebHistory } from 'vue-router';
import DesktopView from '../views/DesktopView.vue';
import SettingsView from '../views/SettingsView.vue';
import PostListView from '../views/PostListView.vue';
import PostEditorView from '../views/PostEditorView.vue';
import PostDetailView from '../views/PostDetailView.vue';
import CalculatorView from '../views/CalculatorView.vue';
import MusicPlayerView from '../views/MusicPlayerView.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', component: DesktopView },
    { path: '/setting', component: SettingsView },
    { path: '/post', component: PostListView },
    { path: '/post/:id', component: PostDetailView }, // New route for viewing a post
    { path: '/post/editor/:id?', component: PostEditorView },
    { path: '/calculator', component: CalculatorView },
    { path: '/music', component: MusicPlayerView },
  ]
});

export default router;
