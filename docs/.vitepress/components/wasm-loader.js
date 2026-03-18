import init, { check } from '../wasm/ilk.js'

let initPromise = null

export async function getCheck() {
  if (!initPromise) {
    initPromise = init().then(() => check)
  }
  return initPromise
}
