using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Playback
{
    public class LightConfigDefFileParser
    {
        public LightConfig[] LightConfigs { get; private set; }

        public void Parse(string lightConfigDefFile)
        {
            // I'm currently planning to just let exceptions bubble up and get caught

            // I think this is just about the only CSV reader built in to .NET (don't ask me why it's in the VB namespace)
            using (Microsoft.VisualBasic.FileIO.TextFieldParser csv = new Microsoft.VisualBasic.FileIO.TextFieldParser(lightConfigDefFile))
            {
                csv.TextFieldType = Microsoft.VisualBasic.FileIO.FieldType.Delimited;
                csv.SetDelimiters(",");

                int devNumIndex = 0;
                int devNoteIndex = 0;
                int devChannelIndex = 0;
                int devFormatIndex = 0;
                int corrFormatIndex = 0;
                string[] headers;
                // This is a do-while because we might have something above the top level (like additional labels) - matter of fact the existing document does have that
                do
                {
                    headers = csv.ReadFields();
                    for (int i = 0; i < headers.Length; i++)
                    {
                        if (headers[i].Contains("#"))
                            devNumIndex = i;
                        else if (headers[i].ToUpper().Contains("NOTE"))
                            devNoteIndex = i;
                        else if (headers[i].ToUpper().Contains("DMX"))
                            devChannelIndex = i;
                        else if (headers[i].ToUpper().Contains("FORMAT"))
                            devFormatIndex = i;
                        else if (headers[i].ToUpper().Contains("CORRECTION"))
                            corrFormatIndex = i;
                    }
                } while (devNumIndex == 0 && devChannelIndex == 0 && devFormatIndex == 0);

                List<LightConfig> lightConfigs = new List<LightConfig>();

                while (!csv.EndOfData)
                {
                    string[] fields = csv.ReadFields();

                    // First we get the light format and match it to the device channel
                    // If we ever fail to parse, just skip the line
                    if (!int.TryParse(fields[devNumIndex], out int deviceNumber) || !int.TryParse(fields[devChannelIndex], out int channelNumber))
                        continue;

                    string format = fields[devFormatIndex];

                    List<double> corrections;
                    if (corrFormatIndex == 0)
                        corrections = new List<double>(new double[format.Length]);
                    else
                    {
                        corrections = new List<double>();
                        for (int i = corrFormatIndex; i < fields.Length; i++)
                        {
                            if (!double.TryParse(fields[i], out double correctionValue))
                                continue;
                            corrections.Add(correctionValue);
                        }
                    }

                    Logger.LogDebug("Adding LightConfig, channel {0}, format {1}, corrections {2}", channelNumber, fields[devFormatIndex], string.Join(",", corrections));
                    lightConfigs.Add(new LightConfig(deviceNumber, channelNumber, fields[devFormatIndex], corrections, fields[devNoteIndex]));
                }

                // Okay, now we make an array of configs, using the light index as the index of the array
                // (We're doing this here instead of up above so they can have them out of order and it won't break)
                LightConfigs = new LightConfig[lightConfigs.Max(lc => lc.LightIndex) + 1];
                for (int i = 0; i < LightConfigs.Length; i++)
                {
                    // I could use null for nonexistent configs, but then I have to check for null everywhere
                    // So I'll just use an "empty" one (meaning it has no config string)
                    LightConfig possibleConfig = lightConfigs.Find(lc => lc.LightIndex == i);

                    if (possibleConfig != null)
                        LightConfigs[i] = possibleConfig;
                    else
                        LightConfigs[i] = new LightConfig(i, 0, "", new List<double>(), "");

                    Logger.LogDebug("Arraying LightConfig {0}, channel {1}, format {2}", LightConfigs[i].LightIndex, LightConfigs[i].StartChannel, LightConfigs[i].ConfigurationString);
                }
            }
        }
    }
}
