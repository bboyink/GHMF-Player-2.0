using System.Linq;

namespace Playback
{
    public class CommandFile
    {
        public CommandLine[] Commands { get; private set; }

        private int currentLine = 0;

        public CommandFile(string controlFile)
        {
            Parse(controlFile);
        }

        public CommandFile(CommandLine[] commands)
        {
            Commands = commands.OrderBy(cl => cl.TimeInMilliseconds).ToArray();
        }

        private void Parse(string controlFile)
        {
            if (!System.IO.File.Exists(controlFile))
                throw new System.ArgumentException("File '" + controlFile + "' not found.");

            currentLine = 0;

            bool remap = false;
            int[] fixtures = {
                1, 2, 3, 4, 6, // mod 1: 1-5
                7, 8, 9, 10, 12, // mod 2: 6-10
                13, 14, 15, 16, 18, //mod 3: 11-15
                19, 20, 21, 22, 23, // mod 4: 16-20
                24, 25, 26, 27, 29, // mod 5: 21-25
                30, 31, 32, 33, 35, // mod 6: 29-30
                36, 37, 38, 39, 41, // mod 7: 31-35
                42, 43, 44, 45, 46, 47, // peacock front: 36-41
                50, 51, // spout: 42-43
                52, 53, // doves: 44-45
                5, 11, 17, 28, 34, 40, // mod back center, skip mod 4: none
                48, 49 // peacock back: none
            };
            string oldFCW;
            string newFCW;
            string FCWs;

            using (System.IO.StreamReader controlReader = System.IO.File.OpenText(controlFile))
            {
                System.Collections.Generic.List<CommandLine> commands = new System.Collections.Generic.List<CommandLine>();
                string[] lines = (controlReader.ReadToEnd().Split(new string[] { "\r\n", "\n" }, System.StringSplitOptions.None));
                foreach (string line in lines)
                {
                    if (line.Length > 0 && char.IsDigit(line.Trim()[0]))
                    {
                        FCWs = line;
                        if (remap)
                        {
                            for (int i = 500; i < 700; i += 100) // remap 500s and 600s
                            {
                                for (int j = 1; j < 54; j++)
                                {
                                    oldFCW = $"{(i + j).ToString()}-";
                                    newFCW = $"{(i == 500 ? "5X" : "6X")}{(fixtures[j - 1]).ToString("D2")}-"; // use X to avoid recursive replacements
                                    FCWs = FCWs.Replace(oldFCW, newFCW);
                                }
                            }
                            FCWs = FCWs.Replace("X", ""); // get rid of any Xs we inserted
                        }
                        commands.Add(new CommandLine(FCWs));
                    }
                    else
                    {
                        // Some versions of the old choreography software led off with a version announcement like "ct0-382" - we want to ignore lines like that
                        // If the line doesn't start with a digit, throw it away after checking for special situations
                        if (line.ToLower().Contains("created with ghmf"))
                            remap = true; // Java choreographer scripts need remapping of fixtures
                    }
                }

                // Juuuuust in case, since we can, let's make sure the commands are in increasing time order
                Commands = commands.OrderBy(cl => cl.TimeInMilliseconds).ToArray();
            }
        }

        public CommandLine GetCurrentLine()
        {
            if (currentLine < Commands.Length)
                return Commands[currentLine];
            else
                return null;
        }

        public void NextLine()
        {
            currentLine++;
        }
    }
}
