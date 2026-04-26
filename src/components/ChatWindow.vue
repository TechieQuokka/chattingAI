<template>
  <div class="chat-window" ref="scrollRef">
    <!-- Empty state -->
    <div v-if="!chatStore.hasMessages" class="empty-state">
      <div class="empty-icon">
        <SparklesIcon :size="32" />
      </div>
      <h2 class="empty-title">AI Chat</h2>
      <p class="empty-sub">
        {{ modelStore.isActive
          ? `${modelStore.activeModel} 모델이 준비되었습니다`
          : '상단에서 모델을 선택하고 활성화하세요' }}
      </p>
    </div>

    <!-- Messages -->
    <div v-else class="messages-list">
      <MessageBubble
        v-for="msg in chatStore.messages"
        :key="msg.id"
        :message="msg"
      />
      <div ref="bottomRef" />
    </div>
  </div>
</template>

<script setup>
import { ref, watch, nextTick, onMounted } from 'vue'
import { Sparkles as SparklesIcon } from 'lucide-vue-next'
import { useChatStore } from '@/stores/chat'
import { useModelStore } from '@/stores/model'
import MessageBubble from './MessageBubble.vue'

const chatStore = useChatStore()
const modelStore = useModelStore()

const scrollRef = ref(null)
const bottomRef = ref(null)

function scrollToBottom() {
  nextTick(() => {
    if (bottomRef.value) {
      bottomRef.value.scrollIntoView({ behavior: 'smooth' })
    }
  })
}

// 메시지 추가 시 자동 스크롤
watch(
  () => chatStore.messages.length,
  () => scrollToBottom()
)

// 스트리밍 중 자동 스크롤
watch(
  () => chatStore.isStreaming,
  () => scrollToBottom()
)

onMounted(() => scrollToBottom())
</script>

<style scoped>
.chat-window {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 1.5em 1.5em 0.5em;
  display: flex;
  flex-direction: column;
}

/* Empty state */
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  gap: 0.75em;
  padding: 3em;
  pointer-events: none;
  user-select: none;
}

.empty-icon {
  width: 64px;
  height: 64px;
  border-radius: var(--radius-xl);
  background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  margin-bottom: 0.5em;
  box-shadow: var(--shadow-glow);
}

.empty-title {
  font-size: 1.4em;
  font-weight: 600;
  color: var(--text-primary);
  letter-spacing: -0.02em;
}

.empty-sub {
  font-size: 0.88em;
  color: var(--text-muted);
  max-width: 280px;
  line-height: 1.5;
}

/* Messages */
.messages-list {
  display: flex;
  flex-direction: column;
  gap: 1.25em;
  width: 100%;
  max-width: 860px;
  margin: 0 auto;
  padding-bottom: 1em;
}
</style>
