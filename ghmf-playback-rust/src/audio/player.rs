use super::AudioError;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::{debug, info};

pub struct AudioPlayer {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Arc<Mutex<Option<Sink>>>,
    current_volume: Arc<Mutex<f32>>,
    start_time: Arc<Mutex<Option<std::time::Instant>>>,
    pause_time: Arc<Mutex<Option<std::time::Instant>>>,
    accumulated_time: Arc<Mutex<Duration>>,
    current_file: Arc<Mutex<Option<String>>>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, AudioError> {
        info!("Initializing audio player");
        
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| AudioError::DeviceError(e.to_string()))?;
        
        Ok(Self {
            _stream: stream,
            stream_handle,
            sink: Arc::new(Mutex::new(None)),
            current_volume: Arc::new(Mutex::new(0.35)),  // Default 35%
            start_time: Arc::new(Mutex::new(None)),
            pause_time: Arc::new(Mutex::new(None)),
            accumulated_time: Arc::new(Mutex::new(Duration::from_secs(0))),
            current_file: Arc::new(Mutex::new(None)),
        })
    }

    pub fn play(&self, path: &str) -> Result<(), AudioError> {
        info!("Playing audio file: {}", path);
        
        if !std::path::Path::new(path).exists() {
            return Err(AudioError::FileNotFound(path.to_string()));
        }

        // Stop any current playback
        self.stop();

        // Open and decode the file
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new(file)
            .map_err(|e| AudioError::DecodeError(e.to_string()))?;

        // Create a new sink
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| AudioError::DeviceError(e.to_string()))?;

        // Apply current volume
        let volume = *self.current_volume.lock().unwrap();
        sink.set_volume(volume);

        // Add source but START PAUSED - don't auto-play
        sink.append(source);
        sink.pause();  // Start paused by default

        // Store the sink and reset time tracking
        *self.sink.lock().unwrap() = Some(sink);
        *self.start_time.lock().unwrap() = None;  // Not playing yet
        *self.pause_time.lock().unwrap() = None;
        *self.accumulated_time.lock().unwrap() = Duration::from_secs(0);
        *self.current_file.lock().unwrap() = Some(path.to_string());

        debug!("Audio loaded (paused, ready to play)");
        Ok(())
    }

    pub fn play_blank(&self, duration_ms: u64) -> Result<(), AudioError> {
        info!("Playing blank audio for {} ms", duration_ms);
        
        // Stop any current playback
        self.stop();

        // Create silence source
        let source = rodio::source::SineWave::new(440.0)
            .amplify(0.0)
            .take_duration(Duration::from_millis(duration_ms));

        // Create a new sink
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| AudioError::DeviceError(e.to_string()))?;

        sink.append(source);
        sink.play();

        *self.sink.lock().unwrap() = Some(sink);
        *self.start_time.lock().unwrap() = Some(std::time::Instant::now());

        Ok(())
    }

    pub fn pause(&self) {
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
            if !sink.is_paused() {
                // Accumulate time before pausing
                if let Some(start) = *self.start_time.lock().unwrap() {
                    let mut acc = self.accumulated_time.lock().unwrap();
                    *acc += start.elapsed();
                }
                *self.start_time.lock().unwrap() = None;
                *self.pause_time.lock().unwrap() = Some(std::time::Instant::now());
                sink.pause();
                debug!("Audio paused");
            }
        }
    }

    pub fn resume(&self) {
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
            if sink.is_paused() {
                *self.start_time.lock().unwrap() = Some(std::time::Instant::now());
                *self.pause_time.lock().unwrap() = None;
                sink.play();
                debug!("Audio resumed");
            }
        }
    }
    
    pub fn seek(&self, position: Duration) -> Result<(), AudioError> {
        // Get the current file path
        let file_path = if let Some(path) = self.current_file.lock().unwrap().clone() {
            path
        } else {
            return Err(AudioError::DeviceError("No file loaded".to_string()));
        };
        
        // Remember if we were playing
        let was_playing = self.is_playing();
        
        // Stop current playback
        if let Some(sink) = self.sink.lock().unwrap().take() {
            sink.stop();
        }
        
        // Reload the file
        let file = BufReader::new(File::open(&file_path)?);
        let source = Decoder::new(file)
            .map_err(|e| AudioError::DecodeError(e.to_string()))?;
        
        // Skip to the desired position - this may skip past the end if position is too large
        let source = source.skip_duration(position);
        
        // Create a new sink
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| AudioError::DeviceError(e.to_string()))?;
        
        // Apply current volume
        let volume = *self.current_volume.lock().unwrap();
        sink.set_volume(volume);
        
        // Add the source - if skip went past the end, this will be empty
        sink.append(source);
        
        // Check if the sink is already empty (seeked past the end)
        let is_empty = sink.empty();
        
        // If we were playing, resume; otherwise pause
        if was_playing {
            if is_empty {
                // Seeked past the end - just set to end position and pause
                *self.accumulated_time.lock().unwrap() = position;
                *self.start_time.lock().unwrap() = None;
                sink.pause();
            } else {
                sink.play();
                // Set accumulated time and reset start time atomically
                *self.accumulated_time.lock().unwrap() = position;
                *self.start_time.lock().unwrap() = Some(std::time::Instant::now());
                *self.pause_time.lock().unwrap() = None;
            }
        } else {
            sink.pause();
            *self.accumulated_time.lock().unwrap() = position;
            *self.start_time.lock().unwrap() = None;
        }
        
        // Store the new sink
        *self.sink.lock().unwrap() = Some(sink);
        
        debug!("Seeked to position: {:?}", position);
        Ok(())
    }

    pub fn stop(&self) {
        if let Some(sink) = self.sink.lock().unwrap().take() {
            sink.stop();
            *self.start_time.lock().unwrap() = None;
            *self.pause_time.lock().unwrap() = None;
            *self.accumulated_time.lock().unwrap() = Duration::from_secs(0);
            debug!("Audio stopped");
        }
    }

    pub fn set_volume(&self, volume: f32) {
        let volume = volume.clamp(0.0, 1.0);
        *self.current_volume.lock().unwrap() = volume;
        
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
            sink.set_volume(volume);
        }
    }

    pub fn get_volume(&self) -> f32 {
        *self.current_volume.lock().unwrap()
    }

    pub fn is_playing(&self) -> bool {
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
            !sink.is_paused() && !sink.empty()
        } else {
            false
        }
    }

    pub fn is_paused(&self) -> bool {
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
            sink.is_paused()
        } else {
            false
        }
    }
    
    pub fn is_finished(&self) -> bool {
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
            sink.empty()
        } else {
            true
        }
    }

    pub fn get_position(&self) -> Duration {
        let accumulated = *self.accumulated_time.lock().unwrap();
        
        if let Some(start) = *self.start_time.lock().unwrap() {
            // Currently playing - add elapsed time since resume
            accumulated + start.elapsed()
        } else {
            // Paused or stopped - return accumulated time only
            accumulated
        }
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        self.stop();
    }
}
