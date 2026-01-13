# GHMF Playback 2.0 - Electron Migration Plan

## Project Overview
Migrating from Rust + egui to Electron + React + TypeScript while preserving all functionality and screens.

## Technology Stack

### Frontend (Renderer Process)
- **React 18** with TypeScript
- **Material-UI (MUI)** - Component library
- **React Router** - Navigation between views
- **WaveSurfer.js** - Audio waveform visualization
- **Recharts** or **Chart.js** - Additional visualizations
- **Zustand** or **Redux Toolkit** - State management

### Backend (Main Process)
- **Electron** - Desktop framework
- **TypeScript** - Type safety
- **node-serialport** - DMX USB communication (Enttec)
- **net (Node.js)** - PLC TCP/IP communication
- **fs/fs-extra** - File system operations
- **csv-parse** - Parse CSV config files

### Audio
- **Howler.js** - Audio playback with better control
- **Web Audio API** - Advanced audio processing if needed

## Project Structure
```
ghmf-playback-electron/
├── electron/
│   ├── main.ts              # Main process entry
│   ├── preload.ts           # Preload script (context bridge)
│   ├── hardware/
│   │   ├── dmx.ts          # DMX Enttec control
│   │   ├── plc.ts          # PLC TCP/IP client
│   │   └── audio.ts        # Audio player backend
│   ├── config/
│   │   ├── csv-loader.ts   # Load CSV configs
│   │   └── ctl-parser.ts   # Parse CTL files
│   └── ipc/
│       └── handlers.ts      # IPC message handlers
├── src/
│   ├── App.tsx
│   ├── main.tsx
│   ├── components/
│   │   ├── Sidebar.tsx
│   │   ├── StatusBar.tsx
│   │   └── common/
│   ├── views/
│   │   ├── PlaylistView.tsx
│   │   ├── PlaybackView.tsx
│   │   ├── LightingView.tsx
│   │   ├── DmxMapView.tsx
│   │   ├── LightGroupsView.tsx
│   │   ├── ProceduresView.tsx
│   │   └── OperatorView.tsx
│   ├── stores/
│   │   ├── audioStore.ts
│   │   ├── playlistStore.ts
│   │   ├── dmxStore.ts
│   │   └── plcStore.ts
│   └── types/
│       └── index.ts         # TypeScript interfaces
├── public/
├── Config/                  # Your existing config files
├── Music/                   # Your existing music/playlists
├── package.json
├── tsconfig.json
├── electron-builder.json    # Build configuration
└── README.md
```

## Screen/View Mapping (Rust → Electron)

### 1. Sidebar (Navigation)
**Rust:** `src/gui/sidebar.rs` → **Electron:** `src/components/Sidebar.tsx`
- Material-UI Drawer component
- Icon buttons for each view
- Active view highlighting

### 2. Playlist View
**Rust:** `src/gui/playlist_panel.rs` → **Electron:** `src/views/PlaylistView.tsx`
- Available songs list (drag & drop)
- Selected songs list (reorderable)
- Save/Load playlist buttons
- Theme selection dropdown
- Start with playlist checkbox

### 3. Playback View
**Rust:** `src/gui/playback_panel.rs` → **Electron:** `src/views/PlaybackView.tsx`
- Song title and playlist name display
- **7-second scrolling waveform** (WaveSurfer.js with custom config)
- Timeline with tick marks every second
- Play/Pause/Stop/Restart buttons
- Volume controls (left/right)
- Position indicator (mm:ss / total)

### 4. Lighting Control
**Rust:** `src/gui/lighting_panel.rs` → **Electron:** `src/views/LightingView.tsx`
- Fixture grid/list
- Color pickers (RGB/RGBW)
- Intensity sliders
- Preset buttons
- Real-time DMX output

### 5. DMX Mapping
**Rust:** `src/gui/dmx_map_panel.rs` → **Electron:** `src/views/DmxMapView.tsx`
- Fixture assignments table
- DMX channel visualization
- Edit fixture mappings

