import * as net from 'net';
import { getMainWindow } from '../main';

interface PLCStatus {
  connected: boolean;
  host: string | null;
  port: number | null;
  error: string | null;
}

export class PLCClient {
  private client: net.Socket | null = null;
  private connected: boolean = false;
  private host: string | null = null;
  private port: number | null = null;

  async connect(host: string, port: number): Promise<PLCStatus> {
    return new Promise((resolve) => {
      this.client = new net.Socket();

      this.client.connect(port, host, () => {
        this.connected = true;
        this.host = host;
        this.port = port;
        resolve({
          connected: true,
          host,
          port,
          error: null,
        });
      });

      this.client.on('data', (data) => {
        const message = data.toString('utf8');
        const mainWindow = getMainWindow();
        if (mainWindow) {
          mainWindow.webContents.send('plc:data', message);
        }
      });

      this.client.on('error', (err) => {
        this.connected = false;
        resolve({
          connected: false,
          host: null,
          port: null,
          error: err.message,
        });
      });

      this.client.on('close', () => {
        this.connected = false;
      });
    });
  }

  disconnect(): void {
    if (this.client) {
      this.client.destroy();
      this.client = null;
    }
    this.connected = false;
  }

  send(data: string): Promise<boolean> {
    return new Promise((resolve) => {
      if (!this.client || !this.connected) {
        resolve(false);
        return;
      }

      this.client.write(data, (err) => {
        if (err) {
          console.error('PLC send error:', err);
          resolve(false);
        } else {
          resolve(true);
        }
      });
    });
  }

  getStatus(): PLCStatus {
    return {
      connected: this.connected,
      host: this.host,
      port: this.port,
      error: null,
    };
  }
}
