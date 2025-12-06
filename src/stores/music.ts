import { defineStore } from 'pinia';
import { ref, computed, watch } from 'vue';
import { useSystemStore } from './system';

export interface Song {
  id: string;
  title: string;
  artist: string;
  url: string;
  duration: string;
  cover?: string;
  lrcUrl?: string;
  lrcContent?: string;
}

export interface FlatPlaylist {
  id: string;
  name: string;
  cover: string;
  songs: Song[];
  type?: 'local' | 'asmr-200';
  sourceId?: string;
}

export interface Folder {
  id: string;
  name: string;
  type: 'folder';
  children: MusicItem[];
}

export interface Asmr200Item {
  id: string;
  name: string;
  type: 'asmr-200';
  sourceId: string;
  cover?: string;
}

export type MusicItem = Folder | FlatPlaylist | Asmr200Item;

export const useMusicStore = defineStore('music', () => {
  const ASMR_API_BASE = (() => {
    if (import.meta.env.VITE_DEPLOY_TARGET === 'github') {
      return 'https://api.asmr-200.com';
    }
    return '/api-asmr200';
  })();
  const systemStore = useSystemStore();
  const musicCollection = ref<MusicItem[]>([]);
  const currentPlaylistId = ref<string | null>(null);
  const currentSongId = ref<string | null>(null);
  const isPlaying = ref(false);
  const activePlayablePlaylist = ref<FlatPlaylist | null>(null);
  const currentLrcContent = ref<string | null>(null);
  
  const volume = ref(systemStore.volume / 100); 
  const progress = ref(0);
  const duration = ref(0);

  const audio = new Audio();
  audio.volume = volume.value;

  watch(() => systemStore.volume, (newVal) => {
    const norm = newVal / 100;
    if (Math.abs(audio.volume - norm) > 0.01) {
      audio.volume = norm;
      volume.value = norm;
    }
  });

  const setVolume = (val: number) => {
    volume.value = val;
    audio.volume = val;
    systemStore.volume = Math.round(val * 100);
  };

  const formatDuration = (seconds: number): string => {
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = Math.floor(seconds % 60);
    const paddedSeconds = remainingSeconds < 10 ? '0' + remainingSeconds : remainingSeconds;
    return `${minutes}:${paddedSeconds}`;
  };

  // Helper to remove file extensions (e.g., "Song.mp3" -> "Song")
  const stripExtension = (filename: string) => {
    return filename.replace(/\.[^/.]+$/, "");
  };

  const findAudioFiles = (
    items: any[],
    songs: Song[],
    lrcCandidates: { title: string, url: string }[],
    workInfo: any,
    playlistId: string,
    songCounter: { value: number },
    path: string = ''
  ) => {
    for (const item of items) {
      if (item.type === 'audio' && item.mediaStreamUrl) {
        songs.push({
          id: `${playlistId}-s${songCounter.value++}`,
          title: item.title || `Track ${songCounter.value - 1}`,
          artist: workInfo.vas && workInfo.vas.length > 0 ? workInfo.vas[0].name : workInfo.circle.name,
          url: item.mediaStreamUrl,
          duration: formatDuration(item.duration || 0),
          cover: workInfo.samCoverUrl || workInfo.mainCoverUrl,
          lrcUrl: undefined 
        });
      } else if (item.type === 'text' && item.title?.endsWith('.lrc') && item.mediaStreamUrl) {
        lrcCandidates.push({ title: item.title, url: item.mediaStreamUrl });
      }
      if (item.children) {
        findAudioFiles(item.children, songs, lrcCandidates, workInfo, playlistId, songCounter, `${path}/${item.title}`);
      }
    }
  };

  const fetchAsmr200Playlist = async (sourceId: string, playlistId: string): Promise<FlatPlaylist | null> => {
    try {
      const workInfoRes = await fetch(`${ASMR_API_BASE}/api/workInfo/${sourceId}`);
      if (!workInfoRes.ok) throw new Error(`Failed to fetch ASMR work info`);
      const workInfo = await workInfoRes.json();

      const tracksRes = await fetch(`${ASMR_API_BASE}/api/tracks/${sourceId}?v=2`);
      if (!tracksRes.ok) throw new Error(`Failed to fetch ASMR tracks`);
      const tracksData = await tracksRes.json();

      const songs: Song[] = [];
      const lrcCandidates: { title: string, url: string }[] = []; 
      const songCounter = { value: 1 }; 
      findAudioFiles(tracksData, songs, lrcCandidates, workInfo, playlistId, songCounter, 'root');

      for (const lrc of lrcCandidates) {
        // 去除 .lrc 后缀
        const lrcBaseName = stripExtension(lrc.title);
        
        // 关键修改：使用 filter 找到所有同名歌曲（不管是 mp3, wav 还是 mp4）
        const matchingSongs = songs.filter(s => stripExtension(s.title) === lrcBaseName);
        
        if (matchingSongs.length > 0) {
          // 给所有匹配到的歌曲都赋值 lrcUrl
          matchingSongs.forEach(song => {
            song.lrcUrl = lrc.url;
            console.log(`Matched LRC [${lrc.title}] to Song [${song.title}]`); // 这样你应该能看到 mp3 也被匹配了
          });
        } else {
          // 模糊匹配逻辑也改为遍历所有
          songs.forEach(s => {
             if (s.title.includes(lrcBaseName) && !s.lrcUrl) {
                s.lrcUrl = lrc.url;
                console.log(`Fuzzy matched LRC [${lrc.title}] to Song [${s.title}]`);
             }
          });
        }
      }

      if (songs.length === 0) return null;

      return {
        id: playlistId,
        name: workInfo.title,
        cover: workInfo.mainCoverUrl || workInfo.samCoverUrl || '',
        songs: songs,
        type: 'asmr-200',
        sourceId: sourceId
      };

    } catch (error: any) {
      console.error(`Error fetching ASMR-200 playlist:`, error);
      return null;
    }
  };

  // ... (findFirstPlayable, init, findMusicItemById code remains same) ...
  const findFirstPlayable = (items: MusicItem[]): string | null => {
    for (const item of items) {
      if (item.type !== 'folder') return item.id;
      if (item.type === 'folder' && item.children) {
        const found = findFirstPlayable(item.children);
        if (found) return found;
      }
    }
    return null;
  };

  const init = async () => {
    try {
      const res = await fetch(`${import.meta.env.BASE_URL}music.json`);
      if (res.ok) {
        const rawMusicCollection = await res.json();
        processMusicPaths(rawMusicCollection); // Process paths after loading
        musicCollection.value = rawMusicCollection;
        const firstPlayableId = findFirstPlayable(musicCollection.value);
        if (firstPlayableId) await selectPlaylist(firstPlayableId);
      }
    } catch (e: any) { console.error("Failed to load music data", e); }

    audio.addEventListener('timeupdate', () => { progress.value = audio.currentTime; });
    audio.addEventListener('loadedmetadata', () => { duration.value = audio.duration; });
    audio.addEventListener('ended', () => { next(); });
  };

  const currentPlaylist = computed(() => activePlayablePlaylist.value);
  const currentSong = computed(() => {
    if (!currentPlaylist.value?.songs) return null;
    return currentPlaylist.value.songs.find(s => s.id === currentSongId.value);
  });

  const findMusicItemById = (items: MusicItem[], id: string): MusicItem | null => {
    for (const item of items) {
      if (item.id === id) return item;
      if (item.type === 'folder' && item.children) {
        const found = findMusicItemById(item.children, id);
        if (found) return found;
      }
    }
    return null;
  };

  // Helper to prepend BASE_URL to relative paths
  const processMusicPaths = (items: MusicItem[]) => {
    items.forEach(item => {
      if (item.type !== 'folder') { // FlatPlaylist or Asmr200Item
        const playableItem = item as FlatPlaylist | Asmr200Item;
        if (playableItem.cover && !playableItem.cover.startsWith('http')) {
          playableItem.cover = import.meta.env.BASE_URL + playableItem.cover;
        } else if (!playableItem.cover) { // If cover is null or empty, provide placeholder
          playableItem.cover = import.meta.env.BASE_URL + 'music/placeholder.png';
        }
        if (playableItem.type === 'local') { // Only local playlists have song URLs
          playableItem.songs.forEach(song => {
            if (song.url && !song.url.startsWith('http')) {
              song.url = import.meta.env.BASE_URL + song.url;
            }
            if (song.cover && !song.cover.startsWith('http')) {
              song.cover = import.meta.env.BASE_URL + song.cover;
            } else if (!song.cover) { // If song cover is null or empty
              song.cover = import.meta.env.BASE_URL + 'music/placeholder.png';
            }
          });
        }
      } else if (item.type === 'folder' && item.children) {
        processMusicPaths(item.children); // Recurse for folders
      }
    });
  };

  const selectPlaylist = async (id: string) => {
    currentPlaylistId.value = id;
    const selectedItem = findMusicItemById(musicCollection.value, id);

    if (!selectedItem || selectedItem.type === 'folder') {
      activePlayablePlaylist.value = null;
      currentSongId.value = null;
      isPlaying.value = false;
      audio.pause();
      audio.src = '';
      return;
    }

    if (selectedItem.type === 'asmr-200') {
      const asmrItem = selectedItem as Asmr200Item;
      const fetchedPl = await fetchAsmr200Playlist(asmrItem.sourceId, asmrItem.id);
      activePlayablePlaylist.value = fetchedPl || null;
    } else {
      activePlayablePlaylist.value = selectedItem as FlatPlaylist;
    }
  };

  const playSong = async (song: Song) => {
    if (!song) return;
    console.log('PlaySong called:', song.title, 'LRC URL:', song.lrcUrl);
    if (currentSongId.value === song.id) {
      togglePlay();
      return;
    }

    currentSongId.value = song.id;
    audio.src = song.url;
    audio.volume = volume.value;
    
    // LRC Fetching
    currentLrcContent.value = null; // 清除上一首的歌词
    if (song.lrcUrl) {
      try {
        // === 修复 CORS 问题的关键代码 ===
        // 将绝对 URL 转换为走本地代理的相对 URL
        let fetchUrl = song.lrcUrl;
        if (import.meta.env.VITE_DEPLOY_TARGET !== 'github' && fetchUrl.startsWith('https://api.asmr-200.com')) {
          fetchUrl = fetchUrl.replace('https://api.asmr-200.com', '/api-asmr200');
        }
        // ==============================

        const lrcRes = await fetch(fetchUrl);
        if (lrcRes.ok) {
          currentLrcContent.value = await lrcRes.text();
          console.log(`LRC loaded successfully`);
        } else {
          console.warn(`LRC fetch failed: ${lrcRes.status}`);
        }
      } catch (e: any) {
        console.error(`playSong: Error fetching LRC:`, e.message || e);
      }
    }

    audio.play();
    isPlaying.value = true;
  };

  const togglePlay = () => {
    if (audio.paused && audio.src) {
      audio.play();
      isPlaying.value = true;
    } else {
      audio.pause();
      isPlaying.value = false;
    }
  };

  const next = () => {
    if (!currentPlaylist.value?.songs || !currentSongId.value) return;
    const songs = currentPlaylist.value.songs;
    const idx = songs.findIndex(s => s.id === currentSongId.value);
    
    let nextSong: Song | undefined;
    if (idx !== -1 && idx < songs.length - 1) {
      nextSong = songs[idx + 1];
    } else if (songs.length > 0) {
      nextSong = songs[0];
    }

    if (nextSong) {
      playSong(nextSong);
    }
  };

  const prev = () => {
    if (!currentPlaylist.value?.songs || !currentSongId.value) return;
    const songs = currentPlaylist.value.songs;
    const idx = songs.findIndex(s => s.id === currentSongId.value);
    
    let prevSong: Song | undefined;
    if (idx > 0) {
      prevSong = songs[idx - 1];
    } else if (songs.length > 0) {
      prevSong = songs[songs.length - 1];
    }

    if (prevSong) {
      playSong(prevSong);
    }
  };
  
  const seek = (time: number) => {
    audio.currentTime = time;
    progress.value = time;
  };

  return {
    musicCollection, currentPlaylistId, currentSongId, isPlaying, volume, progress, duration, currentLrcContent,
    currentPlaylist, currentSong,
    init, selectPlaylist, playSong, togglePlay, next, prev, seek, setVolume, findMusicItemById
  };
});

