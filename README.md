# Avstack-based Jitsi WebRTC demo for Unity

## Original Repository

[github](https://github.com/avstack)

## Current Status

<u>**UNSTABLE**</u>

### How to see currently working part

First, ensure you have set up the rust development
environment or at least a build toolchain.

Clone this repository and check out
[fix-unimplemented-method](http://192.168.0.31:30000/abiria/mcp-unity-webrtc/tree/fix-unimplemented-method)
branch then run below.

```sh
$ cargo build -rp jitsi-meet-signalling-c -F tls-rustls-native-roots --no-default-features
```

Copy or move generated
`target/release/(lib)?jitsi_meet_signalling_c.(so|dll|dylib)`
file (according to the platform you are using) into
`jitsi-meet-unity/Runtime/Plugins/` directory.

Open package manager window inside Unity editor and select
`Add package from disk...` menu and install the
`jitsi-meet-unity` package.

Make symbolic link or just copy the
`jitsi-meet-unity/Samples~/BasicSendReceive` directory into Assets directory
of your current unity project.

Then run it.

(Optionally check the `Load on startup` checkbox for your
DLL file, which is required for release build)
