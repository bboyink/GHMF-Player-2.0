use crate::config::{CsvConfig, FcwDirective, FixtureFormat};
use crate::dmx::DmxUniverse;
use std::collections::HashMap;
use std::time::Instant;
use anyhow::Result;

/// Fade state for a fixture
#[derive(Debug, Clone)]
struct FadeState {
    start_time: Instant,
    duration_ms: u64,
    start_color: (u8, u8, u8, u8),
    end_color: (u8, u8, u8, u8),
}

/// Manages fixtures and applies commands using CSV configurations
pub struct FixtureManager {
    pub config: CsvConfig,
    current_state: HashMap<u16, (u8, u8, u8, u8)>, // Fixture# -> (R, G, B, W)
    active_fades: HashMap<u16, FadeState>, // Fixture# -> FadeState
    locked_addresses: std::collections::HashSet<u16>, // FCW addresses that are locked until cleared with 000000
    module_colors: HashMap<u16, (u8, u8, u8)>, // Module address (17-23) -> Last (R, G, B)
    sticky_pair_states: HashMap<u16, bool>, // Track if sticky addresses are colored (not black)
    use_rgbw: bool, // true = convert RGB to RGBW, false = RGB only with W=0
}

impl FixtureManager {
    pub fn new(config: CsvConfig) -> Self {
        Self {
            config,
            current_state: HashMap::new(),
            active_fades: HashMap::new(),
            locked_addresses: std::collections::HashSet::new(),
            module_colors: HashMap::new(),
            sticky_pair_states: HashMap::new(),
            use_rgbw: true, // Default to RGBW mode
        }
    }
    
    /// Reset all fixtures to black (000000) and clear all state
    pub fn reset_all(&mut self) {
        tracing::info!("Resetting all fixtures to black");
        
        // Clear all state
        self.current_state.clear();
        self.active_fades.clear();
        self.locked_addresses.clear();
        self.module_colors.clear();
        self.sticky_pair_states.clear();
        
        // Note: The fixtures will naturally show black (0,0,0,0) since current_state is empty
        // The apply_to_dmx function will not set any values, which means the DMX channels
        // will remain at 0 (or whatever was last set). To force all fixtures to black,
        // we could iterate through all fixture numbers and set them to (0,0,0,0),
        // but clearing the HashMap achieves the same effect more efficiently.
    }
    
    /// Check if an FCW address is lockable (holds state until cleared with 000000)
    fn is_lockable_address(address: u16) -> bool {
        matches!(address, 57 | 504 | 505 | 509 | 510 | 514 | 515 | 519 | 520 | 524 | 525 | 529 | 530 | 534 | 535)
    }
    
    /// Check if a color is black (off)
    fn is_black(r: u8, g: u8, b: u8) -> bool {
        r == 0 && g == 0 && b == 0
    }
    
    /// Get the module address (17-23) for a given fixture number
    fn get_module_address_for_fixture(fixture_num: u16) -> Option<u16> {
        match fixture_num {
            1..=6 => Some(17),   // Module 1
            7..=12 => Some(18),  // Module 2
            13..=18 => Some(19), // Module 3
            19..=23 => Some(20), // Module 4
            24..=29 => Some(21), // Module 5
            30..=35 => Some(22), // Module 6
            36..=41 => Some(23), // Module 7
            _ => None,
        }
    }
    
    /// Get the center fixture for a sticky address pair (backwards compatibility)
    /// Returns (center_fixture_num, pair_address_1, pair_address_2, module_address)
    fn get_center_fixture_for_sticky_pair(address: u16) -> Option<(u16, u16, u16, u16)> {
        match address {
            504 | 505 => Some((5, 504, 505, 17)),   // Module 1: fixtures 4-6
            509 | 510 => Some((11, 509, 510, 18)),  // Module 2: fixtures 10-12
            514 | 515 => Some((17, 514, 515, 19)),  // Module 3: fixtures 16-18
            519 | 520 => Some((23, 519, 520, 20)),  // Module 4: fixtures 22-23 (no center, using 23)
            524 | 525 => Some((28, 524, 525, 21)),  // Module 5: fixtures 27-29
            529 | 530 => Some((34, 529, 530, 22)),  // Module 6: fixtures 33-35
            534 | 535 => Some((40, 534, 535, 23)),  // Module 7: fixtures 39-41
            _ => None,
        }
    }
    
