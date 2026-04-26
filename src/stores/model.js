import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/api/client'

export const useModelStore = defineStore('model', () => {
  const availableModels = ref([])
  const activeModel = ref(null)
  const status = ref('unloaded')
  const thinkingEnabled = ref(false)
  const supportsThinking = ref(false)
  const error = ref(null)

  const isActive = computed(() => status.value === 'active')
  const isLoading = computed(() => status.value === 'loading' || status.value === 'unloading')

  async function fetchModels() {
    try {
      const data = await api.listModels()
      availableModels.value = data.models || []
    } catch (e) {
      console.error('Failed to fetch models:', e)
    }
  }

  async function fetchStatus() {
    try {
      const data = await api.modelStatus()
      activeModel.value = data.active_model
      status.value = data.status
      thinkingEnabled.value = data.thinking_enabled
      supportsThinking.value = data.supports_thinking ?? false
    } catch (e) {
      console.error('Failed to fetch model status:', e)
    }
  }

  async function loadModel(modelName) {
    error.value = null
    status.value = 'loading'
    activeModel.value = modelName
    try {
      const data = await api.loadModel(modelName)
      status.value = 'active'
      supportsThinking.value = data.supports_thinking ?? false
      // think 미지원 모델이면 강제 off
      if (!supportsThinking.value) {
        thinkingEnabled.value = false
      }
    } catch (e) {
      error.value = e.message
      status.value = 'unloaded'
      activeModel.value = null
      supportsThinking.value = false
      throw e
    }
  }

  async function unloadModel() {
    error.value = null
    status.value = 'unloading'
    try {
      await api.unloadModel()
      status.value = 'unloaded'
      activeModel.value = null
      supportsThinking.value = false
      thinkingEnabled.value = false
    } catch (e) {
      error.value = e.message
      await fetchStatus()
      throw e
    }
  }

  async function toggleThinking() {
    if (!supportsThinking.value) return
    const newVal = !thinkingEnabled.value
    try {
      await api.setThinking(newVal)
      thinkingEnabled.value = newVal
    } catch (e) {
      console.error('Failed to toggle thinking:', e)
    }
  }

  return {
    availableModels,
    activeModel,
    status,
    thinkingEnabled,
    supportsThinking,
    error,
    isActive,
    isLoading,
    fetchModels,
    fetchStatus,
    loadModel,
    unloadModel,
    toggleThinking,
  }
})
