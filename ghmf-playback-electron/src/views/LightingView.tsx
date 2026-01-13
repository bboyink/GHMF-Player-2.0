import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  Paper,
  Grid,
  Slider,
  Button,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
} from '@mui/material';
import { DMXFixture, ColorPreset } from '../types';

const LightingView: React.FC = () => {
  const [fixtures, setFixtures] = useState<DMXFixture[]>([]);
  const [colors, setColors] = useState<ColorPreset[]>([]);
  const [selectedFixture, setSelectedFixture] = useState<number | null>(null);
  const [rgbw, setRgbw] = useState({ r: 0, g: 0, b: 0, w: 0 });
  const [intensity, setIntensity] = useState(255);

  useEffect(() => {
    loadConfig();
  }, []);

  const loadConfig = async () => {
    try {
      const fixtureData = await window.electronAPI.config.loadDMXMap();
      const colorData = await window.electronAPI.config.loadColorMap();
      setFixtures(fixtureData);
      setColors(colorData);
    } catch (error) {
      console.error('Error loading config:', error);
    }
  };

  const applyColor = async () => {
    if (selectedFixture === null) return;

    const fixture = fixtures.find(f => f.id === selectedFixture);
    if (!fixture) return;

    const channels: Record<number, number> = {
      [fixture.startChannel]: rgbw.r,
      [fixture.startChannel + 1]: rgbw.g,
      [fixture.startChannel + 2]: rgbw.b,
      [fixture.startChannel + 3]: rgbw.w,
    };

    await window.electronAPI.dmx.setChannels(channels);
  };

  const applyPreset = async (preset: ColorPreset) => {
    setRgbw({ r: preset.red, g: preset.green, b: preset.blue, w: preset.white });
  };

  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        Lighting Control
      </Typography>

      <Grid container spacing={3}>
        <Grid item xs={12} md={4}>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>
              Fixture Selection
            </Typography>
            <FormControl fullWidth>
              <InputLabel>Select Fixture</InputLabel>
              <Select
                value={selectedFixture || ''}
                label="Select Fixture"
                onChange={(e) => setSelectedFixture(Number(e.target.value))}
              >
                {fixtures.map((fixture) => (
                  <MenuItem key={fixture.id} value={fixture.id}>
                    {fixture.name}
                  </MenuItem>
                ))}
              </Select>
            </FormControl>
          </Paper>
        </Grid>

        <Grid item xs={12} md={8}>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>
              Color Control (RGBW)
            </Typography>

            <Box sx={{ mb: 2 }}>
              <Typography variant="body2">Red: {rgbw.r}</Typography>
              <Slider
                value={rgbw.r}
                onChange={(_, val) => setRgbw(prev => ({ ...prev, r: val as number }))}
                max={255}
                sx={{ color: 'red' }}
              />
            </Box>

            <Box sx={{ mb: 2 }}>
              <Typography variant="body2">Green: {rgbw.g}</Typography>
              <Slider
                value={rgbw.g}
                onChange={(_, val) => setRgbw(prev => ({ ...prev, g: val as number }))}
                max={255}
                sx={{ color: 'green' }}
              />
            </Box>

            <Box sx={{ mb: 2 }}>
              <Typography variant="body2">Blue: {rgbw.b}</Typography>
              <Slider
                value={rgbw.b}
                onChange={(_, val) => setRgbw(prev => ({ ...prev, b: val as number }))}
                max={255}
                sx={{ color: 'blue' }}
              />
            </Box>

            <Box sx={{ mb: 2 }}>
              <Typography variant="body2">White: {rgbw.w}</Typography>
              <Slider
                value={rgbw.w}
                onChange={(_, val) => setRgbw(prev => ({ ...prev, w: val as number }))}
                max={255}
              />
            </Box>

            <Button variant="contained" onClick={applyColor} fullWidth sx={{ mb: 2 }}>
              Apply Color
            </Button>

            <Typography variant="h6" gutterBottom>
              Color Presets
            </Typography>
            <Grid container spacing={1}>
              {colors.map((color, idx) => (
                <Grid item xs={6} sm={4} key={idx}>
                  <Button
                    variant="outlined"
                    onClick={() => applyPreset(color)}
                    fullWidth
                  >
                    {color.name}
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

export default LightingView;
