# GHMF Playback 2.0 - Rust Edition

Cross-platform fountain playback system with synchronized audio and DMX lighting control.

## Features

- **Cross-Platform Audio**: WAV and MP3 playback on Windows, macOS, and Linux
- **DMX Lighting Control**: Enttec DMX USB Pro support with 512 channels
- **Command Synchronization**: Precise timing for choreographed fountain shows
- **Modern GUI**: Built with egui for native performance
- **No External Dependencies**: No .NET Framework or proprietary drivers needed

## Hardware Requirements

- **Enttec DMX USB Pro** (recommended) or DMX USB Pro Mk2
- Standard audio output device
- USB port for DMX interface

## Building

### Prerequisites

1. Install Rust: https://rustup.rs/
2. Clone this repository
3. Connect Enttec DMX USB Pro (optional for testing)

### Compile

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run
cargo run

# Run with logging
RUST_LOG=debug cargo run
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test dmx

# Run with output
cargo test -- --nocapture
```

## Project Structure

```
src/
├── main.rs           # Application entry point
├── audio/            # Audio playback system
│   ├── player.rs     # Audio player
│   └── decoder.rs    # Format detection
├── dmx/              # DMX lighting control
│   ├── enttec.rs     # Enttec USB Pro driver
│   └── universe.rs   # DMX universe management
├── commands/         # FCW command system
│   ├── command.rs    # Command parsing
│   ├── command_file.rs # File loading
│   └── executor.rs   # Synchronized execution
├── lighting/         # Lighting system
│   ├── color.rs      # RGB/RGBW colors
│   └── channel.rs    # Channel management
├── playlist/         # Playlist management
├── config/           # Configuration
└── utils/            # Utilities
```

## Command Format (FCW)

Commands follow the format: `TIME ADDRESS-DATA`

- `TIME`: Milliseconds from start
- `ADDRESS`: Device address (0-999)
- `DATA`: Value (0-999) or RGB hex (000000-FFFFFF)

Example:
```
0 500-255 501-128
1000 500-FF0000 501-00FF00
2000 500-0 501-0
```

## Development Status

### ✅ Completed
- Core project structure
- Audio playback (rodio)
- Enttec DMX USB Pro driver
- Command parsing and execution
- Basic color management
- Configuration system
- **Modern GUI with egui** 🎉
  - Playback controls
  - Lighting control panel
  - Settings dialog
  - Status indicators

### 🚧 In Progress
- Full lighting system
- Playlist management
- PLC integration
- File browser dialogs

### 📋 TODO
- Command timeline visualization
- Advanced lighting effects
- Testing with real hardware
- Documentation
- Installer creation

## Contributing

This is a project for the City of Grand Haven. For questions or contributions, please contact the project maintainers.

## License

Copyright © City of Grand Haven

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rodio Documentation](https://docs.rs/rodio/)
- [Enttec DMX USB Pro API](https://dol2kh495zr52.cloudfront.net/pdf/misc/dmx_usb_pro_api_spec.pdf)
- [DMX512 Specification](https://www.esta.org/)

## Migration from C#

See [RUST_MIGRATION_PLAN.md](../RUST_MIGRATION_PLAN.md) for the complete migration strategy and architecture decisions.
