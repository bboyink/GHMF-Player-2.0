using System.Drawing;
using System.Linq;

namespace Playback
{
    // These are the expected types of channel - Amber is not fully supported as of v1.0
    public enum ChannelType
    {
        Undefined,
        Red,
        Green,
        Blue,
        Amber,
        White,
        Raw,
        DMX
    }

    public class Lighting : System.IDisposable
    {
        private readonly bool enabled;

        public string[] LightNames { get { return lights.Select(l => l.FriendlyName).ToArray(); } }
        // The LightGroups and UngroupedLights are only called on initialization so I'm not worried about making them fast
        public LightModule[] LightGroups
        {
            get
            {
                return modules1to7.Concat(additionalModules).Concat(new[] { UngroupedLights }).ToArray();
            }
        }
        public LightModule UngroupedLights
        {
            get
            {
                System.Collections.Generic.IEnumerable<int> takenLights = modules1to7.Concat(additionalModules).SelectMany(i => i.LightIndices).Distinct();
                // If it's in this list, then it's not ungrouped; if it's null, then it's not ungrouped; if it's light 0 (which is invalid), then it's not ungrouped
                return new LightModule("Ungrouped", lights.Select((light, index) => new { light, index }).Where(l => l.light != null && l.index != 0 && l.light.Channels.Length > 0 && !l.light.HasRaw && !takenLights.Contains(l.index)).Select(l => l.index));
            }
        }
        public string CurrentMotion { get; private set; }

        private readonly Light[] lights;

        private readonly LightModule[] modules1to7;
        private readonly LightModule[] additionalModules;
        private readonly LightModule[] modulesAandB;

        private bool rightShiftActive = false;
        private bool rotateShiftActive = false;
        private int shifts;

        private System.Timers.Timer shiftTimer;

        public bool DMXConnected { get { return !enabled || (!OpenDMX.done && OpenDMX.handle != 0); } }
        private System.Threading.Thread reconnectThread = null;
        public delegate void DMXConnectionChangedEventHandler(object sender, bool connected);
        public event DMXConnectionChangedEventHandler DMXConnectionChanged;

        // This is a silly constructor and I should probably refactor it to one big int[][] or a class container or something
        public Lighting(LightConfig[] lightConfigs, LightModule[] numberedModules, LightModule[] otherModules, LightModule[] letteredModules, bool disabled)
        {
            shiftTimer = new System.Timers.Timer();
            shiftTimer.Elapsed += shiftTimer_Elapsed;
            shiftTimer.Stop();

            enabled = !disabled;

            if (lightConfigs.Length == 0)
                throw new System.ArgumentException("No lights were found in the configuration");

            lights = new Light[lightConfigs.Length];
            for (int i = 0; i < lightConfigs.Length; i++)
                if (lightConfigs[i] != null)
                    lights[i] = new Light(lightConfigs[i]);

            modules1to7 = numberedModules;
            additionalModules = otherModules;
            modulesAandB = letteredModules;

            // Verify we don't have any overlapping channels (like a 3-channel light on 1(,2,3) then a 3-channel light on 3)
            // Isn't LINQ fun? We're collapsing the nested list of lights->channels into one long list then seeing if there's more than one with the same index
            System.Collections.Generic.IEnumerable<Channel> overlaps = lights.Where(l => l != null).SelectMany(l => l.Channels).GroupBy(ch => ch.Index).Where(g => g.Count() > 1).Select(g => g.First());
            if (overlaps.Count() > 0)
                throw new System.NotSupportedException("Overlap on channel(s) " + string.Join(", ", overlaps.Select(c => c.Index)));
        }

        // This guy is called on every update, which is why I have each light cache its color
        public LEDColor GetLightColor(int index) { return lights[index].Color; }

        void shiftTimer_Elapsed(object sender, System.Timers.ElapsedEventArgs e)
        {
            ShiftLights();
        }

        public void Dispose()
        {
            DisconnectDMX();
            if (shiftTimer != null)
                shiftTimer.Dispose();
        }

        #region Refresh

        public void Refresh()
        {
            ExecuteFade();
        }

