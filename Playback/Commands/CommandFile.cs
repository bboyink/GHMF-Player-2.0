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

            using (System.IO.StreamReader controlReader = System.IO.File.OpenText(controlFile))
            {
                System.Collections.Generic.List<CommandLine> commands = new System.Collections.Generic.List<CommandLine>();
                string[] lines = (controlReader.ReadToEnd().Split(new string[] { "\r\n", "\n" }, System.StringSplitOptions.None));
                foreach (string line in lines)
                {
                    // Some versions of the old choreography software led off with a version announcement like "ct0-382" - we want to ignore lines like that
                    if (line.Length > 0 && char.IsDigit(line.Trim()[0]))
                        commands.Add(new CommandLine(line));
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
