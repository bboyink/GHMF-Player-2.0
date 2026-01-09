# GHMF Playback 2.0 - Rust Migration Plan

## Executive Summary

This document outlines the complete strategy for converting the C# GHMF Playback application to Rust with cross-platform support (Windows, macOS, Linux). The Rust version will provide better performance, memory safety, and native cross-platform audio/DMX control.

## Current Architecture Analysis

### Core Components

1. **Audio Playback** (`Player.cs`)
   - Uses NAudio with WASAPI for Windows-specific audio
   - Supports WAV via MediaFoundationReader (also handles MP3)
   - Real-time volume control and VU metering
   - Latency compensation

2. **DMX Lighting Control** (`Lighting/` directory)
   - OpenDMX via FTDI driver (FTD2XX.dll) - Windows only
   - 512 DMX channels
   - Support for RGB, RGBW, and raw DMX devices
   - Light grouping, fading, color mixing
   - Channel mapping and fixtures

3. **Command System** (`Commands/` directory)
   - FCW (Fountain Control Word) format: `AAA-DDD` or `AAA-DDDDDD`
   - Time-synchronized command execution
   - Support for lighting, water control, and DMX devices

4. **Playlist Management** (`Playlist.cs`)
   - Encrypted playlist format (`.fple`)
   - Song sequencing with comments

5. **GUI** (Windows Forms)
   - Playback controls
   - Light control interface
   - Settings management
   - VU meters and status displays

6. **PLC Communications** (`PLCComms.cs`)
   - Water fountain control integration

---

## Rust Architecture

### Project Structure

```
ghmf-playback-rust/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs                 # Application entry point
│   ├── lib.rs                  # Library exports
│   ├── audio/
│   │   ├── mod.rs              # Audio module
│   │   ├── player.rs           # Audio player implementation
│   │   ├── decoder.rs          # Audio decoding (WAV/MP3)
│   │   └── volume.rs           # Volume control
│   ├── dmx/
│   │   ├── mod.rs              # DMX module
│   │   ├── enttec.rs           # Enttec DMX USB Pro driver
│   │   ├── universe.rs         # DMX universe (512 channels)
│   │   └── device.rs           # DMX device abstraction
│   ├── lighting/
│   │   ├── mod.rs              # Lighting module
│   │   ├── light.rs            # Light fixture
│   │   ├── color.rs            # Color management (RGB/RGBW)
│   │   ├── channel.rs          # DMX channel mapping
│   │   ├── module.rs           # Light grouping
│   │   └── effects.rs          # Fading, shifting effects
│   ├── commands/
│   │   ├── mod.rs              # Command module
│   │   ├── command.rs          # FCW command structure
│   │   ├── command_file.rs     # Command file parser
│   │   └── executor.rs         # Time-synchronized execution
│   ├── playlist/
│   │   ├── mod.rs              # Playlist module
│   │   └── playlist.rs         # Playlist management
│   ├── gui/
│   │   ├── mod.rs              # GUI module
│   │   ├── app.rs              # Main application window
│   │   ├── playback_controls.rs
│   │   ├── light_control.rs
│   │   └── settings.rs
│   ├── plc/
│   │   ├── mod.rs              # PLC communications
│   │   └── comms.rs            # Serial/network PLC interface
│   ├── config/
│   │   ├── mod.rs              # Configuration module
│   │   └── settings.rs         # Settings management
│   └── utils/
│       ├── mod.rs              # Utilities
│       ├── logger.rs           # Logging system
│       └── error.rs            # Error types
└── tests/
    ├── audio_tests.rs
    ├── dmx_tests.rs
    └── command_tests.rs
```

---

## Recommended Rust Crates

### 1. Audio Playback

**Primary: `rodio`** (Cross-platform audio playback)
```toml
rodio = "0.19"           # Audio playback & decoding
cpal = "0.15"            # Cross-platform audio I/O (used by rodio)
symphonia = "0.5"        # Audio decoding (WAV, MP3, FLAC, etc.)
```

