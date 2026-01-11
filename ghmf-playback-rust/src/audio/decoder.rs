use super::AudioError;
use std::path::Path;
use std::fs::File;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;

pub struct AudioDecoder;

impl AudioDecoder {
    pub fn get_duration(path: &str) -> Result<std::time::Duration, AudioError> {
        if !Path::new(path).exists() {
            return Err(AudioError::FileNotFound(path.to_string()));
        }

        // Open the media source
        let file = File::open(path).map_err(|e| AudioError::DecoderError(e.to_string()))?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        // Create a hint to help the format registry
        let mut hint = Hint::new();
        if let Some(ext) = Path::new(path).extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        }

        // Probe the media source
        let mut probed = symphonia::default::get_probe()
            .format(&hint, mss, &Default::default(), &Default::default())
            .map_err(|e| AudioError::DecoderError(e.to_string()))?;

        // Get the default track
        let track = probed.format.default_track()
            .ok_or_else(|| AudioError::DecoderError("No audio track found".to_string()))?;

        // Calculate duration from time base and number of frames
        if let Some(n_frames) = track.codec_params.n_frames {
            if let Some(time_base) = track.codec_params.time_base {
                let duration_secs = (n_frames as f64) * time_base.numer as f64 / time_base.denom as f64;
                return Ok(std::time::Duration::from_secs_f64(duration_secs));
            }
        }

        // Fallback: try to get duration from metadata
        if let Some(metadata) = probed.format.metadata().current() {
            // Check for duration in metadata (some formats store it)
        }

        // If we can't determine duration, return error
        Err(AudioError::DecoderError("Could not determine audio duration".to_string()))
    }

    pub fn is_supported(path: &str) -> bool {
        let path = Path::new(path);
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            matches!(ext.as_str(), "wav" | "mp3" | "flac" | "ogg")
        } else {
            false
        }
    }
}

