namespace Playback
{
    public static class ClassExtensions
    {
        public static System.TimeSpan Truncate(this System.TimeSpan unroundedTime)
        {
            double totalSeconds = (int)unroundedTime.TotalSeconds;

            return System.TimeSpan.FromSeconds(totalSeconds);
        }

        public static string ToDisplayTime(this System.TimeSpan ts)
        {
            if ((int)ts.TotalHours > 0)
                return ts.ToString(@"h\:mm\:ss");
            else
                return ts.ToString(@"m\:ss");
        }
    }
}
