/** WASM boot — correct paths for GitHub Pages subpaths, canvas mount before run(). */

function mountCanvas(frameInner, loader) {
  function tick() {
    const canvas = document.querySelector('#frame canvas, canvas');
    if (!canvas) {
      requestAnimationFrame(tick);
      return;
    }
    loader?.classList.add('hidden');
    if (frameInner && canvas.parentElement !== frameInner) {
      frameInner.appendChild(canvas);
    }
    canvas.setAttribute('tabindex', '0');
    canvas.focus({ preventScroll: true });
  }
  requestAnimationFrame(tick);
}

export async function bootGame({ frameInner, loader, errorEl }) {
  const wasmModule = new URL('./wasm/quantum_tetris.js', import.meta.url);
  const wasmBinary = new URL('./wasm/quantum_tetris_bg.wasm', import.meta.url);

  try {
    const head = await fetch(wasmBinary, { method: 'HEAD' });
    if (!head.ok) {
      throw new Error(`WASM not found (${head.status})`);
    }
  } catch (e) {
    if (e.message?.includes('WASM not found')) throw e;
    // HEAD may fail offline or on some hosts — continue and let init report.
  }

  const { default: init, run_wasm, set_web_locale } = await import(wasmModule.href);

  window.__qtSetWebLocale = set_web_locale;

  mountCanvas(frameInner, loader);

  await init(wasmBinary);
  set_web_locale(window.qtI18n?.getLocale?.() ?? 'fr');
  run_wasm();
}

export function bindLocaleSync(notify) {
  window.addEventListener('storage', (e) => {
    if (e.key === 'qt-lang') {
      notify(window.qtI18n?.getLocale?.() ?? 'fr');
    }
  });
}
