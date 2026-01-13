import { Howl } from 'howler';
import { getMainWindow } from '../main';

interface AudioState {
  isLoaded: boolean;
  isPlaying: boolean;
  duration: number;
  position: number;
}

export class AudioPlayer {
  private sound: Howl | null = null;
  private state: AudioState = {
    isLoaded: false,
    isPlaying: false,
    duration: 0,
    position: 0,
  };
  private updateInterval: NodeJS.Timeout | null = null;
  private leftVolume: number = 1.0;
  private rightVolume: number = 1.0;

  async load(filePath: string): Promise<{ success: boolean; duration?: number; error?: string }> {
    return new Promise((resolve) => {
      // Clean up existing sound
      if (this.sound) {
        this.sound.unload();
      }

      this.sound = new Howl({
        src: [filePath],
        html5: true, // Use HTML5 Audio for better performance with large files
        preload: true,
        onload: () => {
          this.state.isLoaded = true;
          this.state.duration = this.sound?.duration() || 0;
          resolve({
            success: true,
            duration: this.state.duration,
          });
        },
        onloaderror: (_id: number, error: any) => {
          console.error('Audio load error:', error);
          resolve({
            success: false,
            error: String(error),
          });
        },
        onend: () => {
          this.state.isPlaying = false;
          this.stopUpdating();
          const mainWindow = getMainWindow();
          if (mainWindow) {
            mainWindow.webContents.send('audio:ended');
          }
        },
      });
    });
  }

  play(): boolean {
    if (!this.sound || !this.state.isLoaded) {
      return false;
    }

    this.sound.play();
    this.state.isPlaying = true;
    this.startUpdating();
    return true;
  }

  pause(): boolean {
    if (!this.sound || !this.state.isLoaded) {
      return false;
    }

    this.sound.pause();
    this.state.isPlaying = false;
    this.stopUpdating();
    return true;
  }

  stop(): boolean {
    if (!this.sound || !this.state.isLoaded) {
      return false;
    }

    this.sound.stop();
    this.state.isPlaying = false;
    this.state.position = 0;
    this.stopUpdating();
    return true;
  }

  seek(position: number): boolean {
    if (!this.sound || !this.state.isLoaded) {
      return false;
    }

    this.sound.seek(position);
    this.state.position = position;
    return true;
  }

  setVolume(left: number, right: number): boolean {
    if (!this.sound) {
      return false;
    }

    this.leftVolume = Math.max(0, Math.min(1, left));
    this.rightVolume = Math.max(0, Math.min(1, right));

    // Average for stereo control (Howler doesn't support per-channel volume)
    const avgVolume = (this.leftVolume + this.rightVolume) / 2;
    this.sound.volume(avgVolume);

    return true;
  }

  getPosition(): number {
    if (!this.sound || !this.state.isLoaded) {
      return 0;
    }

    return this.sound.seek() as number;
  }

  getDuration(): number {
    return this.state.duration;
  }

  private startUpdating(): void {
    // Update position every 50ms
    this.updateInterval = setInterval(() => {
      if (this.sound && this.state.isPlaying) {
        this.state.position = this.sound.seek() as number;
        const mainWindow = getMainWindow();
        if (mainWindow) {
          mainWindow.webContents.send('audio:timeUpdate', this.state.position);
        }
      }
    }, 50);
  }

  private stopUpdating(): void {
    if (this.updateInterval) {
      clearInterval(this.updateInterval);
      this.updateInterval = null;
    }
  }
}
