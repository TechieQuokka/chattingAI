const BASE_URL = 'http://127.0.0.1:3456'

async function request(method, path, body = null) {
  const opts = {
    method,
    headers: { 'Content-Type': 'application/json' },
  }
  if (body) opts.body = JSON.stringify(body)
  const res = await fetch(`${BASE_URL}${path}`, opts)
  if (!res.ok) {
    const err = await res.json().catch(() => ({ error: res.statusText }))
    throw new Error(err.error || 'Request failed')
  }
  return res.json()
}

export const api = {
  // Health
  health: () => request('GET', '/health'),

  // Models
  listModels: () => request('GET', '/models'),
  modelStatus: () => request('GET', '/model/status'),
  loadModel: (model) => request('POST', '/model/load', { model }),
  unloadModel: () => request('POST', '/model/unload'),

  // Chat
  clearHistory: () => request('DELETE', '/chat/history'),
  setThinking: (enabled) => request('POST', '/chat/thinking', { enabled }),

  // SSE Chat stream
  chatStream(message, thinking, onThinking, onAnswer, onDone, onError) {
    const controller = new AbortController()

    fetch(`${BASE_URL}/chat`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ message, thinking }),
      signal: controller.signal,
    })
      .then(async (res) => {
        if (!res.ok) {
          const err = await res.json().catch(() => ({ error: res.statusText }))
          onError(new Error(err.error || 'Chat failed'))
          return
        }

        const reader = res.body.getReader()
        const decoder = new TextDecoder()
        let buffer = ''

        while (true) {
          const { done, value } = await reader.read()
          if (done) break

          buffer += decoder.decode(value, { stream: true })
          const lines = buffer.split('\n')
          buffer = lines.pop() // keep incomplete line

          let currentEvent = null
          for (const line of lines) {
            if (line.startsWith('event: ')) {
              currentEvent = line.slice(7).trim()
            } else if (line.startsWith('data: ')) {
              const dataStr = line.slice(6).trim()
              if (!dataStr || dataStr === 'keep-alive') continue
              try {
                const data = JSON.parse(dataStr)
                if (currentEvent === 'thinking') {
                  onThinking(data.chunk || '')
                } else if (currentEvent === 'answer') {
                  onAnswer(data.chunk || '')
                } else if (currentEvent === 'done') {
                  onDone()
                } else if (currentEvent === 'error') {
                  onError(new Error(data.error || 'Stream error'))
                }
              } catch (e) {
                // ignore parse errors
              }
              currentEvent = null
            }
          }
        }
      })
      .catch((err) => {
        if (err.name !== 'AbortError') onError(err)
      })

    return () => controller.abort()
  },
}
