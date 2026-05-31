/** WASM boot — progress UI, canvas mount, locale sync. */

const MOUNT_TIMEOUT_MS = 60_000;
const WASM_ASSET_VERSION = String(Date.now());
const WASM_JS = wasmAssetUrl('./wasm/quantum_tetris.js');
const WASM_BIN = wasmAssetUrl('./wasm/quantum_tetris_bg.wasm');

class WasmLoadError extends Error {
  constructor(kind, message) {
    super(message);
    this.name = 'WasmLoadError';
    this.kind = kind;
  }
}

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

function wasmAssetUrl(path) {
  const url = new URL(path, import.meta.url);
  url.searchParams.set('v', WASM_ASSET_VERSION);
  return url;
}

function displayUrl(url) {
  const parsed = new URL(url, window.location.href);
  return `${parsed.pathname}${parsed.search}`;
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
  let res;
  try {
    res = await fetch(url, { cache: 'no-store' });
  } catch (error) {
    throw new WasmLoadError('wasmNetwork', readableError(error));
  }
  if (!res.ok) {
    throw new WasmLoadError(
      'wasmHttp',
      `${res.status} ${res.statusText || 'HTTP error'} at ${displayUrl(url)}`,
    );
  }
  const total = Number(res.headers.get('content-length') || 0);
  if (!res.body?.getReader || !total) {
    onProgress?.(0, 0);
    try {
      return await res.arrayBuffer();
    } catch (error) {
      throw new WasmLoadError('wasmNetwork', readableError(error));
    }
  }
  const reader = res.body.getReader();
  const chunks = [];
  let loaded = 0;
  while (true) {
    let item;
    try {
      item = await reader.read();
    } catch (error) {
      throw new WasmLoadError('wasmNetwork', readableError(error));
    }
    const { done, value } = item;
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

function assertWasmMagic(buffer) {
  const magic = new Uint8Array(buffer, 0, Math.min(4, buffer.byteLength));
  if (
    magic.length !== 4 ||
    magic[0] !== 0x00 ||
    magic[1] !== 0x61 ||
    magic[2] !== 0x73 ||
    magic[3] !== 0x6d
  ) {
    throw new WasmLoadError(
      'wasmNotBinary',
      `${buffer.byteLength} bytes; first bytes ${hexPrefix(buffer) || 'none'}`,
    );
  }
}

function hexPrefix(buffer) {
  return Array.from(new Uint8Array(buffer, 0, Math.min(8, buffer.byteLength)))
    .map((byte) => byte.toString(16).padStart(2, '0'))
    .join('');
}

function readableError(error) {
  if (error instanceof Error && error.message) {
    return error.message;
  }
  return String(error || 'unknown error');
}

function wasmErrorMessage(error) {
  const key = error instanceof WasmLoadError ? `error.${error.kind}` : 'error.wasmInit';
  const fallback = t('error.wasmUnknown', 'WASM failed to load.');
  const base = t(key, fallback);
  const detail = readableError(error);
  return `${base}\n${t('error.detail', 'Detail')}: ${detail}`;
}

function errorMessageWithDetail(key, fallback, detail) {
  return `${t(key, fallback)}\n${t('error.detail', 'Detail')}: ${detail}`;
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
    fail(errorMessageWithDetail(
      'error.wasmJs',
      'WASM JavaScript bundle missing',
      readableError(e),
    ));
    return;
  }

  const { default: init, run_wasm, set_web_locale } = mod;
  if (typeof init !== 'function' || typeof run_wasm !== 'function') {
    fail(errorMessageWithDetail(
      'error.wasmExports',
      'WASM exports missing',
      `loaded ${displayUrl(WASM_JS.href)}`,
    ));
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
    assertWasmMagic(wasmBytes);
    loaderText(t('play.initializing', 'Starting engine…'));
    try {
      await init(wasmBytes);
    } catch (error) {
      throw new WasmLoadError('wasmInit', readableError(error));
    }
  } catch (e) {
    console.error(e);
    fail(wasmErrorMessage(e));
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
