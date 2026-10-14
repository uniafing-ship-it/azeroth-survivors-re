# Native iOS port — experimental feasibility stage

This folder starts a **native** iPhone build of the open-source
`warcraft-survivors` Rust/Bevy implementation. It does NOT transform
`AzerothSurvivors.exe` into an iOS app.

## Stage 0 — compatibility

GitHub Actions job `Native iOS cargo probe` uses a macOS runner with the
iPhoneOS SDK and Rust target `aarch64-apple-ios`. It runs:

```sh
cargo check --locked --target aarch64-apple-ios \
  --package warcraft-survivors --no-default-features
```

This identifies iOS-incompatible Rust dependencies and source modules; a
successful `cargo check` does NOT imply the game links, runs, or renders on
an iPhone.

## Next milestones

1. Fix iOS-only compilation errors (particularly native audio, storage,
   `winit` platform and resource-path handling).
2. Cross-link `warcraft-survivors` as an ARM64 iOS executable on macOS.
3. Bundle in a real signed Xcode project. Apple codesigning requires
   a certificate and provisioning profile; this workflow has neither.
4. Package only game assets the user has rights to use into the app's sandbox,
   supporting *offline* mode; do not commit or publish copyrighted WoW MPQs.
5. Add native Bevy `TouchInput` joystick, buttons, screen scaling and UI.
6. Verify rendering, performance, asset loading, audio and device rotation on
   the actual iPhone.

`Info.plist` is a **draft** for a future `.app` bundle and is not proof
of successful packaging or installation. The workflow currently does not
create an `.ipa`.

## Restrictions

- No Apple Developer subscription or Apple ID is configured.
- No private game archives are uploaded by this workflow.
- GitHub hosted macOS runners can test compilation without any local Mac.
- A device-installable app will require additional work and valid signing.

Rust/Bevy iOS approach: Rust `bin` compiled for Apple iOS, packaged as the
application executable, following the Bevy iOS mobile example.
