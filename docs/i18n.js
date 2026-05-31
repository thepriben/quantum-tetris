/** English-first i18n — syncs with in-game WASM locale via set_web_locale. */
(function () {
  const STORAGE_KEY = 'qt-lang';
  const DEFAULT = 'en';

  const T = {
    fr: {
      'meta.title': 'Quantum Tetris',
      'lang.group': 'Langue',
      'lang.fr': 'Français',
      'lang.en': 'English',
      'home.title': 'Quantum Tetris',
      'home.subtitle': 'Tetris — événements stochastiques via circuits quantiques',
      'home.lead':
        'Règles Tetris habituelles. Pièce, état d’apparition, cadence, bonus : chaque événement stochastique exécute un circuit prédéfini ; les bits mesurés déterminent l’effet en jeu.',
      'home.stack':
        '<strong>Architecture.</strong> Bevy (Rust) → WASM dans le navigateur. <a href="https://github.com/Renmusxd/RustQIP">RustQIP</a> (<code>RustQipBackend</code>) simule le statevector par défaut en local et en ligne.',
      'home.principle':
        '<strong>Le joueur :</strong> ← → ↑ ↓ et Espace. <strong>Le jeu :</strong> tout le reste.',
      'home.quantumTitle': 'Mécaniques',
      'home.label.game': 'En jeu',
      'home.label.quantum': 'Circuit',
      'home.moment.piece.title': 'Pièce en cours & « suiv. »',
      'home.moment.piece.game': 'Détermine la forme active et l\'aperçu suivant.',
      'home.moment.piece.quantum':
        '<code>quantum-teleportation-gate-v1</code> (×2) — mesure de Bell inspirée de la téléportation ; les bits mesurés fixent la famille (I, O, T…) et la variante.',
      'home.moment.brain.title': 'Rotation & colonne',
      'home.moment.brain.game': 'Fixe l\'orientation à l\'apparition et la colonne d\'entrée.',
      'home.moment.brain.quantum':
        '<code>imp-brain-v1</code> — 2 qubits mesurés → rotation (0–3) et colonne d\'apparition.',
      'home.moment.hunter.title': 'Cadence de chute',
      'home.moment.hunter.game': 'Intervalle entre deux descentes ; diminue avec le niveau.',
      'home.moment.hunter.quantum':
        '<code>enemy-profile-hunter-v1</code> — 2 qubits mesurés → intervalle de chute pour la pièce en cours.',
      'home.moment.observe.title': 'Espace — chute forcée',
      'home.moment.observe.game': 'Pose immédiate ; bonus de score, parfois une ligne supplémentaire.',
      'home.moment.observe.quantum':
        '<code>observation-pulse-v1</code> — mesure à la pose forcée ; les bits choisissent le bonus.',
      'home.moment.line.title': 'Ligne complétée',
      'home.moment.line.game': 'Multiplicateur de points ×1 à ×4 selon le tirage.',
      'home.moment.line.quantum':
        '<code>q-shard-stabilizer-v1</code> — après effacement d\'une ligne ; les bits fixent le multiplicateur (×1–×4).',
      'home.circuitsLink': 'Référence circuits',
      'home.github': 'GitHub',
      'home.readme': 'Build local',
      'play.starting': 'Chargement…',
      'play.downloading': 'Téléchargement du jeu…',
      'play.initializing': 'Démarrage du moteur…',
      'play.hint': 'Premier chargement volumineux. Attendre la fin du téléchargement.',
      'error.wasmJs':
        'Bundle JavaScript WASM introuvable — lancez ./scripts/build_wasm.sh puis servez docs/.',
      'error.wasmExports': 'Le bundle WASM ne contient pas les exports attendus.',
      'error.wasmNetwork':
        'Téléchargement du jeu interrompu. Vérifiez la connexion puis rechargez la page.',
      'error.wasmHttp':
        'Le binaire WASM n’est pas publié à l’URL attendue. Le workflow Pages n’a probablement pas terminé ou l’artefact est incomplet.',
      'error.wasmNotBinary':
        'L’URL du WASM répond, mais ne renvoie pas un binaire WebAssembly. GitHub Pages sert peut-être une page HTML d’erreur ou un cache incomplet.',
      'error.wasmInit':
        'Le binaire WASM est téléchargé, mais le navigateur ne peut pas l’initialiser. Rechargez sans cache ; si ça persiste, le JS et le WASM publiés ne correspondent peut-être pas.',
      'error.wasmUnknown': 'Chargement WASM impossible.',
      'error.detail': 'Détail',
      'error.canvas': 'Le canvas n\'a pas démarré — recharger la page.',
      'error.run': 'Erreur au lancement du jeu — voir la console.',
    },
    en: {
      'meta.title': 'Quantum Tetris',
      'lang.group': 'Language',
      'lang.fr': 'Français',
      'lang.en': 'English',
      'home.title': 'Quantum Tetris',
      'home.subtitle': 'Tetris — stochastic events from quantum circuits',
      'home.lead':
        'Standard Tetris rules. Piece, spawn state, cadence, bonuses: each stochastic event runs a predefined circuit; measured bits drive the in-game effect.',
      'home.stack':
        '<strong>Architecture.</strong> Bevy (Rust) → WASM in the browser. <a href="https://github.com/Renmusxd/RustQIP">RustQIP</a> (<code>RustQipBackend</code>) simulates the statevector by default locally and online.',
      'home.principle':
        '<strong>The player:</strong> ← → ↑ ↓ and Space. <strong>The game:</strong> everything else.',
      'home.quantumTitle': 'Mechanics',
      'home.label.game': 'In-game',
      'home.label.quantum': 'Circuit',
      'home.moment.piece.title': 'Active piece & “next”',
      'home.moment.piece.game': 'Sets the active shape and the next preview.',
      'home.moment.piece.quantum':
        '<code>quantum-teleportation-gate-v1</code> (×2) — teleportation-inspired Bell measurement; measured bits pick the family (I, O, T…) and variant.',
      'home.moment.brain.title': 'Rotation & column',
      'home.moment.brain.game': 'Sets spawn orientation and entry column.',
      'home.moment.brain.quantum':
        '<code>imp-brain-v1</code> — 2 measured qubits → rotation (0–3) and spawn column.',
      'home.moment.hunter.title': 'Drop cadence',
      'home.moment.hunter.game': 'Interval between grid steps; decreases as level rises.',
      'home.moment.hunter.quantum':
        '<code>enemy-profile-hunter-v1</code> — 2 measured qubits → drop interval for the active piece.',
      'home.moment.observe.title': 'Space — hard drop',
      'home.moment.observe.game': 'Instant lock; score bonus, sometimes one extra line.',
      'home.moment.observe.quantum':
        '<code>observation-pulse-v1</code> — measure on hard drop; bits select the score bonus.',
      'home.moment.line.title': 'Line clear',
      'home.moment.line.game': 'Score multiplier ×1 to ×4 from the draw.',
      'home.moment.line.quantum':
        '<code>q-shard-stabilizer-v1</code> — after a line clear; bits set the score multiplier (×1–×4).',
      'home.circuitsLink': 'Circuit reference',
      'home.github': 'GitHub',
      'home.readme': 'Local build',
      'play.starting': 'Loading…',
      'play.downloading': 'Downloading game…',
      'play.initializing': 'Starting engine…',
      'play.hint': 'Large initial download. Wait for the progress bar to finish.',
      'error.wasmJs':
        'WASM JavaScript bundle missing — run ./scripts/build_wasm.sh, then serve docs/.',
      'error.wasmExports': 'WASM bundle is present but does not expose the expected game entry points.',
      'error.wasmNetwork':
        'Game download was interrupted. Check the connection, then reload the page.',
      'error.wasmHttp':
        'The WASM binary is not published at the expected URL. The Pages workflow may still be running or the artifact is incomplete.',
      'error.wasmNotBinary':
        'The WASM URL responded, but it did not return a WebAssembly binary. GitHub Pages may be serving an HTML error page or an incomplete cache.',
      'error.wasmInit':
        'The WASM binary downloaded, but the browser could not initialize it. Hard-refresh the page; if it persists, the published JS and WASM may not match.',
      'error.wasmUnknown': 'WASM failed to load.',
      'error.detail': 'Detail',
      'error.canvas': 'Canvas did not start — reload the page.',
      'error.run': 'Game failed to start — see console.',
    },
  };

  function getLocale() {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      return stored === 'fr' ? 'fr' : DEFAULT;
    } catch {
      return DEFAULT;
    }
  }

  function t(key) {
    const dict = T[getLocale()];
    return dict?.[key] ?? key;
  }

  function notifyGame(lang) {
    const l = lang === 'en' ? 'en' : 'fr';
    if (typeof window.__qtSetWebLocale === 'function') {
      window.__qtSetWebLocale(l);
    } else if (typeof window.__qtQueueLocale === 'function') {
      window.__qtQueueLocale(l);
    }
  }

  function setLocale(lang) {
    const l = lang === 'en' ? 'en' : 'fr';
    try {
      localStorage.setItem(STORAGE_KEY, l);
    } catch {
      /* private mode */
    }
    apply();
    notifyGame(l);
    window.dispatchEvent(new CustomEvent('qt-locale', { detail: l }));
  }

  function apply() {
    const lang = getLocale();
    document.documentElement.lang = lang;
    const dict = T[lang];
    document.querySelectorAll('[data-i18n]').forEach((el) => {
      const key = el.getAttribute('data-i18n');
      const val = dict[key];
      if (val == null) return;
      const attr = el.getAttribute('data-i18n-attr');
      if (attr) {
        el.setAttribute(attr, val);
      } else if (el.hasAttribute('data-i18n-html')) {
        el.innerHTML = val;
      } else {
        el.textContent = val;
      }
    });
    document.title = dict['meta.title'] || document.title;
    document.querySelectorAll('.lang-btn[data-lang]').forEach((btn) => {
      const active = btn.getAttribute('data-lang') === lang;
      btn.classList.toggle('active', active);
      btn.setAttribute('aria-pressed', active ? 'true' : 'false');
    });
    const group = document.querySelector('.lang-flags');
    if (group && dict['lang.group']) {
      group.setAttribute('aria-label', dict['lang.group']);
    }
  }

  window.qtI18n = { getLocale, setLocale, apply, t };

  function bindLangButtons() {
    document.querySelectorAll('.lang-btn[data-lang]').forEach((btn) => {
      btn.addEventListener('click', () => {
        setLocale(btn.getAttribute('data-lang'));
      });
    });
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
      apply();
      bindLangButtons();
    });
  } else {
    apply();
    bindLangButtons();
  }
})();
