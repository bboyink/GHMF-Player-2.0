use super::DmxError;

const DMX_UNIVERSE_SIZE: usize = 512;

/// Represents a DMX512 universe (512 channels)
pub struct DmxUniverse {
    channels: [u8; DMX_UNIVERSE_SIZE],
}

impl DmxUniverse {
    pub fn new() -> Self {
        Self {
            channels: [0u8; DMX_UNIVERSE_SIZE],
        }
    }

    /// Set a single channel value (1-indexed, 1-512)
    pub fn set_channel(&mut self, channel: usize, value: u8) -> Result<(), DmxError> {
        if channel == 0 || channel > DMX_UNIVERSE_SIZE {
            return Err(DmxError::InvalidChannel(channel));
        }
        self.channels[channel - 1] = value;
        Ok(())
    }

    /// Set multiple consecutive channels (1-indexed)
    pub fn set_channels(&mut self, start_channel: usize, values: &[u8]) -> Result<(), DmxError> {
        if start_channel == 0 || start_channel > DMX_UNIVERSE_SIZE {
            return Err(DmxError::InvalidChannel(start_channel));
        }

        let end_channel = start_channel + values.len() - 1;
        if end_channel > DMX_UNIVERSE_SIZE {
            return Err(DmxError::InvalidChannel(end_channel));
        }

        for (i, &value) in values.iter().enumerate() {
            self.channels[start_channel - 1 + i] = value;
        }

        Ok(())
    }

    /// Get a channel value (1-indexed)
    pub fn get_channel(&self, channel: usize) -> Result<u8, DmxError> {
        if channel == 0 || channel > DMX_UNIVERSE_SIZE {
            return Err(DmxError::InvalidChannel(channel));
        }
        Ok(self.channels[channel - 1])
    }

    /// Clear all channels to 0
    pub fn clear(&mut self) {
        self.channels.fill(0);
    }
    
    /// Clear all channels to 0 except those in the ignore list (1-indexed)
    pub fn clear_except(&mut self, ignore_channels: &[u16]) {
        for (idx, channel) in self.channels.iter_mut().enumerate() {
            let channel_num = (idx + 1) as u16;
            if !ignore_channels.contains(&channel_num) {
                *channel = 0;
            }
        }
    }

    /// Get the raw channel array
    pub fn as_slice(&self) -> &[u8] {
        &self.channels
    }

    /// Get a mutable reference to the raw channel array
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.channels
    }
}

impl Default for DmxUniverse {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_channel() {
        let mut universe = DmxUniverse::new();
        
        universe.set_channel(1, 255).unwrap();
        assert_eq!(universe.get_channel(1).unwrap(), 255);
        
        universe.set_channel(512, 128).unwrap();
        assert_eq!(universe.get_channel(512).unwrap(), 128);
    }

    #[test]
    fn test_invalid_channels() {
        let mut universe = DmxUniverse::new();
        
        assert!(universe.set_channel(0, 100).is_err());
        assert!(universe.set_channel(513, 100).is_err());
        assert!(universe.get_channel(0).is_err());
        assert!(universe.get_channel(513).is_err());
    }

    #[test]
    fn test_set_multiple_channels() {
        let mut universe = DmxUniverse::new();
        let values = [100, 150, 200];
        
        universe.set_channels(1, &values).unwrap();
        
        assert_eq!(universe.get_channel(1).unwrap(), 100);
        assert_eq!(universe.get_channel(2).unwrap(), 150);
        assert_eq!(universe.get_channel(3).unwrap(), 200);
    }

    #[test]
    fn test_clear() {
        let mut universe = DmxUniverse::new();
        
        universe.set_channel(1, 255).unwrap();
        universe.set_channel(512, 255).unwrap();
        
        universe.clear();
        
        assert_eq!(universe.get_channel(1).unwrap(), 0);
        assert_eq!(universe.get_channel(512).unwrap(), 0);
    }
}
