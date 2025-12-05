<template>
  <div class="desktop-view">
    <div class="grid-container">
      <GridItem 
        v-for="icon in icons" 
        :key="icon.id" 
        :icon="icon" 
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useDesktopStore } from '../stores/desktop';
import { storeToRefs } from 'pinia';
import GridItem from '../components/desktop/GridItem.vue';

const store = useDesktopStore();
const { icons } = storeToRefs(store);

onMounted(() => {
  store.loadConfig(`${import.meta.env.BASE_URL}config.json`);
});
</script>

<style scoped>
.desktop-view {
  width: 100%;
  height: 100%;
  padding: 20px;
  padding-top: 60px; /* Space for status bar trigger/clock top margin */
  box-sizing: border-box;
  overflow-y: auto;
}

.grid-container {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  grid-auto-rows: 80px; /* Base row height */
  gap: 16px;
  max-width: 500px; /* Limit width on Desktop */
  margin: 0 auto;
}
</style>
