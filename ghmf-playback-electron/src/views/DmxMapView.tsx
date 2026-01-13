import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from '@mui/material';
import { DMXFixture } from '../types';

const DmxMapView: React.FC = () => {
  const [fixtures, setFixtures] = useState<DMXFixture[]>([]);

  useEffect(() => {
    loadDMXMap();
  }, []);

  const loadDMXMap = async () => {
    try {
      const data = await window.electronAPI.config.loadDMXMap();
      setFixtures(data);
    } catch (error) {
      console.error('Error loading DMX map:', error);
    }
  };

  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        DMX Mapping
      </Typography>

      <Paper>
        <TableContainer>
          <Table>
            <TableHead>
              <TableRow>
                <TableCell>ID</TableCell>
                <TableCell>Fixture Name</TableCell>
                <TableCell>Start Channel</TableCell>
                <TableCell>Channel Width</TableCell>
                <TableCell>Fixture Type</TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {fixtures.map((fixture) => (
                <TableRow key={fixture.id}>
                  <TableCell>{fixture.id}</TableCell>
                  <TableCell>{fixture.name}</TableCell>
                  <TableCell>{fixture.startChannel}</TableCell>
                  <TableCell>{fixture.channelWidth}</TableCell>
                  <TableCell>{fixture.fixtureType}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </TableContainer>
      </Paper>
    </Box>
  );
};

export default DmxMapView;
