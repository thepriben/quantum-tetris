/** French-first i18n for static GitHub Pages. Toggle with (en) / (fr). */
(function () {
  const STORAGE_KEY = 'qt-lang';
  const DEFAULT = 'fr';

  const T = {
    fr: {
      'meta.title': 'Quantum Tetris — Guide quantum & gameplay',
      'meta.playTitle': 'Quantum Tetris — Jouer',
      'lang.toggle': '(en)',
      'home.badge': 'Guide quantum & gameplay',
      'home.title': 'Quantum Tetris',
      'home.subtitle': 'Explications · circuits Qiskit · mécaniques de jeu',
      'home.lead':
        'Pas de dés classique : <strong>chaque tirage du jeu</strong> est une mesure quantique exécutée par Qiskit (ou un simulateur Born calibré dans le navigateur). Seuls tes déplacements au clavier restent classiques — tout le reste vient d\'un circuit.',
      'home.quantumTitle': 'Ce qui est quantique dans le gameplay',
      'home.quantumList':
        '<ul class="quantum-list">' +
        '<li><strong>Pièce en cours & suivante</strong> — deux tirs du circuit de téléportation (<code>quantum-teleportation-gate-v1</code>) : paire de Bell → famille (I, O, T…), qubit message → variante</li>' +
        '<li><strong>Rotation & colonne d\'apparition</strong> — <code>imp-brain-v1</code></li>' +
        '<li><strong>Vitesse de chute</strong> — <code>enemy-profile-hunter-v1</code></li>' +
        '<li><strong>Chute forcée (Espace)</strong> — mesure volontaire <code>observation-pulse-v1</code></li>' +
        '<li><strong>Ligne effacée</strong> — stabilisateur <code>q-shard-stabilizer-v1</code> (multiplicateur de score)</li>' +
        '</ul>',
      'hint.move': 'BOUGER',
      'hint.rotate': 'TOURNER',
      'hint.faster': 'VITE',
      'hint.drop': 'POSER',
      'home.play': 'Jouer maintenant',
      'home.playNote': 'La page de jeu est épurée — retrouve ici tout le détail quantique.',
      'home.controlsTitle': 'Contrôles (classiques — joueur)',
      'home.localClassic': 'Classique local :',
      'home.localQiskit': 'Qiskit Aer local :',
      'home.circuitsLink': 'Circuits quantiques',
      'home.github': 'GitHub',
      'home.readme': 'Architecture (README)',
      'home.circuitsTitle': 'Circuits Qiskit — diagrammes & effets',
      'home.circuit.spawn.title': 'Apparition — pièce actuelle & suivante',
      'home.circuit.spawn.text':
        'À chaque spawn, <strong>quatre mesures</strong> : deux téléportations (pièce en jeu + preview «&nbsp;suiv.&nbsp;»), puis rotation/colonne et vitesse. C\'est le cœur du gameplay quantique.',
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
      'play.home': '← Guide quantum',
      'play.starting': 'Chargement du moteur quantique…',
    },
    en: {
      'meta.title': 'Quantum Tetris — Quantum & gameplay guide',
      'meta.playTitle': 'Quantum Tetris — Play',
      'lang.toggle': '(fr)',
      'home.badge': 'Quantum & gameplay guide',
      'home.title': 'Quantum Tetris',
      'home.subtitle': 'Explanations · Qiskit circuits · game mechanics',
      'home.lead':
        'No classical dice: <strong>every random draw in the game</strong> is a quantum measurement run through Qiskit (or a Born-rule simulator matched to Qiskit in the browser). Only your keyboard moves are classical — everything else comes from a circuit.',
      'home.quantumTitle': 'What is quantum in the gameplay',
      'home.quantumList':
        '<ul class="quantum-list">' +
        '<li><strong>Current & next piece</strong> — two teleportation shots (<code>quantum-teleportation-gate-v1</code>): Bell pair → family (I, O, T…), message qubit → variant</li>' +
        '<li><strong>Rotation & spawn column</strong> — <code>imp-brain-v1</code></li>' +
        '<li><strong>Drop speed</strong> — <code>enemy-profile-hunter-v1</code></li>' +
        '<li><strong>Hard drop (Space)</strong> — deliberate measure <code>observation-pulse-v1</code></li>' +
        '<li><strong>Line clear</strong> — stabilizer <code>q-shard-stabilizer-v1</code> (score multiplier)</li>' +
        '</ul>',
      'hint.move': 'MOVE',
      'hint.rotate': 'ROTATE',
      'hint.faster': 'FASTER',
      'hint.drop': 'DROP',
      'home.play': 'Play now',
      'home.playNote': 'The game page is minimal — all quantum details live here.',
      'home.controlsTitle': 'Controls (classical — player)',
      'home.localClassic': 'Local classic:',
      'home.localQiskit': 'Local Qiskit Aer:',
      'home.circuitsLink': 'Quantum circuits',
      'home.github': 'GitHub',
      'home.readme': 'Architecture (README)',
      'home.circuitsTitle': 'Qiskit circuits — diagrams & effects',
      'home.circuit.spawn.title': 'Spawn — current & next piece',
      'home.circuit.spawn.text':
        'On every spawn, <strong>four measurements</strong>: two teleportations (active piece + “next” preview), then rotation/column and speed. This is the core quantum gameplay loop.',
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
      'play.home': '← Quantum guide',
      'play.starting': 'Loading quantum engine…',
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
    const page = document.body.dataset.page;
    if (page === 'play') {
      document.title = dict['meta.playTitle'] || document.title;
    } else if (page === 'index') {
      document.title = dict['meta.title'] || document.title;
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
