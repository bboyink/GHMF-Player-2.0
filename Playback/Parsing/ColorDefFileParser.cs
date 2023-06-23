using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Playback
{
    public class ColorDefFileParser
    {
        public LEDColor[] Colors { get; private set; }

        public void Parse(string colorDefFile)
        {
            // I'm currently planning to just let exceptions bubble up and get caught

            // I think this is just about the only CSV reader built in to .NET (don't ask me why it's in the VB namespace)
            using (Microsoft.VisualBasic.FileIO.TextFieldParser csv = new Microsoft.VisualBasic.FileIO.TextFieldParser(colorDefFile))
            {
                csv.TextFieldType = Microsoft.VisualBasic.FileIO.FieldType.Delimited;
                csv.SetDelimiters(",");

                int colorIndex = 0;
                int rgbIndex = 0;
                int descIndex = 0;
                string[] headers;
                // This is a do-while because we might have something above the top level (like additional labels) - matter of fact the existing document does have that
                do
                {
                    headers = csv.ReadFields();
                    for (int i = 0; i < headers.Length; i++)
                    {
                        if (headers[i].ToUpper().Contains("INDEX"))
                            colorIndex = i;
                        else if (headers[i].ToUpper().Contains("COLOR"))
                            rgbIndex = i;
                        else if (headers[i].ToUpper().Contains("DESC"))
                            descIndex = i;
                    }
                } while (colorIndex == 0 && rgbIndex == 0);

                List<LEDColor> ledColors = new List<LEDColor>();

                while (!csv.EndOfData)
                {
                    string[] fields = csv.ReadFields();

                    // First we get the light format and match it to the device channel
                    // If we ever fail to parse, just skip the line
                    int colorNumber;
                    if (!int.TryParse(fields[colorIndex], out colorNumber))
                        continue;

                    Logger.LogDebug("Adding LEDColor, index {0}, value {1}", colorNumber, fields[rgbIndex]);
                    ledColors.Add(new LEDColor(colorNumber, fields[rgbIndex], fields[descIndex]));
                }

                // Okay, now we make an array of configs, using the color index as the index of the array
                Colors = new LEDColor[ledColors.Max(ledc => ledc.Index) + 1];
                for (int i = 0; i < Colors.Length; i++)
                {
                    // If there's no color defined, use black
                    LEDColor possibleColor = ledColors.Find(c => c.Index == i);

                    if (possibleColor != null)
                        Colors[i] = possibleColor;
                    else
                        Colors[i] = LEDColor.Black;

                    if (possibleColor.Description.ToUpper().Contains("CURTAIN") && possibleColor.Description.Contains("16"))
                        LEDColor.BackCurtain16 = possibleColor;
                    if (possibleColor.Description.ToUpper().Contains("CURTAIN") && possibleColor.Description.Contains("32"))
                        LEDColor.BackCurtain32 = possibleColor;
                    if (possibleColor.Description.ToUpper().Contains("CURTAIN") && possibleColor.Description.Contains("48"))
                        LEDColor.BackCurtain48 = possibleColor;
                    if (possibleColor.Description.ToUpper().Contains("VOICE"))
                        LEDColor.VoiceSlashSpout = possibleColor;

                    Logger.LogDebug("Arraying LEDColor {0} ({1}), red {2}, green {3}, blue {4}, white {5}", Colors[i].Index, Colors[i].Description, Colors[i].R, Colors[i].G, Colors[i].B, Colors[i].W);
                }
            }
        }
    }
}