### 6. Light Groups
**Rust:** `src/gui/light_group_panel.rs` → **Electron:** `src/views/LightGroupsView.tsx`
- Create/edit light groups
- Assign fixtures to groups
- Group presets

### 7. Procedures
**Rust:** `src/gui/procedures_panel.rs` → **Electron:** `src/views/ProceduresView.tsx`
- Procedure list
- Time-based triggers
- Manual trigger buttons

### 8. Operator View
**Rust:** `src/gui/operator_panel.rs` → **Electron:** `src/views/OperatorView.tsx`
- Show control panel
- Weather display
- PLC status
- Announcement triggers
- Today's playlist loader

### 9. Status Bar
**Rust:** `src/gui/status_panel.rs` → **Electron:** `src/components/StatusBar.tsx`
- Status messages (success/warning/error)
- DMX connection status
- PLC connection status
- RGBW mode indicator

## Core Functionality Migration

### Audio System
**Rust:** `src/audio/player.rs` → **Electron:** `electron/hardware/audio.ts` + `src/stores/audioStore.ts`

```typescript
// Howler.js setup for proper pause/play tracking
const sound = new Howl({
  src: [audioPath],
  html5: true,
  onload: () => { /* loaded */ },
  onplay: () => { /* playing */ },
  onpause: () => { /* paused */ },
  onend: () => { /* ended */ }
});
```

**Key Fix:** Position only advances when actually playing (not like Rust version)

### Waveform (7-Second Window)
**Rust:** `src/gui/playback_panel.rs` lines 127-240 → **Electron:** `WaveSurfer.js` component

```typescript
// WaveSurfer config for 7-second scrolling window
wavesurfer.init({
  container: '#waveform',
  waveColor: '#4a5568',
  progressColor: '#00ceff',
  height: 100,
  responsive: true,
  scrollParent: true,
  minPxPerSec: 100, // Adjust for 7-second visible width
  // Custom plugin for playhead locking at 1s mark
});
```

### DMX Control
**Rust:** `src/dmx/enttec.rs` → **Electron:** `electron/hardware/dmx.ts`

```typescript
import SerialPort from 'serialport';

class EnttecDmxPro {
  private port: SerialPort;
  private universe: Uint8Array = new Uint8Array(512);
  
  async connect(portPath: string) {
    this.port = new SerialPort(portPath, { baudRate: 250000 });
  }
  
  setChannel(channel: number, value: number) {
    this.universe[channel] = value;
  }
  
  sendDMX() {
    // Enttec protocol: [0x7E, 0x06, len_lsb, len_msb, ...data, 0xE7]
    const packet = Buffer.concat([...]);
    this.port.write(packet);
  }
}
```

### PLC Communication
**Rust:** `src/plc/plc_client.rs` → **Electron:** `electron/hardware/plc.ts`

```typescript
import * as net from 'net';

class PlcClient {
  private socket: net.Socket;
  
  async connect(host: string, port: number) {
    this.socket = new net.Socket();
    await new Promise((resolve, reject) => {
      this.socket.connect(port, host, resolve);
      this.socket.on('error', reject);
    });
  }
  
  async sendCommand(command: string) {
    this.socket.write(command + '\n');
  }
}
```

### CTL File Parsing
**Rust:** `src/commands/ctl_file.rs` → **Electron:** `electron/config/ctl-parser.ts`

```typescript
interface CtlCommand {
  time_ms: number;
  fixture: number;
  command: string; // e.g., "255" or "FF0000"
}

function parseCtlFile(filePath: string): CtlCommand[] {
  const content = fs.readFileSync(filePath, 'utf-8');
  const lines = content.split('\n');
  // Parse format: time_ms,fixture-command
  // e.g., "5000,501-255" or "10000,502-FF00AA"
}
```

### CSV Config Loading
**Rust:** `src/config/csv_config.rs` → **Electron:** `electron/config/csv-loader.ts`

- ColorMap.csv (legacy colors)
- DMXMap.csv (fixture definitions)
- FCWMap.csv (fountain/water mappings)

## IPC Communication (Main ↔ Renderer)

