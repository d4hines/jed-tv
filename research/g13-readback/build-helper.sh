#!/usr/bin/env bash
set -euo pipefail
g13_source=$(cd -- "$(dirname -- "$0")" && pwd)
g13_root=$(cd -- "$g13_source/../.." && pwd)
g13_build=${TV_READBACK_BUILD_DIR:-"$g13_root/assets/research/g13-readback/build"}
g13_jdk=/nix/store/mp9lqgch671kqq9qad0gsmw5r373izcn-openjdk-headless-21.0.12.1+1
mkdir -p "$g13_build"
cd "$g13_build"
if [ ! -f android-4.1.1.4.jar ]; then
  curl --fail --location --max-time 60 --output android-4.1.1.4.jar \
    https://repo.maven.apache.org/maven2/com/google/android/android/4.1.1.4/android-4.1.1.4.jar
fi
echo '84072541cbb711eff89f7277100ff854929a446dba7ceb1b195c340e0b4fd3cb  android-4.1.1.4.jar' | sha256sum -c -
"$g13_jdk/bin/javac" --release 8 -Xlint:-options -cp android-4.1.1.4.jar -d . "$g13_source/../../research/probes/g13-readback/ServerActivity.java"
"$g13_jdk/bin/java" -cp "$g13_root/assets/research/tools/r8-8.3.37.jar" com.android.tools.r8.D8 \
  --min-api 32 --lib android-4.1.1.4.jar --output classes.zip local/tvreadback/ServerActivity.class
nix shell nixpkgs#python3 -c python3 "$g13_source/package-apk.py" --build-dir "$PWD"
if [ ! -f readback.jks ]; then
  "$g13_jdk/bin/keytool" -genkeypair -keystore readback.jks \
    -storepass readback-local -keypass readback-local -alias readback \
    -keyalg RSA -keysize 2048 -validity 3650 -dname 'CN=Local TV Readback'
fi
chmod 0600 readback.jks
nix shell nixpkgs#apksigner -c apksigner sign --ks readback.jks \
  --ks-pass pass:readback-local --out readback.apk readback-unsigned.apk
nix shell nixpkgs#apksigner -c apksigner verify --verbose readback.apk
