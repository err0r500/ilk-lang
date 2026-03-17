<script setup>
import { ref, onMounted, onUnmounted, shallowRef } from 'vue'
import { EditorView, keymap, lineNumbers, highlightActiveLine } from '@codemirror/view'
import { EditorState, Compartment } from '@codemirror/state'
import { defaultKeymap, history, historyKeymap } from '@codemirror/commands'
import { syntaxHighlighting, HighlightStyle } from '@codemirror/language'
import { tags as t } from '@lezer/highlight'
import { vim } from '@replit/codemirror-vim'
import { ilkLang } from './ilk-lang.js'

const vimCompartment = new Compartment()
const lineNumberCompartment = new Compartment()

function makeRelativeLineNumbers(state) {
  const cursorLine = state.doc.lineAt(state.selection.main.head).number
  return lineNumbers({
    formatNumber: (n) => n === cursorLine ? String(n) : String(Math.abs(n - cursorLine))
  })
}

const DEMO_CODE = `
type Event = {...} & {_type Concrete<String>}

type Command = {
  fields {...}

  @source [fields]
  emits []Event
}

createCart = Command {
   fields {
       // make this field optional (cartId?)
       // or change its type (Int) to see the validation fail
       cartId Uuid
   }

   emits [
      {_type "CartCreated", id Uuid = fields.cartId}
    ]
}
`

// GitHub Dark theme colors
const ilkHighlight = HighlightStyle.define([
  { tag: t.comment, color: '#6a737d' },
  { tag: t.string, color: '#9ecbff' },
  { tag: t.number, color: '#79b8ff' },
  { tag: t.keyword, color: '#f97583' },
  { tag: t.meta, color: '#b392f0' },
  { tag: t.typeName, color: '#79b8ff' },
  { tag: t.operator, color: '#f97583' },
  { tag: t.variableName, color: '#e1e4e8' },
  { tag: t.bracket, color: '#e1e4e8' },
])

const darkTheme = EditorView.theme({
  '&': {
    backgroundColor: '#24292e',
    color: '#e1e4e8',
    height: '100%',
  },
  '.cm-content': {
    fontFamily: 'var(--vp-font-family-mono)',
    fontSize: '13px',
    lineHeight: '1.6',
    padding: '12px 0',
    caretColor: '#fff',
  },
  '.cm-line': {
    padding: '0 12px',
  },
  '.cm-gutters': {
    backgroundColor: '#24292e',
    color: '#6a737d',
    border: 'none',
    paddingLeft: '8px',
  },
  '.cm-activeLineGutter': {
    backgroundColor: '#2d333b',
  },
  '.cm-activeLine': {
    backgroundColor: '#2d333b',
  },
  '.cm-cursor': {
    borderLeftColor: '#fff',
  },
  '&.cm-focused .cm-selectionBackground, ::selection': {
    backgroundColor: '#3392ff44',
  },
  '.cm-scroller': {
    overflow: 'auto',
  },
}, { dark: true })

const code = ref(DEMO_CODE)
const output = ref(null)
const wasmReady = ref(false)
const vimMode = ref(false)
const editorContainer = ref(null)
const editorView = shallowRef(null)
let wasmCheck = null
let debounceTimer = null

onMounted(async () => {
  // Initialize CodeMirror
  const startState = EditorState.create({
    doc: code.value,
    extensions: [
      vimCompartment.of([]),
      lineNumberCompartment.of(lineNumbers()),
      highlightActiveLine(),
      history(),
      keymap.of([...defaultKeymap, ...historyKeymap]),
      ilkLang,
      syntaxHighlighting(ilkHighlight),
      darkTheme,
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          code.value = update.state.doc.toString()
          clearTimeout(debounceTimer)
          debounceTimer = setTimeout(runCheck, 300)
        }
        if (vimMode.value && update.selectionSet) {
          update.view.dispatch({
            effects: lineNumberCompartment.reconfigure(makeRelativeLineNumbers(update.state))
          })
        }
      }),
    ],
  })

  editorView.value = new EditorView({
    state: startState,
    parent: editorContainer.value,
  })

  // Load WASM (served as a public asset, not resolved by Rollup at build time)
  try {
    const wasmUrl = `${import.meta.env.BASE_URL}wasm/ilk.js`
    const mod = await import(/* @vite-ignore */ wasmUrl)
    await mod.default()
    wasmCheck = mod.check
    wasmReady.value = true
    runCheck()
  } catch (e) {
    console.error('Failed to load WASM compiler:', e)
  }
})

onUnmounted(() => {
  if (editorView.value) {
    editorView.value.destroy()
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

function reset() {
  if (editorView.value) {
    editorView.value.dispatch({
      changes: { from: 0, to: editorView.value.state.doc.length, insert: DEMO_CODE }
    })
  }
  runCheck()
}

function toggleVim() {
  vimMode.value = !vimMode.value
  if (editorView.value) {
    const state = editorView.value.state
    editorView.value.dispatch({
      effects: [
        vimCompartment.reconfigure(vimMode.value ? vim() : []),
        lineNumberCompartment.reconfigure(
          vimMode.value ? makeRelativeLineNumbers(state) : lineNumbers()
        )
      ]
    })
  }
}
</script>

<template>
  <div class="playground">
    <div class="pane editor-pane">
      <div class="pane-header">
        <span class="pane-title">Editor</span>
        <div class="header-actions">
          <button class="toggle-btn" :class="{ active: vimMode }" @click="toggleVim">Vim</button>
          <button class="reset-btn" @click="reset">Reset</button>
        </div>
      </div>
      <div class="editor-wrapper" ref="editorContainer"></div>
    </div>
    <div class="pane output-pane">
      <div class="pane-header">
        <span class="pane-title">Output</span>
      </div>
      <div class="output-body">
        <div v-if="!wasmReady" class="status loading">Loading compiler…</div>
        <div v-else-if="output === null" class="status loading">Running…</div>
        <div v-else-if="output.ok" class="status valid">✓ Your model is sound</div>
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
  flex-direction: column;
  height: calc(100vh - 180px);
  min-height: 400px;
  margin: 24px 0;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  overflow: hidden;
}

.pane {
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.editor-pane {
  flex: 4;
  border-bottom: 1px solid var(--vp-c-divider);
}

.output-pane {
  flex: 1;
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

.pane-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--vp-c-text-2);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.reset-btn,
.toggle-btn {
  font-size: 12px;
  padding: 2px 10px;
  border-radius: 4px;
  border: 1px solid var(--vp-c-divider);
  background: var(--vp-c-bg);
  color: var(--vp-c-text-1);
  cursor: pointer;
  transition: background 0.15s;
}

.reset-btn:hover,
.toggle-btn:hover {
  background: var(--vp-c-bg-mute);
}

.toggle-btn.active {
  background: var(--vp-c-brand-1);
  color: #fff;
  border-color: var(--vp-c-brand-1);
}

.editor-wrapper {
  flex: 1;
  overflow: hidden;
  background: #24292e;
}

.editor-wrapper :deep(.cm-editor) {
  height: 100%;
}

.editor-wrapper :deep(.cm-vim-panel) {
  background: #1a1d21;
  color: #e1e4e8;
  padding: 2px 8px;
  font-family: var(--vp-font-family-mono);
  font-size: 12px;
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
    height: auto;
  }

  .editor-pane {
    min-height: 300px;
  }

  .output-pane {
    min-height: 120px;
  }
}
</style>