**Features:**
- ✅ Cross-platform (Windows, macOS, Linux)
- ✅ WAV and MP3 support built-in
- ✅ Volume control
- ✅ Precise timing and seeking
- ✅ No external dependencies
- ✅ Sample rate conversion

**Alternative: `kira`** (Game audio engine with better timing control)
```toml
kira = "0.9"
```

### 2. DMX Control - Enttec USB Pro

**Primary: `serialport`** (Cross-platform serial communication)
```toml
serialport = "4.5"
```

**Implementation Strategy:**
- Implement Enttec DMX USB Pro protocol directly
- Protocol is well-documented and simple
- Cross-platform via serialport (no FTDI driver needed!)
- Real DMX512 support

**Enttec USB Pro Protocol:**
```rust
// Message format: [Start Byte][Label][Data Length LSB][Data Length MSB][Data][End Byte]
const START_BYTE: u8 = 0x7E;
const END_BYTE: u8 = 0xE7;
const SEND_DMX_LABEL: u8 = 6;

// Send DMX packet structure:
// [0x7E][0x06][LengthLSB][LengthMSB][0x00][...512 DMX channels...][0xE7]
```

**Advantages over OpenDMX:**
- ✅ True DMX512 protocol (more compatible)
- ✅ No Windows-only FTDI driver needed
- ✅ Works on macOS and Linux natively
- ✅ Better hardware support
- ✅ More reliable timing

### 3. GUI Framework

**Option A: `egui`** (Immediate mode, cross-platform)
```toml
eframe = "0.29"          # Framework for egui apps
egui = "0.29"            # Immediate mode GUI
```

**Pros:**
- ✅ Cross-platform (native + WASM)
- ✅ Modern, clean UI
- ✅ Easy to use
- ✅ Good for real-time displays (VU meters)
- ✅ Active development

**Option B: `iced`** (Elm-inspired, declarative)
```toml
iced = "0.13"
```

**Pros:**
- ✅ Beautiful, responsive UI
- ✅ Cross-platform
- ✅ Type-safe state management

**Option C: `tauri`** (Web-based UI with Rust backend)
```toml
tauri = "2.0"
```

**Pros:**
- ✅ Web technologies (HTML/CSS/JS) for UI
- ✅ Rust backend for performance
- ✅ Smaller than Electron
- ✅ Easy for web developers

**Recommendation:** Start with **`egui`** for simplicity and real-time performance.

### 4. Serialization & Configuration

```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"       # JSON config files
toml = "0.8"             # TOML config (optional, more readable)
```

### 5. Async Runtime

```toml
tokio = { version = "1.40", features = ["full"] }
```

**Used for:**
- Non-blocking audio/DMX operations
- Timer management
- Concurrent command execution

### 6. Error Handling

```toml
thiserror = "1.0"        # Easy error type definitions
anyhow = "1.0"           # Flexible error handling
```

### 7. Logging

```toml
tracing = "0.1"          # Structured logging
tracing-subscriber = "0.3"
```

### 8. Time & Synchronization

```toml
chrono = "0.4"           # Date/time handling
```

---

## Complete Cargo.toml

```toml
[package]
name = "ghmf-playback"
version = "2.0.0"
edition = "2021"
authors = ["City of Grand Haven"]
description = "Cross-platform fountain playback system with synchronized audio and DMX lighting"

[dependencies]
# Audio
rodio = "0.19"
symphonia = "0.5"
cpal = "0.15"

# DMX/Serial
serialport = "4.5"

# GUI
eframe = "0.29"
egui = "0.29"

# Async runtime
tokio = { version = "1.40", features = ["full", "rt-multi-thread", "macros", "time"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Utilities
chrono = "0.4"
once_cell = "1.19"
crossbeam-channel = "0.5"

[dev-dependencies]
tempfile = "3.12"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true

[profile.dev]
opt-level = 1
```

