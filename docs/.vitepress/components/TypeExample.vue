<script setup>
import { ref, onMounted, computed, watch } from 'vue'
import init, { check } from '../wasm/ilk.js'
import { createHighlighter } from 'shiki'
import ilkLang from '../langs/ilk.tmLanguage.json'

const props = defineProps({
  typeCode: { type: String, required: true },
  instances: { type: Array, required: true }
})

const activeTab = ref(0)
const output = ref(null)
const wasmReady = ref(false)
const typeHtml = ref('')
const instanceHtmls = ref([])
let wasmCheck = null
let highlighter = null

const fullCode = computed(() => {
  return props.typeCode.trim() + '\n\n' + props.instances[activeTab.value].code.trim()
})

async function highlight() {
  highlighter = await createHighlighter({
    themes: ['github-dark'],
    langs: [ilkLang]
  })
  typeHtml.value = highlighter.codeToHtml(props.typeCode.trim(), {
    lang: 'ilk',
    theme: 'github-dark'
  })
  instanceHtmls.value = props.instances.map(inst =>
    highlighter.codeToHtml(inst.code.trim(), {
      lang: 'ilk',
      theme: 'github-dark'
    })
  )
}

function runCheck() {
  if (!wasmCheck) return
  try {
    output.value = JSON.parse(wasmCheck(fullCode.value))
  } catch (e) {
    output.value = { ok: false, errors: [{ severity: 'error', message: String(e), start: 0, end: 0 }] }
  }
}

onMounted(async () => {
  await highlight()

  try {
    await init()
    wasmCheck = check
    wasmReady.value = true
    runCheck()
  } catch (e) {
    console.error('Failed to load WASM compiler:', e)
  }
})

watch(activeTab, () => {
  runCheck()
})
</script>

<template>
  <div class="type-example">
    <div class="code-panes">
      <div class="type-pane">
        <div class="pane-header">Type</div>
        <div class="code-body" v-html="typeHtml"></div>
      </div>
      <div class="instance-pane">
        <div class="tabs">
          <button
            v-for="(inst, i) in instances"
            :key="i"
            class="tab"
            :class="{ active: activeTab === i, pass: inst.expect === 'pass', fail: inst.expect !== 'pass' }"
            @click="activeTab = i"
          ><span class="tab-icon">{{ inst.expect === 'pass' ? '✓' : '✗' }}</span> {{ inst.label }}</button>
        </div>
        <div class="code-body" v-html="instanceHtmls[activeTab]"></div>
      </div>
    </div>
    <div class="output-pane">
      <div v-if="!wasmReady" class="status loading">Loading compiler…</div>
      <div v-else-if="output === null" class="status loading">Running…</div>
      <div v-else-if="output.ok" class="status valid">✓ valid</div>
      <div v-else class="errors">
        <div v-for="(err, i) in output.errors" :key="i" class="error-item">
          <span class="badge" :class="err.severity">{{ err.severity }}</span>
          <span class="error-msg">{{ err.message }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.type-example {
  margin: 24px 0;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  overflow: hidden;
}

.code-panes {
  display: flex;
  align-items: stretch;
}

.type-pane {
  flex: 1;
  border-right: 1px solid var(--vp-c-divider);
  display: flex;
  flex-direction: column;
}

.instance-pane {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.type-pane .code-body,
.instance-pane .code-body {
  flex: 1;
}

.pane-header {
  padding: 6px 12px;
  background: var(--vp-c-bg-soft);
  border-bottom: 1px solid var(--vp-c-divider);
  font-size: 12px;
  font-weight: 600;
  color: var(--vp-c-text-2);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.tabs {
  display: flex;
  background: var(--vp-c-bg-soft);
  border-bottom: 1px solid var(--vp-c-divider);
}

.tab {
  padding: 6px 12px;
  font-size: 12px;
  font-weight: 600;
  color: var(--vp-c-text-2);
  background: transparent;
  border: none;
  border-left: 3px solid transparent;
  cursor: pointer;
  transition: background 0.15s;
}

.tab:hover {
  background: var(--vp-c-bg-mute);
}

.tab.active {
  background: var(--vp-c-bg);
  border-bottom: 2px solid var(--vp-c-text-1);
}

.tab.pass {
  border-left-color: var(--vp-c-green-1, #3dd68c);
  color: var(--vp-c-green-1, #3dd68c);
}

.tab.fail {
  border-left-color: var(--vp-c-red-1, #f44336);
  color: var(--vp-c-red-1, #f44336);
}

.tab-icon {
  font-weight: bold;
}

.code-body {
  padding: 12px;
  background: #24292e;
  font-family: var(--vp-font-family-mono);
  font-size: 13px;
  line-height: 1.6;
  overflow-x: auto;
  min-height: 80px;
}

.code-body :deep(pre) {
  margin: 0;
  padding: 0;
  background: transparent !important;
}

.code-body :deep(code) {
  background: transparent !important;
}

.output-pane {
  padding: 12px;
  border-top: 1px solid var(--vp-c-divider);
  font-family: var(--vp-font-family-mono);
  font-size: 13px;
}

.status.loading {
  color: var(--vp-c-text-3);
}

.status.valid {
  color: var(--vp-c-green-1, #3dd68c);
  font-weight: 600;
}

.errors {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.error-item {
  display: flex;
  gap: 8px;
  align-items: baseline;
}

.badge {
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
}

.badge.error {
  color: var(--vp-c-red-1, #f44336);
}

.error-msg {
  color: var(--vp-c-text-1);
}

@media (max-width: 640px) {
  .code-panes {
    flex-direction: column;
  }

  .type-pane {
    border-right: none;
    border-bottom: 1px solid var(--vp-c-divider);
  }
}
</style>
