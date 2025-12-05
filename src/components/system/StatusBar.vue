<template>
  <!-- Persistent Status Strip -->
  <div class="status-bar-strip" @click="open">
    <div class="sb-left">
      <span class="sb-time">{{ timeString }}</span>
    </div>
    <div class="sb-right">
      <Signal :size="16" class="sb-icon" />
      <Wifi :size="16" class="sb-icon" />
      <div class="battery-container">
        <span class="battery-text">100%</span>
        <Battery :size="16" class="sb-icon" />
      </div>
    </div>
  </div>

  <!-- Control Center Overlay -->
  <div class="status-bar-overlay" :class="{ open: isStatusBarOpen }" @click.self="close">
    <div class="status-panel">
      <div class="handle-bar"></div>
      
      <div class="panel-grid" v-if="config">
        <!-- Connectivity Toggles (Dynamic) -->
        <div class="panel-module square-group">
           <div 
            v-for="toggle in config.toggles" 
            :key="toggle.id"
            class="toggle-btn" 
            :class="{ active: toggle.active }"
            @click="toggle.active = !toggle.active"
           >
             <component :is="getIcon(toggle.icon)" :size="20" />
           </div>
        </div>

        <!-- Sliders -->
        <div class="panel-module sliders-group">
          <div class="slider-row">
            <Sun class="icon" :size="20" />
            <div class="slider-track">
              <div class="slider-fill" :style="{ width: brightnessLocal + '%' }"></div>
               <input 
                type="range" 
                min="10" 
                max="100" 
                v-model="brightnessLocal" 
                @input="updateBrightness"
              />
            </div>
          </div>
          <div class="slider-row">
            <Volume2 class="icon" :size="20" />
             <div class="slider-track">
              <div class="slider-fill" :style="{ width: volumeLocal + '%' }"></div>
              <input 
                type="range" 
                min="0" 
                max="100" 
                v-model="volumeLocal" 
                @input="updateVolume"
              />
            </div>
          </div>
        </div>

         <!-- Shortcuts (Dynamic) -->
        <div class="panel-module shortcuts-row">
          <div 
            v-for="shortcut in config.shortcuts" 
            :key="shortcut.id"
            class="shortcut-btn" 
            @click="handleShortcut(shortcut)"
          >
            <div class="btn-icon">
              <component :is="getIcon(shortcut.icon)" :size="24" />
            </div>
            <span>{{ t(shortcut.label_key) }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue';
import { useSystemStore } from '../../stores/system';
import { storeToRefs } from 'pinia';
import { useRouter } from 'vue-router';
import * as LucideIcons from 'lucide-vue-next';
import { 
  Sun, Volume2, 
  Wifi, Battery, Signal
} from 'lucide-vue-next';
import { useI18n } from '../../composables/useI18n';

// Types for Config
interface ToggleConfig {
  id: string;
  icon: string;
  active: boolean;
}

interface ShortcutConfig {
  id: string;
  icon: string;
  label_key: string;
  action: 'route' | 'link';
  url: string;
}

interface StatusBarConfig {
  toggles: ToggleConfig[];
  shortcuts: ShortcutConfig[];
}

const store = useSystemStore();
const { isStatusBarOpen, brightness, volume, timezone } = storeToRefs(store);
const router = useRouter();
const { t } = useI18n();

const brightnessLocal = ref(brightness.value);
const volumeLocal = ref(volume.value);
const timeString = ref('');
const config = ref<StatusBarConfig | null>(null);

watch(brightness, (val) => brightnessLocal.value = val);
watch(volume, (val) => volumeLocal.value = val);

const updateBrightness = () => {
  store.setBrightness(Number(brightnessLocal.value));
};

const updateVolume = () => {
  store.volume = Number(volumeLocal.value);
};

const open = () => {
  store.toggleStatusBar(true);
};

const close = () => {
  store.toggleStatusBar(false);
};

// Helper to dynamically load icons
const getIcon = (name: string) => {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const icon = (LucideIcons as any)[name];
  return icon || LucideIcons.HelpCircle;
};

const handleShortcut = (shortcut: ShortcutConfig) => {
  if (shortcut.action === 'route') {
    router.push(shortcut.url);
    close();
  } else if (shortcut.action === 'link') {
    window.open(shortcut.url, '_blank');
    close();
  }
};

// Time update logic
let timer: number;
const updateTime = () => {
  const now = new Date();
  try {
    timeString.value = now.toLocaleTimeString([], { 
      hour: '2-digit', 
      minute: '2-digit', 
      timeZone: timezone.value 
    });
  } catch {
    timeString.value = now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  }
};

onMounted(async () => {
  updateTime();
  timer = setInterval(updateTime, 1000);

  // Load Config
  try {
    const res = await fetch('/status-bar.json');
    if (res.ok) {
      config.value = await res.json();
    }
  } catch (e) {
    console.error("Failed to load status bar config", e);
  }
});

watch(timezone, updateTime);

onUnmounted(() => {
  clearInterval(timer);
});
</script>

<style scoped>
/* Strip Styles */
.status-bar-strip {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 44px; /* iOS style height */
  padding: 0 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  z-index: 10000;
  color: white;
  font-weight: 600;
  font-size: 14px;
  box-sizing: border-box;
  cursor: pointer;
  text-shadow: 0 1px 2px rgba(0,0,0,0.3);
  background: linear-gradient(to bottom, rgba(0,0,0,0.4), transparent);
}

.sb-left, .sb-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.battery-container {
  display: flex;
  align-items: center;
  gap: 4px;
}

/* Overlay Styles */
.status-bar-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 9999;
  background: rgba(0,0,0,0.01); /* Invisible trigger area elsewhere? No, backdrop */
  pointer-events: none;
  transition: backdrop-filter 0.3s;
}