---

## Key Implementation Details

### 1. Audio Player (Rust)

```rust
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct AudioPlayer {
    sink: Arc<Mutex<Sink>>,
    _stream: OutputStream,
    current_volume: Arc<Mutex<f32>>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        
        Ok(Self {
            sink: Arc::new(Mutex::new(sink)),
            _stream: stream,
            current_volume: Arc::new(Mutex::new(1.0)),
        })
    }

    pub fn play(&self, path: &str) -> Result<()> {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new(file)?;
        
        let sink = self.sink.lock().unwrap();
        sink.append(source);
        sink.play();
        
        Ok(())
    }

    pub fn set_volume(&self, volume: f32) {
        let sink = self.sink.lock().unwrap();
        sink.set_volume(volume);
        *self.current_volume.lock().unwrap() = volume;
    }

    pub fn pause(&self) {
        self.sink.lock().unwrap().pause();
    }

    pub fn resume(&self) {
        self.sink.lock().unwrap().play();
    }

    pub fn stop(&self) {
        self.sink.lock().unwrap().stop();
    }

    pub fn is_playing(&self) -> bool {
        !self.sink.lock().unwrap().is_paused()
    }

    pub fn get_position(&self) -> Duration {
        // Rodio doesn't directly expose position
        // Need to implement custom time tracking
        Duration::from_secs(0)
    }
}
```

### 2. Enttec DMX USB Pro Driver (Rust)

```rust
use serialport::{SerialPort, SerialPortInfo};
use std::time::Duration;

const START_BYTE: u8 = 0x7E;
const END_BYTE: u8 = 0xE7;
const SEND_DMX_LABEL: u8 = 6;
const DMX_UNIVERSE_SIZE: usize = 512;

pub struct EnttecDmxPro {
    port: Box<dyn SerialPort>,
    universe: [u8; DMX_UNIVERSE_SIZE],
}

impl EnttecDmxPro {
    pub fn new() -> Result<Self> {
        // Auto-detect Enttec device
        let ports = serialport::available_ports()?;
        let enttec_port = ports.iter()
            .find(|p| Self::is_enttec_device(p))
            .ok_or_else(|| anyhow::anyhow!("Enttec DMX USB Pro not found"))?;

        let port = serialport::new(&enttec_port.port_name, 57_600)
            .timeout(Duration::from_millis(100))
            .open()?;

        Ok(Self {
            port,
            universe: [0u8; DMX_UNIVERSE_SIZE],
        })
    }

    fn is_enttec_device(port_info: &SerialPortInfo) -> bool {
        match &port_info.port_type {
            serialport::SerialPortType::UsbPort(info) => {
                info.vid == 0x0403 && // FTDI Vendor ID
                (info.pid == 0x6001 || info.pid == 0x6015) // Enttec Product IDs
            }
            _ => false,
        }
    }

    pub fn set_channel(&mut self, channel: usize, value: u8) {
        if channel > 0 && channel <= DMX_UNIVERSE_SIZE {
            self.universe[channel - 1] = value;
        }
    }

    pub fn send_dmx(&mut self) -> Result<()> {
        // Build Enttec DMX packet
        let mut packet = Vec::with_capacity(DMX_UNIVERSE_SIZE + 6);
        packet.push(START_BYTE);
        packet.push(SEND_DMX_LABEL);
        
        // Length (513 bytes: 1 start code + 512 channels)
        let length = (DMX_UNIVERSE_SIZE + 1) as u16;
        packet.push((length & 0xFF) as u8);        // LSB
        packet.push(((length >> 8) & 0xFF) as u8); // MSB
        
        // DMX Start Code
        packet.push(0x00);
        
        // DMX data
        packet.extend_from_slice(&self.universe);
        
        // End byte
        packet.push(END_BYTE);
        
        self.port.write_all(&packet)?;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.universe.fill(0);
    }
}
```

