import React, { useState } from 'react';
import { Box, AppBar, Toolbar, Typography } from '@mui/material';
import Sidebar from './components/Sidebar';
import StatusBar from './components/StatusBar';
import PlaylistView from './views/PlaylistView';
import PlaybackView from './views/PlaybackView';
import LightingView from './views/LightingView';
import DmxMapView from './views/DmxMapView';
import LightGroupsView from './views/LightGroupsView';
import ProceduresView from './views/ProceduresView';
import OperatorView from './views/OperatorView';
import { ViewType } from './types';

function App() {
  const [currentView, setCurrentView] = useState<ViewType>('playlist');

  const renderView = () => {
    switch (currentView) {
      case 'playlist':
        return <PlaylistView />;
      case 'playback':
        return <PlaybackView />;
      case 'lighting':
        return <LightingView />;
      case 'dmx-map':
        return <DmxMapView />;
      case 'light-groups':
        return <LightGroupsView />;
      case 'procedures':
        return <ProceduresView />;
      case 'operator':
        return <OperatorView />;
      default:
        return <PlaylistView />;
    }
  };

  return (
    <Box sx={{ display: 'flex', height: '100vh', overflow: 'hidden' }}>
      <Sidebar currentView={currentView} onViewChange={setCurrentView} />
      
      <Box sx={{ display: 'flex', flexDirection: 'column', flexGrow: 1 }}>
        <AppBar position="static" elevation={0}>
          <Toolbar>
            <Typography variant="h6" component="div">
              GHMF Playback 2.0
            </Typography>
          </Toolbar>
        </AppBar>

        <Box sx={{ flexGrow: 1, overflow: 'auto', p: 3 }}>
          {renderView()}
        </Box>

        <StatusBar />
      </Box>
    </Box>
  );
}

export default App;
