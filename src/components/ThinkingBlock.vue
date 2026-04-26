<template>
  <div class="thinking-block" :class="{ expanded: isOpen, streaming }">
    <button class="thinking-header" @click="isOpen = !isOpen">
      <div class="thinking-left">
        <span class="thinking-icon-wrap">
          <BrainIcon :size="14" />
        </span>
        <span class="thinking-label">
          {{ streaming ? '생각 중...' : '생각 과정' }}
        </span>
        <span v-if="streaming" class="thinking-dots">
          <span></span><span></span><span></span>
        </span>
      </div>
      <ChevronDownIcon
        :size="14"
        class="chevron"
        :style="{ transform: isOpen ? 'rotate(180deg)' : 'rotate(0deg)' }"
      />
    </button>
    <Transition name="slide">
      <div v-if="isOpen" class="thinking-body">
        <div class="thinking-content" ref="contentRef">{{ content }}</div>
      </div>
    </Transition>
  </div>
</template>

<script setup>
import { ref, watch, nextTick } from 'vue'
import { Brain as BrainIcon, ChevronDown as ChevronDownIcon } from 'lucide-vue-next'

const props = defineProps({
  content: { type: String, default: '' },
  streaming: { type: Boolean, default: false },
})

const isOpen = ref(false)
const contentRef = ref(null)

// Auto-scroll thinking content while streaming
watch(
  () => props.content,
  async () => {
    if (isOpen.value && contentRef.value) {
      await nextTick()
      contentRef.value.scrollTop = contentRef.value.scrollHeight
    }
  }
)
</script>

<style scoped>
.thinking-block {
  border: 1px solid var(--thinking-border);
  border-radius: var(--radius-md);
  background: var(--thinking-bg);
  margin-bottom: 0.75em;
  overflow: hidden;
  transition: border-color var(--transition-normal);
}

.thinking-block.streaming {
  border-color: var(--thinking-icon);
}

.thinking-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 0.6em 0.9em;
  background: transparent;
  border: none;
  cursor: pointer;
  color: var(--thinking-text);
  font-size: 0.82em;
  font-family: var(--font-sans);
  gap: 0.5em;
}

.thinking-left {
  display: flex;
  align-items: center;
  gap: 0.45em;
}

.thinking-icon-wrap {
  color: var(--thinking-icon);
  display: flex;
  align-items: center;
}

.thinking-label {
  font-weight: 500;
  letter-spacing: 0.01em;
}

.thinking-dots {
  display: flex;
  gap: 3px;
  align-items: center;
}

.thinking-dots span {
  width: 4px;
  height: 4px;
  border-radius: 50%;
  background: var(--thinking-icon);
  animation: dot-pulse 1.4s infinite ease-in-out;
}
.thinking-dots span:nth-child(1) { animation-delay: 0s; }
.thinking-dots span:nth-child(2) { animation-delay: 0.2s; }
.thinking-dots span:nth-child(3) { animation-delay: 0.4s; }

@keyframes dot-pulse {
  0%, 80%, 100% { opacity: 0.3; transform: scale(0.8); }
  40% { opacity: 1; transform: scale(1); }
}

.chevron {
  color: var(--thinking-text);
  transition: transform var(--transition-normal);
  flex-shrink: 0;
}

.thinking-body {
  border-top: 1px solid var(--thinking-border);
}

.thinking-content {
  padding: 0.75em 1em;
  font-size: 0.82em;
  line-height: 1.65;
  color: var(--thinking-text);
  font-family: var(--font-mono);
  white-space: pre-wrap;
  word-break: break-word;
  max-height: 300px;
  overflow-y: auto;
}

/* Transition */
.slide-enter-active,
.slide-leave-active {
  transition: max-height 0.25s ease, opacity 0.2s ease;
  overflow: hidden;
  max-height: 400px;
}
.slide-enter-from,
.slide-leave-to {
  max-height: 0;
  opacity: 0;
}
</style>