### 3. Command Parser (Rust)

```rust
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Command {
    pub address: u32,
    pub data: u32,
    pub is_hex_color: bool,
}

impl Command {
    pub fn parse(input: &str) -> Result<Self> {
        let parts: Vec<&str> = input.split('-').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid command format: {}", input));
        }

        let address = parts[0].parse::<u32>()?;
        
        let (data, is_hex_color) = if parts[1].len() >= 6 {
            (u32::from_str_radix(parts[1], 16)?, true)
        } else {
            (parts[1].parse::<u32>()?, false)
        };

        Ok(Self {
            address,
            data,
            is_hex_color,
        })
    }

    pub fn to_rgb(&self) -> Option<(u8, u8, u8)> {
        if self.is_hex_color {
            let r = ((self.data >> 16) & 0xFF) as u8;
            let g = ((self.data >> 8) & 0xFF) as u8;
            let b = (self.data & 0xFF) as u8;
            Some((r, g, b))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandLine {
    pub time: Duration,
    pub commands: Vec<Command>,
}

impl CommandLine {
    pub fn parse(input: &str) -> Result<Self> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Err(anyhow::anyhow!("Empty command line"));
        }

        let time_ms = parts[0].parse::<u64>()?;
        let time = Duration::from_millis(time_ms);

        let commands: Result<Vec<Command>, _> = parts[1..]
            .iter()
            .map(|cmd_str| Command::parse(cmd_str))
            .collect();

        Ok(Self {
            time,
            commands: commands?,
        })
    }
}
```

### 4. Synchronized Command Executor (Rust)

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant, sleep};

pub struct CommandExecutor {
    commands: Vec<CommandLine>,
    current_index: usize,
    start_time: Option<Instant>,
    audio_player: Arc<Mutex<AudioPlayer>>,
    lighting: Arc<Mutex<Lighting>>,
}

impl CommandExecutor {
    pub async fn run(&mut self) {
        self.start_time = Some(Instant::now());
        self.current_index = 0;

        while self.current_index < self.commands.len() {
            let cmd_line = &self.commands[self.current_index];
            let elapsed = self.start_time.unwrap().elapsed();
            
            if cmd_line.time <= elapsed {
                // Execute commands
                self.execute_command_line(cmd_line).await;
                self.current_index += 1;
            } else {
                // Wait until next command time
                let wait_time = cmd_line.time - elapsed;
                sleep(wait_time).await;
            }
        }
    }

