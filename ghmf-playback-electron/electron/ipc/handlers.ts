import { ipcMain } from 'electron';
import { DMXController } from '../hardware/dmx';
import { PLCClient } from '../hardware/plc';
import { AudioPlayer } from '../hardware/audio';
import { ConfigLoader } from '../config/csv-loader';
import { CTLParser } from '../config/ctl-parser';
import * as fs from 'fs-extra';
import * as path from 'path';

const dmx = new DMXController();
const plc = new PLCClient();
const audio = new AudioPlayer();
const config = new ConfigLoader();
const ctlParser = new CTLParser();

export function setupIPCHandlers() {
  // Audio handlers
  ipcMain.handle('audio:load', async (_, filePath: string) => {
    return audio.load(filePath);
  });

  ipcMain.handle('audio:play', async () => {
    return audio.play();
  });

  ipcMain.handle('audio:pause', async () => {
    return audio.pause();
  });

  ipcMain.handle('audio:stop', async () => {
    return audio.stop();
  });

  ipcMain.handle('audio:seek', async (_, position: number) => {
    return audio.seek(position);
  });

  ipcMain.handle('audio:setVolume', async (_, left: number, right: number) => {
    return audio.setVolume(left, right);
  });

  ipcMain.handle('audio:getPosition', async () => {
    return audio.getPosition();
  });

  ipcMain.handle('audio:getDuration', async () => {
    return audio.getDuration();
  });

  // DMX handlers
  ipcMain.handle('dmx:connect', async () => {
    return dmx.connect();
  });

  ipcMain.handle('dmx:disconnect', async () => {
    return dmx.disconnect();
  });

  ipcMain.handle('dmx:setChannel', async (_, channel: number, value: number) => {
    return dmx.setChannel(channel, value);
  });

  ipcMain.handle('dmx:setChannels', async (_, channels: Record<number, number>) => {
    return dmx.setChannels(channels);
  });

  ipcMain.handle('dmx:getStatus', async () => {
    return dmx.getStatus();
  });

  // PLC handlers
  ipcMain.handle('plc:connect', async (_, host: string, port: number) => {
    return plc.connect(host, port);
  });

  ipcMain.handle('plc:disconnect', async () => {
    return plc.disconnect();
  });

  ipcMain.handle('plc:send', async (_, data: string) => {
    return plc.send(data);
  });

  ipcMain.handle('plc:getStatus', async () => {
    return plc.getStatus();
  });

  // Config handlers
  ipcMain.handle('config:loadDMXMap', async () => {
    return config.loadDMXMap();
  });

  ipcMain.handle('config:loadColorMap', async () => {
    return config.loadColorMap();
  });

  ipcMain.handle('config:loadFCWMap', async () => {
    return config.loadFCWMap();
  });

  ipcMain.handle('config:loadSettings', async () => {
    return config.loadSettings();
  });

  ipcMain.handle('config:saveSettings', async (_, settings: any) => {
    return config.saveSettings(settings);
  });

  // File handlers
  ipcMain.handle('files:loadCTL', async (_, filePath: string) => {
    return ctlParser.parse(filePath);
  });

  ipcMain.handle('files:savePlaylist', async (_, filePath: string, playlist: any) => {
    await fs.writeJson(filePath, playlist, { spaces: 2 });
    return { success: true };
  });

  ipcMain.handle('files:loadPlaylist', async (_, filePath: string) => {
    return fs.readJson(filePath);
  });

  ipcMain.handle('files:listSongs', async (_, directory: string) => {
    const files = await fs.readdir(directory);
    return files.filter(f => f.endsWith('.mp3') || f.endsWith('.wav') || f.endsWith('.ctl'));
  });

  ipcMain.handle('files:listPlaylists', async () => {
    const playlistDir = path.join(process.resourcesPath, 'Music', 'Playlists');
    const files = await fs.readdir(playlistDir);
    return files.filter(f => f.endsWith('.playlist'));
  });

  // System handlers
  ipcMain.handle('system:getResourcePath', async (_, subPath: string) => {
    const { getResourcesPath } = require('../main');
    return path.join(getResourcesPath(), subPath);
  });
}
