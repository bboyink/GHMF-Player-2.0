import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  Paper,
  Button,
  TextField,
  Select,
  MenuItem,
  FormControl,
  InputLabel,
  Checkbox,
  FormControlLabel,
  Grid,
  List,
  ListItem,
  ListItemText,
  IconButton,
} from '@mui/material';
import { Add, Delete, ArrowUpward, ArrowDownward, Save, FolderOpen } from '@mui/icons-material';
import { Playlist, Song } from '../types';

const PlaylistView: React.FC = () => {
  const [availableSongs, setAvailableSongs] = useState<Song[]>([]);
  const [playlist, setPlaylist] = useState<Playlist>({
    name: 'New Playlist',
    theme: 'default',
    songs: [],
    startWithPlaylist: false,
  });

  useEffect(() => {
    loadAvailableSongs();
  }, []);

  const loadAvailableSongs = async () => {
    try {
      const musicPath = await window.electronAPI.system.getResourcePath('Music/Production');
      const files = await window.electronAPI.files.listSongs(musicPath);
      
      const songs: Song[] = files
        .filter(f => f.endsWith('.mp3') || f.endsWith('.wav'))
        .map((file, idx) => ({
          id: `song-${idx}`,
          name: file.replace(/\.(mp3|wav)$/, ''),
          path: `${musicPath}/${file}`,
        }));

      setAvailableSongs(songs);
    } catch (error) {
      console.error('Error loading songs:', error);
    }
  };

  const addSongToPlaylist = (song: Song) => {
    setPlaylist(prev => ({
      ...prev,
      songs: [...prev.songs, song],
    }));
  };

  const removeSongFromPlaylist = (index: number) => {
    setPlaylist(prev => ({
      ...prev,
      songs: prev.songs.filter((_, i) => i !== index),
    }));
  };

  const moveSong = (index: number, direction: 'up' | 'down') => {
    if (
      (direction === 'up' && index === 0) ||
      (direction === 'down' && index === playlist.songs.length - 1)
    ) {
      return;
    }

    const newSongs = [...playlist.songs];
    const newIndex = direction === 'up' ? index - 1 : index + 1;
    [newSongs[index], newSongs[newIndex]] = [newSongs[newIndex], newSongs[index]];

    setPlaylist(prev => ({ ...prev, songs: newSongs }));
  };

  const savePlaylist = async () => {
    try {
      const date = new Date().toISOString().split('T')[0];
      const filename = `${date}_${playlist.theme}_${playlist.name.replace(/\s+/g, '_')}.playlist`;
      const playlistPath = await window.electronAPI.system.getResourcePath(`Music/Playlists/${filename}`);
      
      await window.electronAPI.files.savePlaylist(playlistPath, playlist);
      alert('Playlist saved successfully!');
    } catch (error) {
      console.error('Error saving playlist:', error);
      alert('Failed to save playlist');
    }
  };

  const loadPlaylist = async () => {
    try {
      const playlists = await window.electronAPI.files.listPlaylists();
      // TODO: Show dialog to select playlist
      console.log('Available playlists:', playlists);
    } catch (error) {
      console.error('Error loading playlists:', error);
    }
  };

  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        Playlist Manager
      </Typography>

      <Grid container spacing={3}>
        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 2, height: '70vh' }}>
            <Typography variant="h6" gutterBottom>
              Available Songs
            </Typography>
            <List sx={{ maxHeight: 'calc(100% - 40px)', overflow: 'auto' }}>
              {availableSongs.map((song) => (
                <ListItem
                  key={song.id}
                  secondaryAction={
                    <IconButton edge="end" onClick={() => addSongToPlaylist(song)}>
                      <Add />
                    </IconButton>
                  }
                >
                  <ListItemText primary={song.name} />
                </ListItem>
              ))}
            </List>
          </Paper>
        </Grid>

        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 2, height: '70vh', display: 'flex', flexDirection: 'column' }}>
            <Box sx={{ mb: 2 }}>
              <TextField
                fullWidth
                label="Playlist Name"
                value={playlist.name}
                onChange={(e) => setPlaylist(prev => ({ ...prev, name: e.target.value }))}
                sx={{ mb: 2 }}
              />

              <FormControl fullWidth sx={{ mb: 2 }}>
                <InputLabel>Theme</InputLabel>
                <Select
                  value={playlist.theme}
                  label="Theme"
                  onChange={(e) => setPlaylist(prev => ({ ...prev, theme: e.target.value }))}
                >
                  <MenuItem value="default">Default</MenuItem>
                  <MenuItem value="new-songs">New Songs</MenuItem>
                  <MenuItem value="new-wave">New Wave</MenuItem>
                  <MenuItem value="pre-show">Pre-Show</MenuItem>
                  <MenuItem value="other">Other</MenuItem>
                </Select>
              </FormControl>

              <FormControlLabel
                control={
                  <Checkbox
                    checked={playlist.startWithPlaylist}
                    onChange={(e) => setPlaylist(prev => ({ ...prev, startWithPlaylist: e.target.checked }))}
                  />
                }
                label="Start with this playlist"
              />
            </Box>

            <Typography variant="h6" gutterBottom>
              Selected Songs ({playlist.songs.length})
            </Typography>

            <List sx={{ flexGrow: 1, overflow: 'auto', mb: 2 }}>
              {playlist.songs.map((song, index) => (
                <ListItem
                  key={`${song.id}-${index}`}
                  secondaryAction={
                    <Box>
                      <IconButton size="small" onClick={() => moveSong(index, 'up')} disabled={index === 0}>
                        <ArrowUpward />
                      </IconButton>
                      <IconButton size="small" onClick={() => moveSong(index, 'down')} disabled={index === playlist.songs.length - 1}>
                        <ArrowDownward />
                      </IconButton>
                      <IconButton size="small" onClick={() => removeSongFromPlaylist(index)}>
                        <Delete />
                      </IconButton>
                    </Box>
                  }
                >
                  <ListItemText primary={`${index + 1}. ${song.name}`} />
                </ListItem>
              ))}
            </List>

            <Box sx={{ display: 'flex', gap: 2 }}>
              <Button variant="contained" startIcon={<Save />} onClick={savePlaylist} fullWidth>
                Save Playlist
              </Button>
              <Button variant="outlined" startIcon={<FolderOpen />} onClick={loadPlaylist} fullWidth>
                Load Playlist
              </Button>
            </Box>
          </Paper>
        </Grid>
      </Grid>
    </Box>
  );
};

export default PlaylistView;
