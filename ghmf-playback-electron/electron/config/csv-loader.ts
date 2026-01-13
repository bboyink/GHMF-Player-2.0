import * as fs from 'fs-extra';
import * as path from 'path';
import { parse } from 'csv-parse/sync';
import { app } from 'electron';

export class ConfigLoader {
  private configPath: string;

  constructor() {
    // In production, use process.resourcesPath
    // In development, use relative path to parent directory
    if (!app.isPackaged) {
      // In dev: electron/config -> ../../.. gets to workspace root, then ../Config
      this.configPath = path.join(__dirname, '../../../../Config');
    } else {
      this.configPath = path.join(process.resourcesPath, 'Config');
    }
  }

  async loadDMXMap(): Promise<any[]> {
    try {
      const filePath = path.join(this.configPath, 'DMXMap.csv');
      const content = await fs.readFile(filePath, 'utf-8');
      
      const records = parse(content, {
        columns: true,
        skip_empty_lines: true,
        trim: true,
      });

      return records.map((record: any) => ({
        id: parseInt(record.ID || record.id, 10),
        name: record.Name || record.name,
        startChannel: parseInt(record.StartChannel || record['Start Channel'], 10),
        channelWidth: parseInt(record.ChannelWidth || record['Channel Width'], 10),
        fixtureType: record.FixtureType || record['Fixture Type'] || 'generic',
      }));
    } catch (error) {
      console.error('Error loading DMX map:', error);
      return [];
    }
  }

  async loadColorMap(): Promise<any[]> {
    try {
      const filePath = path.join(this.configPath, 'ColorMap.csv');
      const content = await fs.readFile(filePath, 'utf-8');
      
      const records = parse(content, {
        columns: true,
        skip_empty_lines: true,
        trim: true,
      });

      return records.map((record: any) => ({
        name: record.Name || record.name,
        red: parseInt(record.Red || record.R || record.red, 10) || 0,
        green: parseInt(record.Green || record.G || record.green, 10) || 0,
        blue: parseInt(record.Blue || record.B || record.blue, 10) || 0,
        white: parseInt(record.White || record.W || record.white, 10) || 0,
      }));
    } catch (error) {
      console.error('Error loading color map:', error);
      return [];
    }
  }

  async loadFCWMap(): Promise<any[]> {
    try {
      const filePath = path.join(this.configPath, 'FCWMap.CSV');
      const content = await fs.readFile(filePath, 'utf-8');
      
      const records = parse(content, {
        columns: true,
        skip_empty_lines: true,
        trim: true,
      });

      return records.map((record: any) => ({
        fixtureType: record.FixtureType || record['Fixture Type'],
        channelWidth: parseInt(record.ChannelWidth || record['Channel Width'], 10),
        channels: (record.Channels || record.channels || '').split(',').map((c: string) => c.trim()),
      }));
    } catch (error) {
      console.error('Error loading FCW map:', error);
      return [];
    }
  }

  async loadSettings(): Promise<any> {
    try {
      const filePath = path.join(this.configPath, 'Playback.xml');
      if (await fs.pathExists(filePath)) {
        const content = await fs.readFile(filePath, 'utf-8');
        // Simple XML parsing - you might want to use a proper XML parser
        return { xml: content };
      }
      return {};
    } catch (error) {
      console.error('Error loading settings:', error);
      return {};
    }
  }

  async saveSettings(settings: any): Promise<void> {
    try {
      const filePath = path.join(this.configPath, 'Playback.xml');
      await fs.writeFile(filePath, settings.xml || '', 'utf-8');
    } catch (error) {
      console.error('Error saving settings:', error);
      throw error;
    }
  }
}
