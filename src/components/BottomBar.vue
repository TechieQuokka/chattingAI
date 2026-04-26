<template>
  <div class="bottom-bar">
    <!-- Controls row -->
    <div class="controls-row">
      <!-- Thinking toggle -->
      <button
        class="toggle-btn"
        :class="{ active: modelStore.thinkingEnabled && modelStore.supportsThinking }"
        @click="modelStore.toggleThinking"
        :disabled="!modelStore.isActive || !modelStore.supportsThinking"
        :title="thinkingTitle"
      >
        <BrainIcon :size="13" />
        <span>Thinking</span>
        <span class="toggle-indicator">
          {{ modelStore.supportsThinking ? (modelStore.thinkingEnabled ? 'ON' : 'OFF') : 'N/A' }}
        </span>
      </button>

      <!-- Clear -->
      <button
        class="icon-btn"
        @click="handleClear"
        :disabled="!chatStore.hasMessages && !chatStore.isStreaming"
        title="대화 초기화"
      >
        <Trash2Icon :size="14" />
        <span>Clear</span>
      </button>

      <div class="spacer" />

      <!-- Char count -->
      <span v-if="inputText" class="char-count">{{ inputText.length }}</span>
    </div>

    <!-- Input row -->
    <div class="input-row">
      <div class="input-wrap" :class="{ focused, disabled: !modelStore.isActive }">
        <textarea
          ref="textareaRef"
          :value="inputText"
          class="chat-input"
          :placeholder="placeholder"
          :disabled="!modelStore.isActive"
          @compositionstart="isComposing = true"
          @compositionend="onCompositionEnd"
          @input="onInput"
          @keydown="onKeydown"
          @focus="focused = true"
          @blur="focused = false"
          rows="1"
        />
      </div>

      <!-- Send / Stop -->
      <button
        v-if="!chatStore.isStreaming"
        class="send-btn"
        @click="send"
        :disabled="!canSend"
      >
        <SendIcon :size="16" />
      </button>
      <button
        v-else
        class="stop-btn"
        @click="chatStore.stopStream"
      >
        <SquareIcon :size="16" />
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch, nextTick } from 'vue'
import {
  Brain as BrainIcon,
  Trash2 as Trash2Icon,
  Send as SendIcon,
  Square as SquareIcon,
} from 'lucide-vue-next'
import { useModelStore } from '@/stores/model'
import { useChatStore } from '@/stores/chat'

const modelStore = useModelStore()
const chatStore = useChatStore()

const inputText = ref('')
const focused = ref(false)
const isComposing = ref(false)
const textareaRef = ref(null)

const thinkingTitle = computed(() => {
  if (!modelStore.isActive) return '모델을 먼저 활성화하세요'
  if (!modelStore.supportsThinking) return '이 모델은 Thinking을 지원하지 않습니다'
  return modelStore.thinkingEnabled ? 'Thinking ON' : 'Thinking OFF'
})

const placeholder = computed(() => {
  if (!modelStore.isActive) return '모델을 먼저 활성화하세요...'
  if (chatStore.isStreaming) return '응답 중...'
  return '메시지를 입력하세요... (Shift+Enter: 줄바꿈)'
})

const canSend = computed(
  () => modelStore.isActive && inputText.value.trim().length > 0 && !chatStore.isStreaming
)

function onInput(e) {
  // IME 조합 중에는 inputText 업데이트 스킵
  if (!isComposing.value) {
    inputText.value = e.target.value
  }
}

function onCompositionEnd(e) {
  isComposing.value = false
  // 조합 완료 후 최종값 반영
  inputText.value = e.target.value
}

function onKeydown(e) {
  if (e.key === 'Enter' && !e.shiftKey) {
    // IME 조합 중 Enter는 무시
    if (isComposing.value) return
    e.preventDefault()
    if (canSend.value) send()
  }
}

