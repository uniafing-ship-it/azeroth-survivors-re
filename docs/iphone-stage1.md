# iPhone / Safari — Stage 1 experiment

This fork adds an **experimental**, browser-only touch input layer to the
open-source, fan-made recreation. It is not an iOS application and does not
modify, convert, or run the original Windows AzerothSurvivors.exe.

## Changed files

- web/mobile-controls.js — virtual joystick and jump, mobility, pause, start, card buttons
- crates/benilla-app/src/survivors/touch_input.rs — wasm32-only polling of touch state
- survivors/mod.rs — polls touch input in PreUpdate after Bevy InputSystems
- web/index.html — loads mobile control overlay
- scripts/web-build.sh — copies the overlay into web/dist
- .github/workflows/ios-wasm.yml — JS tests, wasm compilation and downloadable artifact
- web/tests/mobile-controls.test.mjs — input regression tests

## Build

In **Actions** select **Azeroth iPhone WASM proof** and choose **Run workflow**
on the iphone-wasm-stage1 branch, or push new commits on that branch. After
success, download artifact azeroth-ios-wasm-client.

IMPORTANT: The artifact contains the *client only*. It is not a playable
standalone web site. The open-source project needs a WoW 1.12.1 build 5875
Data folder served by wenilla-host via /data/* (including terrain.MPQ and
patch.MPQ). Two MPQs provided with the Windows game do not supply the full
WoW client Data set. Do not publish Blizzard game data in a public GitHub
repository or on a public static host without distribution rights.

## Safari checks still required

- Safari 26+ WebGPU and WebAssembly startup on the actual device
- landscape controls and UI scale; joystick movements; click interaction
- FPS, memory pressure, load and crash logs on the user's iPhone
- safe private hosting of content and compliance with rights to assets
- whether a hosting solution can run independently of a home PC
- intermittent/limited connectivity and repeat loading of game resources

No mobile gameplay, WASM compilation or private hosting has been
successfully verified until Actions and iPhone tests are completed.