### IPC Channels
```typescript
// Main → Renderer (events)
'dmx:connected'
'plc:status'
'audio:position'
'audio:loaded'

// Renderer → Main (invoke)
'audio:load'
'audio:play'
'audio:pause'
'audio:stop'
'audio:seek'
'dmx:setChannel'
'dmx:send'
'plc:sendCommand'
'config:loadCSV'
'playlist:save'
'playlist:load'
'ctl:parse'
```

## Migration Steps

### Phase 1: Project Setup (30 min)
1. Create Electron + React + TypeScript project
2. Install dependencies
3. Configure build tools
4. Setup hot reload for development

### Phase 2: UI Skeleton (2 hours)
1. Create all view components (empty shells)
2. Implement sidebar navigation
3. Setup routing
4. Apply Material-UI theming (dark mode)
5. Create status bar

### Phase 3: Core Backend (3 hours)
1. DMX communication module
2. PLC client module
3. Audio player module
4. Config loaders (CSV, CTL)
5. IPC handlers

### Phase 4: View Implementation (6-8 hours)
1. **Playback View** (priority - fix waveform scrolling)
   - WaveSurfer.js integration
   - 7-second scrolling window
   - Timeline with tick marks
   - Play/pause controls
   
2. **Playlist View**
   - Song lists
   - Drag & drop
   - Save/load playlists
   
3. **Lighting View**
   - Fixture control
   - Color pickers
   - Real-time DMX updates
   
4. **Operator View**
   - Show control
   - Announcements
   - PLC integration
   
5. **Other Views**
   - DMX Mapping
   - Light Groups
   - Procedures

### Phase 5: Testing & Polish (2 hours)
1. Test DMX output with hardware
2. Test PLC communication
3. Test audio playback
4. Fix bugs
5. UI polish

## Dependencies

### package.json
```json
{
  "name": "ghmf-playback",
  "version": "2.0.0",
  "main": "dist/electron/main.js",
  "scripts": {
    "dev": "concurrently \"npm run dev:renderer\" \"npm run dev:electron\"",
    "dev:renderer": "vite",
    "dev:electron": "tsc -p electron && electron .",
    "build": "vite build && tsc -p electron && electron-builder"
  },
  "dependencies": {
    "react": "^18.3.1",
    "react-dom": "^18.3.1",
    "react-router-dom": "^6.22.0",
    "@mui/material": "^5.15.0",
    "@mui/icons-material": "^5.15.0",
    "@emotion/react": "^11.11.0",
    "@emotion/styled": "^11.11.0",
    "wavesurfer.js": "^7.7.0",
    "howler": "^2.2.4",
    "zustand": "^4.5.0",
    "serialport": "^12.0.0",
    "csv-parse": "^5.5.0",
    "fs-extra": "^11.2.0"
  },
  "devDependencies": {
    "electron": "^29.0.0",
    "electron-builder": "^24.13.0",
    "typescript": "^5.3.0",
    "vite": "^5.1.0",
    "@vitejs/plugin-react": "^4.2.0",
    "concurrently": "^8.2.0"
  }
}
```

## Key Improvements Over Rust Version

1. **Waveform Scrolling**: WaveSurfer.js handles this natively with proper configuration
2. **Position Tracking**: Howler.js tracks play/pause state correctly
3. **UI Responsiveness**: React's virtual DOM updates efficiently
4. **Debugging**: Chrome DevTools for everything
5. **Styling**: CSS-in-JS with Material-UI theme system
6. **File Dialogs**: Native Electron dialogs
7. **Hot Reload**: Instant UI updates during development

## Timeline Estimate
- **Total Development Time**: 15-20 hours
- **Week 1**: Core functionality (audio, DMX, PLC)
- **Week 2**: UI implementation and polish
- **Week 3**: Testing with hardware, bug fixes

## Next Steps

1. Create empty folder: `ghmf-playback-electron`
2. Open in VS Code
3. I'll scaffold the entire project structure
4. Start with Playback View (waveform fix)
5. Migrate other views progressively

Ready to start when you open the new folder!
