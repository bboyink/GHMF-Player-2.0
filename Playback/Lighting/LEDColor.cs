namespace Playback
{
    public class LEDColor
    {
        public static readonly LEDColor Black = new LEDColor(0, 0, 0, 0, 0);
        public static readonly LEDColor White = new LEDColor(255, 255, 255, 255, 0);

        public static LEDColor BackCurtain16 = new LEDColor(0, 255, 0, 0, 0); // Green
        public static LEDColor BackCurtain32 = new LEDColor(255, 255, 0, 0, 0); // Yellow
        public static LEDColor BackCurtain48 = new LEDColor(128, 255, 0, 0, 0); // Yellow-green
        public static LEDColor VoiceSlashSpout = new LEDColor(255, 255, 255, 255, 0); // White

        public string Description { get; private set; }

        public int Index { get; private set; }

        public byte R { get; private set; }
        public byte G { get; private set; }
        public byte B { get; private set; }
        public byte W { get; private set; }
        //public byte A { get; private set; }

        public LEDColor(byte red, byte green, byte blue, byte white = 0, byte amber = 0, int index = 0, string description = "")
        {
            R = red;
            G = green;
            B = blue;
            W = white;
            //A = amber;
            Index = index;
            Description = description;
        }

        public LEDColor(int index, string rgbString, string description)
        {
            Description = description;

            Index = index;

            // Make sure the string is 6 digits (000000 gets stored as 0 if you use Excel)
            rgbString = rgbString.PadLeft(6, '0');

            // Grab pairs from the string
            string[] parts = new string[rgbString.Length / 2];
            for (int i = 0; i < parts.Length; i++)
            {
                if (rgbString.Length > i * 2 + 1)
                    parts[i] = rgbString.Substring(i * 2, 2);
            }

            // Assuming order is RGB (quite standard)
            R = byte.Parse(parts[0], System.Globalization.NumberStyles.AllowHexSpecifier);
            G = byte.Parse(parts[1], System.Globalization.NumberStyles.AllowHexSpecifier);
            B = byte.Parse(parts[2], System.Globalization.NumberStyles.AllowHexSpecifier);

            // Special case: if they want white, we give them true white if we've got it
            if (R == 255 && G == 255 && B == 255)
                W = 255;
        }

        public override string ToString()
        {
            return R.ToString("00") + G.ToString("00") + B.ToString("00");
        }

        public override bool Equals(object obj)
        {
            if (!(obj is LEDColor)) return false;

            return this == (LEDColor)obj;
        }

        public static bool operator ==(LEDColor a, LEDColor b)
        {
            // If both are null, or both are same instance, return true.
            if (System.Object.ReferenceEquals(a, b))
            {
                return true;
            }

            // If one is null, but not both, return false.
            if (((object)a == null) || ((object)b == null))
            {
                return false;
            }

            return a.R == b.R && a.G == b.G && a.B == b.B && a.W == b.W;
        }

        public static bool operator !=(LEDColor a, LEDColor b)
        {
            return !(a == b);
        }

        public override int GetHashCode()
        {
            unchecked // Overflow is fine, just wrap
            {
                int hash = (int)2166136261;
                hash = hash * 16777619 ^ R.GetHashCode();
                hash = hash * 16777619 ^ G.GetHashCode();
                hash = hash * 16777619 ^ B.GetHashCode();
                hash = hash * 16777619 ^ W.GetHashCode();
                return hash;
            }
        }
    }
}