async function send() {
  const text = inputText.value.trim()
  if (!text) return
  inputText.value = ''
  await nextTick()
  adjustHeight()
  chatStore.sendMessage(text)
}

async function handleClear() {
  await chatStore.clearHistory()
}

function adjustHeight() {
  const el = textareaRef.value
  if (!el) return
  el.style.height = 'auto'
  el.style.height = Math.min(el.scrollHeight, 160) + 'px'
}

watch(inputText, async () => {
  await nextTick()
  adjustHeight()
})
</script>

<style scoped>
.bottom-bar {
  padding: 0.75em 1.25em 1em;
  border-top: 1px solid var(--border-subtle);
  background: var(--glass-bg);
  backdrop-filter: blur(12px);
  display: flex;
  flex-direction: column;
  gap: 0.5em;
}

.controls-row {
  display: flex;
  align-items: center;
  gap: 0.5em;
}

.toggle-btn {
  display: flex;
  align-items: center;
  gap: 0.35em;
  padding: 0.28em 0.7em;
  border-radius: var(--radius-full);
  font-size: 0.78em;
  font-family: var(--font-sans);
  cursor: pointer;
  transition: all var(--transition-fast);
  border: 1px solid var(--border-default);
  background: transparent;
  color: var(--text-muted);
}

.toggle-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.toggle-btn.active {
  border-color: var(--accent-border);
  color: var(--accent-primary);
  background: var(--accent-subtle);
}

.toggle-btn:not(:disabled):hover {
  border-color: var(--accent-border);
}

.toggle-indicator {
  font-weight: 700;
  font-size: 0.85em;
  letter-spacing: 0.05em;
}

.icon-btn {
  display: flex;
  align-items: center;
  gap: 0.35em;
  padding: 0.28em 0.7em;
  border-radius: var(--radius-full);
  font-size: 0.78em;
  font-family: var(--font-sans);
  cursor: pointer;
  transition: all var(--transition-fast);
  border: 1px solid var(--border-default);
  background: transparent;
  color: var(--text-muted);
}

.icon-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.icon-btn:not(:disabled):hover {
  border-color: var(--status-error);
  color: var(--status-error);
}

.spacer { flex: 1; }

.char-count {
  font-size: 0.75em;
  color: var(--text-muted);
  font-variant-numeric: tabular-nums;
}

.input-row {
  display: flex;
  align-items: flex-end;
  gap: 0.6em;
}

.input-wrap {
  flex: 1;
  border: 1px solid var(--input-border);
  border-radius: var(--radius-lg);
  background: var(--input-bg);
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
  overflow: hidden;
}

.input-wrap.focused {
  border-color: var(--input-focus);
  box-shadow: 0 0 0 3px var(--accent-glow);
}

.input-wrap.disabled {
  opacity: 0.5;
}

.chat-input {
  width: 100%;
  background: transparent;
  border: none;
  outline: none;
  padding: 0.7em 1em;
  color: var(--text-primary);
  font-size: 0.92em;
  font-family: var(--font-sans);
  line-height: 1.6;
  resize: none;
  min-height: 42px;
  max-height: 160px;
}

.chat-input::placeholder {
  color: var(--text-muted);
}

.chat-input:disabled { cursor: not-allowed; }

.send-btn,
.stop-btn {
  width: 40px;
  height: 40px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  border: none;
  flex-shrink: 0;
  transition: all var(--transition-fast);
}

.send-btn {
  background: var(--accent-primary);
  color: white;
}
.send-btn:hover:not(:disabled) {
  filter: brightness(1.1);
  box-shadow: var(--shadow-glow);
}
.send-btn:disabled {
  background: var(--bg-overlay);
  color: var(--text-muted);
  cursor: not-allowed;
}

.stop-btn {
  background: rgba(248, 113, 113, 0.15);
  color: var(--status-error);
  border: 1px solid rgba(248, 113, 113, 0.3);
}
.stop-btn:hover {
  background: rgba(248, 113, 113, 0.25);
}
</style>