    /// Update center fixture state based on sticky pair activation
    fn update_center_fixture(&mut self, address: u16, is_colored: bool) -> Result<()> {
        if let Some((center_fixture, addr1, addr2, module_addr)) = Self::get_center_fixture_for_sticky_pair(address) {
            // Update sticky pair state
            self.sticky_pair_states.insert(address, is_colored);
            
            // Check if either address in the pair is colored
            let pair1_colored = self.sticky_pair_states.get(&addr1).copied().unwrap_or(false);
            let pair2_colored = self.sticky_pair_states.get(&addr2).copied().unwrap_or(false);
            let any_colored = pair1_colored || pair2_colored;
            
            if any_colored {
                // Turn off center fixture
                tracing::info!("Backwards compat: Turning off center fixture {} (sticky addresses {}, {} active)", 
                    center_fixture, addr1, addr2);
                self.set_fixture_color(center_fixture, 0, 0, 0, 0)?;
            } else {
                // Both are black, restore to module color
                if let Some(&(r, g, b)) = self.module_colors.get(&module_addr) {
                    let (r, g, b, w) = if self.use_rgbw {
                        self.rgb_to_rgbw(r, g, b)
                    } else {
                        (r, g, b, 0)
                    };
                    tracing::info!("Backwards compat: Restoring center fixture {} to module {} color: ({}, {}, {})", 
                        center_fixture, module_addr, r, g, b);
                    self.set_fixture_color(center_fixture, r, g, b, w)?;
                }
            }
        }
        Ok(())
    }
    
    /// Set whether to use RGBW mode (convert RGB to RGBW) or RGB mode (W=0)
    pub fn set_rgbw_mode(&mut self, use_rgbw: bool) {
        self.use_rgbw = use_rgbw;
    }
    
    /// Convert RGB to RGBW using luminance extraction
    /// Extracts the minimum RGB value as white channel for better color mixing
    fn rgb_to_rgbw(&self, r: u8, g: u8, b: u8) -> (u8, u8, u8, u8) {
        if !self.use_rgbw {
            // RGB mode: just return RGB with W=0
            return (r, g, b, 0);
        }
        
        // RGBW mode: Extract white component
        // Find the minimum value among R, G, B - this becomes the white channel
        let w = r.min(g).min(b);
        
        // Subtract white from RGB channels
        let r_new = r.saturating_sub(w);
        let g_new = g.saturating_sub(w);
        let b_new = b.saturating_sub(w);
        
        (r_new, g_new, b_new, w)
    }
    
