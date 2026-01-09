use super::{CommandFile, CommandLine};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, info};

/// Executes commands synchronized with audio playback
pub struct CommandExecutor {
    command_file: CommandFile,
    start_time: Option<Instant>,
    current_index: usize,
}

impl CommandExecutor {
    pub fn new(command_file: CommandFile) -> Self {
        Self {
            command_file,
            start_time: None,
            current_index: 0,
        }
    }

    /// Start executing commands
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.current_index = 0;
        info!("Command execution started");
    }

    /// Stop executing commands
    pub fn stop(&mut self) {
        self.start_time = None;
        self.current_index = 0;
        info!("Command execution stopped");
    }

    /// Reset to beginning
    pub fn reset(&mut self) {
        self.current_index = 0;
        self.command_file.reset();
    }

    /// Get elapsed time since start
    pub fn elapsed(&self) -> Duration {
        self.start_time
            .map(|t| t.elapsed())
            .unwrap_or(Duration::from_secs(0))
    }

    /// Check if there are more commands to execute
    pub fn has_more(&self) -> bool {
        self.current_index < self.command_file.commands.len()
    }

    /// Get the next command that should be executed based on current time
    pub fn get_next_command(&mut self) -> Option<&CommandLine> {
        if !self.has_more() {
            return None;
        }

        let elapsed = self.elapsed();
        let next_cmd = &self.command_file.commands[self.current_index];

        if next_cmd.time <= elapsed {
            self.current_index += 1;
            debug!("Executing command at {} ms", next_cmd.time_ms());
            Some(next_cmd)
        } else {
            None
        }
    }

    /// Async execution loop (call this in a tokio task)
    pub async fn run_async<F>(&mut self, mut callback: F)
    where
        F: FnMut(&CommandLine),
    {
        self.start();

        while self.has_more() {
            if let Some(cmd_line) = self.get_next_command() {
                callback(cmd_line);
            } else {
                // Sleep for a short interval before checking again
                sleep(Duration::from_millis(1)).await;
            }
        }

        info!("Command execution completed");
    }

    /// Get progress percentage (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        if self.command_file.commands.is_empty() {
            return 1.0;
        }

        let elapsed = self.elapsed();
        let total = self.command_file.total_duration();

        if total.as_millis() == 0 {
            return 1.0;
        }

        (elapsed.as_millis() as f32 / total.as_millis() as f32).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_basic() {
        let content = "0 500-255\n1000 501-128\n2000 502-64";
        let file = CommandFile::parse(content).unwrap();
        let mut executor = CommandExecutor::new(file);

        assert!(!executor.has_more()); // Not started yet
        
        executor.start();
        assert!(executor.has_more());
    }
}
