/** French-first i18n for static GitHub Pages. Toggle with (en) / (fr). */
(function () {
  const STORAGE_KEY = 'qt-lang';
  const DEFAULT = 'fr';

  const T = {
    fr: {
      'meta.title': 'Quantum Tetris',
      'meta.playTitle': 'Quantum Tetris — Jouer',
      'lang.toggle': '(en)',
      'home.title': 'Quantum Tetris',
      'home.lead':
        'Tetris néon où <strong>chaque pièce</strong> est tirée par un circuit quantique. Le jeu navigateur démarre en <strong>mode quantique</strong> (simulateur Born, calibré Qiskit). Passe en classique avec le bouton in-game.',
      'hint.move': 'BOUGER',
      'hint.rotate': 'TOURNER',
      'hint.faster': 'VITE',
      'hint.drop': 'POSER',
      'home.play': 'Jouer dans le navigateur',
      'home.localClassic': 'Classique local :',
      'home.localQiskit': 'Qiskit Aer local :',
      'home.circuitsLink': 'Circuits quantiques',
      'home.github': 'GitHub',
      'home.circuitsTitle': 'Circuits Qiskit par action',
      'home.circuit.spawn':
        '<strong>Nouvelle pièce</strong> — <code>quantum-teleportation-gate-v1</code> (3 qubits) : une paire de Bell choisit la famille (I, O, T, J/L/S/Z), le qubit message fixe la variante. Aussi <code>imp-brain-v1</code> (rotation + colonne) et <code>enemy-profile-hunter-v1</code> (vitesse).',
      'home.circuit.observe':
        '<strong>Espace (chute forcée)</strong> — <code>observation-pulse-v1</code> (2 qubits) : mesure volontaire → bonus de score (parfois une ligne bonus).',
      'home.circuit.line':
        '<strong>Ligne effacée</strong> — <code>q-shard-stabilizer-v1</code> (2 qubits) : stabilisation → multiplicateur de points (×1 à ×4).',
      'play.home': '← Accueil',
      'play.loading': 'Chargement…',
      'play.starting': 'Démarrage du mode quantique…',
      'play.ready': 'Mode quantique · bascule CLASSIQUE / QUANTIQUE in-game',
      'play.error': 'Erreur WASM — lance ./scripts/build_wasm.sh',
    },
    en: {
      'meta.title': 'Quantum Tetris',
      'meta.playTitle': 'Quantum Tetris — Play',
      'lang.toggle': '(fr)',
      'home.title': 'Quantum Tetris',
      'home.lead':
        'Neon arcade Tetris where <strong>every piece</strong> is picked by a quantum circuit. The browser game runs in <strong>quantum mode</strong> by default (Born-rule simulator, Qiskit-matched). Switch to classic with the in-game button.',
      'hint.move': 'MOVE',
      'hint.rotate': 'ROTATE',
      'hint.faster': 'FASTER',
      'hint.drop': 'DROP',
      'home.play': 'Play in browser',
      'home.localClassic': 'Local classic:',
      'home.localQiskit': 'Local Qiskit Aer:',
      'home.circuitsLink': 'Quantum circuits',
      'home.github': 'GitHub',
      'home.circuitsTitle': 'Qiskit circuits per action',
      'home.circuit.spawn':
        '<strong>New piece</strong> — <code>quantum-teleportation-gate-v1</code> (3 qubits): a Bell pair picks the family (I, O, T, J/L/S/Z); the message qubit sets the variant. Also <code>imp-brain-v1</code> (rotation + column) and <code>enemy-profile-hunter-v1</code> (speed).',
      'home.circuit.observe':
        '<strong>Space (hard drop)</strong> — <code>observation-pulse-v1</code> (2 qubits): deliberate measure → score bonus (sometimes an extra line).',
      'home.circuit.line':
        '<strong>Line clear</strong> — <code>q-shard-stabilizer-v1</code> (2 qubits): stabilizer → score multiplier (×1 to ×4).',
      'play.home': '← Home',
      'play.loading': 'Loading…',
      'play.starting': 'Starting quantum mode…',
      'play.ready': 'Quantum mode · switch CLASSIC / QUANTUM in-game',
      'play.error': 'WASM error — run ./scripts/build_wasm.sh first',
    },
  };

  function getLocale() {
    const stored = localStorage.getItem(STORAGE_KEY);
    return stored === 'en' ? 'en' : DEFAULT;
  }

  function setLocale(lang) {
    localStorage.setItem(STORAGE_KEY, lang === 'en' ? 'en' : 'fr');
    apply();
  }

  function apply() {
    const lang = getLocale();
    document.documentElement.lang = lang;
    const dict = T[lang];
    document.querySelectorAll('[data-i18n]').forEach((el) => {
      const key = el.getAttribute('data-i18n');
      const val = dict[key];
      if (val == null) return;
      if (el.hasAttribute('data-i18n-html')) {
        el.innerHTML = val;
      } else {
        el.textContent = val;
      }
    });
    document.title = dict['meta.title'] || document.title;
    if (document.body.dataset.page === 'play') {
      document.title = dict['meta.playTitle'] || document.title;
    }
    const btn = document.getElementById('lang-toggle');
    if (btn) btn.textContent = dict['lang.toggle'];
  }

  window.qtI18n = { getLocale, setLocale, apply };

  document.addEventListener('DOMContentLoaded', () => {
    apply();
    const btn = document.getElementById('lang-toggle');
    if (btn) {
      btn.addEventListener('click', () => {
        setLocale(getLocale() === 'fr' ? 'en' : 'fr');
      });
    }
  });
})();
