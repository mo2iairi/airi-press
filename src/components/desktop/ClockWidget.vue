<template>
  <div class="clock-widget">
    <div class="time">{{ time }}</div>
    <div class="date">{{ date }}</div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { useSystemStore } from '../../stores/system';
import { storeToRefs } from 'pinia';

const store = useSystemStore();
const { timezone, language } = storeToRefs(store);

const time = ref('');
const date = ref('');

const updateTime = () => {
  const now = new Date();
  const locale = language.value === 'zh' ? 'zh-CN' : 'en-US';
  
  // Format: 12:00
  try {
    time.value = now.toLocaleTimeString(locale, { 
      hour: '2-digit', 
      minute: '2-digit', 
      hour12: false,
      timeZone: timezone.value 
    });
    
    // Format: Monday, October 25
    date.value = now.toLocaleDateString(locale, { 
      weekday: 'long', 
      month: 'long', 
      day: 'numeric',
      timeZone: timezone.value
    });
  } catch (e) {
    // Fallback if timezone is invalid
    time.value = now.toLocaleTimeString(locale, { hour: '2-digit', minute: '2-digit', hour12: false });
    date.value = now.toLocaleDateString(locale, { weekday: 'long', month: 'long', day: 'numeric' });
  }
};

let interval: number;

onMounted(() => {
  updateTime();
  interval = setInterval(updateTime, 1000);
});

onUnmounted(() => {
  clearInterval(interval);
});

watch([timezone, language], () => {
  updateTime();
});
</script>

<style scoped>
.clock-widget {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  color: white;
  text-shadow: 0 4px 20px rgba(0,0,0,0.4);
  pointer-events: none; /* Allow clicks to pass through if needed */
}

.time {
  font-size: 4.5rem; /* Larger */
  font-weight: 500; /* Medium weight looks more modern */
  line-height: 1;
  letter-spacing: -2px;
  font-variant-numeric: tabular-nums;
}

.date {
  font-size: 1.2rem;
  font-weight: 400;
  opacity: 0.9;
  margin-top: 8px;
  text-transform: capitalize;
}
</style>
