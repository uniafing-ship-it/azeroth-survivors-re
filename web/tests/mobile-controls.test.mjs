import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import { fileURLToPath } from 'node:url';

const source = readFileSync(new URL('../mobile-controls.js', import.meta.url), 'utf8');

function node(dataset = {}) {
  const handlers = {};
  const classes = new Set();
  return {
    dataset, style: {},
    classList: {
      add(x) { classes.add(x); },
      remove(x) { classes.delete(x); },
      toggle(x, yes) { if (yes) classes.add(x); else classes.delete(x); }
    },
    addEventListener(name, callback) { handlers[name] = callback; },
    emit(name, properties = {}) {
      assert.ok(handlers[name], name);
      handlers[name]({ pointerId: 1, clientX: 66, clientY: 66,
        preventDefault() {}, ...properties });
    },
    setPointerCapture() {},
    getBoundingClientRect() { return { left: 0, top: 0, width: 132, height: 132 }; }
  };
}

function run() {
  const joystick = node(), stick = node();
  const buttons = ['Space', 'KeyF', 'Enter', 'Escape', 'Digit1', 'Digit2', 'Digit3']
    .map(code => node({ code }));
  const root = {
    querySelector(s) { return s === '.joystick' ? joystick : s === '.stick' ? stick : null; },
    querySelectorAll() { return buttons; },
    innerHTML: ''
  };
  const windowEvents = {};
  const window = {
    ontouchstart: null,
    addEventListener(type, cb) { windowEvents[type] = cb; },
    matchMedia() { return { matches: true }; }
  };
  const document = {
    hidden: false,
    body: { appendChild() {} },
    createElement() { return root; },
    addEventListener() {}
  };
  vm.runInNewContext(source, { window, document, navigator: { maxTouchPoints: 2 } });
  return { window, joystick, buttons, windowEvents };
}

test('touch joystick holds diagonals and releases all directions', () => {
  const { window, joystick } = run();
  joystick.emit('pointerdown', {clientX:110, clientY:20});
  assert.equal(window.__azeroth_touch_keys.KeyW, true);
  assert.equal(window.__azeroth_touch_keys.KeyD, true);
  joystick.emit('pointerup');
  for (const key of ['KeyW', 'KeyA', 'KeyS', 'KeyD'])
    assert.equal(window.__azeroth_touch_keys[key], false);
});

test('ability button presses and releases', () => {
  const { window, buttons } = run();
  const ability = buttons.find(x => x.dataset.code === 'KeyF');
  ability.emit('pointerdown');
  assert.equal(window.__azeroth_touch_keys.KeyF, true);
  ability.emit('pointerup');
  assert.equal(window.__azeroth_touch_keys.KeyF, false);
});

test('blur clears all held inputs', () => {
  const { window, joystick, buttons, windowEvents } = run();
  joystick.emit('pointerdown', {clientX:20, clientY:66});
  buttons[0].emit('pointerdown');
  windowEvents.blur();
  assert.equal(window.__azeroth_touch_keys.KeyA, false);
  assert.equal(window.__azeroth_touch_keys.Space, false);
});
