# GHMF Playback 2.0 - Electron Migration
## Quick Start Guide

### What's Been Created

A complete Electron + React + TypeScript application structure with:

✅ **Main Process (Backend - Node.js)**
- `electron/main.ts` - Application entry point
- `electron/preload.ts` - Secure context bridge
- `electron/hardware/` - DMX, PLC, Audio controllers
- `electron/config/` - CSV/CTL file parsers
- `electron/ipc/` - IPC handlers for communication

✅ **Renderer Process (Frontend - React)**
- `src/App.tsx` - Main application
- `src/views/` - All 7 views (Playlist, Playback, Lighting, etc.)
- `src/components/` - Sidebar and StatusBar
- `src/types/` - TypeScript definitions

✅ **Configuration**
- `package.json` - Dependencies and scripts
- `tsconfig.json` - TypeScript config
- `vite.config.ts` - Vite build config
- `electron-builder.json` - Packaging config

### Next Steps

#### 1. Install Dependencies
```bash
cd ghmf-playback-electron
npm install
```

#### 2. Development
```bash
npm run dev
```

This will:
- Start Vite dev server on port 5173
- Launch Electron with hot reload
- Open DevTools automatically

#### 3. Testing

**Audio:**
- Navigate to Playlist view
- Load songs from Music directory
- Switch to Playback view to test playback

**DMX:**
- Navigate to Lighting view
- Connect DMX (requires Enttec USB DMX Pro)
- Test fixture control

**PLC:**
- Navigate to Operator view
- Configure PLC connection (IP: 192.168.1.100, Port: 502)
- Test communication

#### 4. Production Build
```bash
npm run build
npm run build:electron
```

Distributable files will be in `release/` directory.

### Architecture Overview

```
┌─────────────────────────────────────────────┐
│           Renderer Process (React)          │
│  - Views (UI)                               │
│  - Components                               │
│  - State Management                         │
└──────────────┬──────────────────────────────┘
               │ IPC (electronAPI)
┌──────────────▼──────────────────────────────┐
│             Preload Script                  │
│  - Context Bridge (Security Layer)          │
└──────────────┬──────────────────────────────┘
               │
┌──────────────▼──────────────────────────────┐
│           Main Process (Node.js)            │
│  - Hardware Control (DMX, PLC, Audio)       │
│  - File System Operations                   │
│  - Config Management                        │
└─────────────────────────────────────────────┘
```

### Key Features Implemented

**Playback View:**
- 7-second scrolling waveform using WaveSurfer.js
- Play/Pause/Stop/Restart controls
- Dual-channel volume control
- Timeline scrubbing

**Playlist View:**
- Drag-and-drop song management
- Theme selection
- Save/Load playlists
- Reorder songs

**Lighting View:**
- RGBW color control
- Fixture selection
- Color presets
- Real-time DMX output

**DMX Map View:**
- Fixture table display
- Channel assignments
- CSV import

**Operator View:**
- Show control
- Weather display
- Hardware connections
- Announcement triggers
- Daily playlist loader

### Hardware Requirements

- **DMX:** Enttec USB DMX Pro (FTDI chipset)
- **PLC:** TCP/IP connection (default: 192.168.1.100:502)
- **Audio:** Any audio output device

### File Locations (Production)

Config files are automatically copied to the app bundle:
- macOS: `Contents/Resources/Config/`
- Windows: `resources/Config/`
- Linux: `resources/Config/`

Music files:
- macOS: `Contents/Resources/Music/`
- Windows: `resources/Music/`
- Linux: `resources/Music/`

### Troubleshooting

**DMX not connecting:**
- Install FTDI drivers
- Check USB connection
- Verify serial port permissions (macOS/Linux)

**Audio not playing:**
- Check file paths in playlist
- Verify audio file format (MP3/WAV)
- Check system audio settings

**Build errors:**
- Clear `node_modules`: `rm -rf node_modules && npm install`
- Clear build cache: `rm -rf dist/`
- Check Node version: Node 18+ required

### Future Enhancements

- [ ] State persistence (save app state)
- [ ] Undo/redo functionality
- [ ] Advanced waveform features
- [ ] Light group presets
- [ ] Procedure scheduling
- [ ] Weather API integration
- [ ] Logging system
- [ ] Crash reporting

### Differences from Rust Version

**Improvements:**
- Better UI with Material-UI
- Easier to modify/extend
- Better audio library (Howler.js)
- Automatic window management
- Hot reload in development

**To Be Added:**
- Some Rust-specific optimizations
- Custom CTL command processing
- Light group advanced features
- Full procedure engine

### Contributing

To add a new view:
1. Create view file in `src/views/`
2. Add route in `src/App.tsx`
3. Add menu item in `src/components/Sidebar.tsx`
4. Add IPC handlers if backend communication needed

To add hardware support:
1. Create controller in `electron/hardware/`
2. Add IPC handlers in `electron/ipc/handlers.ts`
3. Expose API in `electron/preload.ts`
4. Update TypeScript types in `src/types/`

---

**Ready to run!** Execute `npm install` then `npm run dev` to start development.
