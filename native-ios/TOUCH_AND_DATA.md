# Native touch controls and offline-data strategy (experimental)

## Controls

The iOS-only Bevy module `crates/benilla-app/src/survivors/native_touch.rs`
reads `Touches` after Bevy's input update and simulates the same keys used by
the existing game. It is not JavaScript, Safari or a webview.

| Control | iOS screen area | Game input |
| --- | --- | --- |
| Move | bottom-left virtual joystick | W / A / S / D |
| Jump | bottom-right | Space |
| Mobility | bottom-right above jump | F |
| Start/confirm | top-right | Enter |
| Pause/back | top-right | Escape |
| Level-up options | bottom center | 1 / 2 / 3 |

Multiple fingers can hold direction and trigger an action simultaneously.
The joystick uses a dead zone and releases each synthetic key when its
owning touch ends. Bevy UI draws simple labeled regions. The control layout,
safe areas, portrait behavior, accessibility and real-device touch response
still need testing and refinement.

## Importing resources without a server

The native app's `Info.plist` enables `UIFileSharingEnabled` and
`LSSupportsOpeningDocumentsInPlace`, which Apple documents as the way to
expose a local app `Documents` folder to the iOS Files app.

**After a signed build is installed**, the owner can use Files
(On My iPhone > Azeroth Survivors) to copy personally held resources into:

- `Documents/Data/`, or
- `Documents/WoW/Data/`

The source resolver on iOS checks both paths, after an explicit non-empty
`WOW_DATA` override, before checking the installed bundle itself.

This is only a path-resolution mechanism, not a verified working asset
importer. An existing WoW 1.12.1 build 5875 Data tree may be required.
Neither the uploaded two Windows game MPQs nor a prebuilt WASM client
constitute the complete validated resources. The project does not include
or redistribute any copyrighted WoW assets.

## Still to do

- Check iOS touch compilation; test actual interaction and orientation.
- Link an ARM64 Mach-O and prepare a full Apple app bundle.
- Add provisioning and device installation under the owner's account.
- Test Assets/MPQ reads, sandbox HOME resolution, UI size, Metal rendering,
  audio and FPS on an actual iPhone.
