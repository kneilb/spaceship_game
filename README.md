# README

## Introduction

This is taken from a YouTube Bevy tutorial by Zymartu Games.

We're using these models: Ultimate Space Kit by Quaternius via Poly Pizza.

## Building & Running

### Cross-compiling for Windows

Run the [windows.sh](windows.sh) script to cross compile, copy to the right place & run.

Info [taken from here](https://bevy-cheatbook.github.io/platforms/windows/wsl2.html).

Also needed to:

- start docker in WSL2 `sudo docker &` in a separate terminal (no systemd...)
- install cross `cargo install cross`
- add the rust windows target `rustup target add x86_64-pc-windows-msvc`

### WSL2 Ubuntu

On WSL2 Ubuntu, I had to install a few packages to get bevy to build:

- pkg-config
- libasound2-dev
- libudev-dev

I couldn't get either X11 or Wayland backends to work with WSLg, and switched to cross-compiling for Windows.
