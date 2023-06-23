using System;
using System.Linq;
using NAudio.Wave;
using NAudio.Wave.SampleProviders;

namespace Playback
{
    public class Player : IDisposable
    {
        private System.IO.MemoryStream memStream;
        private WaveStream waveStream;
        private WaveChannel32 waveChannel;
        private WasapiOut audioOutput;
        private SampleChannel sampleChannel;
        private MeteringSampleProvider meteringSampleProvider;

        private NAudio.CoreAudioApi.MMDevice device;
        private TimeSpan latency;
        private int outputUpdatesPerSecond;
        private TimeSpan lastCurrentTime;
        private System.Diagnostics.Stopwatch latencyCompensator;

        public delegate void VolumeChangedEventHandler(object sender, float newVolLeft, float newVolRight);
        public event VolumeChangedEventHandler VolumeChanged;
        public delegate void OutputLevelChangedEventHandler(object sender, float maxSample);
        public event OutputLevelChangedEventHandler OutputLevelChanged;

        private bool paused;

        public enum PlayState
        {
            Stopped,
            Paused,
            Playing
        }

        public PlayState PlaybackState
        {
            get
            {
                if (audioOutput == null || waveStream.CurrentTime.TotalMilliseconds >= waveStream.TotalTime.TotalMilliseconds)
                    return PlayState.Stopped;
                if (paused)
                    return PlayState.Paused;

                switch (audioOutput.PlaybackState)
                {
                    case NAudio.Wave.PlaybackState.Playing:
                        return PlayState.Playing;
                    case NAudio.Wave.PlaybackState.Paused:
                        return PlayState.Paused;
                    case NAudio.Wave.PlaybackState.Stopped:
                    default:
                        return PlayState.Stopped;
                }
            }
        }

        public float MasterVolume
        {
            get
            {
                return device.AudioEndpointVolume.MasterVolumeLevelScalar;
            }
            set
            {
                device.AudioEndpointVolume.MasterVolumeLevelScalar = value;
            }
        }

        public float LeftVolume
        {
            get
            {
                if (device.AudioEndpointVolume.Channels.Count < 2)
                    return MasterVolume;
                else
                    return device.AudioEndpointVolume.Channels[0].VolumeLevelScalar;
            }
            set
            {
                if (device.AudioEndpointVolume.Channels.Count < 2)
                    MasterVolume = value;
                else
                    device.AudioEndpointVolume.Channels[0].VolumeLevelScalar = value;
            }
        }

        public float RightVolume
        {
            get
            {
                if (device.AudioEndpointVolume.Channels.Count < 2)
                    return MasterVolume;
                else
                    return device.AudioEndpointVolume.Channels[1].VolumeLevelScalar;
            }
            set
            {
                if (device.AudioEndpointVolume.Channels.Count < 2)
                    MasterVolume = value;
                else
                    device.AudioEndpointVolume.Channels[1].VolumeLevelScalar = value;
            }
        }

        public string DeviceID
        {
            get
            {
                return device.ID;
            }
        }

        public Player(string deviceID, int latency, int vuUpdatesPerSecond)
        {
            // Use the user's device selection but if we can't (like on first startup) we fall back on the system default
            try
            {
                device = new NAudio.CoreAudioApi.MMDeviceEnumerator().GetDevice(deviceID);
            }
            catch (ArgumentException) // Unexpected ID
            {
                device = new NAudio.CoreAudioApi.MMDeviceEnumerator().GetDefaultAudioEndpoint(NAudio.CoreAudioApi.DataFlow.Render, NAudio.CoreAudioApi.Role.Multimedia);
            }

            device.AudioEndpointVolume.OnVolumeNotification += AudioEndpointVolume_OnVolumeNotification;
            this.latency = TimeSpan.FromMilliseconds(latency);
            this.outputUpdatesPerSecond = vuUpdatesPerSecond;
            lastCurrentTime = TimeSpan.Zero;
        }

        private void Reset()
        {
            paused = false;
            lastCurrentTime = TimeSpan.Zero;
            latencyCompensator = new System.Diagnostics.Stopwatch();

            if (audioOutput != null)
                audioOutput.Dispose();
            if (waveChannel != null)
                waveChannel.Dispose();
            if (waveStream != null)
                waveStream.Dispose();
            if (memStream != null)
                memStream.Dispose();
        }

        public void Dispose()
        {
            Stop();

            Reset();

            if (device != null)
                device.AudioEndpointVolume.OnVolumeNotification -= AudioEndpointVolume_OnVolumeNotification;
            if (meteringSampleProvider != null)
                meteringSampleProvider.StreamVolume -= MeteringSampleProvider_StreamVolume;
        }

        #region Volume

        private void AudioEndpointVolume_OnVolumeNotification(NAudio.CoreAudioApi.AudioVolumeNotificationData data)
        {
            // Okay, I don't much like doing this *not* using the data given to me
            // But it always seems to have both channels at the same level, which is not always correct

            float left, right;
            if (device.AudioEndpointVolume.Channels.Count < 2)
                left = right = device.AudioEndpointVolume.MasterVolumeLevelScalar;
            else
            {
                left = device.AudioEndpointVolume.Channels[0].VolumeLevelScalar;
                right = device.AudioEndpointVolume.Channels[1].VolumeLevelScalar;
            }

            RaiseVolumeChanged(left, right);
        }

        private void RaiseVolumeChanged(float newVolLeft, float newVolRight)
        {
            if (VolumeChanged != null)
                VolumeChanged(this, newVolLeft, newVolRight);
        }

        void MeteringSampleProvider_StreamVolume(object sender, StreamVolumeEventArgs e)
        {
            RaiseOutputLevelChanged(e.MaxSampleValues[1]);
        }

