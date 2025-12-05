import { defineStore } from 'pinia';
import { ref } from 'vue';

export interface DesktopIcon {
  id: string;
  name: string;
  icon: string;
  x: number;
  y: number;
  w: number;
  h: number;
  action: 'link' | 'route';
  url: string;
  bgColor?: string;
}

export const useDesktopStore = defineStore('desktop', () => {
  const icons = ref<DesktopIcon[]>([
    {
      id: 'clock',
      name: 'Clock',
      icon: 'clock',
      x: 1,
      y: 1,
      w: 4,
      h: 2,
      action: 'route',
      url: '', // Special widget
    },
    {
      id: 'settings',
      name: '设置',
      icon: 'settings',
      x: 1,
      y: 3,
      w: 1,
      h: 1,
      action: 'route',
      url: '/setting',
      bgColor: '#8e8e93'
    },
    {
      id: 'posts',
      name: '文章',
      icon: 'book-open',
      x: 2,
      y: 3,
      w: 1,
      h: 1,
      action: 'route',
      url: '/post',
      bgColor: '#007aff'
    },
    {
      id: 'github',
      name: 'GitHub',
      icon: 'github',
      x: 3,
      y: 3,
      w: 1,
      h: 1,
      action: 'link',
      url: 'https://github.com',
      bgColor: '#24292e'
    }
  ]);

  // Future: Load from JSON
  const loadConfig = async (url: string) => {
    try {
      const res = await fetch(url);
      if (res.ok) {
        const data = await res.json();
        icons.value = data;
      }
    } catch (e) {
      console.error("Failed to load desktop config", e);
    }
  };

  return { icons, loadConfig };
});
