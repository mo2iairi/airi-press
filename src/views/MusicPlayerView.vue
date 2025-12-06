<template>
  <div class="music-view page-container">
    <div class="sidebar">
      <div class="sidebar-header">
        <button class="back-btn" @click="router.push('/')">
          <ArrowLeft />
        </button>
        <button v-if="currentViewingFolderId" class="back-btn folder-back-btn" @click="goBackToParentFolder">
          <ChevronLeft />
        </button>
        <h2>{{ t('music_library') }}</h2>
      </div>

      <div class="playlist-list">
        <template v-for="(item, index) in displayedMusicItems" :key="item?.id || index">
          <div class="playlist-item"
            :class="{ 'is-folder': item.type === 'folder', 'active': currentPlaylistId === item.id }"
            @click="handleItemClick(item)">
            <img v-if="item.type !== 'folder'" :src="getCoverUrl(item)" class="pl-cover-sm" />
            <FolderIcon v-else :size="20" class="folder-icon" />
            <span class="pl-name">{{ item.name }}</span>
          </div>
        </template>
      </div>
    </div>

    <div class="main-content" v-if="currentPlaylist">
      <div class="playlist-header">
        <img :src="currentPlaylist.cover" class="pl-cover-lg" />
        <div class="pl-info">
          <h1>{{ currentPlaylist.name }}</h1>
          <p>{{ currentPlaylist.songs.length }} {{ t('songs') }}</p>
          <button class="play-all-btn" @click="playAll">
            <Play :size="18" fill="currentColor" /> {{ t('play') }}
          </button>
        </div>
      </div>

      <div class="song-list">
        <div v-for="(song, index) in currentPlaylist.songs" :key="song.id" class="song-row"
          :class="{ playing: currentSongId === song.id }" @click="playSong(song)">
          <span class="song-index">
            <span v-if="currentSongId === song.id && isPlaying" class="playing-indicator">
              <div class="bar"></div>
              <div class="bar"></div>
              <div class="bar"></div>
            </span>
            <span v-else>{{ index + 1 }}</span>
          </span>
          <div class="song-details">
            <span class="song-title">{{ song.title }}</span>
            <span class="song-artist">{{ song.artist }}</span>
          </div>
          <span class="song-duration">{{ song.duration }}</span>
        </div>
      </div>
    </div>

    <div v-if="currentLrcContent" 
         class="draggable-lyrics"
         :style="{ top: lyricsPos.y + 'px', left: lyricsPos.x + 'px' }"
         @mousedown="startDrag">
      <div class="lyrics-text">
        <span v-if="currentLyricText">{{ currentLyricText }}</span>
        <span v-else class="lyrics-placeholder">♪ ~</span>
      </div>
    </div>

    <div class="player-bar">
      <div class="pb-info">
        <img v-if="currentSong" :src="currentSong.cover" class="pb-cover" />
        <div class="pb-text">
          <div class="pb-title">{{ currentSong?.title || t('music_no_song') }}</div>
          <div class="pb-artist">{{ currentSong?.artist || '--' }}</div>
        </div>
      </div>

      <div class="pb-controls-container">
        <div class="pb-controls">
          <button @click="store.prev()">
            <SkipBack :size="20" />
          </button>
          <button @click="store.togglePlay()" class="pb-play">
            <Pause v-if="isPlaying" :size="24" fill="white" />
            <Play v-else :size="24" fill="white" />
          </button>
          <button @click="store.next()">
            <SkipForward :size="20" />
          </button>
        </div>
        <div class="pb-progress">
          <span>{{ formatTime(progress) }}</span>
          <input type="range" min="0" :max="duration || 100" :value="progress" @input="handleSeek" />
          <span>{{ formatTime(duration) }}</span>
        </div>
      </div>

      <div class="pb-volume">
        <Volume2 :size="18" />
        <input type="range" min="0" max="1" step="0.01" :value="volume" @input="updateVolume" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref, computed } from 'vue'; // 引入 onUnmounted