        void RaiseOutputLevelChanged(float maxSampleValue)
        {
            if (OutputLevelChanged != null)
                OutputLevelChanged(this, maxSampleValue);
        }

        #endregion

        #region PlaybackControl

        public void Play(string soundFile)
        {
            if (!System.IO.File.Exists(soundFile))
                throw new ArgumentException("File '" + soundFile + "' not found.");

            // Make sure we've cleaned up
            Reset();

            memStream = null;
            waveStream = new WaveFileReader(soundFile);
            Initialize(waveStream);
        }

        public void PlayBlank(int lengthMilliseconds)
        {
            // Make sure we've cleaned up
            Reset();

            WaveFormat format = new WaveFormat();
            int numSamples = lengthMilliseconds * format.SampleRate;
            memStream = new System.IO.MemoryStream(new byte[numSamples]);
            waveStream = new RawSourceWaveStream(memStream, format);
            Initialize(waveStream);
        }

        private void Initialize(WaveStream wave)
        {
            waveChannel = new WaveChannel32(wave);
            sampleChannel = new NAudio.Wave.SampleProviders.SampleChannel(waveChannel);
            int samplesPerUpdate = sampleChannel.WaveFormat.SampleRate / outputUpdatesPerSecond;
            meteringSampleProvider = new NAudio.Wave.SampleProviders.MeteringSampleProvider(sampleChannel, samplesPerUpdate);
            meteringSampleProvider.StreamVolume += MeteringSampleProvider_StreamVolume;
            audioOutput = new WasapiOut(device, NAudio.CoreAudioApi.AudioClientShareMode.Exclusive, false, (int)latency.TotalMilliseconds); // a latency of 320 seems to work for event mode but I'm not going to worry about that
            audioOutput.Init(meteringSampleProvider);
            audioOutput.Play();
            device.AudioEndpointVolume.Mute = false;
        }

        public void Play()
        {
            if (audioOutput != null)
            {
                latencyCompensator.Start();
                audioOutput.Play();
                paused = false;
            }
        }

        public void Pause()
        {
            if (audioOutput != null)
            {
                paused = true;
                latencyCompensator.Stop();
                audioOutput.Stop();
            }
        }

        public void Stop()
        {
            if (audioOutput != null)
            {
                audioOutput.Stop();//.Dispose();
                paused = false;
            }
        }

        public void SkipTo(TimeSpan newTime)
        {
            waveStream.Skip((int)(newTime.TotalSeconds - waveStream.CurrentTime.TotalSeconds));
        }

        #endregion

        #region Timing

        public TimeSpan GetPosition()
        {
            if (audioOutput == null || waveStream == null)
                return TimeSpan.Zero;

            // Now, sometimes the latency compensator gets confused and continues running even though we've paused
            if (PlaybackState != PlayState.Playing)
                latencyCompensator.Stop();

            TimeSpan reportedTime = waveStream.CurrentTime;
            if (latency.TotalMilliseconds > 0 && reportedTime.TotalMilliseconds > 0)
            {
                // If we're past our expected latency, something is wrong - don't keep playing
                if (latencyCompensator.ElapsedMilliseconds > latency.TotalMilliseconds)
                    latencyCompensator.Stop();
                if (lastCurrentTime == reportedTime)
                    reportedTime = reportedTime.Add(TimeSpan.FromMilliseconds(latencyCompensator.ElapsedMilliseconds));
                else
                {
                    //Logger.LogDebug("Reported position changed to {0}", reportedTime.ToString());
                    lastCurrentTime = reportedTime;
                    latencyCompensator.Restart();
                }
            }

            if (reportedTime > latency)
                return reportedTime - latency;
            else
                return reportedTime;
        }

        public static TimeSpan GetTime(bool truncate, params string[] songNames)
        {
            TimeSpan totalTime = new TimeSpan();
            foreach (string songName in songNames)
                if (System.IO.File.Exists(songName))
                    using (WaveFileReader tempReader = new WaveFileReader(songName))
                    {
                        if (truncate)
                            totalTime = totalTime.Add(tempReader.TotalTime.Truncate());
                        else
                            totalTime = totalTime.Add(tempReader.TotalTime);
                    }

            return totalTime;
        }

        #endregion

        #region AudioOutputs

        public static string[] GetAudioOutputs()
        {
            return new NAudio.CoreAudioApi.MMDeviceEnumerator()
                .EnumerateAudioEndPoints(NAudio.CoreAudioApi.DataFlow.Render, NAudio.CoreAudioApi.DeviceState.Active)
                .Select(dev => dev.FriendlyName).ToArray();
        }

        public static string UIDFromFriendlyName(string friendlyName)
        {
            NAudio.CoreAudioApi.MMDevice foundDevice = new NAudio.CoreAudioApi.MMDeviceEnumerator()
                .EnumerateAudioEndPoints(NAudio.CoreAudioApi.DataFlow.Render, NAudio.CoreAudioApi.DeviceState.Active)
                .FirstOrDefault(DirectSoundDeviceInfo => DirectSoundDeviceInfo.FriendlyName == friendlyName);

            if (foundDevice == null) return "";
            return foundDevice.ID;
        }

        public static string FriendlyNameFromUID(string uid)
        {
            NAudio.CoreAudioApi.MMDevice foundDevice = new NAudio.CoreAudioApi.MMDeviceEnumerator()
                .EnumerateAudioEndPoints(NAudio.CoreAudioApi.DataFlow.Render, NAudio.CoreAudioApi.DeviceState.Active)
                .FirstOrDefault(DirectSoundDeviceInfo => DirectSoundDeviceInfo.ID == uid);

            if (foundDevice == null) return "";
            return foundDevice.FriendlyName;
        }

        #endregion
    }
}
