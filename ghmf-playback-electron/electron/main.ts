import { app, BrowserWindow, ipcMain } from 'electron';
import * as path from 'path';
import { setupIPCHandlers } from './ipc/handlers';

let mainWindow: BrowserWindow | null = null;

// Get resource path based on environment
export function getResourcesPath(): string {
  if (!app.isPackaged) {
    // In development, use parent directory
    return path.join(__dirname, '../../..');
  }
  return process.resourcesPath;
}

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1600,
    height: 900,
    minWidth: 1200,
    minHeight: 700,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false,
    },
    title: 'GHMF Playback 2.0',
  });

  // Load the app
  const isDev = !app.isPackaged;
  if (isDev) {
    // In development, try port 5174 first (in case 5173 is taken), then 5173
    mainWindow.loadURL('http://localhost:5174').catch(() => {
      if (mainWindow) {
        mainWindow.loadURL('http://localhost:5173');
      }
    });
    mainWindow.webContents.openDevTools();
  } else {
    mainWindow.loadFile(path.join(__dirname, '../renderer/index.html'));
  }

  mainWindow.on('closed', () => {
    mainWindow = null;
  });
}

// App lifecycle
app.whenReady().then(() => {
  createWindow();
  setupIPCHandlers();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

// Export for IPC handlers
export function getMainWindow() {
  return mainWindow;
}
