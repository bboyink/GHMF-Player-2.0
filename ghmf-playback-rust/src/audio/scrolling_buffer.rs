use std::sync::{Arc, RwLock};

/// Optimized circular buffer for real-time audio waveform visualization
/// Inspired by Scrollscope's buffer implementation
#[derive(Clone)]
pub struct ScrollingWaveformBuffer {
    /// The actual sample data (RMS values 0.0 to 1.0)
    samples: Arc<RwLock<Vec<f32>>>,
    /// Current write position in the buffer
    write_index: Arc<RwLock<usize>>,
    /// How many samples are currently valid/visible
    visible_length: Arc<RwLock<usize>>,
    /// Maximum buffer capacity
    capacity: usize,
}

impl ScrollingWaveformBuffer {
    /// Create a new scrolling buffer with specified capacity
    /// capacity: Maximum number of samples to store
    pub fn new(capacity: usize) -> Self {
        Self {
            samples: Arc::new(RwLock::new(vec![0.0; capacity])),
            write_index: Arc::new(RwLock::new(0)),
            visible_length: Arc::new(RwLock::new(capacity)),
            capacity,
        }
    }
    
    /// Update the visible window length (for scrolling display)
    /// This controls how many samples are shown at once
    pub fn set_visible_length(&self, length: usize) {
        let clamped = length.min(self.capacity);
        *self.visible_length.write().unwrap() = clamped;
    }
    
    /// Get current visible length
    pub fn visible_length(&self) -> usize {
        *self.visible_length.read().unwrap()
    }
    
    /// Push a single sample into the buffer (circular)
    pub fn push_sample(&self, sample: f32) {
        let mut samples = self.samples.write().unwrap();
        let mut write_idx = self.write_index.write().unwrap();
        let visible_len = *self.visible_length.read().unwrap();
        
        samples[*write_idx] = sample.clamp(0.0, 1.0);
        *write_idx = (*write_idx + 1) % visible_len;
    }
    
    /// Push multiple samples efficiently (batch operation)
    pub fn push_samples(&self, new_samples: &[f32]) {
        let mut samples = self.samples.write().unwrap();
        let mut write_idx = self.write_index.write().unwrap();
        let visible_len = *self.visible_length.read().unwrap();
        
        for &sample in new_samples {
            samples[*write_idx] = sample.clamp(0.0, 1.0);
            *write_idx = (*write_idx + 1) % visible_len;
        }
    }
    
    /// Get all visible samples in chronological order
    /// Returns a Vec with oldest sample first, newest last
    pub fn get_samples(&self) -> Vec<f32> {
        let samples = self.samples.read().unwrap();
        let write_idx = *self.write_index.read().unwrap();
        let visible_len = *self.visible_length.read().unwrap();
        
        let mut result = Vec::with_capacity(visible_len);
        
        // Read from write_idx (oldest) to end of buffer
        for i in write_idx..visible_len {
            result.push(samples[i]);
        }
        
        // Read from start of buffer to write_idx (newest)
        for i in 0..write_idx {
            result.push(samples[i]);
        }
        
        result
    }
    
    /// Get samples as a linear slice for a specific time window
    /// start_ratio: 0.0 to 1.0, where in the buffer to start
    /// length: how many samples to return
    pub fn get_window(&self, start_ratio: f32, length: usize) -> Vec<f32> {
        let samples = self.samples.read().unwrap();
        let visible_len = *self.visible_length.read().unwrap();
        
        let start_idx = ((start_ratio.clamp(0.0, 1.0) * visible_len as f32) as usize).min(visible_len);
        let end_idx = (start_idx + length).min(visible_len);
        
        samples[start_idx..end_idx].to_vec()
    }
    
    /// Clear the buffer (fill with zeros)
    pub fn clear(&self) {
        let mut samples = self.samples.write().unwrap();
        samples.fill(0.0);
        *self.write_index.write().unwrap() = 0;
    }
    
    /// Get buffer capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

/// Builder for creating ScrollingWaveformBuffer from existing waveform data
pub struct BufferBuilder {
    source_samples: Vec<f32>,
    buffer_samples_per_second: usize,
    duration_secs: f32,
}

impl BufferBuilder {
    /// Create a builder from existing waveform data
    pub fn from_waveform(samples: Vec<f32>, duration_secs: f32) -> Self {
        // Calculate appropriate buffer sample rate for smooth scrolling
        // We want enough samples for smooth display but not too many
        let target_samples = samples.len();
        let samples_per_second = (target_samples as f32 / duration_secs) as usize;
        
        Self {
            source_samples: samples,
            buffer_samples_per_second: samples_per_second.max(100), // At least 100 samples/sec
            duration_secs,
        }
    }
    
    /// Build a scrolling buffer with specified visible window duration
    /// window_duration: How many seconds of audio to display at once (e.g., 7.0)
    pub fn build(&self, window_duration: f32) -> ScrollingWaveformBuffer {
        let window_samples = (self.buffer_samples_per_second as f32 * window_duration) as usize;
        let total_capacity = (self.buffer_samples_per_second as f32 * self.duration_secs) as usize;
        let capacity = total_capacity.max(window_samples * 2); // Extra capacity for smooth scrolling
        
        let buffer = ScrollingWaveformBuffer::new(capacity);
        buffer.set_visible_length(window_samples);
        
        // Pre-fill buffer with downsampled waveform data
        if !self.source_samples.is_empty() {
            let downsample_ratio = self.source_samples.len() as f32 / capacity as f32;
            
            let mut downsampled = Vec::with_capacity(capacity);
            for i in 0..capacity {
                let source_idx = (i as f32 * downsample_ratio) as usize;
                let sample = self.source_samples.get(source_idx).copied().unwrap_or(0.0);
                downsampled.push(sample);
            }
            
            buffer.push_samples(&downsampled);
        }
        
        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_buffer_creation() {
        let buffer = ScrollingWaveformBuffer::new(1000);
        assert_eq!(buffer.capacity(), 1000);
        assert_eq!(buffer.visible_length(), 1000);
    }
    
    #[test]
    fn test_push_and_get() {
        let buffer = ScrollingWaveformBuffer::new(10);
        buffer.push_sample(0.5);
        buffer.push_sample(0.8);
        
        let samples = buffer.get_samples();
        assert_eq!(samples.len(), 10);
        // Check that samples were written
        assert!(samples.iter().any(|&s| s > 0.0));
    }
    
    #[test]
    fn test_circular_behavior() {
        let buffer = ScrollingWaveformBuffer::new(5);
        buffer.set_visible_length(5);
        
        // Fill buffer completely
        for i in 0..10 {
            buffer.push_sample((i as f32) / 10.0);
        }
        
        let samples = buffer.get_samples();
        assert_eq!(samples.len(), 5);
    }
}
