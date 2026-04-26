<template>
  <div id="app-root" :data-theme="theme">
    <!-- Header -->
    <header class="app-header">
      <div class="header-left">
        <div class="app-logo">
          <SparklesIcon :size="16" />
        </div>
        <span class="app-name">{{ modelStore.isActive ? modelStore.activeModel : 'AI Chat' }}</span>
      </div>

      <div class="header-center">
        <ModelSelector />
      </div>

      <div class="header-right">
        <!-- Theme toggle -->
        <button class="theme-btn" @click="toggleTheme" :title="theme === 'dark' ? '라이트 모드' : '다크 모드'">
          <SunIcon v-if="theme === 'dark'" :size="15" />
          <MoonIcon v-else :size="15" />
        </button>
      </div>
    </header>

    <!-- Error banner -->
    <Transition name="banner">
      <div v-if="modelStore.error" class="error-banner">
        <AlertTriangleIcon :size="14" />
        {{ modelStore.error }}
        <button @click="modelStore.error = null">✕</button>
      </div>
    </Transition>

    <!-- Main -->
    <main class="app-main">
      <ChatWindow />
      <BottomBar />
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import {
  Sparkles as SparklesIcon,
  Sun as SunIcon,
  Moon as MoonIcon,
  AlertTriangle as AlertTriangleIcon,
} from 'lucide-vue-next'
import { useModelStore } from '@/stores/model'
import ModelSelector from '@/components/ModelSelector.vue'
import ChatWindow from '@/components/ChatWindow.vue'
import BottomBar from '@/components/BottomBar.vue'

const modelStore = useModelStore()
const theme = ref(localStorage.getItem('theme') || 'dark')

function toggleTheme() {
  theme.value = theme.value === 'dark' ? 'light' : 'dark'
  localStorage.setItem('theme', theme.value)
}

onMounted(async () => {
  await Promise.all([
    modelStore.fetchModels(),
    modelStore.fetchStatus(),
  ])
})
</script>

<style>
/* Global app layout */
#app-root {
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: var(--bg-base);
  transition: background var(--transition-slow);
  position: relative;
}

/* Subtle background gradient */
#app-root::before {
  content: '';
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
  z-index: 0;
}

[data-theme="dark"] #app-root::before {
  background:
    radial-gradient(ellipse 60% 40% at 70% 0%, rgba(124, 106, 247, 0.07) 0%, transparent 60%),
    radial-gradient(ellipse 40% 30% at 0% 80%, rgba(91, 138, 240, 0.05) 0%, transparent 60%);
}

[data-theme="light"] #app-root::before {
  background:
    radial-gradient(ellipse 60% 40% at 70% 0%, rgba(108, 90, 240, 0.04) 0%, transparent 60%),
    radial-gradient(ellipse 40% 30% at 0% 80%, rgba(74, 122, 224, 0.03) 0%, transparent 60%);
}

/* Header */
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 1.25em;
  height: var(--header-height);
  border-bottom: 1px solid var(--border-subtle);
  background: var(--glass-bg);
  backdrop-filter: blur(12px);
  position: relative;
  z-index: 10;
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 0.5em;
  min-width: 140px;
}

.app-logo {
  width: 26px;
  height: 26px;
  border-radius: var(--radius-sm);
  background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}

.app-name {
  font-weight: 600;
  font-size: 0.92em;
  color: var(--text-primary);
  letter-spacing: -0.01em;
}

.header-center {
  flex: 1;
  display: flex;
  justify-content: center;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 0.5em;
  min-width: 140px;
  justify-content: flex-end;
}

.theme-btn {
  width: 32px;
  height: 32px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: 1px solid var(--border-default);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.theme-btn:hover {
  border-color: var(--accent-border);
  color: var(--accent-primary);
  background: var(--accent-subtle);
}

/* Error banner */
.error-banner {
  display: flex;
  align-items: center;
  gap: 0.5em;
  padding: 0.6em 1.25em;
  background: rgba(248, 113, 113, 0.12);
  border-bottom: 1px solid rgba(248, 113, 113, 0.25);
  color: var(--status-error);
  font-size: 0.85em;
  position: relative;
  z-index: 9;
}

.error-banner button {
  margin-left: auto;
  background: none;
  border: none;
  color: inherit;
  cursor: pointer;
  opacity: 0.7;
  font-size: 1em;
}

.error-banner button:hover { opacity: 1; }

.banner-enter-active,
.banner-leave-active { transition: max-height 0.2s ease, opacity 0.2s ease; overflow: hidden; max-height: 60px; }
.banner-enter-from,
.banner-leave-to { max-height: 0; opacity: 0; }

/* Main */
.app-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  position: relative;
  z-index: 1;
}
</style>
