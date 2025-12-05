import { computed } from 'vue';
import { useSystemStore } from '../stores/system';
import { translations, type Language, type TranslationKey } from '../locales/translations';

export function useI18n() {
  const systemStore = useSystemStore();

  const t = (key: string) => {
    const lang = systemStore.language as Language;
    const dict = translations[lang] || translations['en'];
    return dict[key as TranslationKey] || key;
  };

  // Reactive translation helper
  const tr = computed(() => {
    const lang = systemStore.language as Language;
    return translations[lang] || translations['en'];
  });

  return { t, tr };
}
