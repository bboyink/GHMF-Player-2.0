import React from 'react';
import { Box, Typography, Paper } from '@mui/material';

const ProceduresView: React.FC = () => {
  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        Procedures
      </Typography>

      <Paper sx={{ p: 3 }}>
        <Typography variant="body1">
          Procedures configuration coming soon...
        </Typography>
        <Typography variant="body2" color="text.secondary" sx={{ mt: 2 }}>
          This view will allow you to:
        </Typography>
        <ul>
          <li>Create and manage lighting procedures</li>
          <li>Set time-based triggers</li>
          <li>Define sequences of DMX commands</li>
          <li>Manually trigger procedures</li>
        </ul>
      </Paper>
    </Box>
  );
};

export default ProceduresView;
