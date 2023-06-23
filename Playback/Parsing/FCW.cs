using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Playback
{
    [Flags]
    public enum FCWType
    {
        None = 0,
        Water = 1,
        Light = 2,
        Special = 4
    }

    public enum FCWLightRole
    {
        None,
        TurnOnOff,
        Fade
    }

    public enum SpecialFCWAddress
    {
        // I'm sad to have to do this, but some lights need to be off if the spout is at 0
        WaterSpout = 7,

        // These ones define our modules - whatever they affect determines what the modules are
        Module1 = 17,
        Module2 = 18,
        Module3 = 19,
        Module4 = 20,
        Module5 = 21,
        Module6 = 22,
        Module7 = 23,
        Peacock = 27,
        All = 53,
        Voice = 54,
        Helix = 11,
        BackCurtain = 24,
        ModulesA = 49,
        ModulesB = 50,

        // And these are our special commands - usually ones that have strange hardcoded meanings
        SwapAandB = 80,
        ShiftRotate = 85,
        SetShiftRotateTimer = 86,
        Reset = 99
    }

    public class FCW
    {
        public int Address;
        public FCWType Type;
        public FCWLightRole Role;
        public int[] AffectedLights; // Only relevant if this is a Light type FCW

        public FCW(int address, FCWType type, FCWLightRole role, params int[] affectedLights)
        {
            Address = address;
            Type = type;
            Role = role;
            AffectedLights = affectedLights;
        }
    }
}
