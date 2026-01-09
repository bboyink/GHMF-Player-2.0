use crate::config::{CsvConfig, FcwDirective, FixtureFormat};
use crate::dmx::DmxUniverse;
use std::collections::HashMap;
use anyhow::Result;

/// Manages fixtures and applies commands using CSV configurations
pub struct FixtureManager {
    config: CsvConfig,
    current_state: HashMap<u16, (u8, u8, u8, u8)>, // Fixture# -> (R, G, B, W)
}

impl FixtureManager {
    pub fn new(config: CsvConfig) -> Self {
        Self {
            config,
            current_state: HashMap::new(),
        }
    }
    
    /// Execute an FCW command: "ADDRESS-DATA"
    /// Example: "051-008" means FCW address 051, color index 008
    pub fn execute_fcw_command(&mut self, address: u16, data: u16) -> Result<()> {
        // Get the FCW mapping to find affected fixtures
        let mapping = self.config.get_fcw_mapping(address);
        
        if mapping.is_none() {
            tracing::warn!("No FCW mapping found for address {}", address);
            return Ok(());
        }
        
        // Get the color from ColorMap
        let color = self.config.get_color(data);
        
        if color.is_none() {
            tracing::warn!("No color found for index {}", data);
            return Ok(());
        }
        
        let (r, g, b) = color.unwrap().to_rgb()?;
        
        // Collect fixture operations to avoid borrow checker issues
        let operations: Vec<(u16, FcwDirective)> = mapping.unwrap()
            .fixture_directives
            .iter()
            .map(|(num, dir)| (*num, dir.clone()))
            .collect();
        
        // Apply to all affected fixtures based on directive
        for (fixture_num, directive) in operations {
            match directive {
                FcwDirective::On => {
                    self.set_fixture_color(fixture_num, r, g, b, 0)?;
                }
                FcwDirective::Fade => {
                    // TODO: Implement fade logic with timing
                    self.set_fixture_color(fixture_num, 0, 0, 0, 0)?;
                }
                FcwDirective::GreenYellow => {
                    // Special mode - use green/yellow mix
                    self.set_fixture_color(fixture_num, r / 2, g, 0, 0)?;
                }
                FcwDirective::Custom(name) if name == "WHT" => {
                    // White channel mode for RGBW fixtures
                    self.set_fixture_color(fixture_num, 0, 0, 0, 255)?;
                }
                _ => {
                    // Other directives or Off
                }
            }
        }
        
        Ok(())
    }
    
    /// Execute a hex color command: "ADDRESS-RRGGBB"
    /// Example: "051-FF0000" means FCW address 051, red color
    pub fn execute_hex_command(&mut self, address: u16, hex_color: &str) -> Result<()> {
        // Parse hex color
        let hex = hex_color.trim_start_matches('#');
        if hex.len() != 6 {
            anyhow::bail!("Invalid hex color: {}", hex_color);
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;
        
        // Get the FCW mapping
        let mapping = self.config.get_fcw_mapping(address);
        
        if mapping.is_none() {
            tracing::warn!("No FCW mapping found for address {}", address);
            return Ok(());
        }
        
        // Collect fixture operations to avoid borrow checker issues
        let operations: Vec<(u16, FcwDirective)> = mapping.unwrap()
            .fixture_directives
            .iter()
            .map(|(num, dir)| (*num, dir.clone()))
            .collect();
        
        // Apply to all affected fixtures based on directive
        for (fixture_num, directive) in operations {
            match directive {
                FcwDirective::On => {
                    self.set_fixture_color(fixture_num, r, g, b, 0)?;
                }
                FcwDirective::Fade => {
                    self.set_fixture_color(fixture_num, 0, 0, 0, 0)?;
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// Set a specific fixture's color
    pub fn set_fixture_color(&mut self, fixture_num: u16, r: u8, g: u8, b: u8, w: u8) -> Result<()> {
        // Get fixture definition
        let fixture = self.config.get_fixture(fixture_num);
        
        if fixture.is_none() {
            tracing::warn!("No fixture definition found for fixture {}", fixture_num);
            return Ok(());
        }
        
        let fixture = fixture.unwrap();
        
        // Apply color corrections
        let r_corrected = (r as f32 * fixture.corrections.get(0).unwrap_or(&1.0)) as u8;
        let g_corrected = (g as f32 * fixture.corrections.get(1).unwrap_or(&1.0)) as u8;
        let b_corrected = (b as f32 * fixture.corrections.get(2).unwrap_or(&1.0)) as u8;
        let w_corrected = (w as f32 * fixture.corrections.get(3).unwrap_or(&1.0)) as u8;
        
        // Store current state
        self.current_state.insert(fixture_num, (r_corrected, g_corrected, b_corrected, w_corrected));
        
        tracing::debug!(
            "Fixture {} (DMX {}): R={} G={} B={} W={}",
            fixture_num,
            fixture.dmx_channel,
            r_corrected,
            g_corrected,
            b_corrected,
            w_corrected
        );
        
        Ok(())
    }
    
    /// Apply current fixture states to DMX universe
    pub fn apply_to_dmx(&self, universe: &mut DmxUniverse) -> Result<()> {
        for (fixture_num, (r, g, b, w)) in &self.current_state {
            if let Some(fixture) = self.config.get_fixture(*fixture_num) {
                let channel = fixture.dmx_channel as usize;
                
                match fixture.format {
                    FixtureFormat::RGB => {
                        if channel > 0 && channel + 2 <= 512 {
                            universe.set_channel(channel, *r)?;
                            universe.set_channel(channel + 1, *g)?;
                            universe.set_channel(channel + 2, *b)?;
                        }
                    }
                    FixtureFormat::RGBW => {
                        if channel > 0 && channel + 3 <= 512 {
                            universe.set_channel(channel, *r)?;
                            universe.set_channel(channel + 1, *g)?;
                            universe.set_channel(channel + 2, *b)?;
                            universe.set_channel(channel + 3, *w)?;
                        }
                    }
                    FixtureFormat::X => {
                        // Single channel - use brightness (max of RGB)
                        let brightness = (*r).max(*g).max(*b);
                        if channel > 0 && channel <= 512 {
                            universe.set_channel(channel, brightness)?;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Get current color of a fixture
    pub fn get_fixture_color(&self, fixture_num: u16) -> Option<(u8, u8, u8, u8)> {
        self.current_state.get(&fixture_num).copied()
    }
    
    /// Clear all fixtures (blackout)
    pub fn blackout(&mut self) {
        for fixture_num in self.current_state.keys().copied().collect::<Vec<_>>() {
            let _ = self.set_fixture_color(fixture_num, 0, 0, 0, 0);
        }
    }
    
    /// Get all fixture numbers
    pub fn get_all_fixture_numbers(&self) -> Vec<u16> {
        self.config.fixtures.keys().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Tests would require loading actual CSV files or mocking
}
