#!/bin/sh
U="../../tmp/usbt/"
mkdir -p jitsi-meet-unity/Runtime/Plugins/x86_64/
cp ./target/release/jitsi_meet_signalling_c.dll jitsi-meet-unity/Runtime/Plugins/x86_64/ -i
rm -rf "${U%/}/Packages/jitsi-meet-unity"
cp -ri ./jitsi-meet-unity/ "${U%/}/Packages/"
