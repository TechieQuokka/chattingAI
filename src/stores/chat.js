import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/api/client'
import { useModelStore } from './model'

let messageIdCounter = 0

function createMessage(role, content = '') {
  return {
    id: ++messageIdCounter,
    role,
    content,
    thinking: '',
    isStreaming: false,
    error: null,
    timestamp: new Date(),
  }
}

export const useChatStore = defineStore('chat', () => {
  const messages = ref([])
  const isStreaming = ref(false)
  const currentAbort = ref(null)

  const hasMessages = computed(() => messages.value.length > 0)

  async function sendMessage(text) {
    const modelStore = useModelStore()
    if (!modelStore.isActive || isStreaming.value) return

    messages.value.push(createMessage('user', text))

    messages.value.push({
      id: ++messageIdCounter,
      role: 'assistant',
      content: '',
      thinking: '',
      isStreaming: true,
      error: null,
      timestamp: new Date(),
    })
    isStreaming.value = true

    const idx = messages.value.length - 1

    const abort = api.chatStream(
      text,
      modelStore.thinkingEnabled && modelStore.supportsThinking,
      (chunk) => {
        messages.value[idx].thinking += chunk
      },
      (chunk) => {
        messages.value[idx].content += chunk
      },
      () => {
        messages.value[idx].isStreaming = false
        isStreaming.value = false
        currentAbort.value = null
      },
      (err) => {
        messages.value[idx].error = err.message
        messages.value[idx].isStreaming = false
        isStreaming.value = false
        currentAbort.value = null
      },
    )

    currentAbort.value = abort
  }

  function stopStream() {
    if (currentAbort.value) {
      currentAbort.value()
      currentAbort.value = null
      isStreaming.value = false
      const last = messages.value[messages.value.length - 1]
      if (last && last.isStreaming) {
        last.isStreaming = false
      }
    }
  }

  async function clearHistory() {
    stopStream()
    await api.clearHistory()
    messages.value = []
  }

  return {
    messages,
    isStreaming,
    hasMessages,
    sendMessage,
    stopStream,
    clearHistory,
  }
})
