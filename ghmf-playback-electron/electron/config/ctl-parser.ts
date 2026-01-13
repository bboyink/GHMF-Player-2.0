import * as fs from 'fs-extra';

interface CTLCommand {
  time: number; // Time in seconds
  type: string; // Command type
  params: Record<string, any>; // Command parameters
}

interface CTLFile {
  filePath: string;
  commands: CTLCommand[];
  duration: number;
}

export class CTLParser {
  async parse(filePath: string): Promise<CTLFile> {
    try {
      const content = await fs.readFile(filePath, 'utf-8');
      const lines = content.split('\n').filter((line) => line.trim() !== '');

      const commands: CTLCommand[] = [];
      let maxTime = 0;

      for (const line of lines) {
        const trimmed = line.trim();
        
        // Skip comments
        if (trimmed.startsWith('//') || trimmed.startsWith('#')) {
          continue;
        }

        // Parse command format: TIME TYPE PARAMS
        // Example: 0.0 LIGHT fixture=1 r=255 g=0 b=0 w=0
        // Example: 5.5 PLC message="START_SHOW"
        const parts = trimmed.split(/\s+/);
        
        if (parts.length < 2) {
          continue;
        }

        const time = parseFloat(parts[0]);
        const type = parts[1].toUpperCase();
        const params: Record<string, any> = {};

        // Parse key=value parameters
        for (let i = 2; i < parts.length; i++) {
          const paramParts = parts[i].split('=');
          if (paramParts.length === 2) {
            let value: any = paramParts[1];
            
            // Remove quotes from strings
            if (value.startsWith('"') && value.endsWith('"')) {
              value = value.slice(1, -1);
            } else if (!isNaN(Number(value))) {
              // Convert to number if possible
              value = Number(value);
            }
            
            params[paramParts[0]] = value;
          }
        }

        commands.push({ time, type, params });
        maxTime = Math.max(maxTime, time);
      }

      // Sort commands by time
      commands.sort((a, b) => a.time - b.time);

      return {
        filePath,
        commands,
        duration: maxTime,
      };
    } catch (error) {
      console.error('Error parsing CTL file:', error);
      return {
        filePath,
        commands: [],
        duration: 0,
      };
    }
  }
}
