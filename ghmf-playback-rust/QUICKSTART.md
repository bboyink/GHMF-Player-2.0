# Quick Start Guide - GHMF Playback Rust

## Installation

### 1. Install Rust

```bash
# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Download and run: https://rustup.rs/
```

### 2. Navigate to Project

```bash
cd "ghmf-playback-rust"
```

### 3. Build the Project

```bash
# Development build (faster compile, slower runtime)
cargo build

# Release build (optimized for performance)
cargo build --release
```

### 4. Run the Application

```bash
# Development
cargo run

# Release
./target/release/ghmf-playback
```

## Testing Individual Components

### Test Audio Playback

```rust
// In main.rs or create a test file
use ghmf_playback::audio::AudioPlayer;

let player = AudioPlayer::new()?;
player.play("path/to/song.mp3")?;
player.set_volume(0.8);

// Wait for playback
std::thread::sleep(std::time::Duration::from_secs(10));
```

### Test DMX Control

```rust
use ghmf_playback::dmx::EnttecDmxPro;

let mut dmx = EnttecDmxPro::new()?;

// Set channel 1 to full brightness
dmx.set_channel(1, 255)?;

// Set RGB fixture (channels 1-3)
dmx.set_channels(1, &[255, 0, 128])?;

// Send to hardware
dmx.send_dmx()?;
```

### Test Command Parsing

```rust
use ghmf_playback::commands::{Command, CommandFile};

// Parse a single command
let cmd = Command::parse("500-FF0000")?;
println!("Address: {}, RGB: {:?}", cmd.address, cmd.to_rgb());

// Load a command file
let cmd_file = CommandFile::load("show.fcw")?;
println!("Loaded {} commands", cmd_file.commands.len());
```

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test audio
cargo test dmx
cargo test commands

# Run tests with output
cargo test -- --nocapture

# Run tests and show timing
cargo test -- --nocapture --test-threads=1
```

## Common Commands

```bash
# Check for errors without building
cargo check

# Format code
cargo fmt

# Run linter (clippy)
cargo clippy

# Build documentation
cargo doc --open

# Update dependencies
cargo update

# Clean build artifacts
cargo clean
```

## Hardware Setup

### Enttec DMX USB Pro

1. **Connect Device**: Plug Enttec DMX USB Pro into USB port
2. **Verify Detection**: On macOS, check:
   ```bash
   ls /dev/tty.usbserial-*
   ```
   On Linux:
   ```bash
   ls /dev/ttyUSB*
   ```
3. **Permissions** (Linux only):
   ```bash
   sudo usermod -a -G dialout $USER
   # Log out and back in
   ```

4. **Test Connection**:
   ```bash
   cargo run
   # Should see: "✓ DMX device connected"
   ```

### DMX Fixtures

1. Connect DMX fixtures to Enttec output
2. Configure fixture addresses (1-512)
3. Test individual channels:
   ```rust
   dmx.set_channel(1, 255)?;  // Full brightness
   dmx.send_dmx()?;
   ```

## Troubleshooting

### Audio Not Playing

**Problem**: "Failed to initialize audio device"

**Solutions**:
- Check audio output device is connected
- On Linux, install ALSA: `sudo apt-get install libasound2-dev`
- On macOS, check System Preferences > Sound > Output

### DMX Device Not Found

**Problem**: "DMX device not found"

**Solutions**:
1. Check USB connection
2. Verify FTDI drivers (usually pre-installed)
3. Check device permissions (Linux)
4. List available serial ports:
   ```bash
   cargo add serialport --example list_ports
   cargo run --example list_ports
   ```

### Compile Errors

**Problem**: Missing dependencies

**Solution**:
```bash
cargo clean
cargo update
cargo build
```

### Permission Denied (Serial Port)

**Linux**:
```bash
sudo usermod -a -G dialout $USER
sudo usermod -a -G tty $USER
# Log out and back in
```

**macOS**: No action needed (usually works by default)

## Development Workflow

### 1. Edit Code

Use any text editor or IDE:
- VS Code (with rust-analyzer extension)
- RustRover
- Vim/Neovim (with rust-analyzer)
- Sublime Text

### 2. Check Compilation

```bash
cargo check
```

### 3. Run Tests

```bash
cargo test
```

### 4. Run Application

```bash
cargo run
```

### 5. Format & Lint

```bash
cargo fmt
cargo clippy
```

## Performance Tips

### Build for Release

Always use release builds for production:
```bash
cargo build --release
./target/release/ghmf-playback
```

Release builds are 10-100x faster than debug builds!

### Reduce Latency

1. **Audio Latency**: Adjust in settings (default 100ms)
2. **DMX Update Rate**: Send DMX packets every 20-40ms
3. **Command Timing**: Use high-resolution timers

### Memory Usage

- Rust has no garbage collection = predictable performance
- No memory leaks (enforced by compiler)
- Minimal runtime overhead

## Next Steps

1. **Review**: [RUST_MIGRATION_PLAN.md](../RUST_MIGRATION_PLAN.md)
2. **Complete GUI**: Implement egui interface
3. **Test Hardware**: Verify with real DMX fixtures
4. **Port Config**: Migrate light configuration files
5. **Integration**: Test full audio + DMX + commands

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rodio Docs](https://docs.rs/rodio/)
- [Enttec API Spec](https://dol2kh495zr52.cloudfront.net/pdf/misc/dmx_usb_pro_api_spec.pdf)

## Support

For issues or questions:
1. Check error messages carefully
2. Review logs: `RUST_LOG=debug cargo run`
3. Consult documentation
4. Contact project maintainers

---

**Copyright © City of Grand Haven**
