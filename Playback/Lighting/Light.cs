using System.Linq;

namespace Playback
{
    class Light
    {
        public static double LinearizeIntensity(double intensity)
        {
            return intensity * intensity;
        }

        public string FriendlyName { get; private set; }

        private readonly Channel[] unlockedChannels;
        private readonly Channel[] lockedChannels;
        public Channel[] Channels { get { if (Locked || FirmlyLocked) return lockedChannels; else return unlockedChannels; } }
        private readonly FadeableValue<double> unlockedIntensity;
        private readonly FadeableValue<double> lockedIntensity;
        public FadeableValue<double> Intensity { get { if (Locked || FirmlyLocked) return lockedIntensity; else return unlockedIntensity; } }
        private bool locked;
        public bool Locked
        {
            get { return locked; }
            set { locked = value; RegenColor(); }
        }
        private bool firmlyLocked;
        public bool FirmlyLocked
        {
            get { return firmlyLocked; }
            set { firmlyLocked = value; RegenColor(); }
        }
        public bool HasRaw { get; private set; }
        public bool IsSpecialDMX { get; private set; }
        public LEDColor Color { get; private set; }

        public Light(LightConfig config)
        {
            locked = false;

            FriendlyName = config.FriendlyName;

            // For better or worse, I'm using a startIndex of 0 to denote an "empty" light config - since in DMX 0 is a reserved channel
            if (config.StartChannel != 0 && config.ConfigurationString.Length == 0)
                throw new System.ArgumentException("No channel string found for light at index " + config.StartChannel);

            unlockedChannels = new Channel[config.ConfigurationString.Length];
            lockedChannels = new Channel[config.ConfigurationString.Length];
            for (int i = 0; i < unlockedChannels.Length; i++)
            {
                int channelNumber = config.StartChannel + i;
                ChannelType mappedType = ChannelMapping.Map(config.ConfigurationString[i]);
                double corrections = config.Corrections.Count > i ? config.Corrections[i] : 1;
                unlockedChannels[i] = new Channel(channelNumber, mappedType, config.Corrections[i]);
                lockedChannels[i] = new Channel(channelNumber, mappedType, config.Corrections[i]);
            }

            HasRaw = unlockedChannels.Any(ch => ch.Type == ChannelType.Raw);
            IsSpecialDMX = unlockedChannels.Any(ch => ch.Type == ChannelType.DMX);

            unlockedIntensity = new FadeableValue<double>(1, 0, 1, CalculateIntensityFade);
            lockedIntensity = new FadeableValue<double>(1, 0, 1, CalculateIntensityFade);

            RegenColor();

            Logger.LogDebug("Successfully mapped light {0} with channels {1}", config.StartChannel, string.Join(", ", Channels.Select(ch => ch.Type.ToString())));
        }

        private static double CalculateIntensityFade(double portionComplete, double initialValue, double finalValue)
        {
            // This also allows us to change our fading curve if need be
            return (initialValue + (portionComplete * (finalValue - initialValue)));
        }

        public void SetColor(double intensity, LEDColor color, bool lockColor = false)
        {
            if (color == null) return;
            // If this is a back curtain color set, then I can only change it from now on with a different back curtain color set
            if (lockColor)
                Locked = lockColor;

            SetIntensityInternal(intensity, color, lockColor);

            SetChannelInternal(ChannelType.Red, color.R, lockColor);
            SetChannelInternal(ChannelType.Green, color.G, lockColor);
            SetChannelInternal(ChannelType.Blue, color.B, lockColor);
            SetChannelInternal(ChannelType.White, color.W, lockColor);
        }

        public void SetRawValue(byte value)
        {
            SetChannelInternal(ChannelType.Raw, value, false);
        }

        public void Fade(double targetIntensity, LEDColor targetColor, bool lockColor, int msPerRefresh, int totalFadeMS)
        {
            Intensity.Fade(targetIntensity, totalFadeMS);

            SetChannelInternal(ChannelType.Red, targetColor.R, lockColor, true, totalFadeMS);
            SetChannelInternal(ChannelType.Green, targetColor.G, lockColor, true, totalFadeMS);
            SetChannelInternal(ChannelType.Blue, targetColor.B, lockColor, true, totalFadeMS);
            SetChannelInternal(ChannelType.White, targetColor.W, lockColor, true, totalFadeMS);
        }

        private void SetIntensityInternal(double newValue, LEDColor newColor, bool lockColor, bool fade = false, int totalFadeMS = 0)
        {
            FadeableValue<double> intensitySource;
            if (lockColor)
                intensitySource = lockedIntensity;
            else
                intensitySource = unlockedIntensity;
            if (newValue >= 0)
            {
                if (fade)
                    intensitySource.Fade(newValue, totalFadeMS);
                else
                    intensitySource.CurrentValue = newValue;
            }
        }

        private void SetChannelInternal(ChannelType type, byte newValue, bool lockColor, bool fade = false, int totalFadeMS = 0)
        {
            if (Channels.Any(ch => ch.Type == type))
            {
                // Just in case we have two red channels or something, we loop through all channels of the given type
                Channel[] channelSource;
                if (lockColor)
                    channelSource = lockedChannels;
                else
                    channelSource = unlockedChannels;
                foreach (Channel updateChannel in channelSource.Where(ch => ch.Type == type))
                {
                    if (fade)
                    {
                        Logger.LogDebug("Beginning fade on {0} channel {1}", (lockColor ? "Locked" : "Unlocked"), updateChannel.Index);
                        updateChannel.Value.Fade(newValue, totalFadeMS);
                    }
                    else
                    {
                        updateChannel.Value.CurrentValue = newValue;
                        Logger.LogDebug("{0} channel {1} value changed to {2}", (lockColor ? "Locked" : "Unlocked"), updateChannel.Index, updateChannel.Value.CurrentValue);
                    }
                }

                RegenColor();
            }
        }

        private byte GetValue(ChannelType channel, bool removeCorrection = false)
        {
            Channel retrievedChannel = Channels.FirstOrDefault(ch => ch.Type == channel);
            if (retrievedChannel == null)
                return 0;
            else
                return removeCorrection ? retrievedChannel.UncorrectedValue : retrievedChannel.CorrectedValue;
        }

        public void Refresh()
        {
            bool refreshHappened = Intensity.Refresh();
            foreach (Channel channel in Channels)
                refreshHappened |= channel.Value.Refresh();

            // If a refresh has just started or just finished, it's time to update our color
            // (This will cause the monitor to be incorrect (briefly) if a fade is interrupted, but the monitor is a luxury anyways
            if (refreshHappened)
                RegenColor();
        }

        public void RegenColor()
        {
            if (Intensity.CurrentValue == 0)
                Color = LEDColor.Black;
            else
                Color = new LEDColor(
                    GetValue(ChannelType.Red, true),
                    GetValue(ChannelType.Green, true),
                    GetValue(ChannelType.Blue, true),
                    GetValue(ChannelType.White, true),
                    GetValue(ChannelType.Amber, true));
        }
    }
}
