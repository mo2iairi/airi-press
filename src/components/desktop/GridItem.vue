<template>
  <div 
    class="grid-item"
    :style="{
      gridColumnStart: icon.x,
      gridColumnEnd: `span ${icon.w}`,
      gridRowStart: icon.y,
      gridRowEnd: `span ${icon.h}`,
    }"
    @click="handleClick"
  >
    <!-- Special Case for Clock Widget -->
    <ClockWidget v-if="icon.id === 'clock'" />

    <!-- Special Case for Music Widget -->
    <MusicWidget v-else-if="icon.id === 'music'" />

    <!-- Standard Icon -->
    <div v-else class="app-icon-container">
      <div class="app-icon" :style="{ backgroundColor: icon.bgColor || '#333' }">
        <!-- Dynamic Icon rendering using Lucide -->
        <component :is="getIcon(icon.icon)" :size="32" color="white" />
      </div>
      <span class="app-name">{{ t(icon.id) }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import type { DesktopIcon } from '../../stores/desktop';
import ClockWidget from './ClockWidget.vue';
import MusicWidget from './MusicWidget.vue';
import * as LucideIcons from 'lucide-vue-next';
import { useI18n } from '../../composables/useI18n';

const props = defineProps<{
  icon: DesktopIcon
}>();

const router = useRouter();
const { t } = useI18n();

const getIcon = (name: string) => {
  // Convert kebab-case to PascalCase (e.g. book-open -> BookOpen)
  const pascalName = name.replace(/(^\w|-\w)/g, (m) => m.replace('-', '').toUpperCase());
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const icon = (LucideIcons as any)[pascalName];
  return icon || LucideIcons.HelpCircle;
};

const handleClick = () => {
  if (props.icon.id === 'clock') return; // Clock is not clickable usually

  if (props.icon.action === 'route') {
    router.push(props.icon.url);
  } else if (props.icon.action === 'link') {
    window.open(props.icon.url, '_blank');
  }
};
</script>

<style scoped>
.grid-item {
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
  transition: transform 0.1s;
}

.grid-item:active {
  transform: scale(0.95);
}

.app-icon-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  width: 100%;
  height: 100%;
  justify-content: center;
}

.app-icon {
  width: 60px;
  height: 60px;
  border-radius: 14px;
  display: flex;
  justify-content: center;
  align-items: center;
  box-shadow: 0 4px 10px rgba(0,0,0,0.2);
}

.app-name {
  color: white;
  font-size: 12px;
  text-shadow: 0 1px 3px rgba(0,0,0,0.5);
  text-align: center;
}
</style>
