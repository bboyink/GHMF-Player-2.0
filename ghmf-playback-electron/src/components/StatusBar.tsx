import React, { useEffect, useState } from 'react';
import { Box, Paper, Typography, Chip } from '@mui/material';
import { Circle } from '@mui/icons-material';
import { StatusMessage, DMXStatus, PLCStatus } from '../types';

const StatusBar: React.FC = () => {
  const [dmxStatus, setDmxStatus] = useState<DMXStatus>({ connected: false, port: null, error: null });
  const [plcStatus, setPlcStatus] = useState<PLCStatus>({ connected: false, host: null, port: null, error: null });
  const [message, setMessage] = useState<StatusMessage>({ type: 'info', message: 'Ready', timestamp: Date.now() });

  useEffect(() => {
    // Poll DMX and PLC status
    const interval = setInterval(async () => {
      try {
        const dmx = await window.electronAPI.dmx.getStatus();
        setDmxStatus(dmx);

        const plc = await window.electronAPI.plc.getStatus();
        setPlcStatus(plc);
      } catch (error) {
        console.error('Status update error:', error);
      }
    }, 2000);

    return () => clearInterval(interval);
  }, []);

  const getStatusColor = (connected: boolean) => {
    return connected ? 'success' : 'error';
  };

  const getMessageColor = () => {
    switch (message.type) {
      case 'success':
        return 'success.main';
      case 'warning':
        return 'warning.main';
      case 'error':
        return 'error.main';
      default:
        return 'text.primary';
    }
  };

  return (
    <Paper
      sx={{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        px: 2,
        py: 1,
        borderRadius: 0,
      }}
      elevation={3}
    >
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
        <Typography variant="body2" sx={{ color: getMessageColor() }}>
          {message.message}
        </Typography>
      </Box>

      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
        <Chip
          icon={<Circle />}
          label={`DMX: ${dmxStatus.connected ? 'Connected' : 'Disconnected'}`}
          size="small"
          color={getStatusColor(dmxStatus.connected)}
          variant="outlined"
        />
        <Chip
          icon={<Circle />}
          label={`PLC: ${plcStatus.connected ? 'Connected' : 'Disconnected'}`}
          size="small"
          color={getStatusColor(plcStatus.connected)}
          variant="outlined"
        />
        <Chip
          label="RGBW Mode"
          size="small"
          color="primary"
          variant="outlined"
        />
      </Box>
    </Paper>
  );
};

export default StatusBar;
