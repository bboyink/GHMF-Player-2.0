namespace Playback
{
    public class LightModule
    {
        public readonly string Name;
        public readonly int[] LightIndices;
        public readonly string[] LightNames;

        public LightModule(string name, System.Collections.Generic.IEnumerable<int> lightIndices, System.Collections.Generic.IEnumerable<string> lightNames = null)
        {
            Name = name;
            LightIndices = System.Linq.Enumerable.ToArray(lightIndices);
            if (lightNames != null)
            {
                LightNames = System.Linq.Enumerable.ToArray(lightNames);
            }
            else
            {
                LightNames = new string[LightIndices.Length];
                for (int i = 0; i < LightNames.Length; i++)
                {
                    LightNames[i] = "";
                }
            }
        }
    }
}