.status-bar-overlay.open {
  pointer-events: auto;
  background: rgba(0,0,0,0.2);
  backdrop-filter: blur(5px);
}

.status-panel {
  position: absolute;
  top: 10px;
  /* Center the panel for a generic 'mobile/tablet' feel */
  left: 0;
  right: 0;
  margin: 0 auto;
  width: 360px; /* Fixed width for desktop view */
  max-width: 95%; /* Responsive margin */
  
  background: rgba(30, 30, 35, 0.85);
  backdrop-filter: blur(30px);
  -webkit-backdrop-filter: blur(30px);
  border-radius: 24px;
  padding: 24px;
  box-shadow: 0 20px 40px rgba(0,0,0,0.4);
  display: flex;
  flex-direction: column;
  gap: 20px;
  color: white;
  transform: translateY(-120%);
  transition: transform 0.35s cubic-bezier(0.32, 1, 0.32, 1);
  border: 1px solid rgba(255,255,255,0.1);
}

/* Mobile adaptation */
@media (max-width: 600px) {
  .status-panel {
    top: 0;
    width: 100%;
    max-width: none;
    border-radius: 0 0 24px 24px;
    padding-top: 50px; /* clear status bar */
    border-left: none;
    border-right: none;
    border-top: none;
  }
}

.status-bar-overlay.open .status-panel {
  transform: translateY(0);
}

.handle-bar {
  width: 40px;
  height: 4px;
  background: rgba(255,255,255,0.2);
  border-radius: 2px;
  align-self: center;
  position: absolute;
  bottom: 8px;
  display: none; /* Hidden for new desktop style, maybe visible on mobile */
}

@media (max-width: 600px) {
  .handle-bar { display: block; }
}

/* Panel Modules */
.panel-grid {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.panel-module {
  background: rgba(255,255,255,0.08);
  border-radius: 16px;
  padding: 16px;
}

/* Connectivity Square */
.square-group {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
  padding: 16px;
}

.toggle-btn {
  aspect-ratio: 1;
  border-radius: 50%;
  background: rgba(255,255,255,0.1);
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  transition: all 0.2s;
}

.toggle-btn.active {
  background: #007aff;
  color: white;
}

.toggle-btn:hover {
  transform: scale(1.05);
}

/* Sliders */
.sliders-group {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.slider-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.slider-track {
  flex: 1;
  height: 32px;
  background: rgba(255,255,255,0.1);
  border-radius: 16px;
  position: relative;
  overflow: hidden;
  display: flex;
  align-items: center;
  /* Removed padding to allow fill to reach edges */
}

.slider-fill {
  position: absolute;
  left: 0;
  top: 0;
  height: 100%;
  background: white;
  border-radius: 16px 0 0 16px; /* Rounded left only unless full */
  pointer-events: none;
  z-index: 1;
}

.slider-track input {
  width: 100%;
  height: 100%;
  opacity: 0; /* Hide default input completely */
  cursor: pointer;
  position: absolute;
  z-index: 2;
  margin: 0;
  padding: 0;
}

/* Shortcuts */
.shortcuts-row {
  display: flex;
  justify-content: space-around;
  background: transparent;
  padding: 0;
}

.shortcut-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  width: 70px;
}

.btn-icon {
  width: 50px;
  height: 50px;
  background: rgba(255,255,255,0.1);
  border-radius: 14px;
  display: flex;
  justify-content: center;
  align-items: center;
  transition: background 0.2s;
}

.shortcut-btn:hover .btn-icon {
  background: rgba(255,255,255,0.2);
}

.shortcut-btn span {
  font-size: 12px;
  opacity: 0.8;
}
</style>
