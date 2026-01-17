use super::AudioError;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use std::fs::File;
use std::path::Path;

/// Represents waveform data for visualization
#[derive(Clone, Debug)]
pub struct WaveformData {
    pub samples: Vec<f32>,  // RMS values for each bar (0.0 to 1.0)
    pub duration_secs: f32,
    pub sample_rate: u32,
}

impl WaveformData {
    /// Generate waveform data from an audio file
    /// bars: number of bars to generate (e.g., 100 for 100 bars across the timeline)
    pub fn from_file<P: AsRef<Path>>(path: P, bars: usize) -> Result<Self, AudioError> {
        let file = File::open(path.as_ref())?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());
        
        let mut hint = Hint::new();
        if let Some(ext) = path.as_ref().extension() {
            if let Some(ext_str) = ext.to_str() {
                hint.with_extension(ext_str);
            }
        }
        
        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();
        
        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &format_opts, &metadata_opts)
            .map_err(|e| AudioError::DecoderError(format!("Failed to probe file: {}", e)))?;
        
        let mut format = probed.format;
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .ok_or_else(|| AudioError::DecoderError("No valid audio track found".to_string()))?;
        
        let track_id = track.id;
        let sample_rate = track.codec_params.sample_rate.unwrap_or(44100);
        
        // Calculate duration
        let duration_secs = if let Some(n_frames) = track.codec_params.n_frames {
            n_frames as f32 / sample_rate as f32
        } else {
            0.0
        };
        
        let dec_opts = DecoderOptions::default();
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .map_err(|e| AudioError::DecoderError(format!("Failed to create decoder: {}", e)))?;
        
        // Collect all samples
        let mut all_samples: Vec<f32> = Vec::new();
        
        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(_) => break,
            };
            
            if packet.track_id() != track_id {
                continue;
            }
            
            match decoder.decode(&packet) {
                Ok(decoded) => {
                    // Convert samples to f32 and take RMS across channels
                    match decoded {
                        AudioBufferRef::F32(buf) => {
                            let channels = buf.spec().channels.count();
                            for frame_idx in 0..buf.frames() {
                                let mut sum = 0.0_f32;
                                for ch in 0..channels {
                                    let sample = buf.chan(ch)[frame_idx];
                                    sum += sample * sample;
                                }
                                all_samples.push((sum / channels as f32).sqrt());
                            }
                        }
                        AudioBufferRef::U8(buf) => {
                            let channels = buf.spec().channels.count();
                            for frame_idx in 0..buf.frames() {
                                let mut sum = 0.0_f32;
                                for ch in 0..channels {
                                    let sample = (buf.chan(ch)[frame_idx] as f32 - 128.0) / 128.0;
                                    sum += sample * sample;
                                }
                                all_samples.push((sum / channels as f32).sqrt());
                            }
                        }
                        AudioBufferRef::U16(buf) => {
                            let channels = buf.spec().channels.count();
                            for frame_idx in 0..buf.frames() {
                                let mut sum = 0.0_f32;
                                for ch in 0..channels {
                                    let sample = (buf.chan(ch)[frame_idx] as f32 - 32768.0) / 32768.0;
                                    sum += sample * sample;
                                }
                                all_samples.push((sum / channels as f32).sqrt());
                            }
                        }
                        AudioBufferRef::U32(buf) => {
                            let channels = buf.spec().channels.count();
                            for frame_idx in 0..buf.frames() {
                                let mut sum = 0.0_f32;
                                for ch in 0..channels {
                                    let sample = (buf.chan(ch)[frame_idx] as f32 - 2147483648.0) / 2147483648.0;
                                    sum += sample * sample;
                                }
                                all_samples.push((sum / channels as f32).sqrt());
                            }
                        }
                        AudioBufferRef::S8(buf) => {
                            let channels = buf.spec().channels.count();
                            for frame_idx in 0..buf.frames() {
                                let mut sum = 0.0_f32;
                                for ch in 0..channels {
                                    let sample = buf.chan(ch)[frame_idx] as f32 / 128.0;
                                    sum += sample * sample;
                                }
                                all_samples.push((sum / channels as f32).sqrt());
                            }
                        }
                        AudioBufferRef::S16(buf) => {
                            let channels = buf.spec().channels.count();
                            for frame_idx in 0..buf.frames() {
                                let mut sum = 0.0_f32;
                                for ch in 0..channels {
                                    let sample = buf.chan(ch)[frame_idx] as f32 / 32768.0;
                                    sum += sample * sample;
                                }
                                all_samples.push((sum / channels as f32).sqrt());
                            }
                        }
                        AudioBufferRef::S32(buf) => {
                            let channels = buf.spec().channels.count();
                            for frame_idx in 0..buf.frames() {
                                let mut sum = 0.0_f32;
                                for ch in 0..channels {
                                    let sample = buf.chan(ch)[frame_idx] as f32 / 2147483648.0;
                                    sum += sample * sample;
                                }
                                all_samples.push((sum / channels as f32).sqrt());
                            }
                        }
                        AudioBufferRef::F64(buf) => {
                            let channels = buf.spec().channels.count();
                            for frame_idx in 0..buf.frames() {
                                let mut sum = 0.0_f32;
                                for ch in 0..channels {
                                    let sample = buf.chan(ch)[frame_idx] as f32;
                                    sum += sample * sample;
                                }
                                all_samples.push((sum / channels as f32).sqrt());
                            }
                        }
                        AudioBufferRef::U24(buf) => {
                            let channels = buf.spec().channels.count();
                            for frame_idx in 0..buf.frames() {
                                let mut sum = 0.0_f32;
                                for ch in 0..channels {
                                    // u24 doesn't have into_i32, need to convert through inner value
                                    let sample_bytes = buf.chan(ch)[frame_idx];
                                    // Approximate conversion for u24
                                    let sample = (sample_bytes.inner() as f32 - 8388608.0) / 8388608.0;
                                    sum += sample * sample;
                                }
                                all_samples.push((sum / channels as f32).sqrt());
                            }
                        }
                        AudioBufferRef::S24(buf) => {
                            let channels = buf.spec().channels.count();
                            for frame_idx in 0..buf.frames() {
                                let mut sum = 0.0_f32;
                                for ch in 0..channels {
                                    // i24 doesn't have into_i32, need to convert through inner value
                                    let sample_bytes = buf.chan(ch)[frame_idx];
                                    let sample = sample_bytes.inner() as f32 / 8388608.0;
                                    sum += sample * sample;
                                }
                                all_samples.push((sum / channels as f32).sqrt());
                            }
                        }
                    }
                }
                Err(_) => continue,
            }
        }
        
        // Generate RMS bars by dividing samples into bins
        let samples_per_bar = (all_samples.len() / bars).max(1);
        let mut rms_bars = Vec::with_capacity(bars);
        
        for i in 0..bars {
            let start = i * samples_per_bar;
            let end = ((i + 1) * samples_per_bar).min(all_samples.len());
            
            if start >= all_samples.len() {
                rms_bars.push(0.0);
                continue;
            }
            
            let chunk = &all_samples[start..end];
            let rms: f32 = (chunk.iter().map(|&s| s * s).sum::<f32>() / chunk.len() as f32).sqrt();
            rms_bars.push(rms.clamp(0.0, 1.0));
        }
        
        Ok(WaveformData {
            samples: rms_bars,
            duration_secs,
            sample_rate,
        })
    }
    
    /// Create placeholder waveform data (for when no file is loaded)
    pub fn placeholder(bars: usize) -> Self {
        let samples = (0..bars)
            .map(|i| ((i as f32 * 0.3).sin().abs() * 0.5 + 0.3).clamp(0.1, 1.0))
            .collect();
        
        WaveformData {
            samples,
            duration_secs: 0.0,
            sample_rate: 44100,
        }
    }
}
