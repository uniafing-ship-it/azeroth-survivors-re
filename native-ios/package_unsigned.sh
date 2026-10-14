#!/usr/bin/env bash
# Diagnostic unsigned iPhoneOS bundle, NO WoW assets and NO Apple signature.
set -euo pipefail
cd "$(dirname "$0")/.."
binary="${IOS_EXECUTABLE:-target/aarch64-apple-ios/release/warcraft-survivors}"
out="${IOS_OUTPUT_DIR:-native-ios/artifacts}"
app="$out/Payload/AzerothSurvivors.app"
if [[ ! -s "$binary" ]]; then
  echo "Missing or empty native iOS binary: $binary" >&2
  exit 1
fi
mkdir -p "$app" "$out/reports"
cp native-ios/Info.plist "$app/Info.plist"
cp "$binary" "$app/warcraft-survivors"
chmod +x "$app/warcraft-survivors"
plutil -lint "$app/Info.plist"
file "$app/warcraft-survivors" | tee "$out/reports/binary-type.txt"
xcrun lipo -archs "$app/warcraft-survivors" | tee "$out/reports/architectures.txt"
xcrun otool -L "$app/warcraft-survivors" | tee "$out/reports/dynamic-libraries.txt"
if ! xcrun lipo -archs "$app/warcraft-survivors" | grep -qw arm64; then
  echo "Not an ARM64 binary" >&2
  exit 1
fi
echo "Unsigned iPhoneOS application; Apple provisioning and user-authorized signing are required" > "$out/README_UNSIGNED.txt"
( cd "$out" && zip -qr "azeroth-survivors-UNSIGNED-NOT-INSTALLABLE.ipa" Payload )
shasum -a 256 "$out/azeroth-survivors-UNSIGNED-NOT-INSTALLABLE.ipa" | tee "$out/reports/sha256.txt"
du -h "$binary" "$out/azeroth-survivors-UNSIGNED-NOT-INSTALLABLE.ipa" | tee "$out/reports/sizes.txt"
echo "Created UNSIGNED diagnostic bundle only (no playable release)"