        private void ExecuteFade()
        {
            for (int i = 0; i < lights.Length; i++)
            {
                lights[i].Refresh();
                UpdateMessage(i);
            }
        }

        #endregion

        #region DMXConnection

        // There's been talk of moving away from this DMX implementation and over to an Ethernet-based one
        // I've found this which might be helpful if we get there:
        // https://edmx.codeplex.com/SourceControl/latest#eDMX/ArtNet/Engine.cs

        public void ConnectDMX()
        {
            if (enabled)
            {
                OpenDMX.start();
                RaiseDMXConnectionChanged(DMXConnected);
            }
        }

        public void DisconnectDMX()
        {
            reconnectThread?.Abort();
            if (enabled)
            {
                OpenDMX.stop();
                RaiseDMXConnectionChanged(DMXConnected);
            }
        }

        public void ReconnectDMX()
        {
            if (reconnectThread == null && enabled)
            {
                reconnectThread = new System.Threading.Thread(() => ReconnectDMXInternal()) { IsBackground = true };
                reconnectThread.Start();
            }
        }

        private void ReconnectDMXInternal()
        {
            try
            {
                do
                {
                    // Keep trying til we can reconnect
                    OpenDMX.stop();
                    System.Threading.Thread.Sleep(10);
                    OpenDMX.start();
                } while (!DMXConnected);
                RaiseDMXConnectionChanged(true);
            }
            catch (System.Threading.ThreadAbortException) { } // One last thing to do either way
            catch (System.Exception e)
            {
                Logger.LogError(e.ToString());
            }
            finally
            {
                reconnectThread = null;
            }
        }

        private void RaiseDMXConnectionChanged(bool connected)
        {
            DMXConnectionChanged?.Invoke(this, connected);
        }

        #endregion

        #region ColorAndIntensityControl

        public void SetLightColor(int lightIndex, LEDColor color, double intensity = -1, bool lockColor = false)
        {
            if (lights.Length > lightIndex && lights[lightIndex] != null)
            {
                lights[lightIndex].SetColor(intensity, color, lockColor);
                UpdateMessage(lightIndex);
            }
            else
                Logger.LogWarning("No light found at light index {0}", lightIndex);
        }

        public void SetRawValue(int lightIndex, byte value)
        {
            if (lights.Length > lightIndex && lights[lightIndex] != null)
            {
                if (lights[lightIndex].HasRaw)
                {
                    lights[lightIndex].SetRawValue(value);
                    UpdateMessage(lightIndex);
                }
            }
            else
                Logger.LogWarning("No light found at light index {0}", lightIndex);
        }
        public void SetSpecialDMXValue(int lightIndex, byte value)
        {
            if (lights.Length > lightIndex && lights[lightIndex] != null)
            {
                // for special DMX controls, send to the first channel only
                OpenDMX.setDmxValue(lights[lightIndex].Channels[0].Index, value);
                if (DMXConnected && OpenDMX.status != FT_STATUS.FT_OK)
                    DisconnectDMX();
            }
            else
                Logger.LogWarning("No DMX found at index {0}", lightIndex);
        }

        public void FadeLight(int lightIndex, LEDColor targetColor, double targetIntensity, bool lockColor, int fadeTimeMilliseconds)
        {
            // So we need to fade from current intensity to finalValue over the course of fadeTimeMilliseconds
            if (lights.Length > lightIndex && lights[lightIndex] != null)
                lights[lightIndex].Fade(targetIntensity, targetColor, lockColor, PlaybackForm.RefreshRate, fadeTimeMilliseconds);
        }

        public void FirmlyLockLight(int lightIndex, bool setLock)
        {
            if (lights.Length > lightIndex && lights[lightIndex] != null)
            {
                lights[lightIndex].FirmlyLocked = setLock;
            }
        }

