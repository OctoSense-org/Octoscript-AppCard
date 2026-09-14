import { WasmWebGL } from './makepad_platform/web_gl.js';

const encoder = new TextEncoder();
const decoder = new TextDecoder();
const origin = window.location.origin;
const assetBase = new URL('./card-assets/', window.location.href);
const status = document.querySelector('#status');
const events = [];
let instance;
let nativeReady = false;
let pending = [];

function publish(event) {
  events.push(event);
  if (events.length > 200) events.shift();
  window.dispatchEvent(new CustomEvent('octosense:native-event', { detail: event }));
  if (window.parent !== window) window.parent.postMessage(event, origin);
  if (event.type === 'octosense:error') {
    status.textContent = event.error;
    status.hidden = false;
    console.error('Octosense Makepad:', event);
  }
  if (event.type === 'octosense:ready') {
    nativeReady = true;
    status.hidden = true;
    for (const command of pending) send(command);
    pending = [];
  }
  if (event.type === 'octosense:rendered') {
    window.__octosense.snapshot = event;
    status.hidden = true;
  }
}

function send(command) {
  if (!instance || !nativeReady) {
    pending.push(command);
    return;
  }
  const bytes = encoder.encode(JSON.stringify(command));
  const ptr = instance.exports.octosense_alloc(bytes.length);
  new Uint8Array(instance._memory.buffer, ptr, bytes.length).set(bytes);
  if (!instance.exports.octosense_command(ptr, bytes.length)) {
    throw new Error('Native runtime rejected the command encoding');
  }
}

function mount(payload) {
  const command = { ...payload, type: 'octosense:render' };
  if (command.id === undefined || command.generation === undefined) {
    throw new Error('Each mount needs id and generation');
  }
  function validateAssets(value) {
    if (!value || typeof value !== 'object') return;
    for (const [key, child] of Object.entries(value)) {
      if (key === 'src' && typeof child === 'string') {
        const url = new URL(child);
        const tail = url.pathname.slice(assetBase.pathname.length);
        if (url.origin !== origin || url.href !== child || !url.pathname.startsWith(assetBase.pathname)
            || url.username || url.password || url.search || url.hash
            || !/^[A-Za-z0-9_.-]+\/assets\/[A-Za-z0-9_.-]+$/.test(tail)
            || tail.split('/').some(part => part === '.' || part === '..')) {
          throw new Error('Card artwork must stay inside the iframe same-origin card-assets directory');
        }
      } else validateAssets(child);
    }
  }
  validateAssets(command.data);
  validateAssets(command.kit);
  send(command);
}

window.__octosense = {
  renderer: 'Makepad/WASM', events, snapshot: null, mount,
  inspect() { send({ type: 'octosense:inspect' }); },
  get ready() { return nativeReady; },
};

window.addEventListener('message', (event) => {
  if (event.source !== window.parent || event.origin !== origin) return;
  const message = event.data;
  if (!message || typeof message !== 'object') return;
  try {
    if (message.type === 'octosense:render') mount(message.payload ?? message);
    if (message.type === 'octosense:inspect') window.__octosense.inspect();
    if (message.type === 'octosense:ping' && nativeReady) {
      publish({ type: 'octosense:ready', renderer: 'Makepad/WASM', protocol: 1 });
    }
  } catch (error) {
    publish({ type: 'octosense:error', id: message.id, generation: message.generation, error: String(error) });
  }
});

try {
  instance = await WasmWebGL.fetch_and_instantiate_wasm('./octosense-wizard.wasm');
  if (!instance?.exports?.octosense_command) throw new Error('Makepad WASM did not instantiate');
  const canvas = document.querySelector('canvas');
  window.__octosense.webgl = new WasmWebGL(instance, {}, canvas);
  setInterval(() => {
    try {
      const pointer = instance.exports.octosense_events();
      const length = instance.exports.octosense_events_len();
      const batch = JSON.parse(decoder.decode(new Uint8Array(instance._memory.buffer, pointer, length)));
      batch.forEach(publish);
    } catch (error) {
      if (!window.__octosense.failed) publish({ type: 'octosense:error', error: String(error) });
      window.__octosense.failed = true;
    }
  }, 30);
} catch (error) {
  publish({ type: 'octosense:error', error: String(error) });
}
