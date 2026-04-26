<template>
  <div class="model-selector">
    <!-- Status dot -->
    <div class="status-dot" :class="statusClass" :title="statusLabel" />

    <!-- Model dropdown -->
    <div class="dropdown-wrap" ref="dropdownRef">
      <button
        class="dropdown-trigger"
        @click="toggleDropdown"
        :disabled="modelStore.isLoading"
      >
        <span class="model-name">
          {{ selectedModel || modelStore.activeModel || '모델 선택' }}
        </span>
        <ChevronDownIcon :size="14" :class="{ rotated: open }" />
      </button>

      <Transition name="dropdown">
        <div v-if="open" class="dropdown-menu">
          <div v-if="modelStore.availableModels.length === 0" class="dropdown-empty">
            모델 없음
          </div>
          <button
            v-for="m in modelStore.availableModels"
            :key="m"
            class="dropdown-item"
            :class="{ active: m === modelStore.activeModel }"
            @click="selectModel(m)"
          >
            <span>{{ m }}</span>
            <CheckIcon v-if="m === modelStore.activeModel" :size="13" />
          </button>
        </div>
      </Transition>
    </div>

    <!-- Load / Unload button -->
    <button
      v-if="!modelStore.isLoading"
      class="action-btn"
      :class="modelStore.isActive ? 'unload' : 'load'"
      @click="toggleModel"
      :disabled="!selectedModel && !modelStore.isActive"
    >
      <PowerIcon :size="14" />
      <span>{{ modelStore.isActive ? '비활성화' : '활성화' }}</span>
    </button>

    <div v-else class="loading-status">
      <div class="spinner" />
      <span>{{ modelStore.status === 'loading' ? '로딩 중...' : '해제 중...' }}</span>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import {
  ChevronDown as ChevronDownIcon,
  Check as CheckIcon,
  Power as PowerIcon,
} from 'lucide-vue-next'
import { useModelStore } from '@/stores/model'
import { useChatStore } from '@/stores/chat'

const modelStore = useModelStore()
const chatStore = useChatStore()

const open = ref(false)
const selectedModel = ref(null)
const dropdownRef = ref(null)

const statusClass = computed(() => ({
  active:   modelStore.status === 'active',
  loading:  modelStore.isLoading,
  unloaded: modelStore.status === 'unloaded',
}))

const statusLabel = computed(() => ({
  active:   '활성화됨',
  loading:  '로딩 중...',
  unloading:'해제 중...',
  unloaded: '비활성화됨',
}[modelStore.status] || ''))

function toggleDropdown() {
  open.value = !open.value
  if (open.value) modelStore.fetchModels()
}

function selectModel(m) {
  selectedModel.value = m
  open.value = false
}

async function toggleModel() {
  if (modelStore.isActive) {
    chatStore.clearHistory()
    await modelStore.unloadModel()
  } else {
    const target = selectedModel.value || modelStore.availableModels[0]
    if (target) {
      chatStore.clearHistory()
      await modelStore.loadModel(target)
    }
  }
}

// Close dropdown on outside click
function onOutsideClick(e) {
  if (dropdownRef.value && !dropdownRef.value.contains(e.target)) {
    open.value = false
  }
}
onMounted(() => document.addEventListener('mousedown', onOutsideClick))
onUnmounted(() => document.removeEventListener('mousedown', onOutsideClick))
</script>

<style scoped>
.model-selector {
  display: flex;
  align-items: center;
  gap: 0.6em;
}

/* Status dot */
.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
  transition: background var(--transition-normal);
}
.status-dot.active   { background: var(--status-active); box-shadow: 0 0 6px var(--status-active); }
.status-dot.loading  { background: var(--status-loading); animation: pulse 1s ease-in-out infinite; }
.status-dot.unloaded { background: var(--status-inactive); }

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50%       { opacity: 0.4; }
}

/* Dropdown */
.dropdown-wrap {
  position: relative;
}

.dropdown-trigger {
  display: flex;
  align-items: center;
  gap: 0.5em;
  padding: 0.4em 0.8em;
  background: var(--bg-overlay);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: 0.85em;
  font-family: var(--font-sans);
  cursor: pointer;
  transition: all var(--transition-fast);
  min-width: 160px;
}

.dropdown-trigger:hover:not(:disabled) {
  border-color: var(--accent-border);
  background: var(--bg-hover);
}

.dropdown-trigger:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.model-name {
  flex: 1;
  text-align: left;
  font-family: var(--font-mono);
  font-size: 0.9em;
}

.dropdown-trigger svg {
  transition: transform var(--transition-fast);
  color: var(--text-muted);
}
.dropdown-trigger svg.rotated { transform: rotate(180deg); }

.dropdown-menu {
  position: absolute;
  top: calc(100% + 6px);
  left: 0;
  min-width: 100%;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  overflow-y: auto;
  max-height: 260px;
  z-index: 100;
}

.dropdown-empty {
  padding: 0.75em 1em;
  color: var(--text-muted);
  font-size: 0.85em;
}

.dropdown-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 0.6em 1em;
  background: transparent;
  border: none;
  color: var(--text-primary);
  font-size: 0.85em;
  font-family: var(--font-mono);
  cursor: pointer;
  transition: background var(--transition-fast);
  text-align: left;
}

.dropdown-item:hover { background: var(--bg-hover); }
.dropdown-item.active {
  background: var(--accent-subtle);
  color: var(--accent-primary);
}

/* Load/Unload button */
.action-btn {
  display: flex;
  align-items: center;
  gap: 0.4em;
  padding: 0.4em 0.85em;
  border-radius: var(--radius-md);
  font-size: 0.82em;
  font-family: var(--font-sans);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
  border: 1px solid;
}

.action-btn.load {
  background: var(--accent-primary);
  color: white;
  border-color: var(--accent-primary);
}
.action-btn.load:hover:not(:disabled) {
  filter: brightness(1.1);
  box-shadow: var(--shadow-glow);
}

.action-btn.unload {
  background: transparent;
  color: var(--text-secondary);
  border-color: var(--border-default);
}
.action-btn.unload:hover:not(:disabled) {
  border-color: var(--status-error);
  color: var(--status-error);
}

.action-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

/* Loading status */
.loading-status {
  display: flex;
  align-items: center;
  gap: 0.5em;
  color: var(--text-muted);
  font-size: 0.82em;
}

.spinner {
  width: 14px;
  height: 14px;
  border: 2px solid var(--border-default);
  border-top-color: var(--accent-primary);
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
}

@keyframes spin { to { transform: rotate(360deg); } }

/* Transitions */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: opacity 0.15s ease, transform 0.15s ease;
}
.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
