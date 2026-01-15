use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use csv::ReaderBuilder;
use anyhow::{Context, Result};

/// Color definition from legacy_colors.json
#[derive(Debug, Clone)]
pub struct ColorDefinition {
    pub index: u16,
    pub hex_color: String,  // 6-character hex (RRGGBB)
    pub description: String,
}

impl ColorDefinition {
    /// Parse hex color to RGB values
    pub fn to_rgb(&self) -> Result<(u8, u8, u8)> {
        let hex = self.hex_color.trim_start_matches('#');
        if hex.len() != 6 {
            anyhow::bail!("Invalid hex color: {}", self.hex_color);
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;
        
        Ok((r, g, b))
    }
}

/// DMX fixture format
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FixtureFormat {
    RGB,   // 3 channels: R, G, B
    RGBW,  // 4 channels: R, G, B, W
    X,     // Single channel
}

impl FixtureFormat {
    pub fn channel_count(&self) -> usize {
        match self {
            FixtureFormat::RGB => 3,
            FixtureFormat::RGBW => 4,
            FixtureFormat::X => 1,
        }
    }
}

/// DMX fixture definition from DMXMap.csv
#[derive(Debug, Clone)]
pub struct FixtureDefinition {
    pub fixture_number: u16,
    pub note: String,
    pub dmx_channel: u16,
    pub format: FixtureFormat,
    pub corrections: Vec<f32>,  // Color correction multipliers
}

/// FCW command directive
#[derive(Debug, Clone, PartialEq)]
pub enum FcwDirective {
    Off,         // Empty or no action
    On,          // "XXX" - Turn on with color
    Fade,        // "FADE" - Fade out/transition
    GreenYellow, // "G/Y" - Special green/yellow mode
    Custom(String), // Other custom directives
}

impl FcwDirective {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_uppercase().as_str() {
            "" => FcwDirective::Off,
            "XXX" => FcwDirective::On,
            "FADE" => FcwDirective::Fade,
            "G/Y" => FcwDirective::GreenYellow,
            "WHT" => FcwDirective::Custom("WHT".to_string()),
            other => FcwDirective::Custom(other.to_string()),
        }
    }
}

/// FCW mapping from light_groups.json
#[derive(Debug, Clone)]
pub struct FcwMapping {
    pub fcw_address: u16,
    pub water_directive: FcwDirective,  // Second column
    pub fixture_directives: HashMap<u16, FcwDirective>, // Fixture# -> Directive
}

/// Configuration manager for all CSV files
#[derive(Clone)]
pub struct CsvConfig {
    pub colors: HashMap<u16, ColorDefinition>,
    pub fixtures: HashMap<u16, FixtureDefinition>,
    pub fcw_mappings: HashMap<u16, FcwMapping>,
}

impl CsvConfig {
    /// Load all configuration files from a directory
    pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let dir = dir.as_ref();
        
        let colors = Self::load_legacy_colors_json(dir.join("legacy_colors.json"))?;
        let fixtures = Self::load_dmx_map(dir.join("DMXMap.csv"))?;
        let fcw_mappings = Self::load_light_groups_json(dir.join("light_groups.json"))?;
        
