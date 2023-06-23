namespace Playback
{
    public class CommandLine
    {
        private static char[] commandSplitChars = new[] { ' ' };

        // I used a property here to control setting
        public uint TimeInMilliseconds { get; private set; }
        // Bear in mind that despite the property the contents may change - it just can't be re-instantiated
        public Command[] Commands { get; private set; }

        public CommandLine(string unparsedLine)
        {
            Commands = new Command[0];

            Parse(unparsedLine);
        }

        public CommandLine(uint executeTime, params Command[] commands)
        {
            TimeInMilliseconds = executeTime;
            Commands = commands;
        }

        public void Parse(string unparsedLine)
        {
            // Format:
            // MM:SS.TAAA-DDD AAA-DDD AAA-DDD (etc)
            // MM: minutes timestamp
            // SS: seconds timestamp
            // T: tenths-of-a-second timestamp
            // AAA: address of device (see FCW documentation)
            // DDD: data of device (see FCW documentation)

            // Addition (2015/02/20): ignore anything between parentheses
            unparsedLine = System.Text.RegularExpressions.Regex.Replace(unparsedLine, @" ?\(.*\) ?", " ").Trim();
            if (unparsedLine.Trim() == "")
                return;

            // First we get that timestamp (minutes, seconds, tenths)
            bool includesHour = (unparsedLine.LastIndexOf(':') != 2);
            int timeStampLength = includesHour ? 10 : 7;
            string timeStamp = unparsedLine.Substring(0, timeStampLength);
            // Format it like hh:mm:ss.f so I can use the "c" specifier - the original format may be only mm:ss.f so add the hh: if necessary
            if (!includesHour)
                timeStamp = "00:" + timeStamp;
            System.TimeSpan parsedTimeStamp = System.TimeSpan.ParseExact(timeStamp, "c", System.Globalization.CultureInfo.CurrentCulture.DateTimeFormat);
            TimeInMilliseconds = (uint)parsedTimeStamp.TotalMilliseconds;

            // Now let's dump that timestamp
            string commandsOnly = unparsedLine.Substring(timeStampLength).TrimStart(' ');
            if (string.IsNullOrWhiteSpace(commandsOnly))
            {
                // It's a timestamped comment only
                return;
            }

            // Now that that's out of the way, the rest of it is AAA-DDD, separated by spaces, so we'll split into groups first
            // Now, for manual entries they might just do "AAA DDD" but we'll let them, if it's just one command
            string[] commandStrings;
            if (commandsOnly.Length <= 7)
                commandStrings = commandsOnly.Split(commandSplitChars, 1);
            else
                commandStrings = commandsOnly.Split(commandSplitChars, System.StringSplitOptions.RemoveEmptyEntries);
            if (commandStrings.Length == 1 && string.IsNullOrWhiteSpace(commandStrings[0]))
                throw new System.ArgumentException("Invalid command line " + unparsedLine + " in file");

            try
            {
                Commands = new Command[commandStrings.Length];
                for (int i = 0; i < Commands.Length; i++)
                    Commands[i] = new Command(commandStrings[i]);
            } catch (System.Exception e)
            {
                throw new System.ArgumentException("Invalid command line " + unparsedLine + " in file", e);
            }
        }
    }
}
