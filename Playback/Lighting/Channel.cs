namespace Playback
{
    class Channel
    {
        public int Index { get; }
        public ChannelType Type { get; }
        public FadeableValue<byte> Value { get; }
        public byte CorrectedValue { get { return (byte)(UncorrectedValue * Correction); } }
        public byte UncorrectedValue { get { return Value.CurrentValue; } }
        public double Correction { get; }

        public Channel(int index, ChannelType type, double correction = 1)
        {
            Index = index;
            Type = type;
            Value = new FadeableValue<byte>(0, 0, 255, CalculateChannelValueFade);
            Correction = correction;
        }

        private static byte CalculateChannelValueFade(double portionComplete, byte initialValue, byte finalValue)
        {
            // This allows us to change the value along a curve - right now, all are the same, but we could pull this out to channel creation as well
            // We can also handle up and down differently if we want
            return (byte)(initialValue + (portionComplete * portionComplete * (finalValue - initialValue)));
        }
    }
}