import { useRouter } from 'vue-router';
import { useMusicStore, type Song, type MusicItem, type Folder, type FlatPlaylist, type Asmr200Item } from '../stores/music';
import { storeToRefs } from 'pinia';
import { ArrowLeft, Play, Pause, SkipBack, SkipForward, Volume2, ChevronLeft, Folder as FolderIcon } from 'lucide-vue-next';
import { useI18n } from '../composables/useI18n';

const router = useRouter();
const store = useMusicStore();
const { musicCollection, currentPlaylistId, currentPlaylist, currentSongId, currentSong, isPlaying, progress, duration, volume, currentLrcContent } = storeToRefs(store);
const { t } = useI18n();

const currentViewingFolderId = ref<string | null>(null);

const displayedMusicItems = computed((): MusicItem[] => {
  const items = !currentViewingFolderId.value ? musicCollection.value :
    (() => {
      const currentFolder = store.findMusicItemById(musicCollection.value, currentViewingFolderId.value);
      return (currentFolder && currentFolder.type === 'folder') ? (currentFolder as Folder).children : [];
    })();

  const filteredItems = items.filter(i => i !== null && i !== undefined);
  return filteredItems;
});

const handleItemClick = (item: MusicItem) => {
  if (item.type === 'folder') {
    currentViewingFolderId.value = item.id;
  } else {
    store.selectPlaylist(item.id);
  }
};

const goBackToParentFolder = () => {
  if (!currentViewingFolderId.value) return;
  const parentId = findParentOfId(musicCollection.value, currentViewingFolderId.value);
  currentViewingFolderId.value = parentId;
};

const findParentOfId = (items: MusicItem[], childId: string): string | null => {
  for (const item of items) {
    if (item.type === 'folder' && item.children) {
      if (item.children.some(child => child.id === childId)) return item.id;
      const foundInChild = findParentOfId(item.children, childId);
      if (foundInChild) return foundInChild;
    }
  }
  return null;
};

const playSong = (song: Song) => store.playSong(song);
const playAll = () => {
  if (currentPlaylist.value && currentPlaylist.value.songs.length > 0) {
    const firstSong = currentPlaylist.value.songs[0];
    if (firstSong) store.playSong(firstSong);
  }
};
const handleSeek = (e: Event) => store.seek(Number((e.target as HTMLInputElement).value));
const updateVolume = (e: Event) => store.setVolume(Number((e.target as HTMLInputElement).value));

const formatTime = (seconds: number) => {
  if (!seconds || isNaN(seconds)) return '0:00';
  const m = Math.floor(seconds / 60);
  const s = Math.floor(seconds % 60);
  return `${m}:${s.toString().padStart(2, '0')}`;
};

const getCoverUrl = (item: MusicItem): string => {
  if (item.type !== 'folder') {
    const playableItem = item as FlatPlaylist | Asmr200Item;
    return playableItem.cover || 'music/placeholder.png';
  }
  return 'music/placeholder.png';
};

// --- LRC Logic ---
interface LyricLine {
  time: number;
  text: string;
}

const parsedLrc = computed((): LyricLine[] => {
  if (!currentLrcContent.value) return [];
  const lines = currentLrcContent.value.split('\n');
  const result: LyricLine[] = [];
  const timeRegex = /\[(\d{2}):(\d{2})\.(\d{2,3})\](.*)/;

  for (const line of lines) {
    const match = line.match(timeRegex);
    if (match) {
      const minutes = parseInt(match[1]);
      const seconds = parseInt(match[2]);
      const milliseconds = parseInt(match[3].length === 2 ? match[3] + '0' : match[3]);
      const time = minutes * 60 + seconds + milliseconds / 1000;
      const text = match[4].trim();
      if (text) result.push({ time, text });
    }
  }
  return result.sort((a, b) => a.time - b.time);
});

const currentLyricText = computed(() => {
  if (!parsedLrc.value.length) return '';
  let index = parsedLrc.value.findIndex(line => line.time > progress.value);
  if (index === -1) index = parsedLrc.value.length; 

  const currentIndex = index > 0 ? index - 1 : 0;
  return parsedLrc.value[currentIndex]?.text || '';
});

