#!/bin/sh
# Build the native-widget camera for the Mate: cargo (aarch64 OHOS) → stage the
# .so → sign with the DevEco auto-signing profile of ~/DevEcoStudioProjects/MyApplication
# (a DEBUG profile bound to com.example.myapplication) → hvigor → install → launch.
#   build.sh [--no-launch] [--build-only]
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
DEVECO=/Applications/DevEco-Studio.app/Contents
export JAVA_HOME=$DEVECO/jbr/Contents/Home
export PATH=$JAVA_HOME/bin:$DEVECO/tools/node/bin:$DEVECO/sdk/default/openharmony/toolchains:$PATH
export DEVECO_SDK_HOME=$DEVECO/sdk NODE_HOME=$DEVECO/tools/node
D=${DEVICE:-5ZGYD25B13020968}
cd "$HERE"
echo "==> cargo"
cargo build --release --target aarch64-unknown-linux-ohos 2>&1 | grep -E "^error|Finished" -A 6 | head -40
cp target/aarch64-unknown-linux-ohos/release/libcamera_oh.so deveco/entry/libs/arm64-v8a/
cp "$DEVECO/sdk/default/openharmony/native/llvm/lib/aarch64-linux-ohos/libc++_shared.so" deveco/entry/libs/arm64-v8a/
cd deveco
python3 - <<'PY'
import re
src = open('/Users/ychen/DevEcoStudioProjects/MyApplication/build-profile.json5').read()
block = re.search(r'"signingConfigs":\s*\[(.*?)\n\s*\],', src, re.S).group(0)
bp = 'build-profile.json5'; s = open(bp).read()
if '"signingConfigs": [],' in s: s = s.replace('"signingConfigs": [],', block.rstrip(',') + ',')
s = re.sub(r'"compatibleSdkVersion":\s*"[^"]+"', '"compatibleSdkVersion": "6.0.1(21)"', s)
open(bp, 'w').write(s)
a = 'AppScope/app.json5'; t = open(a).read()
t = re.sub(r'"bundleName": "[^"]+"', '"bundleName": "com.example.myapplication"', t); open(a, 'w').write(t)
PY
echo "==> hvigor"
node $DEVECO/tools/hvigor/bin/hvigorw.js assembleHap --mode module -p product=default -p buildMode=release --no-daemon 2>&1 | sed 's/\x1b\[[0-9;]*m//g' | grep -E "Error Message|ERROR|BUILD" | sort -u
[ "${1:-}" = "--build-only" ] && exit 0
# The module is named like the Makepad host's ("makepad"): both HAPs share the bundle, and only a HAP
# with the same module name installs in place, keeping the permission grants across a host swap.
H=entry/build/default/outputs/default/makepad-default-signed.hap
ls -la "$H" | awk '{print "hap", $5, "bytes"}'
hdc -t $D shell "aa force-stop com.example.myapplication" >/dev/null 2>&1 || true
hdc -t $D file send "$H" /data/local/tmp/camera.hap | tail -1
hdc -t $D shell bm install -p /data/local/tmp/camera.hap 2>&1 | grep -q success || { hdc -t $D shell "bm uninstall -n com.example.myapplication" >/dev/null; hdc -t $D shell bm install -p /data/local/tmp/camera.hap 2>&1 | tail -1; }
[ "${1:-}" = "--no-launch" ] && exit 0
hdc -t $D shell "power-shell wakeup; hilog -r; aa start -a EntryAbility -b com.example.myapplication" | tail -1
sleep ${SETTLE:-6}
hdc -t $D shell "hilog -x" | LC_ALL=C sed 's/\x1b\[[0-9;]*m//g' | LC_ALL=C grep -a "camera-oh\|xcomp:" | sed 's/.*camera-oh: //' | cut -c 1-180 | tail -20
