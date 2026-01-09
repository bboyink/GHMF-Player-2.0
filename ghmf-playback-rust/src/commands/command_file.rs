use super::{CommandError, CommandLine};
use std::fs;
use std::path::Path;
use tracing::{debug, info};

/// Represents a parsed command file (FCW script)
pub struct CommandFile {
    pub commands: Vec<CommandLine>,
    current_index: usize,
}

impl CommandFile {
    /// Load and parse a command file
    pub fn load(path: &str) -> Result<Self, CommandError> {
        info!("Loading command file: {}", path);
        
        if !Path::new(path).exists() {
            return Err(CommandError::FileNotFound(path.to_string()));
        }

        let content = fs::read_to_string(path)?;
        Self::parse(&content)
    }

    /// Parse command file content
    pub fn parse(content: &str) -> Result<Self, CommandError> {
        let mut commands = Vec::new();
        let mut needs_remap = false;

        // Check if this is an old Java choreographer script
        for line in content.lines() {
            let line = line.trim();
            
            if line.to_lowercase().contains("created with ghmf") {
                needs_remap = true;
                debug!("Detected old Java choreographer script, will remap fixtures");
            }

            // Skip empty lines and comments
            if line.is_empty() || !line.chars().next().unwrap_or(' ').is_ascii_digit() {
                continue;
            }

            // Parse and optionally remap the command line
            let line_str = if needs_remap {
                Self::remap_fixtures(line)
            } else {
                line.to_string()
            };

            match CommandLine::parse(&line_str) {
                Ok(cmd_line) => commands.push(cmd_line),
                Err(e) => {
                    debug!("Skipping invalid command line '{}': {}", line, e);
                    // Continue parsing, don't fail on invalid lines
                }
            }
        }

        // Sort by time (just in case they're out of order)
        commands.sort_by_key(|c| c.time);

        info!("Loaded {} command lines", commands.len());

        Ok(Self {
            commands,
            current_index: 0,
        })
    }

    /// Remap fixture addresses for old Java choreographer scripts
    fn remap_fixtures(line: &str) -> String {
        // Fixture remapping for modules 1-7
        const FIXTURES: [usize; 53] = [
            1, 2, 3, 4, 6,      // mod 1: 1-5
            7, 8, 9, 10, 12,    // mod 2: 6-10
            13, 14, 15, 16, 18, // mod 3: 11-15
            19, 20, 21, 22, 23, // mod 4: 16-20
            24, 25, 26, 27, 29, // mod 5: 21-25
            30, 31, 32, 33, 35, // mod 6: 26-30
            36, 37, 38, 39, 41, // mod 7: 31-35
            42, 43, 44, 45, 46, 47, // peacock front: 36-41
            50, 51,             // spout: 42-43
            52, 53,             // doves: 44-45
            5, 11, 17, 28, 34, 40, // mod back center
            48, 49              // peacock back
        ];

        let mut result = line.to_string();

        // Remap 500s and 600s series
        for base in [500, 600] {
            for (j, &fixture) in FIXTURES.iter().enumerate().take(53).skip(0) {
                let old = format!("{}-", base + j + 1);
                let prefix = if base == 500 { "5X" } else { "6X" };
                let new = format!("{}{:02}-", prefix, fixture);
                result = result.replace(&old, &new);
            }
        }

        // Remove temporary X markers
        result.replace('X', "")
    }

    /// Get the current command line
    pub fn current(&self) -> Option<&CommandLine> {
        self.commands.get(self.current_index)
    }

    /// Advance to the next command line
    pub fn next(&mut self) -> Option<&CommandLine> {
        self.current_index += 1;
        self.current()
    }

    /// Reset to the beginning
    pub fn reset(&mut self) {
        self.current_index = 0;
    }

    /// Get all command lines
    pub fn all(&self) -> &[CommandLine] {
        &self.commands
    }

    /// Get the total duration of the command file
    pub fn total_duration(&self) -> std::time::Duration {
        self.commands
            .last()
            .map(|c| c.time)
            .unwrap_or(std::time::Duration::from_secs(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_file() {
        let content = r#"
            0 500-255 501-128
            1000 500-FF0000 501-00FF00
            2000 500-0 501-0
        "#;

        let file = CommandFile::parse(content).unwrap();
        assert_eq!(file.commands.len(), 3);
        assert_eq!(file.commands[0].time_ms(), 0);
        assert_eq!(file.commands[1].time_ms(), 1000);
        assert_eq!(file.commands[2].time_ms(), 2000);
    }
}
