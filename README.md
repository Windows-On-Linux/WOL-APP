# Windows on Linux APP

## This is the public repository of the WOL Project

WOL is an application to install Windows applications in Linux through WINE. Ispired by PlayOnLinux 4

# Build

You must install Node.JS, Rust, Cargo, libgtk-3-dev, webkit2gtk-4.0 libappindicator3-dev, librsvg2-dev, terminator, and patchelf
```console
npm i
npm run tauri dev
```

# Installation

You can install WOL using the binaries in the release section

# Create WINE installation script

The installation script is very simple, it's just a bash script that create a wineprefix in ~/wol and then execute all commands to make WINE application work

Check the Ableton Live 11 Suite installation script, The first app that WOL can install, to see the installation script: https://github.com/Windows-On-Linux/Ableton-On-Linux

If you want to add your script in the official repository open an issue on GitHub: https://github.com/Windows-On-Linux/Repo or create a pull request that follows Ableton 11 Live Suite script