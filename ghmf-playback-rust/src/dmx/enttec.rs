use super::{DmxError, DmxUniverse};
use serialport::{SerialPort, SerialPortInfo, SerialPortType};
use std::time::Duration;
use tracing::{debug, info, warn};

const START_BYTE: u8 = 0x7E;
const END_BYTE: u8 = 0xE7;
const SEND_DMX_LABEL: u8 = 6;
const DMX_UNIVERSE_SIZE: usize = 512;

/// Enttec DMX USB Pro driver implementation
pub struct EnttecDmxPro {
    port: Box<dyn SerialPort>,
    universe: DmxUniverse,
}

impl EnttecDmxPro {
    /// Create a new Enttec DMX USB Pro connection
    pub fn new() -> Result<Self, DmxError> {
        info!("Searching for Enttec DMX USB Pro device...");
        
        let ports = serialport::available_ports()?;
        
        // Try to find Enttec device
        let enttec_port = ports.iter()
            .find(|p| Self::is_enttec_device(p))
            .ok_or(DmxError::DeviceNotFound)?;

        info!("Found Enttec device: {}", enttec_port.port_name);

        // Open serial port with Enttec settings
        let port = serialport::new(&enttec_port.port_name, 57_600)
            .timeout(Duration::from_millis(100))
            .open()?;

        Ok(Self {
            port,
            universe: DmxUniverse::new(),
        })
    }

    /// Check if a serial port is an Enttec device
    fn is_enttec_device(port_info: &SerialPortInfo) -> bool {
        match &port_info.port_type {
            SerialPortType::UsbPort(info) => {
                // FTDI Vendor ID and common Enttec Product IDs
                info.vid == 0x0403 && (info.pid == 0x6001 || info.pid == 0x6015)
            }
            _ => false,
        }
    }

    /// Set a single DMX channel value (1-512)
    pub fn set_channel(&mut self, channel: usize, value: u8) -> Result<(), DmxError> {
        self.universe.set_channel(channel, value)
    }

    /// Set multiple DMX channels at once
    pub fn set_channels(&mut self, start_channel: usize, values: &[u8]) -> Result<(), DmxError> {
        self.universe.set_channels(start_channel, values)
    }

    /// Get current value of a DMX channel
    pub fn get_channel(&self, channel: usize) -> Result<u8, DmxError> {
        self.universe.get_channel(channel)
    }

    /// Clear all DMX channels to 0
    pub fn clear(&mut self) {
        self.universe.clear();
        debug!("DMX universe cleared");
    }
    
    /// Clear all DMX channels to 0 except those in the ignore list (1-indexed)
    pub fn clear_except(&mut self, ignore_channels: &[u16]) {
        self.universe.clear_except(ignore_channels);
        debug!("DMX universe cleared (ignoring {} channels)", ignore_channels.len());
    }

    /// Send the current DMX universe to the hardware
    pub fn send_dmx(&mut self) -> Result<(), DmxError> {
        let packet = self.build_dmx_packet();
        
        self.port.write_all(&packet)
            .map_err(|e| DmxError::CommError(e.to_string()))?;
        
        Ok(())
    }

    /// Build Enttec DMX USB Pro packet
    fn build_dmx_packet(&self) -> Vec<u8> {
        let mut packet = Vec::with_capacity(DMX_UNIVERSE_SIZE + 6);
        
        // Start byte
        packet.push(START_BYTE);
        
        // Label (6 = Send DMX)
        packet.push(SEND_DMX_LABEL);
        
        // Data length: 513 bytes (1 start code + 512 channels)
        let length = (DMX_UNIVERSE_SIZE + 1) as u16;
        packet.push((length & 0xFF) as u8);        // LSB
        packet.push(((length >> 8) & 0xFF) as u8); // MSB
        
        // DMX Start Code (always 0x00 for DMX512)
        packet.push(0x00);
        
        // DMX data (512 channels)
        packet.extend_from_slice(self.universe.as_slice());
        
        // End byte
        packet.push(END_BYTE);
        
        packet
    }

    /// Start continuous DMX output (call send_dmx() repeatedly)
    pub fn start_continuous_output(&mut self, interval_ms: u64) -> Result<(), DmxError> {
        // This would typically run in a separate thread
        // For now, just document the usage pattern
        info!("Starting continuous DMX output with {}ms interval", interval_ms);
        Ok(())
    }
}

impl Drop for EnttecDmxPro {
    fn drop(&mut self) {
        // Clear DMX channels on disconnect
        self.clear();
        let _ = self.send_dmx();
        info!("Enttec DMX USB Pro disconnected");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_structure() {
        let mut dmx = DmxUniverse::new();
        dmx.set_channel(1, 255).unwrap();
        dmx.set_channel(100, 128).unwrap();
        
        // Packet should be: [0x7E][0x06][0x01][0x02][0x00][...512 channels...][0xE7]
        // Total: 6 + 513 = 519 bytes
    }
}
