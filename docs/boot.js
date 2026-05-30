/** WASM boot — progress UI, canvas mount, locale sync. */

const MOUNT_TIMEOUT_MS = 60_000;
const WASM_JS = new URL('./wasm/quantum_tetris.js', import.meta.url);
const WASM_BIN = new URL('./wasm/quantum_tetris_bg.wasm', import.meta.url);

let bootStarted = false;
let pendingLocale = null;
let setWebLocaleFn = null;

function normalizeLang(lang) {
  return lang === 'en' ? 'en' : 'fr';
}

function loaderText(msg) {
  const el = document.getElementById('loader-text');
  if (el) el.textContent = msg;
}

function t(key, fallback) {
  return window.qtI18n?.t?.(key) ?? fallback;
}

/** Called from i18n.js before WASM is ready. */
export function queueWebLocale(lang) {
  const l = normalizeLang(lang);
  pendingLocale = l;
  setWebLocaleFn?.(l);
}

function mountCanvas(frameInner, loader, onFail) {
  const deadline = Date.now() + MOUNT_TIMEOUT_MS;
  function tick() {
    const canvas = document.querySelector('canvas');
    if (!canvas) {
      if (Date.now() > deadline) {
        onFail?.();
        return;
      }
      requestAnimationFrame(tick);
      return;
    }
    loader?.classList.add('hidden');
    if (frameInner && canvas.parentElement !== frameInner) {
      frameInner.appendChild(canvas);
    }
    canvas.setAttribute('tabindex', '0');
    try {
      canvas.focus({ preventScroll: true });
    } catch {
      /* ignore focus errors */
    }
  }
  requestAnimationFrame(tick);
}

async function fetchWasmBytes(url, onProgress) {
  const res = await fetch(url, { cache: 'no-store' });
  if (!res.ok) {
    throw new Error(`WASM HTTP ${res.status}`);
  }
  const total = Number(res.headers.get('content-length') || 0);
  if (!res.body?.getReader || !total) {
    onProgress?.(0, 0);
    return res.arrayBuffer();
  }
  const reader = res.body.getReader();
  const chunks = [];
  let loaded = 0;
  while (true) {
    const { done, value } = await reader.read();
    if (done) break;
    chunks.push(value);
    loaded += value.length;
    onProgress?.(loaded, total);
  }
  const out = new Uint8Array(loaded);
  let offset = 0;
  for (const chunk of chunks) {
    out.set(chunk, offset);
    offset += chunk.length;
  }
  return out.buffer;
}

export async function bootGame({ frameInner, loader, errorEl }) {
  if (bootStarted) return;
  bootStarted = true;

  const fail = (msg) => {
    loader?.classList.add('hidden');
    if (errorEl) {
      errorEl.textContent = msg;
      errorEl.classList.add('visible');
    }
  };

  loaderText(t('play.starting', 'Loading…'));

  let mod;
  try {
    mod = await import(WASM_JS.href);
  } catch (e) {
    console.error(e);
    fail(t('error.wasmJs', 'WASM JavaScript bundle missing'));
    return;
  }

  const { default: init, run_wasm, set_web_locale } = mod;
  if (typeof init !== 'function' || typeof run_wasm !== 'function') {
    fail(t('error.wasmExports', 'WASM exports missing'));
    return;
  }

  setWebLocaleFn = (lang) => {
    if (typeof set_web_locale === 'function') {
      set_web_locale(normalizeLang(lang));
    }
  };
  window.__qtSetWebLocale = (lang) => queueWebLocale(lang);

  try {
    loaderText(t('play.downloading', 'Downloading game…'));
    const wasmBytes = await fetchWasmBytes(WASM_BIN.href, (loaded, total) => {
      if (!total) {
        loaderText(t('play.downloading', 'Downloading game…'));
        return;
      }
      const pct = Math.min(99, Math.round((loaded / total) * 100));
      const mb = (total / (1024 * 1024)).toFixed(0);
      loaderText(`${t('play.downloading', 'Downloading game…')} ${pct}% (~${mb} MB)`);
    });
    loaderText(t('play.initializing', 'Starting engine…'));
    await init(await WebAssembly.compile(wasmBytes));
  } catch (e) {
    console.error(e);
    fail(t('error.wasmBinary', 'WASM binary unavailable or failed to initialize'));
    return;
  }

  const lang = pendingLocale ?? window.qtI18n?.getLocale?.() ?? 'en';
  setWebLocaleFn(lang);

  mountCanvas(frameInner, loader, () => {
    fail(t('error.canvas', 'Canvas timeout'));
  });

  try {
    run_wasm();
  } catch (e) {
    console.error(e);
    fail(t('error.run', 'Game start failed'));
  }
}

export function bindLocaleSync(notify) {
  window.addEventListener('storage', (e) => {
    if (e.key === 'qt-lang') {
      notify(normalizeLang(window.qtI18n?.getLocale?.() ?? 'en'));
    }
  });
  window.addEventListener('qt-locale', (e) => {
    notify(normalizeLang(e.detail ?? 'en'));
  });
}

/** Hide broken circuit PNGs (e.g. local dev without render step). */
export function guardCircuitImages() {
  document.querySelectorAll('.circuit-fig img').forEach((img) => {
    img.addEventListener('error', () => {
      img.closest('.circuit-fig')?.classList.add('circuit-fig--missing');
    }, { once: true });
  });
}
