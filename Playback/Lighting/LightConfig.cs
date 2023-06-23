namespace Playback
{
    public class LightConfig
    {
        public string FriendlyName { get; private set; }
        public int LightIndex { get; private set; }
        public int StartChannel { get; private set; }
        public string ConfigurationString { get; private set; }
        public System.Collections.Generic.List<double> Corrections { get; private set; }

        public LightConfig(int lightIndex, int startChannel, string configString, System.Collections.Generic.List<double> corrections, string friendlyName)
        {
            FriendlyName = friendlyName;
            LightIndex = lightIndex;
            StartChannel = startChannel;
            ConfigurationString = configString;
            Corrections = corrections;
        }
    }
}
