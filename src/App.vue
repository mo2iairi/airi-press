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
}

.os-container.dark {
  --text-color: #fff;
  --window-bg: #1c1c1e;
}

.os-container.glass {
  --text-color: #fff;
  --window-bg: rgba(28, 28, 30, 0.6);
  backdrop-filter: blur(20px);
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