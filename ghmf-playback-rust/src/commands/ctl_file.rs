use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::{Context, Result};

/// A single FCW command at a specific time
#[derive(Debug, Clone)]
pub struct CtlCommand {
    pub time_ms: u64,
    pub fcw_address: u16,
    pub data: u16,
    pub is_hex_color: bool,
    pub hex_color: Option<String>,
}

impl CtlCommand {
    /// Parse command like "051-008" or "051-FF00AA"
    pub fn parse(time_ms: u64, command_str: &str) -> Result<Self> {
        let parts: Vec<&str> = command_str.split('-').collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid command format: {}", command_str);
        }
        
        let fcw_address: u16 = parts[0].parse()
            .context(format!("Invalid FCW address: {}", parts[0]))?;
        
        // Check if data is hex color (6 characters) or numeric (1-3 digits)
        let data_str = parts[1];
        if data_str.len() == 6 && data_str.chars().all(|c| c.is_ascii_hexdigit()) {
            // Hex color format
            Ok(Self {
                time_ms,
                fcw_address,
                data: 0,
                is_hex_color: true,
                hex_color: Some(data_str.to_uppercase()),
            })
        } else {
            // Numeric format
            let data: u16 = data_str.parse()
                .context(format!("Invalid data value: {}", data_str))?;
            Ok(Self {
                time_ms,
                fcw_address,
                data,
                is_hex_color: false,
                hex_color: None,
            })
        }
    }
}

/// A line in the CTL file with timestamp and commands
#[derive(Debug, Clone)]
pub struct CtlLine {
    pub time_ms: u64,
    pub commands: Vec<CtlCommand>,
    pub is_blank: bool,
}

/// Complete CTL file with all commands
#[derive(Debug, Clone)]
pub struct CtlFile {
    pub version: String,
    pub lines: Vec<CtlLine>,
    pub total_duration_ms: u64,
}

impl CtlFile {
    /// Load and parse a CTL file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path.as_ref())
            .context(format!("Failed to open CTL file: {:?}", path.as_ref()))?;
        let reader = BufReader::new(file);
        
        let mut version = String::new();
        let mut lines = Vec::new();
        let mut max_time = 0u64;
        
        for (line_num, line_result) in reader.lines().enumerate() {
            let line = line_result?;
            let line = line.trim();
            
            // Skip empty lines
            if line.is_empty() {
                continue;
            }
            
            // Skip lines containing /b (case insensitive)
            if line.to_lowercase().contains("/b") {
                continue;
            }
            
            // First line is version header - skip it
            if line_num == 0 || line.contains("TIME") || line.contains("time") {
                version = line.to_string();
                continue;
            }
            
            // Parse timestamp and commands
            if let Some(space_pos) = line.find(' ') {
                let time_str = &line[..space_pos];
                let commands_str = &line[space_pos + 1..];
                
                // Parse time in format "MM:SS.T" where T is tenths of a second
                let time_ms = Self::parse_time(time_str)?;
                max_time = max_time.max(time_ms);
                
                // Check for blank command (\B)
                if commands_str.trim() == "(\\B)" {
                    lines.push(CtlLine {
                        time_ms,
                        commands: Vec::new(),
                        is_blank: true,
                    });
                    continue;
                }
                
                // Parse commands
                let mut commands = Vec::new();
                for cmd_str in commands_str.split_whitespace() {
                    // Skip comments and blank markers
                    if cmd_str.starts_with('(') || cmd_str.starts_with('\\') {
                        continue;
                    }
                    
                    match CtlCommand::parse(time_ms, cmd_str) {
                        Ok(cmd) => commands.push(cmd),
                        Err(e) => {
                            tracing::warn!("Failed to parse command '{}': {}", cmd_str, e);
                        }
                    }
                }
                
                if !commands.is_empty() {
                    lines.push(CtlLine {
                        time_ms,
                        commands,
                        is_blank: false,
                    });
                }
            }
        }
        
        tracing::info!("Loaded CTL file with {} command lines, duration: {}ms", 
            lines.len(), max_time);
        
        Ok(Self {
            version,
            lines,
            total_duration_ms: max_time,
        })
    }
    
    /// Parse time string "MM:SS.T" to milliseconds
    fn parse_time(time_str: &str) -> Result<u64> {
        let parts: Vec<&str> = time_str.split(':').collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid time format: {}", time_str);
        }
        
        let minutes: u64 = parts[0].parse()
            .context("Invalid minutes")?;
        
        let sec_parts: Vec<&str> = parts[1].split('.').collect();
        if sec_parts.len() != 2 {
            anyhow::bail!("Invalid seconds format: {}", parts[1]);
        }
        
        let seconds: u64 = sec_parts[0].parse()
            .context("Invalid seconds")?;
        let tenths: u64 = sec_parts[1].parse()
            .context("Invalid tenths")?;
        
        Ok(minutes * 60 * 1000 + seconds * 1000 + tenths * 100)
    }
    
    /// Get commands that should execute at a specific time (within 50ms window)
    pub fn get_commands_at_time(&self, time_ms: u64) -> Vec<&CtlCommand> {
        let mut commands = Vec::new();
        
        for line in &self.lines {
            // Match commands within 50ms window (accounting for timing variations)
            if line.time_ms >= time_ms.saturating_sub(25) && 
               line.time_ms <= time_ms + 25 {
                if !line.is_blank {
                    commands.extend(line.commands.iter());
                }
            }
        }
        
        commands
    }
    
    /// Get all command lines between two times
    pub fn get_lines_in_range(&self, start_ms: u64, end_ms: u64) -> Vec<&CtlLine> {
        self.lines.iter()
            .filter(|line| line.time_ms >= start_ms && line.time_ms < end_ms)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_time() {
        assert_eq!(CtlFile::parse_time("00:00.0").unwrap(), 0);
        assert_eq!(CtlFile::parse_time("00:01.5").unwrap(), 1500);
        assert_eq!(CtlFile::parse_time("01:30.7").unwrap(), 90700);
    }
    
    #[test]
    fn test_parse_command() {
        let cmd = CtlCommand::parse(1000, "051-008").unwrap();
        assert_eq!(cmd.fcw_address, 51);
        assert_eq!(cmd.data, 8);
        assert!(!cmd.is_hex_color);
        
        let hex_cmd = CtlCommand::parse(1000, "051-FF00AA").unwrap();
        assert_eq!(hex_cmd.fcw_address, 51);
        assert!(hex_cmd.is_hex_color);
        assert_eq!(hex_cmd.hex_color.as_ref().unwrap(), "FF00AA");
    }
}