// --- 拖拽逻辑实现 ---
// 初始化位置：水平居中，垂直靠下
const lyricsPos = ref({ x: window.innerWidth / 2 - 150, y: window.innerHeight - 150 });
const isDragging = ref(false);
const dragOffset = ref({ x: 0, y: 0 });

const startDrag = (event: MouseEvent) => {
  isDragging.value = true;
  // 计算鼠标点击点与元素左上角的偏差
  dragOffset.value = {
    x: event.clientX - lyricsPos.value.x,
    y: event.clientY - lyricsPos.value.y
  };
  // 监听全局移动和松开事件，防止鼠标移出元素导致拖拽失效
  window.addEventListener('mousemove', onDrag);
  window.addEventListener('mouseup', stopDrag);
};

const onDrag = (event: MouseEvent) => {
  if (!isDragging.value) return;
  // 更新位置 = 鼠标当前位置 - 偏差值
  lyricsPos.value = {
    x: event.clientX - dragOffset.value.x,
    y: event.clientY - dragOffset.value.y
  };
};

const stopDrag = () => {
  isDragging.value = false;
  window.removeEventListener('mousemove', onDrag);
  window.removeEventListener('mouseup', stopDrag);
};

// 组件卸载时清理事件监听
onUnmounted(() => {
  window.removeEventListener('mousemove', onDrag);
  window.removeEventListener('mouseup', stopDrag);
});

onMounted(() => {
  if (musicCollection.value.length === 0) store.init();
});
</script>

<style scoped>
.music-view {
  background: var(--window-bg);
  height: 100%;
  display: flex;
  color: var(--text-color);
  overflow: hidden;
  position: relative;
}

/* ... (Sidebar 和 Main Content 样式保持不变) ... */
.sidebar {
  width: 250px;
  background: var(--sidebar-bg);
  border-right: 1px solid var(--border-color-soft);
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  padding: 35px 20px 20px 20px;
  display: flex;
  align-items: center;
  gap: 10px;
}

.back-btn {
  background: none;
  border: none;
  color: inherit;
  cursor: pointer;
}

h2 {
  margin: 0;
  font-size: 1.2rem;
  font-weight: 600;
}

.playlist-list {
  flex: 1;
  overflow-y: auto;
  padding: 10px;
}

.playlist-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.2s;
}

.playlist-item:hover {
  background: var(--list-item-hover-bg);
}

.playlist-item.active {
  background: var(--list-item-active-bg);
}

.pl-cover-sm {
  width: 40px;
  height: 40px;
  border-radius: 4px;
  object-fit: cover;
}

.pl-name {
  font-size: 0.95rem;
  font-weight: 500;
}

.folder-back-btn {
  position: absolute;
  left: 10px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-color);
}

.playlist-item.is-folder {
  font-weight: 600;
  opacity: 0.8;
  padding-left: 15px;
}

.playlist-item.is-folder .folder-icon {
  color: var(--accent-color);
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  position: relative;
}

.playlist-header {
  padding: 30px;
  display: flex;
  align-items: flex-end;
  gap: 20px;
  background: linear-gradient(to bottom, var(--playlist-header-gradient-start), transparent);
}

.pl-cover-lg {
  width: 180px;
  height: 180px;
  border-radius: 8px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
  object-fit: cover;
}

.pl-info h1 {
  font-size: 3rem;
  margin: 0 0 10px 0;
  font-weight: 800;
}

