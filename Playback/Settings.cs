using System.IO;
using System.Linq;

namespace Playback
{
    public static class Settings
    {
        private static readonly string fileLocation = Path.Combine(System.Environment.GetFolderPath(System.Environment.SpecialFolder.CommonDocuments), "PlaybackSettings.xml");

        public static SettingsHolder CurrentSettings;

        static Settings()
        {
            CurrentSettings = new SettingsHolder();

            try
            {
                if (File.Exists(fileLocation))
                    DeserializeSettings();
            }
            catch (System.Exception e)
            {
                // Something's wrong with the file - use defaults
                Logger.LogError(e.ToString());
            }
        }

        public static void SerializeSettings(string filePath = null)
        {
            filePath = filePath ?? fileLocation;

            // Write to the file with this here StreamWriter
            using (StreamWriter writer = new StreamWriter(filePath))
                new System.Xml.Serialization.XmlSerializer(typeof(SettingsHolder)).Serialize(writer, CurrentSettings);
        }

        public static void DeserializeSettings(string filePath = null)
        {
            filePath = filePath ?? fileLocation;

            // Read from the file with this here StreamReader
            using (StreamReader Reader = new StreamReader(filePath))
                CurrentSettings = (SettingsHolder)new System.Xml.Serialization.XmlSerializer(typeof(SettingsHolder)).Deserialize(Reader);
        }
    }

    public class SettingsHolder
    {
        public string AudioEndpointID;

        public string DefaultColorMap;
        public string DMXMap;
        public string FCWMap;

        public string PlaylistDirectory;
        public string SongDirectory;
        public string AnnouncementDirectory;

        public string PLCIPAddress;
        public string PLCPort;

        public bool ShowStartAt;

        public int PlaybackLatency;
        public int UpdateRate;

        public LogLevel LogLevel;

        public System.Drawing.Point PlaybackPosition;
        public System.Drawing.Size PlaybackSize;

        public int CurrentPlaylistSongColumnWidth;
        public int CurrentPlaylistArtistColumnWidth;

        public int LoopWarnMS;
        public int RefreshWarnMS;

        public SettingsHolder()
        {
            AudioEndpointID = "";

            DefaultColorMap = "";
            DMXMap = "";
            FCWMap = "";

            PlaylistDirectory = "C:\\GHMF\\Playlists";
            SongDirectory = "C:\\GHMF\\Songs";
            AnnouncementDirectory = "";

            PLCIPAddress = "192.168.1.10";
            PLCPort = "444";

            ShowStartAt = false;

            PlaybackLatency = 50;
            UpdateRate = 30;

            LogLevel = Playback.LogLevel.Info;

            PlaybackPosition = new System.Drawing.Point();
            PlaybackSize = new System.Drawing.Size();

            // -2 means autosize - default to that
            CurrentPlaylistSongColumnWidth = -2;
            CurrentPlaylistArtistColumnWidth = -2;

            LoopWarnMS = 5;
            RefreshWarnMS = 5;
        }
    }
}