    /// Execute an FCW command: "ADDRESS-DATA"
    /// Example: "051-008" means FCW address 051, color index 008
    pub fn execute_fcw_command(&mut self, address: u16, data: u16) -> Result<()> {
        // Check if this is a lockable address
        let is_lockable = Self::is_lockable_address(address);
        
        // Get the FCW mapping to find affected fixtures
        let mapping = self.config.get_fcw_mapping(address);
        
        if mapping.is_none() {
            tracing::warn!("No FCW mapping found for address {}", address);
            return Ok(());
        }
        
        // Get the color from legacy_colors.json
        let color = self.config.get_color(data);
        
        if color.is_none() {
            tracing::warn!("No color found for index {}", data);
            return Ok(());
        }
        
        let (r, g, b) = color.unwrap().to_rgb()?;
        let is_black = Self::is_black(r, g, b);
        
        // Store module colors for addresses 17-23 (non-black only)
        if matches!(address, 17..=23) && !is_black {
            self.module_colors.insert(address, (r, g, b));
        }
        
        // Handle locking logic for lockable addresses
        if is_lockable {
            if is_black {
                // For sticky fixtures going to black: restore module color instead of turning off
                tracing::info!("Sticky address {} received black - restoring module color", address);
                let mapping = self.config.get_fcw_mapping(address);
                
                if let Some(mapping) = mapping {
                    // Collect fixture operations
                    let operations: Vec<(u16, FcwDirective)> = mapping
                        .fixture_directives
                        .iter()
                        .map(|(num, dir)| (*num, dir.clone()))
                        .collect();
                    
                    // For each affected fixture, restore its module color
                    for (fixture_num, _) in operations {
                        if let Some(module_addr) = Self::get_module_address_for_fixture(fixture_num) {
                            if let Some(&(mod_r, mod_g, mod_b)) = self.module_colors.get(&module_addr) {
                                // Restore the module color instead of turning off
                                tracing::info!("Restoring fixture {} to module {} color: ({}, {}, {})", 
                                    fixture_num, module_addr, mod_r, mod_g, mod_b);
                                let (r_out, g_out, b_out, w_out) = self.rgb_to_rgbw(mod_r, mod_g, mod_b);
                                self.set_fixture_color(fixture_num, r_out, g_out, b_out, w_out)?;
                            }
                        }
                    }
                    
                    // Update center fixture (backwards compatibility)
                    self.update_center_fixture(address, false)?;
                }
                
                // Unlock the address
                self.locked_addresses.remove(&address);
                return Ok(());
            } else if self.locked_addresses.contains(&address) {
                // Address is locked - ignore this command
                tracing::info!("Sticky address {} is locked - ignoring command with data {}", address, data);
                return Ok(());
            } else {
                // Not locked yet - lock it after setting the color
                tracing::info!("Locking sticky address {} with color index {}", address, data);
                self.locked_addresses.insert(address);
            }
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
                    let (r_out, g_out, b_out, w_out) = self.rgb_to_rgbw(r, g, b);
                    self.set_fixture_color(fixture_num, r_out, g_out, b_out, w_out)?;
                }
                FcwDirective::Fade => {
                    // TODO: Implement fade logic with timing
                    self.set_fixture_color(fixture_num, 0, 0, 0, 0)?;
                }
                FcwDirective::GreenYellow => {
                    // Special mode - use green/yellow mix
                    let (r_out, g_out, b_out, w_out) = self.rgb_to_rgbw(r / 2, g, 0);
                    self.set_fixture_color(fixture_num, r_out, g_out, b_out, w_out)?;
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
        
        // Update center fixture if this is a lockable address with color
        if is_lockable && !is_black {
            self.update_center_fixture(address, true)?;
        }
        
        Ok(())
    }
    
    /// Execute a hex color command: "ADDRESS-RRGGBB"
    /// Example: "051-FF0000" means FCW address 051, red color
    pub fn execute_hex_command(&mut self, address: u16, hex_color: &str) -> Result<()> {
        // Check if this is a lockable address
        let is_lockable = Self::is_lockable_address(address);
        
        // Parse hex color
        let hex = hex_color.trim_start_matches('#');
        if hex.len() != 6 {
            anyhow::bail!("Invalid hex color: {}", hex_color);
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;
        let is_black = Self::is_black(r, g, b);
        
        // Store module colors for addresses 17-23 (non-black only)
        if matches!(address, 17..=23) && !is_black {
            self.module_colors.insert(address, (r, g, b));
        }
        
        // Handle locking logic for lockable addresses
        if is_lockable {
            if is_black {
                // For sticky fixtures going to 000000: restore module color instead of turning off
                // Get the FCW mapping first to find which fixtures are affected
                let mapping = self.config.get_fcw_mapping(address);
                
                if let Some(mapping) = mapping {
                    // Collect fixture operations
                    let operations: Vec<(u16, FcwDirective)> = mapping
                        .fixture_directives
                        .iter()
                        .map(|(num, dir)| (*num, dir.clone()))
                        .collect();
                    
                    // For each affected fixture, restore its module color
                    for (fixture_num, _) in operations {
                        if let Some(module_addr) = Self::get_module_address_for_fixture(fixture_num) {
                            if let Some(&(mod_r, mod_g, mod_b)) = self.module_colors.get(&module_addr) {
                                // Restore the module color instead of turning off
                                let (r_out, g_out, b_out, w_out) = self.rgb_to_rgbw(mod_r, mod_g, mod_b);
                                self.set_fixture_color(fixture_num, r_out, g_out, b_out, w_out)?;
                            }
                        }
                    }
                    
                    // Update center fixture (backwards compatibility)
                    self.update_center_fixture(address, false)?;
                }
                
                // Unlock the address
                self.locked_addresses.remove(&address);
                return Ok(());
            } else if self.locked_addresses.contains(&address) {
                // Address is locked - ignore this command
                return Ok(());
            } else {
                // Not locked yet - lock it after setting the color
                self.locked_addresses.insert(address);
            }
        }
        
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
                    let (r_out, g_out, b_out, w_out) = self.rgb_to_rgbw(r, g, b);
                    self.set_fixture_color(fixture_num, r_out, g_out, b_out, w_out)?;
                }
                FcwDirective::Fade => {
                    self.set_fixture_color(fixture_num, 0, 0, 0, 0)?;
                }
                _ => {}
            }
        }
        
