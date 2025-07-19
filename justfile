#!/usr/bin/env -S just --justfile

### options

# whether or not to use release profile
# possible values: true | false
release := "false"

profile := if release == "true" {
	"release"
} else {
	"dev"
}

# path to signalling-c crate
signalling-c := "jitsi-meet-signalling-c"

# path to the unity project
unity := "./unity-mirror"

# path to the native plugin directory
# unity / "Assets/Plugins"
plugin-dir := "./jitsi-meet-unity/Runtime/Plugins"

# name of the remote unity project
dest := "zz"

# name of the crate containing cpdll script
copy-dll-crate := "copy-dll"

### platform specific options

## windows

# possible values: gnu | msvc
win := "gnu"

win-tt := if win == "gnu" {
	"x86_64-pc-windows-gnu"
} else if win == "msvc" {
	"x86_64-pc-windows-msvc"
} else {
	error("unknown windows build toolchain: \"" + win + "\"")
}

# ssh username and hostname of windows
win-ssh-host := "oro36@win11"

# path to unity project on windows
win-unity-path := dest

# full ssh remote path for windows
win-dest := win-ssh-host + ":" + win-unity-path

## osx

# possible values: m1 | x86
osx := "m1"

osx-tt := if osx == "m1" {
	"aarch64-apple-darwin"
} else if osx == "x86" {
	"x86_64-apple-darwin"
} else {
	error("unknown osx architecture: \"" + osx + "\"")
}

# ssh username and hostname of osx
osx-ssh-host := "swkim@osx1"

# path to unity project on osx
osx-unity-path := dest

# full ssh remote path for osx
osx-dest := osx-ssh-host + ":" + osx-unity-path

### environment variables

# rust backtrace
export RUST_BACKTRACE := "0"

### aliases

alias a := all
alias aw := all-windows
alias ao := all-osx
alias s := sync
alias sa := sync-all
alias sw := sync-windows
alias ds := delsync-all
alias dsw := delsync-windows
alias dso := delsync-osx
alias bc := bc-all
alias bcl := bc-linux
alias bcw := bc-windows
alias bco := bc-osx
alias b := build
alias ba := build-all
alias zb := zigbuild
alias wb := xwinbuild
alias bl := build-linux
alias bw := build-windows
alias bo := build-osx
alias cpdll := copy-dll
alias cd := copy-dll
alias cdf := copy-dll-for
alias cda := copy-dll-all
alias cdl := copy-dll-linux
alias cdw := copy-dll-windows
alias cdo := copy-dll-osx

### recipes

## combinations

# list all available recipes
[no-exit-message]
default:
	@just \
		--chooser 'fzf \
			--height 40% \
			--reverse' \
		--choose

# everything everywhere all at once
all: bc-all sync-all

# eeaao for windows
all-windows: bc-windows sync-windows

# eeaao for osx
all-osx: bc-osx sync-osx

# build and load dll for both platforms
bc-all: build-all copy-dll-all

# build and load dll for windows platforms
bc-linux: build-linux copy-dll-linux

# build and load dll for windows platforms
bc-windows: build-windows copy-dll-windows

# build and load dll for osx platforms
bc-osx: build-osx copy-dll-osx

## sync

# sync unity project to both platforms
sync-all: sync-windows sync-osx

# sync unity project to given target
sync target:
	@# -Pzrvi
	rsync {{ unity / "" }} {{ target }} \
		--exclude Logs \
		--exclude Library \
		--exclude Temp \
		--compress \
		--recursive \
		--copy-links \
		--partial \
		--progress \
		--times \
		--itemize-changes \
		--verbose

# sync unity project to windows
sync-windows: (sync win-dest)

# sync unity project to osx
sync-osx: (sync osx-dest)

# sync only DLL files
dllsync-windows:
	rsync {{ plugin-dir / "*.dll" }} \
		{{ win-dest + "\\Assets\\Scenes\\Plugins" }} \
		--compress \
		--recursive \
		--itemize-changes \
		--times \
		--verbose

## delsync

# delete synced unity project on both platforms
delsync-all: delsync-windows delsync-osx

# delete synced unity project on windows
delsync-windows:
	ssh {{ win-ssh-host }} \
		"if exist {{ win-unity-path }} \
		rmdir /s/q {{ win-unity-path }}"

# delete synced unity project on osx
delsync-osx:
	ssh {{ osx-ssh-host }} \
		"rm -rf {{ osx-unity-path }}"

## build

# cross compile to all platforms
build-all: build-linux build-windows build-osx

# compile
build target="x86_64-unknown-linux-gnu": (xbuild target)

# cross compile to given target
xbuild target buildcmd="build":
	cargo {{ buildcmd }} \
		--package jitsi-meet-signalling-c \
		--no-default-features \
		--features tls-rustls-native-roots \
		--target {{ target }} \
		--profile {{ profile }}

# cross compile to given target with zigbuild
zigbuild target: (xbuild target "zigbuild")

# cross compile to given target with xwinbuild
xwinbuild target: (xbuild target "xwin build")

# cross compile to linux platform
build-linux: build

# cross compile to windows platform
build-windows: (xbuild win-tt
	if win == "gnu" {
		"build"
	} else {
		"xwin build"
	}
)

# cross compile to osx platform
build-osx: (zigbuild osx-tt)

# cpdll

# load built DLL file from target to unity project for the all platforms
copy-dll-all: copy-dll-linux copy-dll-windows copy-dll-osx

# run the CLI with the given arguments
# since cargo will build the project automatically when needed,
# it does not have explicit dependency
copy-dll *argv:
	cargo run \
		--release \
		--package {{ copy-dll-crate }} \
		-- {{ argv }}

# load built DLL file from target to unity project for the given platform
copy-dll-for os *argv: (copy-dll
	snakecase(signalling-c)
	plugin-dir
	"-t"
	os
	"--parents"
	argv
)

# load built DLL file from target to unity project for linux
copy-dll-linux: (copy-dll-for "linux")

# load built DLL file from target to unity project for windows
copy-dll-windows: (copy-dll-for
	if win == "gnu" {
		"windows"
	} else {
		"windows-msvc"
	}
)
# "--prefix" "__"

# load built DLL file from target to unity project for osx
copy-dll-osx: (copy-dll-for 
	if osx == "x86" {
		"osx"
	} else {
		"osx-m1"
	}
)

## etc

# check whether the native plugin directory does exist or not
check-plugins:
	{{ if path_exists(plugin-dir) == "false" { error("plugin directory " + plugin-dir + " does not exists") } else { "" } }}

# cleanse native plugins directory of the unity project
clean-dll: check-plugins
	empty.sh {{ plugin-dir }}

# embed jitsi-meet-unity inside unity-mirror
link:
	ln -sf ../../jitsi-meet-unity \
		{{ unity / "Packages/jitsi-meet-unity" }}
	ln -sf ../../jitsi-meet-unity/Samples~/BasicSendReceive \
		{{ unity / "Assets/BasicSendReceive" }}
