use super::CommandError;
use std::time::Duration;

/// Represents a single FCW (Fountain Control Word) command
/// Format: AAA-DDD or AAA-DDDDDD
/// AAA: address (0-999)
/// DDD: data (0-999) or DDDDDD: RGB hex color (000000-FFFFFF)
#[derive(Debug, Clone)]
pub struct Command {
    pub address: u32,
    pub data: u32,
    pub is_hex_color: bool,
}

impl Command {
    /// Parse a command string like "500-255" or "500-FF00AA"
    pub fn parse(input: &str) -> Result<Self, CommandError> {
        let parts: Vec<&str> = input.split('-').collect();
        
        if parts.len() != 2 {
            return Err(CommandError::ParseError(
                format!("Expected format AAA-DDD or AAA-DDDDDD, got: {}", input)
            ));
        }

        let address = parts[0].parse::<u32>()
            .map_err(|_| CommandError::ParseError(
                format!("Invalid address: {}", parts[0])
            ))?;

        // If data part is 6 characters, treat as hex color
        let (data, is_hex_color) = if parts[1].len() >= 6 {
            let hex_value = u32::from_str_radix(parts[1], 16)
                .map_err(|_| CommandError::ParseError(
                    format!("Invalid hex color: {}", parts[1])
                ))?;
            (hex_value, true)
        } else {
            let value = parts[1].parse::<u32>()
                .map_err(|_| CommandError::ParseError(
                    format!("Invalid data: {}", parts[1])
                ))?;
            (value, false)
        };

        Ok(Self {
            address,
            data,
            is_hex_color,
        })
    }

    /// Convert hex color data to RGB tuple
    pub fn to_rgb(&self) -> Option<(u8, u8, u8)> {
        if self.is_hex_color {
            let r = ((self.data >> 16) & 0xFF) as u8;
            let g = ((self.data >> 8) & 0xFF) as u8;
            let b = (self.data & 0xFF) as u8;
            Some((r, g, b))
        } else {
            None
        }
    }

    /// Format command as string
    pub fn to_string(&self) -> String {
        if self.is_hex_color {
            format!("{:03}-{:06X}", self.address, self.data)
        } else {
            format!("{:03}-{:03}", self.address, self.data)
        }
    }
}

/// Represents a line of commands at a specific time
/// Format: "1000 500-255 501-128 502-FF00AA"
#[derive(Debug, Clone)]
pub struct CommandLine {
    pub time: Duration,
    pub commands: Vec<Command>,
}

impl CommandLine {
    /// Parse a command line string
    pub fn parse(input: &str) -> Result<Self, CommandError> {
        let input = input.trim();
        
        if input.is_empty() {
            return Err(CommandError::ParseError("Empty command line".to_string()));
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return Err(CommandError::ParseError("No time specified".to_string()));
        }

        // First part is the time in milliseconds
        let time_ms = parts[0].parse::<u64>()
            .map_err(|_| CommandError::ParseError(
                format!("Invalid time: {}", parts[0])
            ))?;
        
        let time = Duration::from_millis(time_ms);

        // Remaining parts are commands
        let commands: Result<Vec<Command>, CommandError> = parts[1..]
            .iter()
            .map(|cmd_str| Command::parse(cmd_str))
            .collect();

        Ok(Self {
            time,
            commands: commands?,
        })
    }

    /// Get time in milliseconds
    pub fn time_ms(&self) -> u64 {
        self.time.as_millis() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numeric_command() {
        let cmd = Command::parse("500-255").unwrap();
        assert_eq!(cmd.address, 500);
        assert_eq!(cmd.data, 255);
        assert!(!cmd.is_hex_color);
    }

    #[test]
    fn test_parse_hex_command() {
        let cmd = Command::parse("500-FF00AA").unwrap();
        assert_eq!(cmd.address, 500);
        assert_eq!(cmd.data, 0xFF00AA);
        assert!(cmd.is_hex_color);
        
        let (r, g, b) = cmd.to_rgb().unwrap();
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 170);
    }

    #[test]
    fn test_parse_command_line() {
        let line = CommandLine::parse("1000 500-255 501-128 502-FF00AA").unwrap();
        assert_eq!(line.time_ms(), 1000);
        assert_eq!(line.commands.len(), 3);
        
        assert_eq!(line.commands[0].address, 500);
        assert_eq!(line.commands[1].address, 501);
        assert_eq!(line.commands[2].address, 502);
    }

    #[test]
    fn test_invalid_command() {
        assert!(Command::parse("invalid").is_err());
        assert!(Command::parse("500").is_err());
        assert!(Command::parse("500-").is_err());
    }
}
