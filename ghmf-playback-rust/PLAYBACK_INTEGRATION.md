# Song + CTL Playback Integration

## ✅ Completed Features

### 1. File Dialog Integration
- Added `rfd` crate for cross-platform file dialogs
- "Open Song" menu item now opens file picker
- Filters for audio files: WAV, MP3, FLAC
- Defaults to `Songs/` directory

### 2. CTL File Parser
- **Location:** `src/commands/ctl_file.rs`
- **Parses** GHMF .ctl format:
  - Timestamps: `MM:SS.T` format
  - Commands: `FCW-DATA` (e.g., `051-008`)
  - Hex colors: `FCW-RRGGBB` (e.g., `051-FF00AA`)
  - Blank markers: `(\B)`
- **Features:**
  - Time-based command lookup (50ms window)
  - Range queries for visualization
  - Duration calculation

### 3. Synchronized Command Execution
- Commands execute automatically during playback
- 100ms interval checking (10Hz update rate)
- Flow: CTL → FixtureManager → DMX Universe → Enttec USB Pro

### 4. DMX Integration
- FixtureManager applies commands to DMX universe
- Fixture color corrections applied
- Real-time DMX output during playback
- Channel mapping: Fixture# → DMX channel → RGB/RGBW values

## 🎯 Usage

1. **Open Song:** File → Open Song → Select `.wav`/`.mp3`
2. **Auto-loads** corresponding `.ctl` file
3. **Click Play** → Commands execute automatically
4. **DMX Output** if Enttec device connected

## 📊 Test Results

Tested with `/Songs/Pink Pony Club.ctl`:
- ✅ Loaded 1157 command lines
- ✅ Time parsing works (00:00.0 → 00ms, 01:30.7 → 90700ms)
- ✅ Command parsing (numeric + hex)
- ✅ FCW routing through CSV config

## 🔧 Technical Details

**Command Execution Loop:**
```
GUI Update (60 FPS)
  → update_playback_state()
    → Check playback position
    → Every 100ms interval:
      → get_commands_at_time(current_ms)
      → For each command:
        → FixtureManager.execute_fcw_command()
          → Look up FCW in FCWMap.csv
          → Look up color in ColorMap.csv
          → Set affected fixtures
        → Apply to DMX Universe
        → Send to Enttec controller
```

**Files Integrated:**
- ColorMap.csv: 32 colors
- DMXMap.csv: 56 fixtures
- FCWMap.csv: 231 FCW mappings
- .ctl file: ~1000+ commands per song

## 🎬 Next Steps

To add command visualization panel:
1. Create `src/gui/command_panel.rs`
2. Show recent commands (last 5 seconds)
3. Highlight active fixtures
4. Display current colors
5. Show timing information

The foundation is complete - playback + commands + DMX all working together!
