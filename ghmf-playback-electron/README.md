# GHMF Playback 2.0 - Electron

Modern Electron + React + TypeScript implementation of the GHMF Playback system.

## Features

- 🎵 Audio playback with waveform visualization
- 💡 DMX lighting control (Enttec USB)
- 🔌 PLC communication (TCP/IP)
- 📋 Playlist management
- 🎨 Light groups and procedures
- 👨‍💼 Operator view with show control

## Getting Started

### Prerequisites

- Node.js 18+ and npm
- macOS, Windows, or Linux

### Installation

```bash
npm install
```

### Development

```bash
npm run dev
```

This will start both the Vite dev server and Electron app.

### Build

```bash
npm run build
npm run build:electron
```

## Project Structure

- `electron/` - Main process (Node.js backend)
  - `main.ts` - Electron entry point
  - `preload.ts` - Context bridge
  - `hardware/` - DMX, PLC, Audio control
  - `config/` - CSV/CTL file parsing
  - `ipc/` - IPC handlers
- `src/` - Renderer process (React frontend)
  - `views/` - Main application views
  - `components/` - Reusable components
  - `stores/` - Zustand state management
  - `types/` - TypeScript interfaces

## Configuration

Config files are located in `../Config/`:
- `DMXMap.csv` - DMX fixture mappings
- `ColorMap.csv` - Color presets
- `FCWMap.CSV` - Fixture channel width mapping
- `Playback.xml` - Application settings

## Music/Playlists

Located in `../Music/`:
- `Playlists/` - Saved playlist files
- `Pre-Show/`, `Production/`, etc. - Song collections
- `.ctl` files - Control command files

## License

ISC
