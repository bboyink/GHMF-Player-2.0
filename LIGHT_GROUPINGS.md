# GHMF Playback Light Groupings

## Overview
This document describes how the 45 DMX light fixtures and 11 firework channels are organized into logical groups for control via FCW (Fountain Control Word) commands.

## Physical Modules

### Modules 1-3, 5-7 (Standard Layout)
Each module has 5 lights:
- **Front Left**
- **Front Right**
- **Center**
- **Back Left**
- **Back Right**

### Module 4 (Special Layout)
Has 5 lights with a different arrangement:
- **Front Left**
- **Front Right**
- **Front Center Left**
- **Front Center Right**
- **Center**

## Special Effects Groups

### Peacock Lights (6 fixtures)
- Peacock Far Left
- Peacock Left
- Peacock Left Center
- Peacock Right Center
- Peacock Right
- Peacock Far Right

### Voice Lights (2 fixtures)
- Voice 1 (RGBW)
- Voice 2 (RGBW)

### Dove Lights (2 fixtures)
- Dove Left (RGBW)
- Dove Right (RGBW)

### Firework Effects (11 channels)
- FW 1 through FW 11 (DMX fixtures 900-910)

## FCW Command Groupings

### Module-Based Groups

| FCW | Group | Description |
|-----|-------|-------------|
| **49** | Modules 1, 2, 3 | All 5 lights in each module (15 total) |
| **50** | Modules 4, 5, 6 + Peacock Center | All 5 lights in each module + Peacock Left/Right Center (17 total) |
| **17** | Module 1 Only | All 5 lights in Module 1 |
| **18** | Module 2 Only | All 5 lights in Module 2 |
| **19** | Module 3 Only | All 5 lights in Module 3 |
| **20** | Module 4 + Peacock Center | All 5 lights in Module 4 + Peacock Left/Right Center |
| **21** | Module 5 Only | All 5 lights in Module 5 |
| **22** | Module 6 Only | All 5 lights in Module 6 |
| **23** | Module 7 Only | All 5 lights in Module 7 |

### "All" Commands

| FCW | Group | Description |
|-----|-------|-------------|
| **51** | All Modules + Voice | All 35 module lights + Voice 1 & 2 (37 total) |
| **52** | All Modules + Voice | Same as FCW 51 (37 total) |
| **53** | Everything | All modules + Peacock + Voice + Dove (45 total) |

### Peacock-Only Commands

| FCW | Group | Description |
|-----|-------|-------------|
| **25** | All Peacock | All 6 peacock lights |
| **26** | All Peacock | All 6 peacock lights |
| **27** | All Peacock | All 6 peacock lights |
| **41** | All Peacock | All 6 peacock lights |

### Voice-Only Commands

| FCW | Group | Description |
|-----|-------|-------------|
| **54** | Voice (White) | Voice 1 & 2 - White channel only |
| **55** | Voice (Colored) | Voice 1 & 2 - RGB channels |

### Module Subgroups

| FCW | Group | Description |
|-----|-------|-------------|
| **56** | Front Pairs | Front left/right of Modules 1-7 (14 lights) |
| **57** | Back Pairs + Peacock | Back left/right of Modules 1-3, 5-7 + all Peacock (18 lights) |

### Individual Fixture Control

| FCW Range | Description |
|-----------|-------------|
| **501-545** | Individual control for fixtures 1-45 (one FCW per fixture) |

### Firework Control

| FCW Range | Description |
|-----------|-------------|
| **900-910** | Individual firework channels (FW 1 through FW 11) |

## Fade Commands

Most group commands support both instant and fade modes:
- **Instant**: Use the base FCW number (e.g., FCW 49)
- **Fade**: Add 100 to the FCW number (e.g., FCW 149 fades what FCW 49 sets instantly)

### Fade-Enabled Commands
- FCW 49 → FCW 149
- FCW 50 → FCW 150
- FCW 51 → FCW 151
- FCW 52 → (No fade equivalent defined)
- FCW 17-23 → FCW 117-123
- FCW 27 → FCW 127
- FCW 53 → FCW 153
- FCW 55 → FCW 155
- FCW 56-57 → FCW 156-157
- FCW 501-545 → FCW 601-645

## Special Notations

### Color Flags
Some commands in the mapping include special color flags:
- **G/Y**: Green/Yellow color indicator (FCW 24)
- **WHT**: White channel only (FCW 54)

### Command Markers
- **XXX**: Fixture is part of this command group
- **FADE**: Fixture supports fade mode for this command

## Total Fixture Count
- **Module Lights**: 35 (7 modules × 5 lights)
- **Peacock Lights**: 6
- **Voice Lights**: 2
- **Dove Lights**: 2
- **Firework Channels**: 11
- **Grand Total**: 56 controllable outputs

## Notes
- Water commands (FCW 1-13, 217-255, 700-896) are not included in this document as they route to the PLC system
- All other FCW commands route to DMX lighting control
- RGB fixtures use 3 DMX channels (Red, Green, Blue)
- RGBW fixtures use 4 DMX channels (Red, Green, Blue, White)