        private void UpdateMessage(int lightIndex)
        {
            // don't risk clearing a special DMX channel
            if (lights[lightIndex].IsSpecialDMX) return;

            foreach (Channel channel in lights[lightIndex].Channels)
            {
                int[] channelIndices = new[] { channel.Index };
                int[] offChannels = new int[] { };

                if (shiftTimer.Enabled)
                {
                    // Really, in the channel mapping is the best way to tell if we need to turn out this channel
                    // That is, if we're not rotating and this module has gone "off the end"
                    channelIndices = MapChannelForShift(channelIndices[0], out bool lightsOut, out offChannels);
                    if (lightsOut)
                        channel.Value.CurrentValue = 0; // Make it "stick"
                }

                // 0 is a reserved channel
                foreach (int channelIndex in channelIndices)
                    if (channelIndex > 0 && channelIndex < OpenDMX.buffer.Length)
                    {
                        byte value = channel.Value.CurrentValue;
                        if (channel.Type != ChannelType.Raw)
                        {
                            value = (byte)(value * Light.LinearizeIntensity(lights[lightIndex].Intensity.CurrentValue));
                        }
                        OpenDMX.setDmxValue(channelIndex, value);
                    }

                foreach (int channelIndex in offChannels)
                    if (channelIndex > 0 && channelIndex < OpenDMX.buffer.Length)
                        OpenDMX.setDmxValue(channelIndex, 0);
            }
            if (DMXConnected && OpenDMX.status != FT_STATUS.FT_OK)
                DisconnectDMX();
        }

        #endregion

        #region MassChanges

        private void SetAllLights(LEDColor newColor, double newIntensity, bool lockColor = false)
        {
            for (int i = 0; i < lights.Length; i++)
            {
                lights[i].SetColor(newIntensity, newColor, lockColor);
                UpdateMessage(i);
            }
        }

        public void Reset(bool clearLights)
        {
            if (clearLights)
                SetAllLights(LEDColor.Black, 0, true);
            // This happens at the start of each song and whenever we do a hard set on all lights
            EndShift();
            UnlockLights();
            if (clearLights)
                SetAllLights(LEDColor.Black, 0, false);
        }

        public void UnlockLights()
        {
            for (int i = 0; i < lights.Length; i++)
            {
                UnlockLight(i);
                FirmlyLockLight(i, false);
            }
        }

        public void UnlockLight(int lightIndex)
        {
            if (lights.Length > lightIndex && lights[lightIndex] != null)
            {
                lights[lightIndex].Locked = false;
                UpdateMessage(lightIndex);
            }
            else
                Logger.LogWarning("No light found at light index {0}", lightIndex);
        }

        public void SwapModuleLights(bool AtoB, bool BtoA)
        {
            LEDColor currentModuleAColor = lights[modulesAandB[0].LightIndices[0]].Color;
            double currentModuleAIntensity = lights[modulesAandB[0].LightIndices[0]].Intensity.CurrentValue;
            LEDColor currentModuleBColor = lights[modulesAandB[1].LightIndices[0]].Color;
            double currentModuleBIntensity = lights[modulesAandB[1].LightIndices[0]].Intensity.CurrentValue;

            if (AtoB)
                foreach (int lightIndex in modulesAandB[1].LightIndices)
                    SetLightColor(lightIndex, currentModuleAColor, currentModuleAIntensity);
            if (BtoA)
                foreach (int lightIndex in modulesAandB[0].LightIndices)
                    SetLightColor(lightIndex, currentModuleBColor, currentModuleBIntensity);
        }

        #endregion

        #region ShiftControl

        public void BeginShift(bool right, bool rotate, bool repeat)
        {
            rightShiftActive = right;
            rotateShiftActive = rotate;

            shiftTimer.Stop();
            // If it's a single shift they want it now
            if (!repeat)
                ShiftLights();
            else
                shiftTimer.Start();

            if (repeat)
                CurrentMotion = "Shifting " + (right ? "right" : "left") + (rotate ? ", rotating" : "") + " every " + shiftTimer.Interval + " ms";
            else
                CurrentMotion = "None";
        }

        public void EndShift()
        {
            shiftTimer.Stop();
            CurrentMotion = "None";
        }

        public void SetShiftTimer(int milliseconds)
        {
            shiftTimer.Interval = milliseconds;
        }

        private void ShiftLights()
        {
            Logger.LogDebug("Shifting lights " + (rightShiftActive ? "right" : "left"));
            if (rightShiftActive)
                shifts++;
            else
                shifts--;
            shifts %= modules1to7.Length;
        }

