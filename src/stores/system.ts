import { defineStore } from 'pinia';
import { ref } from 'vue';
import { useStorage } from '@vueuse/core';

export const useSystemStore = defineStore('system', () => {
  const brightness = ref(100);
  const volume = ref(50);
  const language = useStorage('language', 'zh');
  const theme = useStorage<'light' | 'dark' | 'glass'>('theme', 'dark');
  const isStatusBarOpen = ref(false);
  
  // GitHub Config (Persisted)
  const githubToken = useStorage('gh_token', '');
  const githubOwner = useStorage('gh_owner', 'mo2iairi');
  const githubRepo = useStorage('gh_repo', 'airi-press');
  const githubBranch = useStorage('gh_branch', 'main');

  // Default wallpaper
  const wallpaper = useStorage('wallpaper', `${import.meta.env.BASE_URL}wallpaper/プロジェクトセカイ カラフルステージ！ feat. 初音ミク/1005b.webp`);
  
  const timezone = ref(Intl.DateTimeFormat().resolvedOptions().timeZone);

  const toggleStatusBar = (state?: boolean) => {
    isStatusBarOpen.value = state ?? !isStatusBarOpen.value;
  };

  const setBrightness = (val: number) => {
    brightness.value = val;
    // Apply to body filter
    document.documentElement.style.filter = `brightness(${val}%)`;
  };

  return {
    brightness,
    volume,
    language,
    theme,
    wallpaper,
    timezone,
    isStatusBarOpen,
    githubToken,
    githubOwner,
    githubRepo,
    githubBranch,
    toggleStatusBar,
    setBrightness
  };
});
