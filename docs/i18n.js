/** French-first i18n — syncs with in-game WASM locale via set_web_locale. */
(function () {
  const STORAGE_KEY = 'qt-lang';
  const DEFAULT = 'fr';

  const T = {
    fr: {
      'meta.title': 'Quantum Tetris',
      'lang.group': 'Langue',
      'lang.fr': 'Français',
      'lang.en': 'English',
      'home.title': 'Quantum Tetris',
      'home.subtitle': 'Tetris — le hasard vient des circuits, pas d\'un dé',
      'home.lead':
        'En surface, c\'est du Tetris. Sous le capot, <strong>chaque tirage</strong> (pièce, vitesse, bonus) passe par un circuit Qiskit — ou son équivalent dans le navigateur. Toi, tu bouges ; le reste, c\'est la mesure.',
      'home.principle':
        '<strong>Toi :</strong> ← → ↑ ↓ et Espace.<br>' +
        '<strong>Le jeu :</strong> tout le reste.',
      'home.quantumTitle': 'Mécaniques',
      'home.moment.spawn.title': 'Nouvelle pièce',
      'home.moment.spawn.text':
        'Quatre tirs à l\'apparition : deux téléportations pour la pièce en jeu et le «&nbsp;suiv.&nbsp;», puis rotation/colonne et cadence de chute.',
      'home.moment.observe.title': 'Espace — chute forcée',
      'home.moment.observe.text':
        'Pose immédiate + tirage bonus. Parfois une ligne en cadeau.',
      'home.moment.line.title': 'Ligne complétée',
      'home.moment.line.text':
        'Multiplicateur de score tiré au sort — ×1 à ×4.',
      'home.classicalNote':
        'Les flèches restent 100&nbsp;% classiques. Le HUD in-game affiche le circuit actif et les bits mesurés.',
      'home.localClassic': 'Desktop classique :',
      'home.localQiskit': 'Desktop Qiskit Aer :',
      'home.circuitsLink': 'Référence circuits',
      'home.github': 'GitHub',
      'home.readme': 'Architecture',
      'play.starting': 'Chargement…',
      'error.wasm': 'Jeu indisponible — build WASM manquant (./scripts/build_wasm.sh)',
    },
    en: {
      'meta.title': 'Quantum Tetris',
      'lang.group': 'Language',
      'lang.fr': 'Français',
      'lang.en': 'English',
      'home.title': 'Quantum Tetris',
      'home.subtitle': 'Tetris — randomness from circuits, not a dice roll',
      'home.lead':
        'Looks like Tetris. Under the hood, <strong>every roll</strong> (piece, speed, bonus) runs through a Qiskit circuit — or its browser twin. You move; the rest is measurement.',
      'home.principle':
        '<strong>You:</strong> ← → ↑ ↓ and Space.<br>' +
        '<strong>The game:</strong> everything else.',
      'home.quantumTitle': 'Mechanics',
      'home.moment.spawn.title': 'New piece',
      'home.moment.spawn.text':
        'Four draws on spawn: two teleportations for the active piece and “next”, then rotation/column and drop cadence.',
      'home.moment.observe.title': 'Space — hard drop',
      'home.moment.observe.text':
        'Instant lock plus a bonus roll. Sometimes an extra line.',
      'home.moment.line.title': 'Line clear',
      'home.moment.line.text':
        'Random score multiplier — ×1 to ×4.',
      'home.classicalNote':
        'Arrow keys stay 100% classical. The in-game HUD shows the active circuit and measured bits.',
      'home.localClassic': 'Desktop classic:',
      'home.localQiskit': 'Desktop Qiskit Aer:',
      'home.circuitsLink': 'Circuit reference',
      'home.github': 'GitHub',
      'home.readme': 'Architecture',
      'play.starting': 'Loading…',
      'error.wasm': 'Game unavailable — WASM build missing (./scripts/build_wasm.sh)',
    },
  };

  function getLocale() {
    const stored = localStorage.getItem(STORAGE_KEY);
    return stored === 'en' ? 'en' : DEFAULT;
  }

  function t(key) {
    return T[getLocale()][key];
  }

  function notifyGame(lang) {
    if (typeof window.__qtSetWebLocale === 'function') {
      window.__qtSetWebLocale(lang);
    }
  }

  function setLocale(lang) {
    localStorage.setItem(STORAGE_KEY, lang === 'en' ? 'en' : 'fr');
    apply();
    notifyGame(lang);
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

  document.addEventListener('DOMContentLoaded', () => {
    apply();
    document.querySelectorAll('.lang-btn[data-lang]').forEach((btn) => {
      btn.addEventListener('click', () => {
        setLocale(btn.getAttribute('data-lang'));
      });
    });
  });
})();
