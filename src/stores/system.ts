import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useSystemStore = defineStore('system', () => {
  const brightness = ref(100);
  const volume = ref(50);
  const language = ref('zh');
  const theme = ref<'light' | 'dark' | 'glass'>('dark');
  const isStatusBarOpen = ref(false);
  
  // Default wallpaper
  const wallpaper = ref(`${import.meta.env.BASE_URL}bg/プロジェクトセカイ カラフルステージ！ feat. 初音ミク/1005b.webp`);
  
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
    toggleStatusBar,
    setBrightness
  };
});
