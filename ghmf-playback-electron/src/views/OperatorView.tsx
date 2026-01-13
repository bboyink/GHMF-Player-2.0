import React, { useState } from 'react';
import {
  Box,
  Typography,
  Paper,
  Button,
  Grid,
  Card,
  CardContent,
  CardActions,
} from '@mui/material';
import { PlayArrow, Cloud, Today } from '@mui/icons-material';

const OperatorView: React.FC = () => {
  const [weather, setWeather] = useState({ temp: 72, condition: 'Sunny' });
  const [showStatus, setShowStatus] = useState('Ready');

  const triggerAnnouncement = async (type: string) => {
    // TODO: Implement announcement trigger
    console.log(`Triggering announcement: ${type}`);
  };

  const loadTodaysPlaylist = async () => {
    // TODO: Implement load today's playlist
    const date = new Date().toISOString().split('T')[0];
    console.log(`Loading playlist for ${date}`);
  };

  const connectPLC = async () => {
    try {
      await window.electronAPI.plc.connect('192.168.1.100', 502);
      alert('PLC connected!');
    } catch (error) {
      console.error('PLC connection error:', error);
    }
  };

  const connectDMX = async () => {
    try {
      await window.electronAPI.dmx.connect();
      alert('DMX connected!');
    } catch (error) {
      console.error('DMX connection error:', error);
    }
  };

  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        Operator View
      </Typography>

      <Grid container spacing={3}>
        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Show Control
              </Typography>
              <Typography variant="body1" sx={{ mb: 2 }}>
                Status: <strong>{showStatus}</strong>
              </Typography>
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
                <Button
                  variant="contained"
                  color="primary"
                  startIcon={<PlayArrow />}
                  onClick={() => setShowStatus('Running')}
                >
                  Start Show
                </Button>
                <Button
                  variant="outlined"
                  onClick={() => setShowStatus('Ready')}
                >
                  Stop Show
                </Button>
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                <Cloud /> Weather
              </Typography>
              <Typography variant="h4">{weather.temp}°F</Typography>
              <Typography variant="body1">{weather.condition}</Typography>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Hardware Connections
              </Typography>
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
                <Button variant="outlined" onClick={connectDMX}>
                  Connect DMX
                </Button>
                <Button variant="outlined" onClick={connectPLC}>
                  Connect PLC
                </Button>
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                <Today /> Today's Playlist
              </Typography>
              <Button
                variant="contained"
                onClick={loadTodaysPlaylist}
                fullWidth
              >
                Load Today's Playlist
              </Button>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12}>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>
              Announcements
            </Typography>
            <Grid container spacing={2}>
              {['Welcome', 'Intermission', 'Final Call', 'Thank You'].map((type) => (
                <Grid item xs={6} sm={3} key={type}>
                  <Button
                    variant="outlined"
                    onClick={() => triggerAnnouncement(type)}
                    fullWidth
                  >
                    {type}
                  </Button>
                </Grid>
              ))}
            </Grid>
          </Paper>
        </Grid>
      </Grid>
    </Box>
  );
};

export default OperatorView;