    async fn execute_command_line(&self, cmd_line: &CommandLine) {
        let mut lighting = self.lighting.lock().await;
        
        for cmd in &cmd_line.commands {
            if cmd.is_hex_color {
                if let Some((r, g, b)) = cmd.to_rgb() {
                    lighting.set_light_color(cmd.address as usize, r, g, b);
                }
            } else {
                lighting.execute_fcw(cmd.address, cmd.data);
            }
        }
    }
}
```

---

## Migration Strategy

### Phase 1: Core Components (2-3 weeks)
1. ✅ Set up Rust project structure
2. ✅ Implement audio player with rodio (WAV/MP3)
3. ✅ Implement Enttec DMX driver
4. ✅ Port command parser and executor
5. ✅ Basic testing

### Phase 2: Lighting System (2 weeks)
1. ✅ Port lighting color system
2. ✅ Port channel mapping
3. ✅ Implement light modules and grouping
4. ✅ Port fading and effects
5. ✅ Integration testing

### Phase 3: Playlist & Configuration (1 week)
1. ✅ Port playlist parser (including decryption)
2. ✅ Implement settings management
3. ✅ Configuration file handling

### Phase 4: GUI (2-3 weeks)
1. ✅ Basic egui application window
2. ✅ Playback controls
3. ✅ Light control panel
4. ✅ Settings dialog
5. ✅ VU meters and status displays

### Phase 5: PLC Integration (1 week)
1. ✅ Port PLC communications
2. ✅ Serial/network interface
3. ✅ Testing with fountain hardware

### Phase 6: Testing & Polish (2 weeks)
1. ✅ Comprehensive testing on Windows/macOS
2. ✅ Performance optimization
3. ✅ Bug fixes
4. ✅ Documentation
5. ✅ Installer creation

**Total Timeline: 10-12 weeks**

---

## Advantages of Rust Version

1. **Cross-Platform Native Support**
   - Single codebase for Windows, macOS, Linux
   - No framework dependencies (like .NET)

2. **Better Performance**
   - Zero-cost abstractions
   - Compiled to native code
   - Lower latency for timing-critical operations

3. **Memory Safety**
   - No null pointer exceptions
   - No memory leaks
   - Thread safety guaranteed by compiler

4. **Modern DMX Support**
   - Enttec USB Pro is industry standard
   - Better compatibility
   - No proprietary drivers needed

5. **Maintainability**
   - Strong type system catches bugs at compile time
   - Better error handling
   - Modern tooling (cargo, rustfmt, clippy)

6. **Future-Proof**
   - No .NET Framework version issues
   - Active Rust ecosystem
   - Easy to extend and modify

---

## Hardware Requirements

### Enttec DMX USB Pro
- **Model**: Enttec DMX USB Pro (recommended) or DMX USB Pro Mk2
- **Connection**: USB 2.0 or higher
- **Compatibility**: Windows, macOS, Linux
- **Driver**: None needed (uses standard USB serial)
- **Price**: ~$100-150 USD
- **Channels**: 512 DMX channels
- **Advantages over OpenDMX**:
  - More reliable
  - Better timing
  - Industry standard
  - Better hardware quality
  - No driver installation required

### Audio Hardware
- Any standard audio output device
- ASIO support recommended for lowest latency (optional)

---

## Testing Strategy

### Unit Tests
- Audio playback functions
- DMX channel calculations
- Command parsing
- Color conversions

### Integration Tests
- Audio + DMX synchronization
- Command execution timing
- Playlist playback

### Hardware Tests
- Actual Enttec device
- Real DMX fixtures
- Audio output validation

---

## Next Steps

1. **Review and approve this plan**
2. **Set up development environment**
   - Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
   - Install IDE (VS Code with rust-analyzer)
3. **Order Enttec DMX USB Pro** (if not already owned)
4. **Begin Phase 1 implementation**
5. **Iterative testing and refinement**

---

## Questions to Address

1. **GUI Requirements**: Do you need exact visual parity with the Windows Forms version, or can we modernize the interface?

2. **PLC Protocol**: What protocol does the PLC use (Modbus, proprietary serial, etc.)?

3. **Playlist Encryption**: Should we keep the XOR encryption for `.fple` files or move to a standard format?

4. **Distribution**: How should the application be distributed (installer, portable executable, app bundle)?

5. **Testing Hardware**: Do you have access to the DMX fixtures and PLC for testing during development?

---

## Resources

- **Rust Book**: https://doc.rust-lang.org/book/
- **Rodio Documentation**: https://docs.rs/rodio/
- **Enttec DMX USB Pro API**: https://dol2kh495zr52.cloudfront.net/pdf/misc/dmx_usb_pro_api_spec.pdf
- **egui Examples**: https://github.com/emilk/egui
- **DMX512 Specification**: Available from ESTA/USITT

---

## Conclusion

This migration will modernize the GHMF Playback system with:
- ✅ True cross-platform support (no .NET Framework hassles)
- ✅ Better performance and reliability
- ✅ Industry-standard DMX hardware
- ✅ Modern, maintainable codebase
- ✅ Native audio support (WAV/MP3) without external drivers

The investment in Rust will pay dividends in reliability, maintainability, and cross-platform deployment.
