<template>
  <div class="settings-view page-container">
    <header class="page-header">
      <button class="back-btn" @click="handleBack">
        <ArrowLeft />
      </button>
      <h1>{{ activeItem ? t(activeItem.label_key) : t('settings') }}</h1>
    </header>

    <!-- Main List -->
    <div v-if="!activeItem" class="settings-list">
      <template v-for="section in settingsConfig" :key="section.section_key">
        <div class="section-title">{{ t(section.section_key) }}</div>
        
        <div class="setting-group">
          <template v-for="item in section.items" :key="item.id">
            
            <!-- Type: Select (Inline) -->
            <div v-if="item.type === 'select'" class="setting-item">
              <div class="label">
                <span>{{ t(item.label_key) }}</span>
              </div>
              <div class="value-container">
                <select v-model="storeAny[item.id]" class="setting-select">
                  <option 
                    v-for="option in item.options" 
                    :key="typeof option === 'string' ? option : option.value"
                    :value="typeof option === 'string' ? option : option.value"
                  >
                    {{ typeof option === 'string' ? option : t(option.label_key) }}
                  </option>
                </select>
              </div>
            </div>

            <!-- Type: Text (Read-only) -->
            <div v-else-if="item.type === 'text'" class="setting-item">
              <div class="label">
                <span>{{ t(item.label_key) }}</span>
              </div>
              <div class="value-container">
                <small>{{ item.value }}</small>
              </div>
            </div>

            <!-- Type: Complex (Navigation Link) -->
            <div 
              v-else 
              class="setting-item clickable" 
              @click="openDetail(item)"
            >
              <div class="label">
                <span>{{ t(item.label_key) }}</span>
              </div>
              <div class="value-container">
                <span v-if="item.id === 'theme'" class="current-value">{{ t(systemStore.theme) }}</span>
                <ChevronRight class="chevron" :size="20" />
              </div>
            </div>

          </template>
        </div>
      </template>
    </div>

    <!-- Detail View -->
    <div v-else class="detail-view">
      
      <!-- Theme Cards UI -->
      <div v-if="activeItem && activeItem.type === 'theme-cards'" class="theme-cards">
        <div 
          v-for="themeOption in ['light', 'dark', 'glass']" 
          :key="themeOption"
          class="theme-card" 
          :class="{ active: systemStore.theme === themeOption }"
          @click="systemStore.theme = themeOption as 'light' | 'dark' | 'glass'"
        >
          <div class="preview" :class="themeOption"></div>
          <span>{{ t(themeOption) }}</span>
        </div>
      </div>

      <!-- Wallpaper Grid UI -->
      <div v-else-if="activeItem && activeItem.type === 'wallpaper-grid'">
        <div class="wallpaper-grid">
          <div 
            v-for="file in activeItem.files" 
            :key="file" 
            class="wallpaper-item"
            :class="{ active: systemStore.wallpaper === getAssetUrl(activeItem.path + file) }"
            @click="systemStore.wallpaper = getAssetUrl(activeItem.path + file)"
            :style="{ backgroundImage: `url('${getAssetUrl(activeItem.path + file)}')` }"
          >
            <div v-if="systemStore.wallpaper === getAssetUrl(activeItem.path + file)" class="check-badge">
                <Check :size="16" />
            </div>
          </div>
        </div>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useSystemStore } from '../stores/system';
import { ArrowLeft, ChevronRight, Check } from 'lucide-vue-next';
import { useI18n } from '../composables/useI18n';

// Types for our JSON config
interface SettingItem {
  id: string;
  type: string;
  label_key: string;
  value?: string;
  options?: (string | { label_key: string, value: string })[];
  path?: string;
  files?: string[];
  display_value_key?: boolean;
  placeholder?: string; // Added placeholder
}

interface SettingSection {
  section_key: string;
  items: SettingItem[];
}

const router = useRouter();
const systemStore = useSystemStore(); 
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const storeAny = systemStore as any;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const { t } = useI18n() as any;

