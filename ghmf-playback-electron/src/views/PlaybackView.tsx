import React, { useState, useEffect, useRef } from 'react';
import {
  Box,
  Typography,
  Paper,
  Button,
  Slider,
  IconButton,
  LinearProgress,
} from '@mui/material';
import { PlayArrow, Pause, Stop, Replay, VolumeUp } from '@mui/icons-material';
import WaveSurfer from 'wavesurfer.js';

const PlaybackView: React.FC = () => {
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);
  const [leftVolume, setLeftVolume] = useState(100);
  const [rightVolume, setRightVolume] = useState(100);
  const [currentSong, setCurrentSong] = useState('No song loaded');
  const [playlistName, setPlaylistName] = useState('No playlist');

  const waveformRef = useRef<HTMLDivElement>(null);
  const wavesurferRef = useRef<WaveSurfer | null>(null);

  useEffect(() => {
    // Initialize WaveSurfer
    if (waveformRef.current && !wavesurferRef.current) {
      wavesurferRef.current = WaveSurfer.create({
        container: waveformRef.current,
        waveColor: '#4FC3F7',
        progressColor: '#1976d2',
        cursorColor: '#fff',
        barWidth: 2,
        barGap: 1,
        height: 80,
        normalize: true,
        backend: 'WebAudio',
        // 7-second scrolling window
        minPxPerSec: 100,
        autoCenter: true,
      });

      wavesurferRef.current.on('ready', () => {
        setDuration(wavesurferRef.current?.getDuration() || 0);
      });

      wavesurferRef.current.on('audioprocess', (time) => {
        setCurrentTime(time);
      });

      wavesurferRef.current.on('finish', () => {
        setIsPlaying(false);
      });
    }

    // Listen for audio updates from main process
    window.electronAPI.audio.onTimeUpdate((time) => {
      setCurrentTime(time);
      if (wavesurferRef.current) {
        wavesurferRef.current.seekTo(time / duration);
      }
    });

    window.electronAPI.audio.onEnded(() => {
      setIsPlaying(false);
    });

    return () => {
      wavesurferRef.current?.destroy();
    };
  }, [duration]);

  const handlePlay = async () => {
    if (isPlaying) {
      await window.electronAPI.audio.pause();
      wavesurferRef.current?.pause();
      setIsPlaying(false);
    } else {
      await window.electronAPI.audio.play();
      wavesurferRef.current?.play();
      setIsPlaying(true);
    }
  };

  const handleStop = async () => {
    await window.electronAPI.audio.stop();
    wavesurferRef.current?.stop();
    setIsPlaying(false);
    setCurrentTime(0);
  };

  const handleRestart = async () => {
    await window.electronAPI.audio.seek(0);
    wavesurferRef.current?.seekTo(0);
    setCurrentTime(0);
  };

  const handleSeek = async (value: number) => {
    await window.electronAPI.audio.seek(value);
    wavesurferRef.current?.seekTo(value / duration);
    setCurrentTime(value);
  };

  const handleVolumeChange = async (channel: 'left' | 'right', value: number) => {
    if (channel === 'left') {
      setLeftVolume(value);
      await window.electronAPI.audio.setVolume(value / 100, rightVolume / 100);
    } else {
      setRightVolume(value);
      await window.electronAPI.audio.setVolume(leftVolume / 100, value / 100);
    }
  };

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        Playback Control
      </Typography>

      <Paper sx={{ p: 3 }}>
        <Box sx={{ mb: 3 }}>
          <Typography variant="h6">{currentSong}</Typography>
          <Typography variant="body2" color="text.secondary">
            Playlist: {playlistName}
          </Typography>
        </Box>

        {/* Waveform Display */}
        <Box sx={{ mb: 3, p: 2, bgcolor: 'background.default', borderRadius: 1 }}>
          <Typography variant="subtitle2" gutterBottom>
            Waveform (7-second scroll)
          </Typography>
          <div ref={waveformRef} />
        </Box>

        {/* Timeline with ticks */}
        <Box sx={{ mb: 3 }}>
          <Slider
            value={currentTime}
            max={duration}
            onChange={(_, value) => handleSeek(value as number)}
            valueLabelDisplay="auto"
            valueLabelFormat={formatTime}
            sx={{ mb: 1 }}
          />
          <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
            <Typography variant="body2">{formatTime(currentTime)}</Typography>
            <Typography variant="body2">{formatTime(duration)}</Typography>
          </Box>
        </Box>

        {/* Transport Controls */}
        <Box sx={{ display: 'flex', justifyContent: 'center', gap: 2, mb: 3 }}>
          <IconButton onClick={handleRestart} size="large">
            <Replay />
          </IconButton>
          <IconButton onClick={handlePlay} size="large" color="primary">
            {isPlaying ? <Pause /> : <PlayArrow />}
          </IconButton>
          <IconButton onClick={handleStop} size="large">
            <Stop />
          </IconButton>
        </Box>

        {/* Volume Controls */}
        <Box sx={{ display: 'flex', gap: 4 }}>
          <Box sx={{ flexGrow: 1 }}>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
              <VolumeUp />
              <Typography variant="body2" sx={{ minWidth: 80 }}>
                Left: {leftVolume}%
              </Typography>
              <Slider
                value={leftVolume}
                onChange={(_, value) => handleVolumeChange('left', value as number)}
                min={0}
                max={100}
              />
            </Box>
          </Box>
          <Box sx={{ flexGrow: 1 }}>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
              <VolumeUp />
              <Typography variant="body2" sx={{ minWidth: 80 }}>
                Right: {rightVolume}%
              </Typography>
              <Slider
                value={rightVolume}
                onChange={(_, value) => handleVolumeChange('right', value as number)}
                min={0}
                max={100}
              />
            </Box>
          </Box>
        </Box>
      </Paper>
    </Box>
  );
};

export default PlaybackView;
