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
            current_volume: Arc::new(Mutex::new(1.0)),
            start_time: Arc::new(Mutex::new(None)),
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

        // Add source and play
        sink.append(source);
        sink.play();

        // Store the sink and start time
        *self.sink.lock().unwrap() = Some(sink);
        *self.start_time.lock().unwrap() = Some(std::time::Instant::now());

        debug!("Audio playback started");
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
            sink.pause();
            debug!("Audio paused");
        }
    }

    pub fn resume(&self) {
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
            sink.play();
            debug!("Audio resumed");
        }
    }

    pub fn stop(&self) {
        if let Some(sink) = self.sink.lock().unwrap().take() {
            sink.stop();
            *self.start_time.lock().unwrap() = None;
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

    pub fn get_position(&self) -> Duration {
        if let Some(start) = *self.start_time.lock().unwrap() {
            start.elapsed()
        } else {
            Duration::from_secs(0)
        }
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        self.stop();
    }
}
