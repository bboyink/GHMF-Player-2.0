using System;

namespace Playback
{
    public class FadeableValue<T> where T : IComparable
    {
        public readonly T MaxValue;
        public readonly T MinValue;

        private T currentValue;
        public T CurrentValue
        {
            get
            {
                return currentValue;
            }
            set
            {
                if (Comparer.Compare(value, MinValue) < 0 || Comparer.Compare(value, MaxValue) > 0)
                    throw new ArgumentOutOfRangeException("Fadeable value must be between " + MinValue + " and " + MaxValue);
                FadeTimer.Reset();
                InitialValue = FinalValue = currentValue = value;
            }
        }
        private T InitialValue { get; set; }
        private T FinalValue { get; set; }
        private int MSToCompletion { get; set; }
        private System.Diagnostics.Stopwatch FadeTimer { get; set; }

        private System.Collections.Generic.Comparer<T> Comparer;
        private Func<double, T, T, T> NewValCalculator;

        public FadeableValue(T value, T minValue, T maxValue, Func<double, T, T, T> calculateCurrent)
        {
            FadeTimer = new System.Diagnostics.Stopwatch();
            Comparer = System.Collections.Generic.Comparer<T>.Default;
            NewValCalculator = calculateCurrent;

            MinValue = minValue;
            MaxValue = maxValue;

            if (Comparer.Compare(MaxValue, MinValue) <= 0)
                throw new ArgumentOutOfRangeException("Fadeable max value (" + maxValue + ") must not be less than min value (" + minValue + ")");

            CurrentValue = value;
        }

        public void Fade(T finalValue, int timeInMS)
        {
            Fade(CurrentValue, finalValue, timeInMS);
        }

        public void Fade(T initialValue, T finalValue, int timeInMS)
        {
            if (Comparer.Compare(initialValue, MinValue) < 0 || Comparer.Compare(initialValue, MaxValue) > 0)
                throw new ArgumentOutOfRangeException("Fadeable initial value must be between " + MinValue + " and " + MaxValue);
            if (Comparer.Compare(finalValue, MinValue) < 0 || Comparer.Compare(finalValue, MaxValue) > 0)
                throw new ArgumentOutOfRangeException("Fadeable final value must be between " + MinValue + " and " + MaxValue);

            InitialValue = CurrentValue = initialValue;
            FinalValue = finalValue;
            MSToCompletion = timeInMS;
            FadeTimer.Restart();
            RefreshInternal(true);

            Logger.LogDebug("Beginning fade from {0} to {1} over {2} ms", initialValue, finalValue, timeInMS);
        }

        public bool Refresh()
        {
            return RefreshInternal(false);
        }

        private bool RefreshInternal(bool firstRefresh)
        {
            if (!FadeTimer.IsRunning)
                return false;

            double completion = (FadeTimer.ElapsedMilliseconds / (double)MSToCompletion);
            if (completion > 1) completion = 1;

            currentValue = NewValCalculator(completion, InitialValue, FinalValue);

            if (completion >= 1)
            {
                // Note that I'm setting the property - this has the nice side effect of clearing out the refresh stuff
                CurrentValue = currentValue;
                Logger.LogDebug("Finishing fade at {0}", currentValue);
                return true;
            }

            // We return true if we've just moved out of black (above) or we've just finished (maybe fading *to* black)
            return firstRefresh;
        }
    }
}
