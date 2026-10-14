// iPhone / touch input for the WebAssembly Azeroth Survivors build.
// The wasm-side touch_input.rs polls window.__azeroth_touch_keys.
// This deliberately does not synthesize keyboard events.
(() => {
  if (window.__azeroth_mobile_loaded) return;
  window.__azeroth_mobile_loaded = true;
  if (!(('ontouchstart' in window) || (navigator.maxTouchPoints || 0) > 0 ||
        (window.matchMedia?.('(pointer: coarse)').matches ?? false))) return;

  const held = Object.create(null);
  window.__azeroth_touch_keys = held;
  const root = document.createElement('div');
  root.id = 'azeroth-touch';
  root.innerHTML = [
    '<style>',
    '#azeroth-touch { position:fixed; inset:0; z-index:35; pointer-events:none;',
    '-webkit-user-select:none;user-select:none;font:600 13px/1 system-ui,sans-serif; }',
    '#azeroth-touch * { box-sizing:border-box; }',
    '#azeroth-touch .joystick { position:absolute;left:max(10px,env(safe-area-inset-left));',
    'bottom:calc(16px + env(safe-area-inset-bottom));width:132px;height:132px;',
    'border:2px solid rgba(235,197,112,.55);border-radius:50%;',
    'background:rgba(15,12,14,.46);pointer-events:auto;touch-action:none; }',
    '#azeroth-touch .stick { position:absolute;left:40px;top:40px;width:48px;height:48px;',
    'border-radius:50%;background:rgba(244,200,111,.65);border:2px solid #e9bd69; }',
    '#azeroth-touch .actions { position:absolute;right:max(10px,env(safe-area-inset-right));',
    'bottom:calc(18px + env(safe-area-inset-bottom));',
    'display:grid;grid-template-columns:repeat(2,58px);gap:10px;pointer-events:none; }',
    '#azeroth-touch button { pointer-events:auto;touch-action:none;border:1.5px solid #cda769;',
    'border-radius:16px;background:rgba(32,25,23,.78);color:#ffe5ad;min-height:53px;',
    'font:600 14px/1 system-ui,sans-serif; }',
    '#azeroth-touch button.pressed {background:rgba(194,131,32,.83);color:#1e1006;}',
    '#azeroth-touch .top {position:absolute;right:max(10px,env(safe-area-inset-right));',
    'top:calc(10px + env(safe-area-inset-top));display:flex;gap:8px;}',
    '#azeroth-touch .top button {min-height:42px;padding:0 12px;}',
    '#azeroth-touch .cards {position:absolute;bottom:calc(8px + env(safe-area-inset-bottom));',
    'left:50%;transform:translateX(-50%);display:flex;gap:8px;}',
    '#azeroth-touch .cards button {min-height:39px;min-width:37px;border-radius:10px;}',
    '@media(max-width:540px) and (orientation:portrait) {',
    '#azeroth-touch .joystick {width:112px;height:112px;bottom:70px;}',
    '#azeroth-touch .stick {left:32px;top:32px;width:44px;height:44px;}',
    '#azeroth-touch .actions {bottom:70px;grid-template-columns:repeat(2,52px);gap:6px;}',
    '}',
    '</style>',
    '<div class="joystick" role="group" aria-label="Джойстик">',
    '<div class="stick"></div></div>',
    '<div class="actions"><button data-code="Space">Прыжок</button>',
    '<button data-code="KeyF">Рывок</button></div>',
    '<div class="top"><button data-code="Enter">Старт</button>',
    '<button data-code="Escape">Пауза</button></div>',
    '<div class="cards"><button data-code="Digit1">1</button>',
    '<button data-code="Digit2">2</button><button data-code="Digit3">3</button></div>'
  ].join('');
  document.body.appendChild(root);

  const joystick = root.querySelector('.joystick');
  const thumb = root.querySelector('.stick');
  let joyPointer = null;
  const dirs = ['KeyW', 'KeyA', 'KeyS', 'KeyD'];
  const center = () => {
    for (const code of dirs) held[code] = false;
    thumb.style.transform = '';
  };
  const drive = ev => {
    const rect = joystick.getBoundingClientRect();
    const dx = ev.clientX - rect.left - rect.width / 2;
    const dy = ev.clientY - rect.top - rect.height / 2;
    held.KeyW = dy < -15; held.KeyS = dy > 15;
    held.KeyA = dx < -15; held.KeyD = dx > 15;
    const distance = Math.hypot(dx, dy);
    const k = distance > 39 ? 39 / distance : 1;
    thumb.style.transform = 'translate(' + (dx * k) + 'px,' + (dy * k) + 'px)';
  };
  joystick.addEventListener('pointerdown', ev => {
    if (joyPointer !== null) return;
    ev.preventDefault();
    joyPointer = ev.pointerId;
    joystick.setPointerCapture?.(ev.pointerId);
    drive(ev);
  });
  joystick.addEventListener('pointermove', ev => {
    if (ev.pointerId === joyPointer) { ev.preventDefault(); drive(ev); }
  });
  const stopJoy = ev => {
    if (ev.pointerId !== joyPointer) return;
    ev.preventDefault(); joyPointer = null; center();
  };
  for (const name of ['pointerup', 'pointercancel', 'lostpointercapture'])
    joystick.addEventListener(name, stopJoy);
  for (const button of root.querySelectorAll('button[data-code]')) {
    const code = button.dataset.code;
    const pressed = new Set();
    button.addEventListener('pointerdown', ev => {
      ev.preventDefault(); pressed.add(ev.pointerId);
      held[code] = true;
      button.classList.add('pressed');
      button.setPointerCapture?.(ev.pointerId);
    });
    const release = ev => {
      if (!pressed.has(ev.pointerId)) return;
      ev.preventDefault(); pressed.delete(ev.pointerId);
      held[code] = pressed.size > 0;
      button.classList.toggle('pressed', held[code]);
    };
    for (const name of ['pointerup', 'pointercancel', 'lostpointercapture'])
      button.addEventListener(name, release);
  }
  const reset = () => {
    joyPointer = null; center();
    for (const code of Object.keys(held)) held[code] = false;
    for (const button of root.querySelectorAll('button[data-code]'))
      button.classList.remove('pressed');
  };
  window.addEventListener('blur', reset);
  window.addEventListener('pagehide', reset);
  document.addEventListener('visibilitychange', () => { if (document.hidden) reset(); });
})();
