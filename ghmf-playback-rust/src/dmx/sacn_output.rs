use sacn::source::SacnSource;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use tracing::{info, warn, error};

#[derive(Debug, Clone, PartialEq)]
pub enum SacnFilterMode {
    AllLights,
    Code900Only,
}

pub struct SacnOutput {
    source: Option<SacnSource>,
    universe: u16,
    filter_mode: SacnFilterMode,
    last_values: [u8; 512],
}

impl SacnOutput {
    pub fn new() -> Self {
        Self {
            source: None,
            universe: 1,
            filter_mode: SacnFilterMode::AllLights,
            last_values: [0; 512],
        }
    }
    
    /// Initialize sACN output on a specific network interface
    pub fn start(&mut self, interface_ip: &str) -> Result<(), String> {
        let ip_addr: Ipv4Addr = interface_ip
            .parse()
            .map_err(|e| format!("Invalid IP address: {}", e))?;
        
        // Create DMX source with bind address
        let bind_addr = SocketAddr::new(IpAddr::V4(ip_addr), 0);
        
        let mut source = SacnSource::with_ip("GHMF Playback", bind_addr)
            .map_err(|e| format!("Failed to create sACN source: {}", e))?;
        
        // Register universe 1
        source.register_universe(self.universe)
            .map_err(|e| format!("Failed to register universe: {}", e))?;
        
        // Note: Priority is set per-send in sacn 0.11, not globally
        
        self.source = Some(source);
        info!("sACN output started on {} universe {}", interface_ip, self.universe);
        
        Ok(())
    }
    
    /// Stop sACN output
    pub fn stop(&mut self) {
        if let Some(mut source) = self.source.take() {
            // Send all zeros before terminating
            let _ = source.send(&[self.universe], &[0; 512], None, None, None);
            info!("sACN output stopped");
        }
        self.last_values = [0; 512];
    }
    
    /// Check if sACN is active
    pub fn is_active(&self) -> bool {
        self.source.is_some()
    }
    
    /// Set filter mode
    pub fn set_filter_mode(&mut self, mode: SacnFilterMode) {
        let mode_str = format!("{:?}", mode);
        self.filter_mode = mode;
        info!("sACN filter mode set to: {}", mode_str);
    }
    
    /// Get current filter mode
    pub fn get_filter_mode(&self) -> SacnFilterMode {
        self.filter_mode.clone()
    }
    
    /// Send DMX data (filters based on mode and only sends on change)
    pub fn send_dmx(&mut self, dmx_universe: &crate::dmx::DmxUniverse, fixture_ids_900: &[usize]) -> Result<(), String> {
        let source = self.source.as_mut()
            .ok_or_else(|| "sACN not initialized".to_string())?;
        
        let mut buffer = [0u8; 512];
        let mut changed = false;
        
        // Copy DMX data based on filter mode
        match self.filter_mode {
            SacnFilterMode::AllLights => {
                // Send all 512 channels
                for i in 0..512 {
                    let value = dmx_universe.get_channel_raw(i);
                    buffer[i] = value;
                    if value != self.last_values[i] {
                        changed = true;
                    }
                }
            },
            SacnFilterMode::Code900Only => {
                // Only send channels for fixtures with IDs >= 900
                // For 900 series fixtures, copy channels 500-512
                for i in 500..512 {
                    let value = dmx_universe.get_channel_raw(i);
                    buffer[i] = value;
                    if value != self.last_values[i] {
                        changed = true;
                    }
                }
            }
        }
        
        // Only send if values changed
        if changed {
            source.send(&[self.universe], &buffer, None, None, None)
                .map_err(|e| format!("Failed to send sACN data: {}", e))?;
            
            self.last_values = buffer;
        }
        
        Ok(())
    }
}

impl Drop for SacnOutput {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Get list of available network interfaces
pub fn get_network_interfaces() -> Vec<(String, String)> {
    use std::process::Command;
    
    let mut interfaces = Vec::new();
    
    #[cfg(target_os = "macos")]
    {
        // Use ifconfig to get network interfaces on macOS
        if let Ok(output) = Command::new("ifconfig").arg("-a").output() {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                let mut current_interface = String::new();
                
                for line in stdout.lines() {
                    if !line.starts_with('\t') && !line.starts_with(' ') {
                        // New interface
                        if let Some(name) = line.split(':').next() {
                            current_interface = name.to_string();
                        }
                    } else if line.contains("inet ") && !current_interface.is_empty() {
                        // Extract IP address
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if let Some(idx) = parts.iter().position(|&x| x == "inet") {
                            if let Some(ip) = parts.get(idx + 1) {
                                let ip_str = ip.to_string();
                                // Skip localhost
                                if !ip_str.starts_with("127.") {
                                    interfaces.push((current_interface.clone(), ip_str));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        // Use ipconfig on Windows
        if let Ok(output) = Command::new("ipconfig").output() {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                let mut current_interface = String::new();
                
                for line in stdout.lines() {
                    if !line.starts_with(' ') && !line.is_empty() {
                        current_interface = line.trim().to_string();
                    } else if line.contains("IPv4 Address") {
                        if let Some(ip_part) = line.split(':').nth(1) {
                            let ip = ip_part.trim().to_string();
                            if !ip.starts_with("127.") {
                                interfaces.push((current_interface.clone(), ip));
                            }
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        // Use ip addr on Linux
        if let Ok(output) = Command::new("ip").arg("addr").output() {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                let mut current_interface = String::new();
                
                for line in stdout.lines() {
                    if !line.starts_with(' ') {
                        // New interface
                        let parts: Vec<&str> = line.split(':').collect();
                        if parts.len() >= 2 {
                            current_interface = parts[1].trim().to_string();
                        }
                    } else if line.contains("inet ") && !current_interface.is_empty() {
                        // Extract IP address
                        let parts: Vec<&str> = line.trim().split_whitespace().collect();
                        if parts.len() >= 2 {
                            if let Some(ip) = parts[1].split('/').next() {
                                if !ip.starts_with("127.") {
                                    interfaces.push((current_interface.clone(), ip.to_string()));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Fallback if no interfaces found
    if interfaces.is_empty() {
        interfaces.push(("Default".to_string(), "0.0.0.0".to_string()));
    }
    
    interfaces
}