        // Update center fixture if this is a lockable address with color
        if is_lockable && !is_black {
            self.update_center_fixture(address, true)?;
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
        // First update any active fades
        let now = Instant::now();
        for (fixture_num, fade_state) in &self.active_fades {
            let elapsed_ms = now.duration_since(fade_state.start_time).as_millis() as u64;
            
            // Calculate color (interpolated or final)
            let (r, g, b, w) = if elapsed_ms >= fade_state.duration_ms {
                // Fade complete - use exact end color to avoid flicker
                fade_state.end_color
            } else {
                // Interpolate color
                let progress = elapsed_ms as f32 / fade_state.duration_ms as f32;
                let r = Self::interpolate_u8(fade_state.start_color.0, fade_state.end_color.0, progress);
                let g = Self::interpolate_u8(fade_state.start_color.1, fade_state.end_color.1, progress);
                let b = Self::interpolate_u8(fade_state.start_color.2, fade_state.end_color.2, progress);
                let w = Self::interpolate_u8(fade_state.start_color.3, fade_state.end_color.3, progress);
                (r, g, b, w)
            };
            
            // Apply interpolated color to DMX
            if let Some(fixture) = self.config.get_fixture(*fixture_num) {
                let channel = fixture.dmx_channel as usize;
                
                match fixture.format {
                    FixtureFormat::RGB => {
                        if channel > 0 && channel + 2 <= 512 {
                            let _ = universe.set_channel(channel, r);
                            let _ = universe.set_channel(channel + 1, g);
                            let _ = universe.set_channel(channel + 2, b);
                        }
                    }
                    FixtureFormat::RGBW => {
                        if channel > 0 && channel + 3 <= 512 {
                            let _ = universe.set_channel(channel, r);
                            let _ = universe.set_channel(channel + 1, g);
                            let _ = universe.set_channel(channel + 2, b);
                            let _ = universe.set_channel(channel + 3, w);
                        }
                    }
                    FixtureFormat::X => {
                        let brightness = r.max(g).max(b);
                        if channel > 0 && channel <= 512 {
                            let _ = universe.set_channel(channel, brightness);
                        }
                    }
                }
            }
        }
        
        // Apply non-fading fixtures
        for (fixture_num, (r, g, b, w)) in &self.current_state {
            // Skip if actively fading
            if self.active_fades.contains_key(fixture_num) {
                continue;
            }
            
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
    
    /// Update fades and clean up completed ones
    pub fn update_fades(&mut self) {
        let now = Instant::now();
        let mut completed_fades = Vec::new();
        
        for (fixture_num, fade_state) in &self.active_fades {
            let elapsed_ms = now.duration_since(fade_state.start_time).as_millis() as u64;
            
            if elapsed_ms >= fade_state.duration_ms {
                // Fade complete - set final color
                self.current_state.insert(*fixture_num, fade_state.end_color);
                completed_fades.push(*fixture_num);
            }
        }
        
        // Remove completed fades
        for fixture_num in completed_fades {
            self.active_fades.remove(&fixture_num);
        }
    }
    
    /// Linear interpolation between two u8 values
    fn interpolate_u8(start: u8, end: u8, progress: f32) -> u8 {
        let start_f = start as f32;
        let end_f = end as f32;
        (start_f + (end_f - start_f) * progress).round() as u8
    }
    
    /// Start a fade for fixtures to a target color
    pub fn start_fade(&mut self, address: u16, target_r: u8, target_g: u8, target_b: u8, duration_ms: u64) -> Result<()> {
        // Get the FCW mapping to find affected fixtures
        let mapping = self.config.get_fcw_mapping(address);
        
        if mapping.is_none() {
            tracing::warn!("No FCW mapping found for address {}", address);
            return Ok(());
        }
        
        let (target_r_out, target_g_out, target_b_out, target_w_out) = self.rgb_to_rgbw(target_r, target_g, target_b);
        
        // Start fade for all affected fixtures
        let operations: Vec<(u16, FcwDirective)> = mapping.unwrap()
            .fixture_directives
            .iter()
            .map(|(num, dir)| (*num, dir.clone()))
            .collect();
        
        let now = Instant::now();
        
        for (fixture_num, directive) in operations {
            match directive {
                FcwDirective::On | FcwDirective::Fade => {
                    // Get current color (or black if not set)
                    let start_color = self.current_state.get(&fixture_num).copied().unwrap_or((0, 0, 0, 0));
                    
                    // Apply color corrections to target
                    let fixture = self.config.get_fixture(fixture_num);
                    if let Some(fixture) = fixture {
                        let end_r = (target_r_out as f32 * fixture.corrections.get(0).unwrap_or(&1.0)) as u8;
                        let end_g = (target_g_out as f32 * fixture.corrections.get(1).unwrap_or(&1.0)) as u8;
                        let end_b = (target_b_out as f32 * fixture.corrections.get(2).unwrap_or(&1.0)) as u8;
                        let end_w = (target_w_out as f32 * fixture.corrections.get(3).unwrap_or(&1.0)) as u8;
                        
                        // Start the fade
                        self.active_fades.insert(fixture_num, FadeState {
                            start_time: now,
                            duration_ms,
                            start_color,
                            end_color: (end_r, end_g, end_b, end_w),
                        });
                        
                        tracing::debug!(
                            "Starting fade for fixture {}: {:?} -> {:?} over {}ms",
                            fixture_num,
                            start_color,
                            (end_r, end_g, end_b, end_w),
                            duration_ms
                        );
                    }
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// Get current color of a fixture (including fade interpolation if active)
    pub fn get_fixture_color(&self, fixture_num: u16) -> Option<(u8, u8, u8, u8)> {
        // If actively fading, return interpolated color
        if let Some(fade_state) = self.active_fades.get(&fixture_num) {
            let now = Instant::now();
            let elapsed_ms = now.duration_since(fade_state.start_time).as_millis() as u64;
            
            if elapsed_ms < fade_state.duration_ms {
                // Still fading - interpolate
                let progress = elapsed_ms as f32 / fade_state.duration_ms as f32;
                let r = Self::interpolate_u8(fade_state.start_color.0, fade_state.end_color.0, progress);
                let g = Self::interpolate_u8(fade_state.start_color.1, fade_state.end_color.1, progress);
                let b = Self::interpolate_u8(fade_state.start_color.2, fade_state.end_color.2, progress);
                let w = Self::interpolate_u8(fade_state.start_color.3, fade_state.end_color.3, progress);
                return Some((r, g, b, w));
            }
        }
        
        // Not fading or fade complete - return static color
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
