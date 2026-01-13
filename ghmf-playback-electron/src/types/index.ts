// Global type definitions for Electron API
export interface ElectronAPI {
  audio: {
    load: (filePath: string) => Promise<{ success: boolean; duration?: number; error?: string }>;
    play: () => Promise<boolean>;
    pause: () => Promise<boolean>;
    stop: () => Promise<boolean>;
    seek: (position: number) => Promise<boolean>;
    setVolume: (left: number, right: number) => Promise<boolean>;
    getPosition: () => Promise<number>;
    getDuration: () => Promise<number>;
    onTimeUpdate: (callback: (time: number) => void) => void;
    onEnded: (callback: () => void) => void;
  };
  dmx: {
    connect: () => Promise<DMXStatus>;
    disconnect: () => Promise<void>;
    setChannel: (channel: number, value: number) => Promise<void>;
    setChannels: (channels: Record<number, number>) => Promise<void>;
    getStatus: () => Promise<DMXStatus>;
  };
  plc: {
    connect: (host: string, port: number) => Promise<PLCStatus>;
    disconnect: () => Promise<void>;
    send: (data: string) => Promise<boolean>;
    getStatus: () => Promise<PLCStatus>;
    onData: (callback: (data: string) => void) => void;
  };
  config: {
    loadDMXMap: () => Promise<DMXFixture[]>;
    loadColorMap: () => Promise<ColorPreset[]>;
    loadFCWMap: () => Promise<FCWMapping[]>;
    loadSettings: () => Promise<any>;
    saveSettings: (settings: any) => Promise<void>;
  };
  files: {
    loadCTL: (filePath: string) => Promise<CTLFile>;
    savePlaylist: (filePath: string, playlist: Playlist) => Promise<{ success: boolean }>;
    loadPlaylist: (filePath: string) => Promise<Playlist>;
    listSongs: (directory: string) => Promise<string[]>;
    listPlaylists: () => Promise<string[]>;
  };
  system: {
    getResourcePath: (subPath: string) => Promise<string>;
  };
}

declare global {
  interface Window {
    electronAPI: ElectronAPI;
  }
}

// DMX Types
export interface DMXFixture {
  id: number;
  name: string;
  startChannel: number;
  channelWidth: number;
  fixtureType: string;
}

export interface DMXStatus {
  connected: boolean;
  port: string | null;
  error: string | null;
}

export interface ColorPreset {
  name: string;
  red: number;
  green: number;
  blue: number;
  white: number;
}

export interface FCWMapping {
  fixtureType: string;
  channelWidth: number;
  channels: string[];
}

// PLC Types
export interface PLCStatus {
  connected: boolean;
  host: string | null;
  port: number | null;
  error: string | null;
}

// Playlist Types
export interface Song {
  id: string;
  name: string;
  path: string;
  duration?: number;
  ctlFile?: string;
}

export interface Playlist {
  name: string;
  theme: string;
  songs: Song[];
  startWithPlaylist: boolean;
}

// CTL File Types
export interface CTLCommand {
  time: number;
  type: string;
  params: Record<string, any>;
}

export interface CTLFile {
  filePath: string;
  commands: CTLCommand[];
  duration: number;
}

// Light Group Types
export interface LightGroup {
  id: string;
  name: string;
  fixtureIds: number[];
  color?: ColorPreset;
}

// Procedure Types
export interface Procedure {
  id: string;
  name: string;
  triggerTime?: string;
  commands: CTLCommand[];
  enabled: boolean;
}

// View Types
export type ViewType =
  | 'playlist'
  | 'playback'
  | 'lighting'
  | 'dmx-map'
  | 'light-groups'
  | 'procedures'
  | 'operator';

// Status Message Types
export type StatusType = 'info' | 'success' | 'warning' | 'error';

export interface StatusMessage {
  type: StatusType;
  message: string;
  timestamp: number;
}
