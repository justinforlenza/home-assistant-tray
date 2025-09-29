# Home Assistant Tray

A lightweight system tray application that embeds your Home Assistant dashboard in a convenient popup window.

## Features

- System tray icon for quick access to Home Assistant
- Borderless popup window positioned near the tray
- Auto-hide on focus loss
- Simple configuration via URL input

## Installation

Download the latest release for your platform from the [Releases](../../releases) page.

## Development

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

## Configuration

On first launch, enter your Home Assistant URL (e.g., `http://homeassistant.local:8123`). Access settings later via the tray menu.

## Built With

- [Tauri](https://tauri.app/) - Desktop application framework
- [Vite](https://vitejs.dev/) - Frontend tooling
