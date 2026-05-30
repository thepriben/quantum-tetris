/** WASM boot — GitHub Pages paths, canvas mount before run(), locale queue. */

const MOUNT_TIMEOUT_MS = 45_000;
const WASM_JS = new URL('./wasm/quantum_tetris.js', import.meta.url);
const WASM_BIN = new URL('./wasm/quantum_tetris_bg.wasm', import.meta.url);

let bootStarted = false;
let pendingLocale = null;
let setWebLocaleFn = null;

function normalizeLang(lang) {
  return lang === 'en' ? 'en' : 'fr';
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

async function wasmReachable() {
  try {
    const res = await fetch(WASM_BIN, {
      method: 'GET',
      cache: 'no-store',
      headers: { Range: 'bytes=0-0' },
    });
    return res.ok || res.status === 206;
  } catch {
    // Preflight failed — still try init (HEAD/Range not supported everywhere).
    return true;
  }
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

  if (!(await wasmReachable())) {
    console.warn('WASM preflight failed — attempting load anyway');
  }

  let mod;
  try {
    mod = await import(WASM_JS.href);
  } catch (e) {
    console.error(e);
    fail(window.qtI18n?.t?.('error.wasm') ?? 'WASM load failed');
    return;
  }

  const { default: init, run_wasm, set_web_locale } = mod;
  if (typeof init !== 'function' || typeof run_wasm !== 'function') {
    fail(window.qtI18n?.t?.('error.wasm') ?? 'WASM exports missing');
    return;
  }

  setWebLocaleFn = (lang) => {
    if (typeof set_web_locale === 'function') {
      set_web_locale(normalizeLang(lang));
    }
  };
  window.__qtSetWebLocale = (lang) => queueWebLocale(lang);

  mountCanvas(frameInner, loader, () => {
    fail(window.qtI18n?.t?.('error.canvas') ?? 'Canvas timeout');
  });

  try {
    await init(WASM_BIN);
  } catch (e) {
    console.error(e);
    fail(window.qtI18n?.t?.('error.wasm') ?? 'WASM init failed');
    return;
  }

  const lang = pendingLocale ?? window.qtI18n?.getLocale?.() ?? 'fr';
  setWebLocaleFn(lang);

  try {
    run_wasm();
  } catch (e) {
    console.error(e);
    fail(window.qtI18n?.t?.('error.run') ?? 'Game start failed');
  }
}

export function bindLocaleSync(notify) {
  window.addEventListener('storage', (e) => {
    if (e.key === 'qt-lang') {
      notify(normalizeLang(window.qtI18n?.getLocale?.() ?? 'fr'));
    }
  });
  window.addEventListener('qt-locale', (e) => {
    notify(normalizeLang(e.detail ?? 'fr'));
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
