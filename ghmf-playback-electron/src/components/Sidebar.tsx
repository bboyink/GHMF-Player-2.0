import React from 'react';
import {
  Drawer,
  List,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Divider,
} from '@mui/material';
import {
  PlaylistPlay,
  PlayCircle,
  Lightbulb,
  Settings,
  Group,
  Schedule,
  AccountCircle,
} from '@mui/icons-material';
import { ViewType } from '../types';

interface SidebarProps {
  currentView: ViewType;
  onViewChange: (view: ViewType) => void;
}

const DRAWER_WIDTH = 240;

const menuItems: Array<{ id: ViewType; label: string; icon: React.ReactNode }> = [
  { id: 'playlist', label: 'Playlist', icon: <PlaylistPlay /> },
  { id: 'playback', label: 'Playback', icon: <PlayCircle /> },
  { id: 'lighting', label: 'Lighting', icon: <Lightbulb /> },
  { id: 'dmx-map', label: 'DMX Map', icon: <Settings /> },
  { id: 'light-groups', label: 'Light Groups', icon: <Group /> },
  { id: 'procedures', label: 'Procedures', icon: <Schedule /> },
  { id: 'operator', label: 'Operator', icon: <AccountCircle /> },
];

const Sidebar: React.FC<SidebarProps> = ({ currentView, onViewChange }) => {
  return (
    <Drawer
      variant="permanent"
      sx={{
        width: DRAWER_WIDTH,
        flexShrink: 0,
        '& .MuiDrawer-paper': {
          width: DRAWER_WIDTH,
          boxSizing: 'border-box',
        },
      }}
    >
      <List sx={{ mt: 8 }}>
        {menuItems.map((item) => (
          <ListItemButton
            key={item.id}
            selected={currentView === item.id}
            onClick={() => onViewChange(item.id)}
          >
            <ListItemIcon>{item.icon}</ListItemIcon>
            <ListItemText primary={item.label} />
          </ListItemButton>
        ))}
      </List>
      <Divider />
    </Drawer>
  );
};

export default Sidebar;
