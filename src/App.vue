<template>
  <div 
    class="os-container" 
    :class="theme"
    :style="{ backgroundImage: `url('${wallpaper}')` }"
  >
    <StatusBar />
    
    <router-view v-slot="{ Component }">
      <transition name="app-zoom" mode="out-in">
        <component :is="Component" />
      </transition>
    </router-view>
  </div>
</template>

<script setup lang="ts">
import StatusBar from './components/system/StatusBar.vue';
import { useSystemStore } from './stores/system';
import { storeToRefs } from 'pinia';

const systemStore = useSystemStore();
const { theme, wallpaper } = storeToRefs(systemStore);
</script>

<style scoped>
.os-container {
  width: 100vw;
  height: 100vh;
  background-color: #1c1c1e;
  background-size: cover;
  background-position: center;
  position: relative;
  transition: background-image 0.5s ease;
}

.os-container.light {
  --text-color: #000;
  --window-bg: rgba(255,255,255,0.9);

  /* New variables for light theme */
  --border-color-soft: rgba(0,0,0,0.1);
  --code-inline-bg: rgba(27,31,35,0.05);
  --code-block-bg: #f6f8fa;
  --code-block-header-bg: #e0e2e5;
  --code-block-header-text: #586069;
  --code-button-bg: rgba(0, 0, 0, 0.1);
  --code-button-hover-bg: rgba(0, 0, 0, 0.2);
  --code-button-text: #333;
  --blockquote-color: #586069;
  --blockquote-border: #dfe2e5;

  --header-bg: rgba(240,240,240,0.9);
  --sidebar-bg: rgba(240,240,240,0.9);
  --sidebar-button-text-inactive: rgba(0,0,0,0.6);
  --sidebar-button-bg-active: rgba(0,0,0,0.1);
  --calc-display-bg: rgba(220,220,220,0.9);
  --calc-button-bg: rgba(0,0,0,0.05);
  --calc-button-active-bg: rgba(0,0,0,0.1);
  --calc-operator-button-bg: #ff9f0a;
  --calc-action-button-bg: #32d74b;
  --input-bg: rgba(0,0,0,0.05);
  --input-border: rgba(0,0,0,0.1);
  --button-text-color: #333;
  --result-bg: rgba(0,0,0,0.05);

  --list-item-hover-bg: rgba(0,0,0,0.05);
  --list-item-active-bg: rgba(0,0,0,0.1);
  --playlist-header-gradient-start: rgba(0,0,0,0.05);
  --player-bar-bg: #f0f0f0;
  --player-control-icon-color: rgba(0,0,0,0.8);
  --player-control-icon-hover-color: black;
  --player-play-btn-bg: black;
  --player-play-btn-text: white;
  --player-play-btn-svg-fill: white;
}

.os-container.dark {
  --text-color: #fff;
  --window-bg: #1c1c1e;
  
  /* New variables for dark theme */
  --border-color-soft: rgba(255,255,255,0.1);
  --code-inline-bg: rgba(27,31,35,0.2);
  --code-block-bg: #282c34;
  --code-block-header-bg: #21252b;
  --code-block-header-text: #9da5b4;
  --code-button-bg: rgba(255, 255, 255, 0.1);
  --code-button-hover-bg: rgba(255, 255, 255, 0.2);
  --code-button-text: #abb2bf;
  --blockquote-color: #6a737d;
  --blockquote-border: rgba(255,255,255,0.2); /* Adjusted for dark theme */

  --header-bg: rgba(0,0,0,0.2);
  --sidebar-bg: rgba(0,0,0,0.2);
  --sidebar-button-text-inactive: rgba(255,255,255,0.6);
  --sidebar-button-bg-active: rgba(255,255,255,0.1);
  --calc-display-bg: rgba(0,0,0,0.3);
  --calc-button-bg: rgba(255,255,255,0.1);
  --calc-button-active-bg: rgba(255,255,255,0.2);
  --calc-operator-button-bg: #ff9f0a;
  --calc-action-button-bg: #32d74b;
  --input-bg: rgba(255,255,255,0.1);
  --input-border: rgba(255,255,255,0.1);
  --button-text-color: white;
  --result-bg: rgba(255,255,255,0.05);

  --list-item-hover-bg: rgba(255,255,255,0.05);
  --list-item-active-bg: rgba(255,255,255,0.1);
  --playlist-header-gradient-start: rgba(255,255,255,0.1);
  --player-bar-bg: #282828;
  --player-control-icon-color: rgba(255,255,255,0.8);
  --player-control-icon-hover-color: white;
  --player-play-btn-bg: white;
  --player-play-btn-text: black;
  --player-play-btn-svg-fill: black;
}

.os-container.glass {
  --text-color: #fff;
  --window-bg: rgba(28, 28, 30, 0.6);
  backdrop-filter: blur(20px);

  /* New variables for glass theme */
  --border-color-soft: rgba(255,255,255,0.1);
  --code-inline-bg: rgba(27,31,35,0.3);
  --code-block-bg: rgba(40, 44, 52, 0.7);
  --code-block-header-bg: rgba(33, 37, 43, 0.8);
  --code-block-header-text: #9da5b4;
  --code-button-bg: rgba(255, 255, 255, 0.15);
  --code-button-hover-bg: rgba(255, 255, 255, 0.25);
  --code-button-text: #abb2bf;
  --blockquote-color: #b0b8c0;
  --blockquote-border: rgba(223, 226, 229, 0.5);

  --header-bg: rgba(0,0,0,0.1);
  --sidebar-bg: rgba(0,0,0,0.1);
  --sidebar-button-text-inactive: rgba(255,255,255,0.6);
  --sidebar-button-bg-active: rgba(255,255,255,0.1);
  --calc-display-bg: rgba(0,0,0,0.2);
  --calc-button-bg: rgba(255,255,255,0.1);
  --calc-button-active-bg: rgba(255,255,255,0.2);
  --calc-operator-button-bg: #ff9f0a;
  --calc-action-button-bg: #32d74b;
  --input-bg: rgba(255,255,255,0.1);
  --input-border: rgba(255,255,255,0.1);
  --button-text-color: white;
  --result-bg: rgba(255,255,255,0.05);

  --list-item-hover-bg: rgba(255,255,255,0.05);
  --list-item-active-bg: rgba(255,255,255,0.1);
  --playlist-header-gradient-start: rgba(255,255,255,0.1);
  --player-bar-bg: rgba(40,40,40,0.8);
  --player-control-icon-color: rgba(255,255,255,0.8);
  --player-control-icon-hover-color: white;
  --player-play-btn-bg: white;
  --player-play-btn-text: black;
  --player-play-btn-svg-fill: black;
}

/* Transitions */
.app-zoom-enter-active,
.app-zoom-leave-active {
  transition: all 0.3s ease;
}

.app-zoom-enter-from {
  opacity: 0;
  transform: scale(0.9);
}

.app-zoom-leave-to {
  opacity: 0;
  transform: scale(1.1);
}
</style>