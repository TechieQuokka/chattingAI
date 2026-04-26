<template>
  <div class="message-wrap" :class="message.role">
    <!-- AI message -->
    <div v-if="message.role === 'assistant'" class="ai-message">
      <div class="ai-avatar">
        <SparklesIcon :size="14" />
      </div>
      <div class="ai-body">
        <!-- Thinking block -->
        <ThinkingBlock
          v-if="message.thinking"
          :content="message.thinking"
          :streaming="message.isStreaming && !message.content"
        />

        <!-- Answer content -->
        <div v-if="message.content || message.isStreaming" class="answer-content prose">
          <template v-for="(part, idx) in parsedContent" :key="idx">
            <CodeBlock
              v-if="part.type === 'code'"
              :code="part.code"
              :language="part.lang"
            />
            <span
              v-else
              v-html="part.html"
            />
          </template>
          <!-- Cursor blink while streaming -->
          <span v-if="message.isStreaming && message.content" class="cursor-blink" />
        </div>

        <!-- Streaming placeholder (thinking done, answer not started) -->
        <div v-if="message.isStreaming && !message.content && !message.thinking" class="answer-placeholder">
          <span></span><span></span><span></span>
        </div>

        <!-- Error -->
        <div v-if="message.error" class="message-error">
          <AlertTriangleIcon :size="14" />
          {{ message.error }}
        </div>
      </div>
    </div>

    <!-- User message -->
    <div v-else class="user-message">
      <div class="user-bubble">{{ message.content }}</div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { Sparkles as SparklesIcon, AlertTriangle as AlertTriangleIcon } from 'lucide-vue-next'
import { marked } from 'marked'
import ThinkingBlock from './ThinkingBlock.vue'
import CodeBlock from './CodeBlock.vue'

const props = defineProps({
  message: { type: Object, required: true },
})

// Parse content: extract code blocks separately for CodeBlock component
const parsedContent = computed(() => {
  if (!props.message.content) return []

  const content = props.message.content
  const parts = []
  const codeBlockRegex = /```(\w*)\n?([\s\S]*?)```/g
  let lastIndex = 0
  let match

  while ((match = codeBlockRegex.exec(content)) !== null) {
    // Text before code block
    if (match.index > lastIndex) {
      const text = content.slice(lastIndex, match.index)
      parts.push({ type: 'text', html: marked.parse(text) })
    }
    // Code block
    parts.push({ type: 'code', lang: match[1] || 'plaintext', code: match[2].trimEnd() })
    lastIndex = match.index + match[0].length
  }

  // Remaining text
  if (lastIndex < content.length) {
    const remaining = content.slice(lastIndex)
    parts.push({ type: 'text', html: marked.parse(remaining) })
  }

  if (parts.length === 0 && content) {
    parts.push({ type: 'text', html: marked.parse(content) })
  }

  return parts
})
</script>

<style scoped>
.message-wrap {
  display: flex;
  flex-direction: column;
  padding: 0.25em 0;
  animation: msg-in 0.2s ease;
}

@keyframes msg-in {
  from { opacity: 0; transform: translateY(8px); }
  to   { opacity: 1; transform: translateY(0); }
}

/* AI Message */
.ai-message {
  display: flex;
  gap: 0.75em;
  align-items: flex-start;
  max-width: 820px;
  width: 100%;
}

.ai-avatar {
  flex-shrink: 0;
  width: 28px;
  height: 28px;
  border-radius: var(--radius-sm);
  background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  margin-top: 2px;
}

.ai-body {
  flex: 1;
  min-width: 0;
}

.answer-content {
  font-size: 0.95em;
}

/* Streaming cursor */
.cursor-blink {
  display: inline-block;
  width: 2px;
  height: 1em;
  background: var(--accent-primary);
  margin-left: 2px;
  vertical-align: text-bottom;
  border-radius: 1px;
  animation: blink 0.9s step-end infinite;
}

@keyframes blink {
  0%, 100% { opacity: 1; }
  50%       { opacity: 0; }
}

/* Placeholder dots */
.answer-placeholder {
  display: flex;
  gap: 5px;
  align-items: center;
  padding: 0.5em 0;
}

.answer-placeholder span {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--accent-primary);
  animation: dot-pulse 1.4s infinite ease-in-out;
}
.answer-placeholder span:nth-child(2) { animation-delay: 0.2s; }
.answer-placeholder span:nth-child(3) { animation-delay: 0.4s; }

@keyframes dot-pulse {
  0%, 80%, 100% { opacity: 0.3; transform: scale(0.7); }
  40% { opacity: 1; transform: scale(1); }
}

/* User Message */
.user-message {
  display: flex;
  justify-content: flex-end;
}

.user-bubble {
  background: var(--msg-user-bg);
  color: var(--msg-user-text);
  padding: 0.65em 1.1em;
  border-radius: var(--radius-lg) var(--radius-lg) 4px var(--radius-lg);
  max-width: 70%;
  font-size: 0.95em;
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-word;
  box-shadow: var(--shadow-md);
}

/* Error */
.message-error {
  display: flex;
  align-items: center;
  gap: 0.4em;
  color: var(--status-error);
  font-size: 0.82em;
  margin-top: 0.4em;
}
</style>
