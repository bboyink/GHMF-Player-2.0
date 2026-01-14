# Troubleshooting Guide

## macOS Crash on Startup (icrate/NSScreen Issue)

### Symptoms
- Application compiles successfully but crashes immediately on startup
- Error message: `invalid message send to -[_TtGCs23_ContiguousArrayStorageCSo8NSScreen_$ countByEnumeratingWithState:objects:count:]: expected return to have type code 'q', but found 'Q'`
- Panic occurs in `icrate-0.0.4` NSEnumerator
- Exit code 134 (SIGABRT)

### Root Cause
This is a known compatibility issue between:
- macOS 26.2+ (newer versions)
- `icrate-0.0.4` crate (Objective-C bindings)
- `winit 0.29.x` dependencies
- `eframe 0.28` with `wgpu` backend

The macOS Objective-C runtime changed how NSScreen enumeration works, breaking the old `icrate` version.

### When This Occurs
- After macOS system update (especially to 26.2+)
- After running `cargo clean` and rebuilding
- When dependencies are updated to incompatible versions

### Solution

**Change the rendering backend from `wgpu` to `glow` in Cargo.toml:**

```toml
# Instead of:
eframe = { version = "0.28", default-features = false, features = ["default_fonts", "wgpu", "wayland", "x11"] }
winit = "0.29.10"

# Use:
eframe = { version = "0.28", default-features = false, features = ["default_fonts", "glow"] }
# Remove the winit line
```

### Steps to Fix

1. Edit `Cargo.toml`
2. Change eframe features from `wgpu` to `glow`
3. Remove any explicit `winit` version pin
4. Rebuild: `cargo build --release`
5. Run: `./target/release/ghmf-playback`

### Prevention
- Consider upgrading to egui/eframe 0.29+ in the future (requires updating `egui_dnd`)
- Document the working eframe configuration
- Test after macOS system updates

### Last Occurrence
- Date: January 13, 2026
- macOS Version: 26.2 (Build 25C56)
- Fixed by: Switching to glow backend
