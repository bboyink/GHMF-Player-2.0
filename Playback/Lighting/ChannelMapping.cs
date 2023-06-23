namespace Playback
{
    static class ChannelMapping
    {
        private static System.Collections.Generic.Dictionary<char, ChannelType> map;

        static ChannelMapping()
        {
            // I'm allowing upper and lower case to mean the same thing, except for pan/tilt which also have a "fine" entry
            map = new System.Collections.Generic.Dictionary<char, ChannelType>
            {
                { 'U', ChannelType.Undefined },
                { 'u', ChannelType.Undefined },
                { 'X', ChannelType.Raw },
                { 'x', ChannelType.Raw },
                { 'R', ChannelType.Red },
                { 'r', ChannelType.Red },
                { 'G', ChannelType.Green },
                { 'g', ChannelType.Green },
                { 'B', ChannelType.Blue },
                { 'b', ChannelType.Blue },
                { 'A', ChannelType.Amber },
                { 'a', ChannelType.Amber },
                { 'W', ChannelType.White },
                { 'w', ChannelType.White }
            };
        }

        public static ChannelType Map(char key)
        {
            try
            {
                return map[key];
            }
            catch (System.Collections.Generic.KeyNotFoundException e)
            {
                // Wrap it up differently for the user
                throw new System.ArgumentException("Key letter " + key + " not mapped internally. Recognized keys are: " + string.Join(", ", map.Keys), e);
            }
        }
    }
}
