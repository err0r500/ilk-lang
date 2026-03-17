<script setup>
import { ref, onMounted } from 'vue'

const DEMO_CODE = `type HttpResponse = {
    status! Concrete<Int>
    body {...}
}

creationSuccess = HttpResponse {
    status 201
    body {uid Uuid}
}

notFound = HttpResponse {
    status 404
}

userProfile = HttpResponse {
    status 200
    body {
        uid Uuid
        name String
        admin Bool
    }
}`

const code = ref(DEMO_CODE)
const output = ref(null)
const wasmReady = ref(false)
let wasmCheck = null
let debounceTimer = null

onMounted(async () => {
  try {
    const mod = await import('/ilk-lang/wasm/ilk.js')
    await mod.default()
    wasmCheck = mod.check
    wasmReady.value = true
    runCheck()
  } catch (e) {
    console.error('Failed to load WASM compiler:', e)
  }
})

function runCheck() {
  if (!wasmCheck) return
  try {
    output.value = JSON.parse(wasmCheck(code.value))
  } catch (e) {
    output.value = { ok: false, errors: [{ severity: 'error', message: String(e), start: 0, end: 0 }] }
  }
}

function onInput(e) {
  code.value = e.target.value
  clearTimeout(debounceTimer)
  debounceTimer = setTimeout(runCheck, 300)
}

function reset() {
  code.value = DEMO_CODE
  runCheck()
}
</script>

<template>
  <div class="playground">
    <div class="pane editor-pane">
      <div class="pane-header">
        <span class="pane-title">Editor</span>
        <button class="reset-btn" @click="reset">Reset</button>
      </div>
      <textarea
        class="editor"
        :value="code"
        @input="onInput"
        spellcheck="false"
        autocomplete="off"
        autocorrect="off"
        autocapitalize="off"
      />
    </div>
    <div class="pane output-pane">
      <div class="pane-header">
        <span class="pane-title">Output</span>
      </div>
      <div class="output-body">
        <div v-if="!wasmReady" class="status loading">Loading compiler…</div>
        <div v-else-if="output === null" class="status loading">Running…</div>
        <div v-else-if="output.ok" class="status valid">✓ Valid</div>
        <div v-else class="errors">
          <div v-for="(err, i) in output.errors" :key="i" class="error-item">
            <span class="badge" :class="err.severity">{{ err.severity }}</span>
            <span class="error-msg">{{ err.message }}</span>
            <span class="error-loc">at offset {{ err.start }}–{{ err.end }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.playground {
  display: flex;
  gap: 12px;
  height: 420px;
  margin: 24px 0;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  overflow: hidden;
}

.pane {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-width: 0;
}

.pane-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 12px;
  background: var(--vp-c-bg-soft);
  border-bottom: 1px solid var(--vp-c-divider);
  flex-shrink: 0;
}

.editor-pane {
  border-right: 1px solid var(--vp-c-divider);
}

.pane-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--vp-c-text-2);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.reset-btn {
  font-size: 12px;
  padding: 2px 10px;
  border-radius: 4px;
  border: 1px solid var(--vp-c-divider);
  background: var(--vp-c-bg);
  color: var(--vp-c-text-1);
  cursor: pointer;
  transition: background 0.15s;
}

.reset-btn:hover {
  background: var(--vp-c-bg-mute);
}

.editor {
  flex: 1;
  width: 100%;
  padding: 12px;
  font-family: var(--vp-font-family-mono);
  font-size: 13px;
  line-height: 1.6;
  background: var(--vp-c-bg);
  color: var(--vp-c-text-1);
  border: none;
  outline: none;
  resize: none;
  tab-size: 4;
}

.output-body {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
  font-family: var(--vp-font-family-mono);
  font-size: 13px;
}

.status {
  padding: 4px 0;
}

.loading {
  color: var(--vp-c-text-3);
}

.valid {
  color: var(--vp-c-green-1, #3dd68c);
  font-weight: 600;
}

.errors {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.error-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: 8px 10px;
  background: var(--vp-c-danger-soft, rgba(255, 85, 85, 0.08));
  border-left: 3px solid var(--vp-c-red-1, #f44336);
  border-radius: 0 4px 4px 0;
}

.badge {
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.badge.error {
  color: var(--vp-c-red-1, #f44336);
}

.badge.warning {
  color: var(--vp-c-yellow-1, #e6a817);
}

.error-msg {
  color: var(--vp-c-text-1);
  word-break: break-word;
}

.error-loc {
  font-size: 11px;
  color: var(--vp-c-text-3);
}

@media (max-width: 640px) {
  .playground {
    flex-direction: column;
    height: auto;
  }

  .editor-pane {
    border-right: none;
    border-bottom: 1px solid var(--vp-c-divider);
    height: 280px;
  }

  .output-pane {
    height: 180px;
  }
}
</style>
