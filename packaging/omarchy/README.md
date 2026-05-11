# Moraya Omarchy Integration

This directory contains scripts and configurations to integrate Moraya seamlessly into the Omarchy Linux environment.

## Components

### 1. `omarchy-install-moraya`
A specialized installation script that:
- Installs necessary build dependencies.
- Compiles Moraya from source using the provided `PKGBUILD`.
- Configures default settings for Omarchy:
  - Enables **Theme Synchronization** with `~/.config/omarchy/current/theme/colors.toml`.
  - Hides the **Title Bar** by default to better suit tiling window managers like Hyprland.
  - Registers the `moraya://` deep-link protocol.

### 2. Arch Linux `PKGBUILD`
Located in `packaging/arch/PKGBUILD`, this script handles the native compilation and packaging for Arch-based systems.

## Usage

To install or update Moraya on Omarchy, simply run:
```bash
./omarchy-install-moraya
```

## Features

- **Dynamic Theming**: Moraya will automatically pull its color palette from your system's Omarchy configuration. When you change your global Omarchy theme, Moraya follows.
- **Tiling-Friendly**: By hiding the native title bar, Moraya maximizes vertical space and looks more native in environments where the window manager handles borders and titles.
- **Deep Linking**: Supports `moraya://` URLs for integrating with external research tools and the Picora browser extension.
