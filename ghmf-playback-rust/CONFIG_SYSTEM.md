# Configuration System

## Overview
The GHMF Playback Rust application uses a hybrid configuration system with JSON files for light mappings and colors, and CSV for fixture definitions.

## Configuration Files

### 1. **legacy_colors.json** (32 colors)
Defines the color palette used by lighting commands.
```json
[
  {
    "index": 1,
    "name": "Red",
    "hex": "#FF0000"
  },
  ...
]
```
**Purpose:** Maps color indices (1-32) to RGB hex values  
**Replaces:** Old ColorMap.csv from C# application

### 2. **light_groups.json** (60+ groups)
Defines light groups and their FCW command mappings.
```json
{
  "groups": [
    {
      "name": "Peacock",
      "fcw_code": "027",
      "fcw_fade_code": "127",
      "fixture_ids": [42, 43, 44, 45, 46, 47, 48, 49]
    },
    ...
  ]
}
```
**Purpose:** Maps FCW codes to fixture groups for lighting control  
**Replaces:** Old FCWMap.CSV from C# application

### 3. **DMXMap.csv** (56 fixtures)
Defines DMX fixture properties and channel mappings.
```csv
#,Note,DMX,Format,Corrections
1,Mod.1 front left,1,RGB,1,1,1
42,Peacock 1,165,RGBW,1,1,1,1
...
```
**Purpose:** Maps fixture IDs to DMX channels, formats (RGB/RGBW), and color corrections

### 4. **dmx_mapping.json** (Reference)
JSON version of DMX mappings for tooling/documentation.
```json
{
  "fixtures": [
    {
      "fixture_id": 42,
      "fixture_name": "Peacock 1",
      "start_channel": 165
    },
    ...
  ]
}
```
**Purpose:** Machine-readable fixture reference (not currently used by application)

## Migration from C# Application

| C# Config File | Rust Replacement | Status |
|----------------|------------------|--------|
| ColorMap.csv | legacy_colors.json | ✅ Migrated |
| FCWMap.CSV | light_groups.json | ✅ Migrated |
| DMXMap.csv | DMXMap.csv | ✅ Still used |
| Playback.xml | playback.toml | ✅ Migrated |

## Configuration Loading

The `CsvConfig` struct (in `src/config/csv_config.rs`) loads all configuration:

```rust
pub struct CsvConfig {
    pub colors: HashMap<u16, ColorDefinition>,
    pub fixtures: HashMap<u16, FixtureDefinition>,
    pub fcw_mappings: HashMap<u16, FcwMapping>,
}

impl CsvConfig {
    pub fn load_from_dir(dir: &Path) -> Result<Self> {
        let colors = load_legacy_colors_json("legacy_colors.json")?;
        let fixtures = load_dmx_map("DMXMap.csv")?;
        let fcw_mappings = load_light_groups_json("light_groups.json")?;
        // ...
    }
}
```

## Fixture Mapping Bug Fix (2026-01-15)

**Issue:** Peacock group (FCW 027) only lighting fixtures 42-45 instead of 42-49.

**Root Cause:** DMXMap.csv only had fixtures 42-45 defined with wrong names and channels:
- Fixture 42: "Voice 1" @ channel 127 (should be "Peacock 1" @ 165)
- Fixtures 46-49: Missing entirely

**Fix:** Updated DMXMap.csv with all 8 Peacock fixtures:
```csv
42,Peacock 1,165,RGBW,1,1,1,1
43,Peacock 2,169,RGBW,1,1,1,1
44,Peacock 3,173,RGBW,1,1,1,1
45,Peacock 4,177,RGBW,1,1,1,1
46,Peacock 5,181,RGBW,1,1,1,1
47,Peacock 6,185,RGBW,1,1,1,1
48,Peacock Left,189,RGBW,1,1,1,1
49,Peacock Right,193,RGBW,1,1,1,1
```

**Lesson:** Both `light_groups.json` AND `DMXMap.csv` must have matching fixture definitions. The application:
1. Loads fixture groups from light_groups.json (which fixtures belong to each FCW code)
2. Loads fixture definitions from DMXMap.csv (how to control each fixture via DMX)
3. Both must be in sync for fixtures to work correctly

## Adding New Fixtures

To add a new fixture:

1. **Add to DMXMap.csv:**
   ```csv
   50,New Fixture,197,RGBW,1,1,1,1
   ```

2. **Add to light_groups.json** (if part of a group):
   ```json
   {
     "name": "New Group",
     "fcw_code": "030",
     "fcw_fade_code": "130",
     "fixture_ids": [50]
   }
   ```

3. **Restart application** to reload configuration

## See Also
- [PLAYBACK_INTEGRATION.md](PLAYBACK_INTEGRATION.md) - Command execution flow
- [LIGHT_GROUPINGS.md](../LIGHT_GROUPINGS.md) - Light group documentation
