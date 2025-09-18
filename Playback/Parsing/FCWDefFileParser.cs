using System.Collections.Generic;
using System.Linq;

namespace Playback
{
    public class FCWDefFileParser
    {
        public FCW[] FCWs { get; private set; }

        private const int WaterCommand = -1;

        private static readonly FCW[] specialFCWs = new[]
        {
            new FCW(80, FCWType.Light | FCWType.Water | FCWType.Special, FCWLightRole.TurnOnOff), // Swap A and B lights
            new FCW(85, FCWType.Light | FCWType.Water | FCWType.Special, FCWLightRole.TurnOnOff), // Shift/rotate lights and water
            new FCW(86, FCWType.Light | FCWType.Water | FCWType.Special, FCWLightRole.None), // Set timer for 85
            new FCW(99, FCWType.Light | FCWType.Water | FCWType.Special, FCWLightRole.TurnOnOff) // Maintenance stuff (mostly stop everything)
        };

        private void CreateFCWArray(int numEntries)
        {
            if (FCWs == null)
                FCWs = new FCW[numEntries];
            else
            {
                // If we're too short we're going to run into some problems, so size up
                // Otherwise, we're already all set
                if (FCWs.Length < numEntries)
                {
                    FCW[] newFCWs = FCWs;
                    System.Array.Resize(ref newFCWs, numEntries);
                    FCWs = newFCWs;
                }
            }
        }

        public void SetSpecialFCWs()
        {
            CreateFCWArray(specialFCWs.Max(fcw => fcw.Address) + 1);

            // Overwrite - don't let them redefine the special commands
            foreach (FCW fcw in specialFCWs)
            {
                FCWs[fcw.Address] = fcw;
                Logger.LogDebug("Adding special FCW {0}", fcw.Address);
            }
        }

        public void ParseUserConfig(string file)
        {
            // I'm currently planning to just let exceptions bubble up and get caught

            // I think this is just about the only CSV reader built in to .NET (don't ask me why it's in the VB namespace)
            using (Microsoft.VisualBasic.FileIO.TextFieldParser csv = new Microsoft.VisualBasic.FileIO.TextFieldParser(file))
            {
                csv.TextFieldType = Microsoft.VisualBasic.FileIO.FieldType.Delimited;
                csv.SetDelimiters(",");

                // Changes required here
                int fcwNumIndex = 0;
                bool set = false;
                string[] headers;
                // This is a do-while because we might have something above the top level (like additional labels) - matter of fact the existing document does have that
                do
                {
                    headers = csv.ReadFields();
                    for (int i = 0; i < headers.Length; i++)
                    {
                        if (headers[i].Contains("FCW"))
                        {
                            fcwNumIndex = i;
                            set = true;
                            break;
                        }
                    }
                } while (!set);

                List<FCW> fcws = new List<FCW>();

                int[] lightNumbers = new int[headers.Length];
                int outInt;
                for (int i = 0; i < headers.Length; i++)
                    if (int.TryParse(headers[i], out outInt))
                        lightNumbers[i] = outInt; // Remember, 0 is not a valid FCW
                    else if (headers[i].ToUpper().Contains("WATER"))
                        lightNumbers[i] = WaterCommand; // Little bit of a hack here to allow them to specify water commands too

                while (!csv.EndOfData)
                {
                    string[] fields = csv.ReadFields();

                    // Get the FCW number and the number of all the marked lights
                    // If we ever fail to parse, just skip the line
                    int fcwNumber;
                    if (!int.TryParse(fields[fcwNumIndex], out fcwNumber))
                        continue;

                    List<int> affectedLights = new List<int>();
                    FCWLightRole role = FCWLightRole.None;
                    FCWType type = FCWType.None;
                    for (int i = 0; i < fields.Length; i++)
                        if (!string.IsNullOrWhiteSpace(fields[i]) && lightNumbers[i] != 0)
                        {
                            if (lightNumbers[i] == WaterCommand)
                                type |= FCWType.Water;
                            else
                            {
                                type |= FCWType.Light;
                                affectedLights.Add(lightNumbers[i]);
                                // The first light we hit will determine our role
                                if (role == FCWLightRole.None)
                                {
                                    if (fields[i].ToUpper().Contains("F"))
                                        role = FCWLightRole.Fade;
                                    else if (fields[i].ToUpper().Contains("D"))
                                        role = FCWLightRole.SpecialDMX;
                                    else
                                        role = FCWLightRole.TurnOnOff;
                                }
                            }
                        }

                    fcws.Add(new FCW(fcwNumber, type, role, affectedLights.ToArray()));
                }

                // Okay, now we make an array of fcws, using the address as the index of the array
                // (We're doing this here instead of up above so they can have missing and out of order entries and it won't break)
                CreateFCWArray(fcws.Max(fcw => fcw.Address) + 1);

                for (int i = 0; i < FCWs.Length; i++)
                {
                    FCWs[i] = fcws.Find(fcw => fcw.Address == i);

                    if (FCWs[i] != null)
                        Logger.LogDebug("Adding FCW {0} of type {1} with role {2}, affecting light(s) {3}", FCWs[i].Address, FCWs[i].Type, FCWs[i].Role, string.Join(", ", FCWs[i].AffectedLights));
                }
            }
        }
    }
}
