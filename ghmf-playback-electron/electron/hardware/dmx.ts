import { SerialPort } from 'serialport';

interface DMXStatus {
  connected: boolean;
  port: string | null;
  error: string | null;
}

export class DMXController {
  private port: SerialPort | null = null;
  private universe: Uint8Array = new Uint8Array(513); // 0-512 (0 is start code)
  private connected: boolean = false;
  private updateInterval: NodeJS.Timeout | null = null;

  constructor() {
    // Initialize universe with all zeros
    this.universe.fill(0);
  }

  async connect(): Promise<DMXStatus> {
    try {
      // Find Enttec USB DMX Pro
      const ports = await SerialPort.list();
      const enttecPort = ports.find(
        (p) =>
          p.manufacturer?.includes('FTDI') ||
          p.productId === '6001' ||
          p.vendorId === '0403'
      );

      if (!enttecPort) {
        return {
          connected: false,
          port: null,
          error: 'Enttec USB DMX Pro not found',
        };
      }

      this.port = new SerialPort({
        path: enttecPort.path,
        baudRate: 57600,
        dataBits: 8,
        stopBits: 2,
        parity: 'none',
      });

      await new Promise<void>((resolve, reject) => {
        this.port!.on('open', () => {
          this.connected = true;
          this.startUpdating();
          resolve();
        });
        this.port!.on('error', reject);
      });

      return {
        connected: true,
        port: enttecPort.path,
        error: null,
      };
    } catch (error: any) {
      return {
        connected: false,
        port: null,
        error: error.message,
      };
    }
  }

  disconnect(): void {
    if (this.updateInterval) {
      clearInterval(this.updateInterval);
      this.updateInterval = null;
    }

    if (this.port && this.port.isOpen) {
      this.port.close();
    }

    this.connected = false;
  }

  setChannel(channel: number, value: number): void {
    if (channel < 1 || channel > 512) {
      throw new Error(`Invalid channel: ${channel} (must be 1-512)`);
    }
    if (value < 0 || value > 255) {
      throw new Error(`Invalid value: ${value} (must be 0-255)`);
    }

    this.universe[channel] = value;
  }

  setChannels(channels: Record<number, number>): void {
    for (const [channel, value] of Object.entries(channels)) {
      this.setChannel(Number(channel), value);
    }
  }

  getStatus(): DMXStatus {
    return {
      connected: this.connected,
      port: this.port?.path || null,
      error: null,
    };
  }

  private startUpdating(): void {
    // Send DMX data every 25ms (40Hz refresh rate)
    this.updateInterval = setInterval(() => {
      this.sendDMX();
    }, 25);
  }

  private sendDMX(): void {
    if (!this.port || !this.port.isOpen) {
      return;
    }

    // Enttec USB DMX Pro message format:
    // Start byte: 0x7E
    // Label: 0x06 (Output Only Send DMX Packet Request)
    // Data length LSB
    // Data length MSB
    // Data (513 bytes: start code + 512 channels)
    // End byte: 0xE7

    const dataLength = 513;
    const message = Buffer.alloc(5 + dataLength + 1);

    message[0] = 0x7e; // Start
    message[1] = 0x06; // Label
    message[2] = dataLength & 0xff; // Length LSB
    message[3] = (dataLength >> 8) & 0xff; // Length MSB
    
    // Copy universe data
    this.universe.forEach((val, idx) => {
      message[4 + idx] = val;
    });

    message[message.length - 1] = 0xe7; // End

    this.port.write(message);
  }
}
