# Installing the experimental iPhone build without buying a Mac

**STATUS: NOT YET TESTED ON AN IPHONE.** This describes how the owner could
install a *future* finished native build. Building an unsigned `.ipa` is
not the same thing as shipping a working iPhone game.

## What is ready

- ARM64 iPhoneOS Rust `cargo check` passed
- ARM64 native Rust binary **linked** on GitHub-hosted macOS
- An *unsigned, asset-free diagnostic* IPA was created on the probe branch
- A second build with native Bevy touchscreen controls is being validated

## Option A: Windows and free Apple ID

1. Obtain the unsigned IPA of a **validated** native build from the private or
   public GitHub Actions artifact, then extract the artifact ZIP to find the
   actual `.ipa`. Do not mistake the artifact ZIP for an IPA.
2. Install Sideloadly from **https://sideloadly.io/** on your own Windows PC.
   Follow its official instructions for Apple-provided iTunes and iCloud
   desktop components.
3. Connect the iPhone once via USB, unlock, and approve **Trust This Computer**.
4. In Sideloadly, select your device and the IPA, then use **your own**
   Apple Account locally to sign and sideload. Never put an Apple Account
   password, app-specific password, private key or provisioning profile
   in GitHub commits, Issues, Actions logs, ChatGPT chat or shared files.
5. Follow the iPhone's prompts to trust the developer and activate
   **Settings → Privacy & Security → Developer Mode**, if required.
6. Start the app. Only after confirming it launches should you import your
   legitimately held game resources.

Free Apple-account signing normally expires after **seven days**.
The Windows sign/refresh process may need a PC again; Sideloadly offers
automatic refresh, but its availability depends on pairing, connectivity,
software versions and Apple services. The program's support for the
user's specific iOS release must be checked on the device.

## Option B: Refresh after initial setup without keeping a PC running

SideStore (**https://docs.sidestore.io/**) describes a way to set up once
using a computer and subsequently refresh sideloaded apps directly on the
device via its LocalDevVPN and Apple Account. It is more complicated than
Sideloadly and is an alternative to investigate if continual computer
independence matters. Its current compatibility with the target iPhone and
Apple Account must be confirmed.

## Importing offline game assets

Only use resources you are legally entitled to access. The public GitHub
repository and IPA contain no WoW copyrighted data.

The iOS app currently searches its local sandbox for:
- `On My iPhone → Azeroth Survivors → Data`
- `On My iPhone → Azeroth Survivors → WoW → Data`

These require the app's native Files integration to work on the real phone.
The open-source recreation generally expects a proper WoW 1.12.1 build 5875
`Data` folder, not the two MPQ patches from the unrelated Windows app.
Data transfer can consume multiple gigabytes and must be tested only
with a full, valid user-owned resource tree.

## Known limits

- Windows `.exe` does not run on iOS.
- Successfully linked or signed packages may still fail during runtime
  initialization, asset discovery, graphics, sound or multitouch gameplay.
- Installation using a free Apple Account does not give a permanent, freely
  redistributable iOS app. Apple enforces provisioning and signing rules.
- No game files or Apple credentials should be uploaded to public GitHub.
