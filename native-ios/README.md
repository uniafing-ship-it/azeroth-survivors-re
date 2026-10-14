# Azeroth Survivors native iOS experiment

This project attempts to cross-compile the **open-source Warcraft Survivors
recreation**, not to convert the original Windows `AzerothSurvivors.exe`.

## Verified: source compatibility

The [ARM64 iPhoneOS cargo check](https://github.com/uniafing-ship-it/azeroth-survivors-re/actions/runs/38553709910)
succeeded on a GitHub-hosted macOS runner, after excluding
Linux Wayland clipboard code and removing the unsupported Lua `os.execute`
system-shell call on iOS.

## Current stage: native linker and unsigned app bundle

A separate workflow `Native iOS link and package` attempts:

```bash
cargo build --locked --release --target aarch64-apple-ios \
  --package warcraft-survivors --no-default-features
bash native-ios/package_unsigned.sh
```

The script validates the Mach-O architecture and packages an unsigned .app and
a **clearly marked unsigned** IPA-shaped ZIP. It carries **NO** game MPQs and
does not upload credentials or Apple developer signing keys.

This does not yet make an installable or playable app. Even after a successful
link, iOS requires proper Apple signing/provisioning. Do **not** attempt to
install the unsigned IPA as though it were a functioning game.

## Remaining work

1. Repair any cross-linker problems in native dependencies (if discovered).
2. Implement native Bevy touch input and render/UI scaling.
3. Load **lawfully possessed** game resources from the app sandbox or
   explicit user-controlled import. The present app expects WoW 1.12.1
   resources; a couple of Windows game MPQs are not sufficient.
4. Test renderer, audio, memory, persistent storage, and resource loading.
5. Create a complete signed Xcode iOS app/IPA with the user's provisioning.
6. Test installation and actual play on their iPhone.

GitHub-hosted macOS runners remove the need to *own* a Mac for compilation
but cannot replace Apple signing credentials or real-device testing.
