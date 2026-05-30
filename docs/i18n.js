/** French-first i18n for static GitHub Pages. Flags: FR / GB. */
(function () {
  const STORAGE_KEY = 'qt-lang';
  const DEFAULT = 'fr';

  const T = {
    fr: {
      'meta.title': 'Quantum Tetris',
      'meta.playTitle': 'Quantum Tetris — Jouer',
      'home.title': 'Quantum Tetris',
      'home.subtitle': 'Tetris où le hasard vient de mesures quantiques',
      'home.lead':
        'Chaque tirage aléatoire du jeu — quelle pièce tombe, à quelle vitesse, quel bonus — est le résultat d\'une <strong>mesure quantique</strong> (règle de Born), exécutée par Qiskit sur desktop ou par un simulateur calibré dans le navigateur.',
      'home.principle':
        '<strong>Classique :</strong> tes flèches ← → ↑ ↓ et la barre d\'espace (déplacement volontaire).<br>' +
        '<strong>Quantique :</strong> tout ce que le jeu «&nbsp;tire au sort&nbsp;» sans ton action directe.',
      'home.quantumTitle': 'Quand le jeu mesure un circuit',
      'home.moment.spawn.title': 'Nouvelle pièce',
      'home.moment.spawn.text':
        'Quatre circuits s\'enchaînent :<br>' +
        '· <span class="circuit">quantum-teleportation-gate-v1</span> ×2 — pièce en jeu et aperçu «&nbsp;suiv.&nbsp;» (famille I/O/T… puis variante)<br>' +
        '· <span class="circuit">imp-brain-v1</span> — rotation initiale et colonne d\'apparition<br>' +
        '· <span class="circuit">enemy-profile-hunter-v1</span> — intervalle de chute',
      'home.moment.observe.title': 'Barre d\'espace',
      'home.moment.observe.text':
        'Le circuit <span class="circuit">observation-pulse-v1</span> modélise une mesure volontaire : la pièce est posée tout de suite et tu peux gagner un bonus de score (parfois une ligne bonus).',
      'home.moment.line.title': 'Ligne complétée',
      'home.moment.line.text':
        'Le circuit <span class="circuit">q-shard-stabilizer-v1</span> tire un multiplicateur de points (×1 à ×4) après la stabilisation de la grille.',
      'home.classicalNote':
        'Seuls les déplacements au clavier restent entièrement classiques. Le HUD affiche le nom du circuit en cours et les bits mesurés.',
      'hint.move': 'BOUGER',
      'hint.rotate': 'TOURNER',
      'hint.faster': 'VITE',
      'hint.drop': 'POSER',
      'home.controlsTitle': 'Contrôles (classiques)',
      'home.localClassic': 'Classique local :',
      'home.localQiskit': 'Qiskit Aer local :',
      'home.circuitsLink': 'Circuits quantiques',
      'home.github': 'GitHub',
      'home.readme': 'Architecture',
      'home.fullscreen': 'Plein écran',
      'play.home': '← Accueil',
      'play.starting': 'Chargement du moteur quantique…',
    },
    en: {
      'meta.title': 'Quantum Tetris',
      'meta.playTitle': 'Quantum Tetris — Play',
      'home.title': 'Quantum Tetris',
      'home.subtitle': 'Tetris where randomness comes from quantum measurements',
      'home.lead':
        'Every random draw in the game — which piece falls, how fast, what bonus — is the outcome of a <strong>quantum measurement</strong> (Born rule), run through Qiskit on desktop or a matched simulator in the browser.',
      'home.principle':
        '<strong>Classical:</strong> your arrow keys ← → ↑ ↓ and Space (voluntary moves).<br>' +
        '<strong>Quantum:</strong> everything the game “rolls” without your direct input.',
      'home.quantumTitle': 'When the game runs a circuit',
      'home.moment.spawn.title': 'New piece',
      'home.moment.spawn.text':
        'Four circuits run in sequence:<br>' +
        '· <span class="circuit">quantum-teleportation-gate-v1</span> ×2 — active piece and “next” preview (I/O/T… family, then variant)<br>' +
        '· <span class="circuit">imp-brain-v1</span> — starting rotation and spawn column<br>' +
        '· <span class="circuit">enemy-profile-hunter-v1</span> — drop interval',
      'home.moment.observe.title': 'Space bar',
      'home.moment.observe.text':
        'The <span class="circuit">observation-pulse-v1</span> circuit models a deliberate measurement: the piece locks instantly and you may earn a score bonus (sometimes an extra line).',
      'home.moment.line.title': 'Line clear',
      'home.moment.line.text':
        'The <span class="circuit">q-shard-stabilizer-v1</span> circuit draws a score multiplier (×1 to ×4) after the board stabilizes.',
      'home.classicalNote':
        'Only keyboard moves stay fully classical. The in-game HUD shows the active circuit name and measured bits.',
      'hint.move': 'MOVE',
      'hint.rotate': 'ROTATE',
      'hint.faster': 'FASTER',
      'hint.drop': 'DROP',
      'home.controlsTitle': 'Controls (classical)',
      'home.localClassic': 'Local classic:',
      'home.localQiskit': 'Local Qiskit Aer:',
      'home.circuitsLink': 'Quantum circuits',
      'home.github': 'GitHub',
      'home.readme': 'Architecture',
      'home.fullscreen': 'Fullscreen',
      'play.home': '← Home',
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
    const page = document.body?.dataset?.page;
    if (page === 'play') {
      document.title = dict['meta.playTitle'] || document.title;
    } else {
      document.title = dict['meta.title'] || document.title;
    }
    document.querySelectorAll('.lang-btn[data-lang]').forEach((btn) => {
      const active = btn.getAttribute('data-lang') === lang;
      btn.classList.toggle('active', active);
      btn.setAttribute('aria-pressed', active ? 'true' : 'false');
    });
  }

  window.qtI18n = { getLocale, setLocale, apply };

  document.addEventListener('DOMContentLoaded', () => {
    apply();
    document.querySelectorAll('.lang-btn[data-lang]').forEach((btn) => {
      btn.addEventListener('click', () => {
        setLocale(btn.getAttribute('data-lang'));
      });
    });
  });
})();
