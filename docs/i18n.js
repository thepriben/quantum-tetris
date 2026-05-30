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
      'home.circuit.spawn.title': 'Nouvelle pièce',
      'home.circuit.spawn.text':
        'Trois circuits s\'enchaînent à chaque apparition : la <strong>téléportation</strong> choisit la forme (paire de Bell + qubit message), <strong>imp-brain</strong> fixe rotation et colonne, <strong>hunter profile</strong> règle la vitesse de chute.',
      'home.circuit.observe.title': 'Espace — chute forcée',
      'home.circuit.observe.text':
        'Le circuit <code>observation-pulse-v1</code> simule une mesure volontaire : bonus de score, parfois une ligne bonus.',
      'home.circuit.line.title': 'Ligne effacée',
      'home.circuit.line.text':
        'Le circuit <code>q-shard-stabilizer-v1</code> stabilise la grille après effacement et multiplie les points (×1 à ×4).',
      'circuit.teleport.alt': 'Diagramme Qiskit — quantum-teleportation-gate-v1 (3 qubits)',
      'circuit.imp.alt': 'Diagramme Qiskit — imp-brain-v1 (rotation et colonne)',
      'circuit.hunter.alt': 'Diagramme Qiskit — enemy-profile-hunter-v1 (vitesse)',
      'circuit.observe.alt': 'Diagramme Qiskit — observation-pulse-v1 (mesure volontaire)',
      'circuit.line.alt': 'Diagramme Qiskit — q-shard-stabilizer-v1 (multiplicateur)',
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
      'home.circuit.spawn.title': 'New piece',
      'home.circuit.spawn.text':
        'Three circuits run on every spawn: <strong>teleportation</strong> picks the shape (Bell pair + message qubit), <strong>imp-brain</strong> sets rotation and column, <strong>hunter profile</strong> sets drop speed.',
      'home.circuit.observe.title': 'Space — hard drop',
      'home.circuit.observe.text':
        'The <code>observation-pulse-v1</code> circuit models a deliberate measure: score bonus, sometimes an extra line.',
      'home.circuit.line.title': 'Line clear',
      'home.circuit.line.text':
        'The <code>q-shard-stabilizer-v1</code> circuit stabilizes the board after a clear and multiplies points (×1 to ×4).',
      'circuit.teleport.alt': 'Qiskit diagram — quantum-teleportation-gate-v1 (3 qubits)',
      'circuit.imp.alt': 'Qiskit diagram — imp-brain-v1 (rotation and column)',
      'circuit.hunter.alt': 'Qiskit diagram — enemy-profile-hunter-v1 (speed)',
      'circuit.observe.alt': 'Qiskit diagram — observation-pulse-v1 (deliberate measure)',
      'circuit.line.alt': 'Qiskit diagram — q-shard-stabilizer-v1 (multiplier)',
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
