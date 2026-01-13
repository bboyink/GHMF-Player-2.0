import React from 'react';
import { Box, Typography, Paper } from '@mui/material';

const LightGroupsView: React.FC = () => {
  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        Light Groups
      </Typography>

      <Paper sx={{ p: 3 }}>
        <Typography variant="body1">
          Light Groups configuration coming soon...
        </Typography>
        <Typography variant="body2" color="text.secondary" sx={{ mt: 2 }}>
          This view will allow you to:
        </Typography>
        <ul>
          <li>Create and manage light groups</li>
          <li>Assign fixtures to groups</li>
          <li>Save and load group presets</li>
          <li>Control multiple fixtures simultaneously</li>
        </ul>
      </Paper>
    </Box>
  );
};

export default LightGroupsView;