const settingsConfig = ref<SettingSection[]>([]); 
const activeItem = ref<SettingItem | null>(null);

onMounted(async () => {
  try {
    const response = await fetch(`${import.meta.env.BASE_URL}settings.json`);
    settingsConfig.value = await response.json();
  } catch (error) {
    console.error('Failed to load settings config:', error);
  }
});

const openDetail = (item: SettingItem) => {
  activeItem.value = item;
};

const handleBack = () => {
  if (activeItem.value) {
    activeItem.value = null;
  } else {
    router.back();
  }
};

const getAssetUrl = (path: string) => {
  // Ensure path doesn't start with / if BASE_URL ends with /
  const cleanPath = path.startsWith('/') ? path.slice(1) : path;
  return `${import.meta.env.BASE_URL}${cleanPath}`;
};
</script>

<style scoped>
.page-container {
  background: var(--window-bg);
  height: 100%;
  overflow-y: auto;
  padding: 20px;
  box-sizing: border-box;
  color: var(--text-color, white);
  display: flex;
  flex-direction: column;
}

.page-header {
  display: flex;
  align-items: center;
  gap: 15px;
  margin-bottom: 20px;
  padding-top: 15px; /* Adjusted: Reduced top padding */
  flex-shrink: 0;
}

.back-btn {
  background: none;
  border: none;
  color: inherit;
  cursor: pointer;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

h1 {
  margin: 0;
  font-size: 1.8rem;
  font-weight: 600;
}

.settings-list {
  display: flex;
  flex-direction: column;
  gap: 25px; /* Adjusted: Increased gap between sections */
  padding-bottom: 40px;
}

.section-title {
  text-transform: uppercase;
  font-size: 0.8rem;
  opacity: 0.6;
  margin-bottom: 10px; /* Adjusted: Increased margin below title */
  padding-left: 15px;
  font-weight: 600;
}

.setting-group {
  background: rgba(255,255,255,0);
  border-radius: 12px;
  /* overflow: hidden; */
  display: flex;
  flex-direction: column;
  padding: 0px 0px;
  margin: 0, 0;
}

.setting-item {
  padding: 0px 0px;
  margin: 3px 0px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  min-height: 48px;
  box-sizing: border-box;
  border-bottom: 1px solid rgba(255,255,255,0.08);
  background: transparent;
  transition: background 0.2s;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-item.clickable:hover {
  background: rgba(255,255,255,0.05);
  cursor: pointer;
}

.label {
  font-size: 1rem;
  font-weight: 500;
}

.value-container {
  display: flex;
  align-items: center;
  gap: 10px;
  opacity: 0.8;
}

.setting-select {
  background: transparent;
  border: none;
  color: inherit;
  font-size: 1rem;
  outline: none;
  text-align: right;
  cursor: pointer;
  appearance: none; 
  padding-right: 0;
}

.setting-select option {
  background: #333;
  color: white;
}

.current-value {
  font-size: 0.95rem;
  opacity: 0.7;
}

.chevron {
  opacity: 0.4;
}

/* Detail Views */
.detail-view {
  animation: fadeIn 0.3s ease;
  flex: 1;
  padding: 0 16px;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

/* Theme Cards */
.theme-cards {
  display: flex;
  justify-content: center;
  gap: 20px;
  padding: 20px 0;
}

.theme-card {
  flex: 1;
  max-width: 120px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  cursor: pointer;
}

.preview {
  width: 100%;
  aspect-ratio: 9/16; 
  border-radius: 16px;
  border: 3px solid transparent;
  box-shadow: 0 4px 20px rgba(0,0,0,0.2);
  transition: all 0.2s;
}

.preview.light { background: #f2f2f7; }
.preview.dark { background: #1c1c1e; }
.preview.glass { 
  background: linear-gradient(135deg, rgba(255,255,255,0.4), rgba(255,255,255,0.1)); 
  position: relative;
}
.preview.glass::after {
  content: '';
  position: absolute;
  inset: 0;
  backdrop-filter: blur(10px);
  border-radius: 12px;
}

.theme-card.active .preview {
  border-color: #007aff;
  transform: scale(1.05);
}

.theme-card span {
  font-size: 0.9rem;
  opacity: 0.8;
}

/* Wallpaper Grid */
.wallpaper-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
  gap: 15px;
  padding: 20px 0;
}

.wallpaper-item {
  aspect-ratio: 9/16;
  border-radius: 12px;
  background-size: cover;
  background-position: center;
  cursor: pointer;
  position: relative;
  border: 3px solid transparent;
  transition: transform 0.2s;
  box-shadow: 0 2px 10px rgba(0,0,0,0.2);
}

.wallpaper-item:active {
  transform: scale(0.96);
}

.wallpaper-item.active {
  border-color: #007aff;
}

.check-badge {
  position: absolute;
  bottom: 8px;
  right: 8px;
  background: #007aff;
  border-radius: 50%;
  width: 24px;
  height: 24px;
  display: flex;
  justify-content: center;
  align-items: center;
  color: white;
  box-shadow: 0 2px 5px rgba(0,0,0,0.3);
}
</style>

<style scoped>
.page-container {
  background: var(--window-bg);
  height: 100%;
  overflow-y: auto;
  padding: 20px;
  box-sizing: border-box;
  color: var(--text-color, white);
}

.page-header {
  display: flex;
  align-items: center;
  gap: 15px;
  margin-bottom: 30px;
  padding-top: 20px;
}

.back-btn {
  background: none;
  border: none;
  color: inherit;
  cursor: pointer;
  padding: 0;
}

h1 {
  margin: 0;
  font-size: 2rem;
}

.settings-list {
  display: flex;
  flex-direction: column;
  gap: 15px;
  padding-bottom: 40px;
}

.section-title {
  text-transform: uppercase;
  font-size: 0.8rem;
  opacity: 0.6;
  margin-top: 10px;
  margin-bottom: 5px;
  padding-left: 10px;
  font-weight: 600;
}

.setting-item {
  background: rgba(255,255,255,0.1);
  padding: 15px;
  border-radius: 12px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.setting-select {
  background: rgba(0,0,0,0.2);
  border: none;
  color: inherit;
  padding: 8px;
  border-radius: 8px;
  outline: none;
}

.label {
  display: flex;
  flex-direction: column;
}

.label small {
  opacity: 0.6;
  font-size: 0.8rem;
}

/* Theme Cards */
.theme-cards {
  display: flex;
  gap: 15px;
  padding: 0 5px;
}

.theme-card {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.preview {
  width: 100%;
  aspect-ratio: 16/9;
  border-radius: 12px;
  border: 2px solid transparent;
  box-shadow: 0 4px 10px rgba(0,0,0,0.2);
  transition: all 0.2s;
}

.preview.light { background: #f0f0f0; }
.preview.dark { background: #1c1c1e; }
.preview.glass { background: linear-gradient(135deg, rgba(255,255,255,0.4), rgba(255,255,255,0.1)); }

.theme-card.active .preview {
  border-color: #007aff;
  transform: scale(1.05);
}

/* Wallpaper Grid */
.wallpaper-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 15px;
}

.wallpaper-item {
  aspect-ratio: 16/9;
  border-radius: 12px;
  background-size: cover;
  background-position: center;
  cursor: pointer;
  position: relative;
  border: 2px solid transparent;
  transition: transform 0.2s;
}

.wallpaper-item:active {
  transform: scale(0.98);
}

.wallpaper-item.active {
  border-color: #007aff;
}

.check-badge {
  position: absolute;
  bottom: 8px;
  right: 8px;
  background: #007aff;
  border-radius: 50%;
  width: 24px;
  height: 24px;
  display: flex;
  justify-content: center;
  align-items: center;
  color: white;
}
</style>
