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
      'home.subtitle': 'Tetris — le hasard vient des circuits quantiques',
      'home.lead':
        'Règles Tetris classiques. Chaque tirage aléatoire (pièce, vitesse, bonus) provient d\'une mesure sur un circuit quantique — Qiskit Aer sur desktop ; simulateur Born en Rust dans le navigateur (mêmes portes, probabilités calibrées sur Qiskit, sans Python).',
      'home.principle':
        '<strong>Le joueur :</strong> ← → ↑ ↓ et Espace. <strong>Le jeu :</strong> tout le reste.',
      'home.quantumTitle': 'Mécaniques',
      'home.label.game': 'En jeu',
      'home.label.quantum': 'Circuit',
      'home.moment.piece.title': 'Pièce en cours & « suiv. »',
      'home.moment.piece.game': 'Détermine la forme active et l\'aperçu suivant.',
      'home.moment.piece.quantum':
        '<code>quantum-teleportation-gate-v1</code> ×2 — paire de Bell → famille (I, O, T…), qubit message → variante.',
      'home.moment.brain.title': 'Rotation & colonne',
      'home.moment.brain.game': 'Fixe l\'orientation à l\'apparition et la colonne d\'entrée.',
      'home.moment.brain.quantum':
        '<code>imp-brain-v1</code> — deux bits mesurés, mappés sur rotation et colonne.',
      'home.moment.hunter.title': 'Cadence de chute',
      'home.moment.hunter.game': 'Intervalle entre deux descentes ; diminue avec le niveau.',
      'home.moment.hunter.quantum':
        '<code>enemy-profile-hunter-v1</code> — vitesse tirée à chaque nouvelle pièce.',
      'home.moment.observe.title': 'Espace — chute forcée',
      'home.moment.observe.game': 'Pose immédiate ; bonus de score, parfois une ligne supplémentaire.',
      'home.moment.observe.quantum':
        '<code>observation-pulse-v1</code> — mesure volontaire, 2 bits → type de bonus.',
      'home.moment.line.title': 'Ligne complétée',
      'home.moment.line.game': 'Multiplicateur de points ×1 à ×4 selon le tirage.',
      'home.moment.line.quantum':
        '<code>q-shard-stabilizer-v1</code> — stabilisation après effacement, bits → multiplicateur.',
      'home.circuitsLink': 'Référence circuits',
      'home.github': 'GitHub',
      'home.readme': 'Build local',
      'play.starting': 'Chargement…',
      'play.downloading': 'Téléchargement du jeu…',
      'play.initializing': 'Démarrage du moteur…',
      'play.hint': 'Premier chargement (~70 Mo). Attendre la fin du téléchargement.',
      'error.wasm': 'Jeu indisponible — build WASM manquant (./scripts/build_wasm.sh)',
      'error.canvas': 'Le canvas n\'a pas démarré — recharger la page.',
      'error.run': 'Erreur au lancement du jeu — voir la console.',
    },
    en: {
      'meta.title': 'Quantum Tetris',
      'lang.group': 'Language',
      'lang.fr': 'Français',
      'lang.en': 'English',
      'home.title': 'Quantum Tetris',
      'home.subtitle': 'Tetris — randomness from quantum circuits',
      'home.lead':
        'Standard Tetris rules. Every random outcome (piece, speed, bonus) comes from a quantum circuit measurement — Qiskit Aer on desktop; Rust Born simulator in the browser (same gates, Qiskit-matched probabilities, no Python).',
      'home.principle':
        '<strong>The player:</strong> ← → ↑ ↓ and Space. <strong>The game:</strong> everything else.',
      'home.quantumTitle': 'Mechanics',
      'home.label.game': 'In-game',
      'home.label.quantum': 'Circuit',
      'home.moment.piece.title': 'Active piece & “next”',
      'home.moment.piece.game': 'Sets the active shape and the next preview.',
      'home.moment.piece.quantum':
        '<code>quantum-teleportation-gate-v1</code> ×2 — Bell pair → family (I, O, T…), message qubit → variant.',
      'home.moment.brain.title': 'Rotation & column',
      'home.moment.brain.game': 'Sets spawn orientation and entry column.',
      'home.moment.brain.quantum':
        '<code>imp-brain-v1</code> — two measured bits mapped to rotation and column.',
      'home.moment.hunter.title': 'Drop cadence',
      'home.moment.hunter.game': 'Interval between grid steps; decreases as level rises.',
      'home.moment.hunter.quantum':
        '<code>enemy-profile-hunter-v1</code> — drop interval drawn on each new piece.',
      'home.moment.observe.title': 'Space — hard drop',
      'home.moment.observe.game': 'Instant lock; score bonus, sometimes one extra line.',
      'home.moment.observe.quantum':
        '<code>observation-pulse-v1</code> — deliberate measure, 2 bits → bonus type.',
      'home.moment.line.title': 'Line clear',
      'home.moment.line.game': 'Score multiplier ×1 to ×4 from the draw.',
      'home.moment.line.quantum':
        '<code>q-shard-stabilizer-v1</code> — post-clear stabilizer, bits → multiplier.',
      'home.circuitsLink': 'Circuit reference',
      'home.github': 'GitHub',
      'home.readme': 'Local build',
      'play.starting': 'Loading…',
      'play.downloading': 'Downloading game…',
      'play.initializing': 'Starting engine…',
      'play.hint': 'Initial download (~70 MB). Wait for the progress bar to finish.',
      'error.wasm': 'Game unavailable — WASM build missing (./scripts/build_wasm.sh)',
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
