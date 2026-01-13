import { contextBridge, ipcRenderer } from 'electron';

// Exposed API to renderer process
contextBridge.exposeInMainWorld('electronAPI', {
  // Audio
  audio: {
    load: (filePath: string) => ipcRenderer.invoke('audio:load', filePath),
    play: () => ipcRenderer.invoke('audio:play'),
    pause: () => ipcRenderer.invoke('audio:pause'),
    stop: () => ipcRenderer.invoke('audio:stop'),
    seek: (position: number) => ipcRenderer.invoke('audio:seek', position),
    setVolume: (left: number, right: number) => ipcRenderer.invoke('audio:setVolume', left, right),
    getPosition: () => ipcRenderer.invoke('audio:getPosition'),
    getDuration: () => ipcRenderer.invoke('audio:getDuration'),
    onTimeUpdate: (callback: (time: number) => void) => {
      ipcRenderer.on('audio:timeUpdate', (_, time) => callback(time));
    },
    onEnded: (callback: () => void) => {
      ipcRenderer.on('audio:ended', callback);
    },
  },

  // DMX
  dmx: {
    connect: () => ipcRenderer.invoke('dmx:connect'),
    disconnect: () => ipcRenderer.invoke('dmx:disconnect'),
    setChannel: (channel: number, value: number) => ipcRenderer.invoke('dmx:setChannel', channel, value),
    setChannels: (channels: Record<number, number>) => ipcRenderer.invoke('dmx:setChannels', channels),
    getStatus: () => ipcRenderer.invoke('dmx:getStatus'),
  },

  // PLC
  plc: {
    connect: (host: string, port: number) => ipcRenderer.invoke('plc:connect', host, port),
    disconnect: () => ipcRenderer.invoke('plc:disconnect'),
    send: (data: string) => ipcRenderer.invoke('plc:send', data),
    getStatus: () => ipcRenderer.invoke('plc:getStatus'),
    onData: (callback: (data: string) => void) => {
      ipcRenderer.on('plc:data', (_, data) => callback(data));
    },
  },

  // Config
  config: {
    loadDMXMap: () => ipcRenderer.invoke('config:loadDMXMap'),
    loadColorMap: () => ipcRenderer.invoke('config:loadColorMap'),
    loadFCWMap: () => ipcRenderer.invoke('config:loadFCWMap'),
    loadSettings: () => ipcRenderer.invoke('config:loadSettings'),
    saveSettings: (settings: any) => ipcRenderer.invoke('config:saveSettings', settings),
  },

  // Files
  files: {
    loadCTL: (filePath: string) => ipcRenderer.invoke('files:loadCTL', filePath),
    savePlaylist: (filePath: string, playlist: any) => ipcRenderer.invoke('files:savePlaylist', filePath, playlist),
    loadPlaylist: (filePath: string) => ipcRenderer.invoke('files:loadPlaylist', filePath),
    listSongs: (directory: string) => ipcRenderer.invoke('files:listSongs', directory),
    listPlaylists: () => ipcRenderer.invoke('files:listPlaylists'),
  },

  // System
  system: {
    getResourcePath: (subPath: string) => ipcRenderer.invoke('system:getResourcePath', subPath),
  },
});