.play-all-btn {
  margin-top: 15px;
  background: var(--accent-color, #fa233b);
  color: var(--button-text-color);
  border: none;
  padding: 10px 24px;
  border-radius: 24px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
}

.song-list {
  flex: 1;
  overflow-y: auto;
  padding: 0 30px 100px 30px;
}

.song-row {
  display: flex;
  align-items: center;
  padding: 10px;
  border-radius: 8px;
  cursor: pointer;
  border-bottom: 1px solid var(--border-color-soft);
}

.song-row:hover {
  background: var(--list-item-hover-bg);
}

.song-row.playing {
  color: var(--accent-color, #fa233b);
}

.song-index {
  width: 40px;
  text-align: center;
  opacity: 0.5;
  font-size: 0.9rem;
}

.song-details {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.song-title {
  font-weight: 500;
  font-size: 1rem;
}

.song-artist {
  font-size: 0.85rem;
  opacity: 0.7;
}

.song-duration {
  opacity: 0.5;
  font-size: 0.9rem;
}

/* --- Draggable Lyrics Styles (Fixed & Floating) --- */
.draggable-lyrics {
  position: fixed; /* 关键：固定定位，脱离文档流 */
  z-index: 9999;   /* 关键：最顶层 */
  
  /* 视觉样式：毛玻璃效果 */
  background: rgba(0, 0, 0, 0.6); 
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  
  padding: 12px 24px;
  border-radius: 30px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
  border: 1px solid rgba(255, 255, 255, 0.15);
  
  /* 交互样式 */
  cursor: move;      /* 鼠标悬停显示可移动 */
  user-select: none; /* 防止选中文字 */
  
  /* 尺寸限制 */
  max-width: 80vw;
  min-width: 200px;
  text-align: center;
  
  /* 动画 */
  transition: transform 0.1s ease-out, background 0.2s;
}

.draggable-lyrics:active {
  background: rgba(0, 0, 0, 0.8);
  transform: scale(1.02);
}

.lyrics-text {
  font-weight: 700;
  font-size: 1.25rem;
  line-height: 1.4;
  color: #d05ce3; /* 默认使用亮紫色/粉色，可读性高 */
  text-shadow: 0 1px 2px rgba(0,0,0,0.8);
  
  /* 文字过长省略 */
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
}

.lyrics-placeholder {
  font-style: italic;
  opacity: 0.8;
  color: #fff;
}

/* ... (Player Bar styles same) ... */
.player-bar {
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 80px;
  background: var(--player-bar-bg);
  border-top: 1px solid var(--border-color-soft);
  display: flex;
  align-items: center;
  padding: 0 20px;
  justify-content: space-between;
  z-index: 10;
}

.pb-info {
  display: flex;
  align-items: center;
  gap: 15px;
  width: 25%;
}

.pb-cover {
  width: 50px;
  height: 50px;
  border-radius: 4px;
}

.pb-text {
  display: flex;
  flex-direction: column;
}

.pb-title {
  font-weight: 600;
  font-size: 0.9rem;
}

.pb-artist {
  font-size: 0.8rem;
  opacity: 0.7;
}

.pb-controls-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 5px;
}

.pb-controls {
  display: flex;
  align-items: center;
  gap: 20px;
}

.pb-controls button {
  background: none;
  border: none;
  color: var(--player-control-icon-color);
  cursor: pointer;
  padding: 0;
}

.pb-controls button:hover {
  color: var(--player-control-icon-hover-color);
}

.pb-play {
  width: 36px;
  height: 36px;
  background: var(--player-play-btn-bg) !important;
  border-radius: 50%;
  color: var(--player-play-btn-text) !important;
  display: flex;
  justify-content: center;
  align-items: center;
}

.pb-play :deep(svg) {
  fill: var(--player-play-btn-svg-fill);
  stroke: none;
}

.pb-progress {
  width: 100%;
  max-width: 500px;
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 0.75rem;
  opacity: 0.8;
}

.pb-progress input {
  flex: 1;
  height: 4px;
}

.pb-volume {
  width: 25%;
  display: flex;
  justify-content: flex-end;
  align-items: center;
  gap: 10px;
}

.pb-volume input {
  width: 100px;
}

.playing-indicator {
  display: flex;
  gap: 2px;
  height: 12px;
  align-items: flex-end;
  justify-content: center;
}

.bar {
  width: 3px;
  background: var(--accent-color, #fa233b);
  animation: bounce 1s infinite ease-in-out;
}

.bar:nth-child(1) {
  animation-delay: 0s;
  height: 60%;
}

.bar:nth-child(2) {
  animation-delay: 0.2s;
  height: 100%;
}

.bar:nth-child(3) {
  animation-delay: 0.4s;
  height: 40%;
}

@keyframes bounce {
  0%, 100% { height: 30%; }
  50% { height: 100%; }
}
</style>