        Ok(Self {
            colors,
            fixtures,
            fcw_mappings,
        })
    }
    
    /// Load legacy colors from JSON file
    fn load_legacy_colors_json<P: AsRef<Path>>(path: P) -> Result<HashMap<u16, ColorDefinition>> {
        let json_data = std::fs::read_to_string(path.as_ref())
            .context(format!("Failed to read legacy_colors.json at {:?}", path.as_ref()))?;
        
        #[derive(serde::Deserialize)]
        struct LegacyColorJson {
            index: u8,
            name: String,
            hex: String,
        }
        
        let legacy_colors: Vec<LegacyColorJson> = serde_json::from_str(&json_data)
            .context("Failed to parse legacy_colors.json")?;
        
        let mut colors = HashMap::new();
        
        for color in legacy_colors {
            colors.insert(color.index as u16, ColorDefinition {
                index: color.index as u16,
                hex_color: color.hex,
                description: color.name,
            });
        }
        
        tracing::info!("Loaded {} legacy colors from legacy_colors.json", colors.len());
        Ok(colors)
    }
    
    /// Load DMXMap.csv
    fn load_dmx_map<P: AsRef<Path>>(path: P) -> Result<HashMap<u16, FixtureDefinition>> {
        let file = File::open(path.as_ref())
            .context(format!("Failed to open DMXMap.csv at {:?}", path.as_ref()))?;
        
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);
        
        let mut fixtures = HashMap::new();
        
        for result in reader.records() {
            let record = result?;
            
            if record.len() < 4 {
                continue;
            }
            
            let fixture_number: u16 = record[0].trim().parse()
                .context("Failed to parse fixture number")?;
            let note = record[1].trim().to_string();
            let dmx_channel: u16 = record[2].trim().parse()
                .context("Failed to parse DMX channel")?;
            
            let format = match record[3].trim().to_uppercase().as_str() {
                "RGB" => FixtureFormat::RGB,
                "RGBW" => FixtureFormat::RGBW,
                "X" => FixtureFormat::X,
                other => {
                    tracing::warn!("Unknown fixture format: {}, defaulting to RGB", other);
                    FixtureFormat::RGB
                }
            };
            
            // Parse correction values (columns 4+)
            let mut corrections = Vec::new();
            for i in 4..record.len() {
                if let Ok(val) = record[i].trim().parse::<f32>() {
                    corrections.push(val);
                }
            }
            
            // Ensure we have enough corrections for the format
            while corrections.len() < format.channel_count() {
                corrections.push(1.0);
            }
            
            fixtures.insert(fixture_number, FixtureDefinition {
                fixture_number,
                note,
                dmx_channel,
                format,
                corrections,
            });
        }
        
        tracing::info!("Loaded {} fixtures from DMXMap.csv", fixtures.len());
        Ok(fixtures)
    }
    
    /// Load light groups from JSON file
    fn load_light_groups_json<P: AsRef<Path>>(path: P) -> Result<HashMap<u16, FcwMapping>> {
        #[derive(serde::Deserialize)]
        struct LightGroup {
            name: String,
            fcw_code: String,
            fcw_fade_code: String,
            fixture_ids: Vec<u8>,
        }
        
        #[derive(serde::Deserialize)]
        struct LightGroupConfig {
            groups: Vec<LightGroup>,
        }
        
        let json_data = std::fs::read_to_string(path.as_ref())
            .context(format!("Failed to read light_groups.json at {:?}", path.as_ref()))?;
        
        let config: LightGroupConfig = serde_json::from_str(&json_data)
            .context("Failed to parse light_groups.json")?;
        
        let mut fcw_mappings = HashMap::new();
        let group_count = config.groups.len();
        
        for group in config.groups {
            // Parse the FCW code (e.g., "017" -> 17)
            if let Ok(fcw_code) = group.fcw_code.parse::<u16>() {
                let mut fixture_directives = HashMap::new();
                
                // Map all fixtures in the group to "On" directive
                for fixture_id in &group.fixture_ids {
                    fixture_directives.insert(*fixture_id as u16, FcwDirective::On);
                }
                
                fcw_mappings.insert(fcw_code, FcwMapping {
                    fcw_address: fcw_code,
                    water_directive: FcwDirective::Off,
                    fixture_directives,
                });
            }
            
            // Parse the fade code (e.g., "117" -> 117)
            if let Ok(fade_code) = group.fcw_fade_code.parse::<u16>() {
                let mut fixture_directives = HashMap::new();
                
                // Map all fixtures in the group to "Fade" directive
                for fixture_id in &group.fixture_ids {
                    fixture_directives.insert(*fixture_id as u16, FcwDirective::Fade);
                }
                
                fcw_mappings.insert(fade_code, FcwMapping {
                    fcw_address: fade_code,
                    water_directive: FcwDirective::Off,
                    fixture_directives,
                });
            }
        }
        
        tracing::info!("Loaded {} FCW mappings from light_groups.json ({} groups)", 
                       fcw_mappings.len(), group_count);
        Ok(fcw_mappings)
    }
    
    /// Get color by index
    pub fn get_color(&self, index: u16) -> Option<&ColorDefinition> {
        self.colors.get(&index)
    }
    
    /// Get fixture by number
    pub fn get_fixture(&self, fixture_num: u16) -> Option<&FixtureDefinition> {
        self.fixtures.get(&fixture_num)
    }
    
    /// Get FCW mapping
    pub fn get_fcw_mapping(&self, fcw_address: u16) -> Option<&FcwMapping> {
        self.fcw_mappings.get(&fcw_address)
    }
    
    /// Get all fixtures affected by an FCW command with a specific directive
    pub fn get_affected_fixtures(&self, fcw_address: u16) -> Vec<(u16, FcwDirective)> {
        if let Some(mapping) = self.get_fcw_mapping(fcw_address) {
            mapping.fixture_directives
                .iter()
                .map(|(num, dir)| (*num, dir.clone()))
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_color_definition_to_rgb() {
        let color = ColorDefinition {
            index: 1,
            hex_color: "FF0000".to_string(),
            description: "Red".to_string(),
        };
        
        let (r, g, b) = color.to_rgb().unwrap();
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }
    
    #[test]
    fn test_fcw_directive_parsing() {
        assert_eq!(FcwDirective::from_str("XXX"), FcwDirective::On);
        assert_eq!(FcwDirective::from_str("FADE"), FcwDirective::Fade);
        assert_eq!(FcwDirective::from_str(""), FcwDirective::Off);
        assert_eq!(FcwDirective::from_str("G/Y"), FcwDirective::GreenYellow);
    }
}
