<template>
  <div class="code-block">
    <div class="code-header">
      <span class="code-lang">{{ language }}</span>
      <button class="copy-btn" @click="copy" :class="{ copied }">
        <CheckIcon v-if="copied" :size="14" />
        <CopyIcon v-else :size="14" />
        <span>{{ copied ? '복사됨' : '복사' }}</span>
      </button>
    </div>
    <div class="code-body">
      <pre><code v-html="highlighted" :class="`language-${language}`"></code></pre>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { Copy as CopyIcon, Check as CheckIcon } from 'lucide-vue-next'
import hljs from 'highlight.js'

const props = defineProps({
  code: { type: String, required: true },
  language: { type: String, default: 'plaintext' },
})

const copied = ref(false)

const highlighted = computed(() => {
  const lang = props.language?.toLowerCase()
  if (lang && lang !== 'plaintext' && hljs.getLanguage(lang)) {
    try {
      return hljs.highlight(props.code, { language: lang }).value
    } catch {}
  }
  return hljs.highlightAuto(props.code).value
})

async function copy() {
  try {
    await navigator.clipboard.writeText(props.code)
    copied.value = true
    setTimeout(() => (copied.value = false), 2000)
  } catch {}
}
</script>

<style scoped>
.code-block {
  border: 1px solid var(--code-border);
  border-radius: var(--radius-md);
  overflow: hidden;
  margin: 0.75em 0;
  background: var(--code-bg);
}

.code-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.5em 1em;
  background: var(--code-header-bg);
  border-bottom: 1px solid var(--code-border);
}

.code-lang {
  font-family: var(--font-mono);
  font-size: 0.75em;
  color: var(--text-muted);
  text-transform: lowercase;
  letter-spacing: 0.05em;
}

.copy-btn {
  display: flex;
  align-items: center;
  gap: 0.35em;
  padding: 0.25em 0.65em;
  background: var(--accent-subtle);
  border: 1px solid var(--accent-border);
  border-radius: var(--radius-sm);
  color: var(--text-accent);
  font-size: 0.75em;
  cursor: pointer;
  transition: all var(--transition-fast);
  font-family: var(--font-sans);
}

.copy-btn:hover {
  background: var(--accent-primary);
  color: #fff;
  border-color: var(--accent-primary);
}

.copy-btn.copied {
  background: rgba(74, 222, 128, 0.15);
  border-color: rgba(74, 222, 128, 0.4);
  color: var(--status-active);
}

.code-body {
  overflow-x: auto;
  padding: 1em 1.25em;
}

pre {
  margin: 0;
  white-space: pre;
  font-family: var(--font-mono);
}

code {
  font-family: var(--font-mono);
  font-size: 0.85em;
  line-height: 1.7;
}
</style>
