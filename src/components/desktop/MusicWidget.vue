<template>
  <div class="music-widget" @click.stop="openApp">
    <!-- Background Blur -->
    <div 
      class="bg-blur" 
      :style="{ backgroundImage: `url(${currentSong && currentSong.cover ? currentSong.cover : getAssetUrl('music/placeholder.png')})` }"
    ></div>

    <div class="content">
      <div class="cover-art">
         <img v-if="currentSong" :src="currentSong.cover || getAssetUrl('music/placeholder.png')" />
         <div v-else class="placeholder-cover">
           <Music :size="32" />
         </div>
      </div>

      <div class="info">
        <div class="song-title">{{ currentSong ? currentSong.title : t('music_no_song') }}</div>
        <div class="artist">{{ currentSong ? currentSong.artist : t('music_select_playlist') }}</div>
        
        <div class="controls" @click.stop>
           <button @click="store.prev()" class="ctrl-btn sm"><SkipBack :size="16" /></button>
           <button @click="store.togglePlay()" class="ctrl-btn play">
             <Pause v-if="isPlaying" :size="20" fill="white" />
             <Play v-else :size="20" fill="white" />
           </button>
           <button @click="store.next()" class="ctrl-btn sm"><SkipForward :size="16" /></button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useMusicStore } from '../../stores/music';
import { storeToRefs } from 'pinia';
import { Play, Pause, SkipBack, SkipForward, Music } from 'lucide-vue-next';
import { useI18n } from '../../composables/useI18n';

const router = useRouter();
const store = useMusicStore();
const { currentSong, isPlaying } = storeToRefs(store);
const { t } = useI18n();

onMounted(() => {
  store.init();
});

const openApp = () => {
  router.push('/music');
};

const getAssetUrl = (path: string) => {
  const cleanPath = path.startsWith('/') ? path.slice(1) : path;
  return `${import.meta.env.BASE_URL}${cleanPath}`;
};
</script>

<style scoped>
.music-widget {
  width: 100%;
  height: 100%;
  border-radius: 20px;
  background: #2c2c2e;
  position: relative;
  overflow: hidden;
  cursor: pointer;
  box-shadow: 0 4px 15px rgba(0,0,0,0.3);
  transition: transform 0.1s;
}

.music-widget:active {
  transform: scale(0.98);
}

.bg-blur {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-size: cover;
  background-position: center;
  filter: blur(20px) brightness(0.4);
  z-index: 0;
}

.content {
  position: relative;
  z-index: 1;
  height: 100%;
  display: flex;
  align-items: center;
  padding: 20px;
  gap: 20px;
  color: white;
}

.cover-art {
  width: 100px;
  height: 100px;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 8px 16px rgba(0,0,0,0.3);
  flex-shrink: 0;
}

.cover-art img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.placeholder-cover {
  width: 100%;
  height: 100%;
  background: rgba(255,255,255,0.1);
  display: flex;
  justify-content: center;
  align-items: center;
}

.info {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  min-width: 0; /* text truncate fix */
}

.song-title {
  font-size: 1.1rem;
  font-weight: 700;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-bottom: 4px;
}

.artist {
  font-size: 0.9rem;
  opacity: 0.7;
  margin-bottom: 16px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.controls {
  display: flex;
  align-items: center;
  gap: 16px;
}

.ctrl-btn {
  background: none;
  border: none;
  color: white;
  cursor: pointer;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: transform 0.1s;
}

.ctrl-btn:hover {
  opacity: 0.8;
}

.ctrl-btn:active {
  transform: scale(0.9);
}

.play {
  width: 40px;
  height: 40px;
  background: white;
  border-radius: 50%;
  color: black; /* Icon color */
}

.play :deep(svg) {
  fill: black; /* Ensure svg fill is black */
  stroke: none; /* Solid fill style */
}

/* Special override since Play/Pause in lucide are usually strokes */
.play :deep(svg) {
  fill: black !important;
  stroke: black !important;
}
/* Actually, lucide icons use stroke by default. For solid play button, fill is better. */
</style>
