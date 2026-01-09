/// DMX channel type enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelType {
    Undefined,
    Red,
    Green,
    Blue,
    Amber,
    White,
    Raw,
    Dmx,
}

/// Represents a single DMX channel
#[derive(Debug, Clone)]
pub struct Channel {
    pub index: usize,
    pub channel_type: ChannelType,
    pub value: u8,
}

impl Channel {
    pub fn new(index: usize, channel_type: ChannelType) -> Self {
        Self {
            index,
            channel_type,
            value: 0,
        }
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }
}
