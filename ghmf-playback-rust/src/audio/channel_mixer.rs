use rodio::{Source, Sample};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// A source that applies independent volume levels to left and right channels
pub struct ChannelMixer<I>
where
    I: Source,
    I::Item: rodio::Sample,
{
    input: I,
    left_volume: Arc<Mutex<f32>>,
    right_volume: Arc<Mutex<f32>>,
    channels: u16,
}

impl<I> ChannelMixer<I>
where
    I: Source,
    I::Item: rodio::Sample,
{
    pub fn new(input: I, left_volume: Arc<Mutex<f32>>, right_volume: Arc<Mutex<f32>>) -> Self {
        let channels = input.channels();
        Self {
            input,
            left_volume,
            right_volume,
            channels,
        }
    }
}

impl<I> Iterator for ChannelMixer<I>
where
    I: Source,
    I::Item: rodio::Sample,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        static mut SAMPLE_INDEX: usize = 0;
        
        self.input.next().map(|sample| {
            // Get current volumes
            let left_vol = *self.left_volume.lock().unwrap();
            let right_vol = *self.right_volume.lock().unwrap();
            
            // For stereo (2 channels), samples alternate: L, R, L, R, ...
            unsafe {
                let is_left = SAMPLE_INDEX % 2 == 0;
                SAMPLE_INDEX += 1;
                
                let volume = if is_left { left_vol } else { right_vol };
                sample.amplify(volume)
            }
        })
    }
}

impl<I> Source for ChannelMixer<I>
where
    I: Source,
    I::Item: rodio::Sample,
{
    fn current_frame_len(&self) -> Option<usize> {
        self.input.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.input.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.input.total_duration()
    }
}