        private int[] MapChannelForShift(int channelIndex, out bool lightsOut, out int[] clearIndices)
        {
            int[] invalidChannel = new[] { 0 }; // Reserved channel for DMX
            clearIndices = new int[0];
            System.Collections.Generic.List<int> channels = new System.Collections.Generic.List<int>();
            System.Collections.Generic.List<int> clearChannels = new System.Collections.Generic.List<int>();
            lightsOut = false;

            // Okay, this is a little confusing at first, but bear with me
            // The goal here is to allow changing the value of a light while shifting - but change the already-shifted light
            // So if we've shifted right twice, and they try to change module 1's color, actually change module 3's color
            // But rather than change the light in question, I'm actually changing where it spits its value

            // To that end, we have to find where the light in question lies, then change the corresponding light wherever that module "is" right now
            int unshiftedModuleNum = -1;
            int indexWithinModule = -1;
            int channelWithinLight = -1;
            for (int moduleNum = 0; moduleNum < modules1to7.Length && unshiftedModuleNum == -1; moduleNum++)
            {
                for (int indexNum = 0; indexNum < modules1to7[moduleNum].LightIndices.Length; indexNum++)
                {
                    channelWithinLight = System.Array.FindIndex(lights[modules1to7[moduleNum].LightIndices[indexNum]].Channels, ch => ch.Index == channelIndex);
                    if (channelWithinLight != -1)
                    {
                        unshiftedModuleNum = moduleNum;
                        indexWithinModule = indexNum;
                        break;
                    }
                }
            }

            if (unshiftedModuleNum == -1 || indexWithinModule == -1 || channelWithinLight == -1)
                return invalidChannel;

            // Make sure our shifted module number is within the acceptable range
            int shiftedModuleNum = (unshiftedModuleNum + shifts) % modules1to7.Length;
            if (shiftedModuleNum < 0)
                shiftedModuleNum += modules1to7.Length;

            // Have we moved off the end? (If the shifted module number has rolled around, then yes we have)
            if (!rotateShiftActive && ((shiftedModuleNum >= unshiftedModuleNum && !rightShiftActive) || (shiftedModuleNum <= unshiftedModuleNum && rightShiftActive)))
                lightsOut = true;

            int shiftedLightIndex;
            if (modules1to7[shiftedModuleNum].LightIndices.Length > indexWithinModule)
                shiftedLightIndex = modules1to7[shiftedModuleNum].LightIndices[indexWithinModule];
            else
                return invalidChannel;

            if (lights.Length > shiftedLightIndex && lights[shiftedLightIndex].Channels.Length > channelWithinLight)
            {
                channels.Add(lights[shiftedLightIndex].Channels[channelWithinLight].Index);
            }
            else
                return invalidChannel;
            
            // If there are more lights in the target than in the source, pick the middle one and map that to all the missing ones
            if (modules1to7[shiftedModuleNum].LightIndices.Length > modules1to7[unshiftedModuleNum].LightIndices.Length && indexWithinModule == System.Math.Floor(modules1to7[unshiftedModuleNum].LightIndices.Length / 2f))
            {
                for (int index = modules1to7[unshiftedModuleNum].LightIndices.Length; index < modules1to7[shiftedModuleNum].LightIndices.Length; index++)
                {
                    channels.Add(lights[modules1to7[shiftedModuleNum].LightIndices[index]].Channels[channelWithinLight].Index);
                }
            }
            // If there are more channels in the target than in the source, clear them out
            if (lights[modules1to7[unshiftedModuleNum].LightIndices[indexWithinModule]].Channels.Length < lights[modules1to7[shiftedModuleNum].LightIndices[indexWithinModule]].Channels.Length)
            {
                for (int i = lights[modules1to7[unshiftedModuleNum].LightIndices[indexWithinModule]].Channels.Length; i < lights[modules1to7[shiftedModuleNum].LightIndices[indexWithinModule]].Channels.Length; i++)
                {
                    clearChannels.Add(lights[modules1to7[shiftedModuleNum].LightIndices[indexWithinModule]].Channels[i].Index);
                }
            }

            clearIndices = clearChannels.ToArray();
            return channels.ToArray();
        }

        #endregion
    }
}
