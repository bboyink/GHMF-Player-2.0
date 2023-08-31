using System;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Windows.Forms;

namespace Playback
{
    public partial class PlaybackForm : Form, IDisposable
    {
        public const int RefreshRate = 10;

        // These probably don't need to be class-level, but this is a late addition to the project
        // so I'm going with the first thing that comes to mind. Probably bad practice
        private int totalFCWCount = 0;
        private int lightFCWCount = 0;
        private int waterFCWCount = 0;
        private int totalFCWsExecuted = 0;
        private int lightFCWsExecuted = 0;
        private int waterFCWsSent = 0;

        private bool skipResetAfterSong = false;

        private Player SoundController;
        private Lighting LightController;
        private PLCComms PLCController;
        private InitStatus InitStatus;

        private FCW[] FCWs;
        private LEDColor[] Colors;
        private Playlist CurrentPlaylist;

        #region InitializationStatus

        private bool fcwsLoaded;
        private bool FCWsLoaded
        {
            get { return fcwsLoaded; }
            set
            {
                if (InitStatus != null)
                    InitStatus.Complete(InitStatus.InitStep.LoadFCWs, value);

                fcwsLoaded = value;
                SetPlaybackEnable(initComplete);
            }
        }
        private bool colorsLoaded;
        private bool ColorsLoaded
        {
            get { return colorsLoaded; }
            set
            {
                // This one happens every song, and we don't want to re-enable that often
                bool alreadyInitialized = initComplete;

                if (InitStatus != null)
                    InitStatus.Complete(InitStatus.InitStep.LoadColors, value);

                colorsLoaded = value;
                if (!alreadyInitialized)
                    SetPlaybackEnable(initComplete);
            }
        }
        private bool playlistsLoaded;
        private bool PlaylistsLoaded
        {
            get { return playlistsLoaded; }
            set
            {
                if (InitStatus != null)
                    InitStatus.Complete(InitStatus.InitStep.LoadPlaylists, value);

                playlistsLoaded = value;
                SetPlaybackEnable(initComplete);
            }
        }
        private bool songsLoaded;
        private bool SongsLoaded
        {
            get { return songsLoaded; }
            set
            {
                if (InitStatus != null)
                    InitStatus.Complete(InitStatus.InitStep.LoadSongs, value);

                songsLoaded = value;
                SetPlaybackEnable(initComplete);
            }
        }
        private bool announcementsLoaded;
        private bool AnnouncementsLoaded
        {
            get { return announcementsLoaded; }
            set
            {
                if (InitStatus != null)
                    InitStatus.Complete(InitStatus.InitStep.LoadAnnouncements, value);

                announcementsLoaded = value;
                SetPlaybackEnable(initComplete);
            }
        }
        private bool soundInitd;
        private bool SoundInitd
        {
            get { return soundInitd; }
            set
            {
                if (InitStatus != null)
                    InitStatus.Complete(InitStatus.InitStep.InitSound, value);

                soundInitd = value;
                SetPlaybackEnable(initComplete);
            }
        }
        private bool lightsInitd;
        private bool LightsInitd
        {
            get { return lightsInitd; }
            set
            {
                if (InitStatus != null)
                    InitStatus.Complete(InitStatus.InitStep.InitLights, value);

                lightsInitd = value;
                SetPlaybackEnable(initComplete);
            }
        }
        private bool plcCommInitd;
        private bool PLCCommInitd
        {
            get { return plcCommInitd; }
            set
            {
                if (InitStatus != null)
                    InitStatus.Complete(InitStatus.InitStep.InitPLCComms, value);

                plcCommInitd = value;
                SetPlaybackEnable(initComplete);
            }
        }
        private bool currentPlaylistLoaded;
        private bool CurrentPlaylistLoaded
        {
            get { return currentPlaylistLoaded; }
            set
            {
                if (InitStatus != null)
                    InitStatus.Complete(InitStatus.InitStep.LoadCurrentPlaylist, value);

                currentPlaylistLoaded = value;
                SetPlaybackEnable(initComplete);
            }
        }

        private bool initComplete { get { return FCWsLoaded && ColorsLoaded && PlaylistsLoaded && SongsLoaded && AnnouncementsLoaded && SoundInitd && LightsInitd && PLCCommInitd && CurrentPlaylistLoaded && LightController.DMXConnected; } }
        private bool canExecute { get { return FCWsLoaded && (PLCCommInitd || (LightsInitd && ColorsLoaded)); } }

        #endregion

        private bool loggedIn = false;
        private bool LoggedIn
        {
            get
            {
                return loggedIn;
            }
            set
            {
                loggedIn = value;
                SetAdminControl(value);
            }
        }
        private bool stopping = false;

        private bool plcDisabled = false;
        private bool dmxDisabled = false;

        #region Creation and Destruction

        public PlaybackForm()
        {
            Logger.LogLevel = Settings.CurrentSettings.LogLevel;
            Logger.LogInfo("Initializing Playback v{0}", System.Reflection.Assembly.GetExecutingAssembly().GetName().Version.ToString());

            InitializeComponent();

            Text += " v" + System.Reflection.Assembly.GetExecutingAssembly().GetName().Version.ToString();

            uint prevExecutionState = NativeMethods.SetThreadExecutionState(NativeMethods.ES_CONTINUOUS | NativeMethods.ES_DISPLAY_REQUIRED | NativeMethods.ES_SYSTEM_REQUIRED);
            if (prevExecutionState == 0)
            {
                ShowMessage("Unable to set thread execution state.");
                Close();
            }

            SetAdminControl(false);
            SetPlaybackEnable(false);

            Location = Settings.CurrentSettings.PlaybackPosition;
            Size = Settings.CurrentSettings.PlaybackSize;
            // Make sure they can't get it stuck offscreen
            if (!IsOnScreen(this))
                StartPosition = FormStartPosition.WindowsDefaultLocation;

            UpdateTimeUI("", "", TimeSpan.Zero, TimeSpan.Zero, TimeSpan.Zero, TimeSpan.Zero);

            playlistFolderBrowser.SelectedPath = Settings.CurrentSettings.PlaylistDirectory;
            songFolderBrowser.SelectedPath = Settings.CurrentSettings.SongDirectory;
            lblStartPoint.Visible = txtStartPoint.Visible = Settings.CurrentSettings.ShowStartAt;

            tlpLists.RowStyles[1].Height = 0;

            if (System.Diagnostics.Debugger.IsAttached)
            {
                plcDisabled = dmxDisabled = true;
            }

            new System.Threading.Thread(() => InitAll()) { IsBackground = true }.Start();
        }

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (SoundController != null)
            {
                SoundController.VolumeChanged -= SoundController_VolumeChanged;
                SoundController.OutputLevelChanged -= SoundController_OutputLevelChanged;
                SoundController.Dispose();
            }
            if (LightController != null)
                LightController.Dispose();
            if (PLCController != null)
                PLCController.Dispose();

            // The following is the auto-generated Dispose method from the designer
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #endregion

        #region Initialization

        private void InitAll()
        {
            if (InitStatus == null || InitStatus.IsDisposed)
            {
                if (InvokeRequired)
                {
                    try
                    {
                        Invoke(new Action(() => InitStatus = new InitStatus()));
                    }
                    catch (ObjectDisposedException) { return; } // We're shutting down, it doesn't matter
                }
                else
                    InitStatus = new InitStatus();
            }
            else
                InitStatus.Reset();

            if (!InitStatus.Visible)
            {
                if (InvokeRequired)
                {
                    try
                    {
                        Invoke(new Action(() => InitStatus.Show(this)));
                    }
                    catch (ObjectDisposedException) { return; } // We're shutting down, it doesn't matter
                }
                else
                    InitStatus.Show(this);
            }

            Logger.LogLevel = Settings.CurrentSettings.LogLevel;

            // During init I'm not going to check for things like file existence
            // Because the user needs to know they don't exist, so I'll let the exceptions bubble up and tell them
            // Really I only expect it to happen the first time we start up
            LoadFCWs();
            LoadColors();
            LoadPlaylists();
            LoadSongs();
            LoadAnnouncements();
            InitSound();
            InitLights();
            InitPLCComms();
        }

        private void LoadFCWs()
        {
            try
            {
                // First we pull in the user-defined water and light commands
                // Then we set up the special ones like stop all (which will straight up overwrite theirs)
                FCWDefFileParser FCWparser = new FCWDefFileParser();
                FCWparser.ParseUserConfig(Settings.CurrentSettings.FCWMap);
                FCWparser.SetSpecialFCWs();
                FCWs = FCWparser.FCWs;

                FCWsLoaded = true;
            }
            catch (Exception e)
            {
                FCWsLoaded = false;
                ShowMessage("An error occurred while creating the FCW map: " + System.Environment.NewLine + e.Message);
                Logger.LogError(e.ToString());
            }
        }

        private void LoadColors(string colorMapFile = null)
        {
            if (colorMapFile == null) colorMapFile = Settings.CurrentSettings.DefaultColorMap;

            try
            {
                ColorDefFileParser colorParser = new ColorDefFileParser();
                // Reload the default each time, then we'll overwrite it if necessary
                // (This way we don't have bleedover with things like the backcurtain colors or the voice color)
                colorParser.Parse(Settings.CurrentSettings.DefaultColorMap);

                if (File.Exists(colorMapFile) && colorMapFile != Settings.CurrentSettings.DefaultColorMap)
                    colorParser.Parse(colorMapFile);

                Colors = colorParser.Colors;

                ColorsLoaded = true;
            }
            catch (Exception e)
            {
                ColorsLoaded = false;
                ShowMessage("An error occurred while creating the color map from " + colorMapFile + ": " + System.Environment.NewLine + e.Message);
                Logger.LogError(e.ToString());
            }
        }

        private void LoadPlaylists(string fullDirPath = null)
        {
            try
            {
                fullDirPath = fullDirPath ?? playlistFolderBrowser.SelectedPath;
                if (!Directory.Exists(fullDirPath))
                    fullDirPath = @"C:\"; // Fall back on the for-sure directory

                // Since we're editing the controls, we gotta do it from the main thread
                if (InvokeRequired)
                {
                    try
                    {
                        Invoke(new Action(() => LoadPlaylists(fullDirPath)));
                    }
                    catch (ObjectDisposedException) { PlaylistsLoaded = false; } // We're shutting down, it doesn't matter
                    return;
                }

                int playlistListIndex = (lvPlaylists.Items.Count > 0 ? lvPlaylists.SelectedIndices[0] : 0);
                int currentPlaylistListIndex = (lvCurPlaylistSongs.Items.Count > 0 ? lvCurPlaylistSongs.SelectedIndices[0] : 0);

                LoadListView(lvPlaylists, lblCurrentPlaylistFolder, fullDirPath, "*" + Playlist.FileExtension, playlistListIndex);
                LoadPlaylist(null, currentPlaylistListIndex);

                PlaylistsLoaded = true;
            }
            catch (Exception e)
            {
                PlaylistsLoaded = false;
                ShowMessage("An error occurred while loading the playlists directory: " + System.Environment.NewLine + e.Message);
                Logger.LogError(e.ToString());
            }
        }

        private void LoadSongs(string fullDirPath = null)
        {
            // Since we're editing the controls, we gotta do it from the main thread
            if (InvokeRequired)
            {
                try
                {
                    Invoke(new Action(() => LoadSongs(fullDirPath)));
                }
                catch (ObjectDisposedException) { SongsLoaded = false; } // We're shutting down, it doesn't matter
                return;
            }

            try
            {
                fullDirPath = fullDirPath ?? songFolderBrowser.SelectedPath;
                if (!Directory.Exists(fullDirPath))
                    fullDirPath = @"C:\"; // Fall back on the for-sure directory

                LoadListView(lvSongs, lblCurrentSongFolder, fullDirPath, "*.wav");

                SongsLoaded = true;
            }
            catch (Exception e)
            {
                SongsLoaded = false;
                ShowMessage("An error occurred while loading the songs directory: " + System.Environment.NewLine + e.Message);
                Logger.LogError(e.ToString());
            }
        }

        private void LoadTreeView(TreeView treeView, string directoryPath)
        {
            // Since we're editing the controls, we gotta do it from the main thread
            if (InvokeRequired)
            {
                try
                {
                    BeginInvoke(new Action(() => LoadTreeView(treeView, directoryPath)));
                }
                catch (ObjectDisposedException) { } // We're shutting down, it doesn't matter
                return;
            }

            treeView.Nodes.Clear();

            // Get all the folder names in the path
            TreeNode currentNode = null;
            foreach (string folder in directoryPath.Split(Path.DirectorySeparatorChar, Path.AltDirectorySeparatorChar))
            {
                if (folder == "")
                    continue;
                // First time around, we add to root, otherwise we want to add subnodes
                if (currentNode == null)
                    currentNode = treeView.Nodes.Add(folder + Path.DirectorySeparatorChar); // C:\ drive (we'd like to keep the \)
                else
                    currentNode = currentNode.Nodes.Add(folder);
            }
            treeView.SelectedNode = currentNode;

            // Show everything up to this point (the stuff below we can leave alone)
            treeView.ExpandAll();

            // Remember, currentNode is the lowest of our nodes
            foreach (string directory in Directory.GetDirectories(directoryPath))
                currentNode.Nodes.Add(new DirectoryInfo(directory).Name);
            currentNode.Expand();
        }

        private void LoadListView(ListView listView, Label folderLabel, string directory, string filter, int selectedIndex = 0)
        {
            // Since we're editing the controls, we gotta do it from the main thread
            if (InvokeRequired)
            {
                try
                {
                    BeginInvoke(new Action(() => LoadListView(listView, folderLabel, directory, filter)));
                }
                catch (ObjectDisposedException) { } // We're shutting down, it doesn't matter
                return;
            }

            listView.Items.Clear();
            foreach (string file in Directory.GetFiles(directory, filter))
                listView.Items.Add(new ListViewItem(Path.GetFileNameWithoutExtension(file)) { Tag = directory });
            if (listView.Items.Count > 0)
                listView.Items[0].Selected = true;

            ResizeColumns(listView);

            if (folderLabel != null)
                folderLabel.Text = directory;

            if (listView.Items.Count > selectedIndex)
                listView.Items[selectedIndex].Selected = true;
        }

        private void LoadPlaylist(string fileName = null, int selectedItem = 0, bool reloading = false)
        {
            // Don't reload while we're playing or weird things will happen
            if (!reloading && SoundController != null && SoundController.PlaybackState != Player.PlayState.Stopped)
                return;

            // Since we're editing the controls, we gotta do it from the main thread
            if (InvokeRequired)
            {
                try
                {
                    Invoke(new Action(() => LoadPlaylist(fileName, selectedItem, reloading)));
                }
                catch (ObjectDisposedException) { CurrentPlaylistLoaded = false; } // We're shutting down, it doesn't matter
                return;
            }

            // Default to the selected item
            fileName = fileName ?? lvPlaylists.SelectedItems[0].Text;
            if (!fileName.EndsWith(Playlist.FileExtension))
                fileName = fileName + Playlist.FileExtension;

            lvCurPlaylistSongs.Items.Clear();
            if (lvPlaylists.SelectedItems.Count > 0)
            {
                CurrentPlaylist = new Playlist(Path.Combine(playlistFolderBrowser.SelectedPath, fileName));
                foreach (string song in CurrentPlaylist.Songs)
                {
                    string songToAdd;
                    string artist;
                    if (Playlist.SongIsComment(song))
                    {
                        songToAdd = Playlist.SongComment;
                        artist = song.Trim();
                    }
                    else
                    {
                        songToAdd = LoggedIn ? song : Path.GetFileNameWithoutExtension(song);
                        artist = GetArtistName(song);
                    }
                    lvCurPlaylistSongs.Items.Add(new ListViewItem(new[] { songToAdd, artist }));
                }

                // If they want something past the end, clamp it to the end
                if (selectedItem >= lvCurPlaylistSongs.Items.Count)
                    selectedItem = lvCurPlaylistSongs.Items.Count - 1;

                // Now clamp it on the other side
                if (selectedItem < 0)
                    selectedItem = 0;

                // And even now, if there's nothing left, we gotta make sure the item exists
                if (lvCurPlaylistSongs.Items.Count > selectedItem)
                {
                    lvCurPlaylistSongs.Items[selectedItem].Selected = true;
                    lvCurPlaylistSongs.Items[selectedItem].EnsureVisible();
                }

                CurrentPlaylistLoaded = true;
            }
            else
                CurrentPlaylistLoaded = false;

            lvCurPlaylistSongs.Columns[0].Width = Settings.CurrentSettings.CurrentPlaylistSongColumnWidth;
            lvCurPlaylistSongs.Columns[1].Width = Settings.CurrentSettings.CurrentPlaylistArtistColumnWidth;

            btnAddToPlaylist.Enabled = btnRemoveFromPlaylist.Enabled = btnMoveUp.Enabled = btnMoveDown.Enabled = CurrentPlaylistLoaded && LoggedIn;
        }

        private void LoadAnnouncements()
        {
            // Since we're editing the controls, we gotta do it from the main thread
            if (InvokeRequired)
            {
                try
                {
                    Invoke(new Action(() => LoadAnnouncements()));
                }
                catch (ObjectDisposedException) { AnnouncementsLoaded = false; } // We're shutting down, it doesn't matter
                return;
            }

            try
            {
                lvAnnouncements.Items.Clear();
                string announcementDirectory = Settings.CurrentSettings.AnnouncementDirectory;

                foreach (string announcement in Directory.EnumerateFiles(announcementDirectory, "*.wav"))
                    lvAnnouncements.Items.Add(new ListViewItem(Path.GetFileNameWithoutExtension(announcement)) { Tag = announcement });

                AnnouncementsLoaded = true;
            }
            catch (Exception e)
            {
                AnnouncementsLoaded = false;
                ShowMessage("An error occurred while loading the announcements directory: " + System.Environment.NewLine + e.Message);
                Logger.LogError(e.ToString());
            }
        }

        private void InitSound()
        {
            string deviceID = Settings.CurrentSettings.AudioEndpointID;
            int latency = Settings.CurrentSettings.PlaybackLatency;

            try
            {
                if (SoundController != null)
                    SoundController.Dispose();
                SoundController = new Player(deviceID, latency, Settings.CurrentSettings.UpdateRate);
                // Fall back on the system default
                if (SoundController.DeviceID != deviceID)
                    Settings.CurrentSettings.AudioEndpointID = SoundController.DeviceID;

                SoundController.VolumeChanged += SoundController_VolumeChanged;
                SoundController.OutputLevelChanged += SoundController_OutputLevelChanged;
                SetUIVolume(SoundController.LeftVolume, SoundController.RightVolume);
                SoundInitd = true;
            }
            catch (Exception e)
            {
                SoundInitd = false;
                ShowMessage("An error occurred while initializing Windows sound: " + System.Environment.NewLine + e.Message);
                Logger.LogError(e.ToString());
            }
        }

        private void InitLights()
        {
            try
            {
                if (LightController != null)
                    LightController.Dispose();
                LightConfigDefFileParser parser = new LightConfigDefFileParser();
                parser.Parse(Settings.CurrentSettings.DMXMap);
                // FCWs 17-23 control individual modules, and 49 and 50 control the A and B groups
                // Since some of my commands (shift, swap) have to know which is which, that's how we figure it out
                LightController = new Lighting(
                    parser.LightConfigs,
                    new[]
                    { 
                        new LightModule("Module 1", FCWs[(int)SpecialFCWAddress.Module1].AffectedLights),
                        new LightModule("Module 2", FCWs[(int)SpecialFCWAddress.Module2].AffectedLights),
                        new LightModule("Module 3", FCWs[(int)SpecialFCWAddress.Module3].AffectedLights),
                        new LightModule("Module 4", FCWs[(int)SpecialFCWAddress.Module4].AffectedLights),
                        new LightModule("Module 5", FCWs[(int)SpecialFCWAddress.Module5].AffectedLights),
                        new LightModule("Module 6", FCWs[(int)SpecialFCWAddress.Module6].AffectedLights),
                        new LightModule("Module 7", FCWs[(int)SpecialFCWAddress.Module7].AffectedLights)
                    },
                    new[]
                    {
                        new LightModule("Peacock", FCWs[(int)SpecialFCWAddress.Peacock].AffectedLights),
                        new LightModule("Other", FCWs[(int)SpecialFCWAddress.Voice].AffectedLights)
                    },
                    new[]
                    {
                        new LightModule("A Modules", FCWs[(int)SpecialFCWAddress.ModulesA].AffectedLights),
                        new LightModule("B Modules", FCWs[(int)SpecialFCWAddress.ModulesB].AffectedLights)
                    },
                    dmxDisabled);

                LightController.DMXConnectionChanged += LightController_DMXConnectionChanged;
                LightController.ConnectDMX();
                ResetLights();

                LightsInitd = LightController.DMXConnected;
            }
            catch (Exception e)
            {
                //LightsInitd = false;
                ShowMessage("An error occurred while initializing the DMX controller: " + System.Environment.NewLine + e.Message);
                Logger.LogError(e.ToString());
            }
        }

        private void InitPLCComms()
        {
            // The double try-catch allows me to catch certain exceptions and filter out the sub-exceptions that I want to specially handle
            try
            {
                try
                {
                    if (PLCController != null)
                        PLCController.Dispose();
                    PLCController = new PLCComms(plcDisabled);
                    PLCController.Connect(Settings.CurrentSettings.PLCIPAddress, int.Parse(Settings.CurrentSettings.PLCPort), 3000);
                    PLCCommInitd = PLCController.Connected;
                }
                catch (System.Net.Sockets.SocketException e)
                {
                    if (e.SocketErrorCode == System.Net.Sockets.SocketError.TimedOut)
                    {
                        ShowMessage("Unable to connect to the PLC. Please ensure the PLC is connected and the settings are correct.");
                        PLCCommInitd = false;
                    }
                    else throw; // And catch it down below
                }
            }
            catch (Exception e)
            {
                PLCCommInitd = false;
                ShowMessage("An error occurred while initializing the PLC communication: " + System.Environment.NewLine + e.Message);
                Logger.LogError(e.ToString());
            }
        }

        #endregion

        #region Cleanup

        private void PlaybackForm_FormClosing(object sender, FormClosingEventArgs e)
        {
            try
            {
                StopPlayback(); // Mostly to be sure we send out the PLC message, in case they close in the middle of playing
            }
            catch { } // Don't really care at this point
            System.Threading.Thread.Sleep(50); // Give the DMX a moment to be sure the clear signal has been sent
            SaveState(true);
        }

        private void SaveState(bool saveFormPositions)
        {
            if (!string.IsNullOrWhiteSpace(playlistFolderBrowser.SelectedPath))
                Settings.CurrentSettings.PlaylistDirectory = playlistFolderBrowser.SelectedPath;
            if (!string.IsNullOrWhiteSpace(songFolderBrowser.SelectedPath))
                Settings.CurrentSettings.SongDirectory = songFolderBrowser.SelectedPath;
            Settings.CurrentSettings.CurrentPlaylistSongColumnWidth = lvCurPlaylistSongs.Columns[0].Width;
            Settings.CurrentSettings.CurrentPlaylistArtistColumnWidth = lvCurPlaylistSongs.Columns[1].Width;

            if (saveFormPositions)
            {
                Settings.CurrentSettings.PlaybackPosition = Location;
                Settings.CurrentSettings.PlaybackSize = Size;
            }

            Settings.SerializeSettings();
        }

        #endregion

        #region DMX

        private void LightController_DMXConnectionChanged(object sender, bool connected)
        {
            DMXConnectionChanged(connected);
        }

        private void DMXConnectionChanged(bool connected)
        {
            if (InvokeRequired)
            {
                BeginInvoke(new Action(() => DMXConnectionChanged(connected)));
                return;
            }

            btnConnectDMX.Visible = !connected;

            if (!connected)
            {
                ShowMessage("The DMX controller has become disconnected. Attempting to reconnect. Please check your connection.");
                LightController.ReconnectDMX();
            }

            LightsInitd = connected;
        }

        #endregion

        #region Playlists

        private void lvCurPlaylistSongs_SelectedIndexChanged(object sender, EventArgs e)
        {
            if (lvCurPlaylistSongs.SelectedIndices.Count > 0)
            {
                CurrentPlaylist.StartIndex = lvCurPlaylistSongs.SelectedIndices[0];
                UpdateTimeUI(CurrentPlaylist.Name, CurrentPlaylist.Songs[CurrentPlaylist.StartIndex], TimeSpan.Zero, Player.GetTime(true, CurrentPlaylist.Songs[CurrentPlaylist.StartIndex]), Player.GetTime(true, CurrentPlaylist.Songs.Take(CurrentPlaylist.StartIndex).ToArray()), Player.GetTime(true, CurrentPlaylist.Songs));
            }
            else
                UpdateTimeUI(CurrentPlaylist.Name, "", TimeSpan.Zero, TimeSpan.Zero, TimeSpan.Zero, TimeSpan.Zero);
        }

        private void lvPlaylists_AfterLabelEdit(object sender, LabelEditEventArgs e)
        {
            // If the label is blank, I'm assuming they wanted to delete it
            if (e.Label == "" || lvPlaylists.Items[e.Item].Text == "" && e.Label == null)
            {
                DeleteSelectedPlaylist();
                return;
            }

            // A null label means they canceled the edit
            if (e.Label == null)
                return;

            string path = playlistFolderBrowser.SelectedPath;
            string fullOldName = Path.Combine(path, lvPlaylists.Items[e.Item].Text + Playlist.FileExtension);
            string fullNewName = Path.Combine(path, e.Label + Playlist.FileExtension);

            try
            {
                File.Move(fullOldName, fullNewName);
            }
            catch (IOException ex)
            {
                ShowMessage(ex.Message);
                Logger.LogError(ex.ToString());
                e.CancelEdit = true;
            }

            ResizeColumns(lvPlaylists);

            LoadPlaylist(e.Label);
        }

        private void lvPlaylists_SelectedIndexChanged(object sender, EventArgs e)
        {
            if (lvPlaylists.SelectedIndices.Count > 0)
                LoadPlaylist();
        }

        private void btnNewPlaylist_Click(object sender, EventArgs e)
        {
            try
            {
                string currentPath = playlistFolderBrowser.SelectedPath;

                // Create the (blank) file
                File.Create(Path.Combine(currentPath, Playlist.FileExtension)).Dispose();

                ListViewItem lvi = new ListViewItem("");
                lvPlaylists.Items.Add(lvi);
                lvi.BeginEdit();
            }
            catch (Exception ex)
            {
                ShowMessage("An error occurred while creating a new playlist:" + Environment.NewLine + ex.Message);
                Logger.LogError(ex.ToString());
            }
        }

        private void btnDeletePlaylist_Click(object sender, EventArgs e)
        {
            DeleteSelectedPlaylist();
        }

        private void btnRenamePlaylist_Click(object sender, EventArgs e)
        {
            RenameSelectedPlaylist();
        }

        private void lvPlaylists_KeyUp(object sender, KeyEventArgs e)
        {
            if (e.KeyCode == Keys.F2)
                RenameSelectedPlaylist();
            else if (e.KeyCode == Keys.Delete)
                DeleteSelectedPlaylist();
        }

        private void RenameSelectedPlaylist()
        {
            if (!LoggedIn)
                return;

            try
            {
                if (lvPlaylists.SelectedItems.Count > 0)
                    lvPlaylists.SelectedItems[0].BeginEdit();
            }
            catch (Exception ex)
            {
                ShowMessage("An error occurred while renaming the selected playlist:" + Environment.NewLine + ex.Message);
                Logger.LogError(ex.ToString());
            }
        }

        private void DeleteSelectedPlaylist()
        {
            if (!LoggedIn)
                return;

            try
            {
                if (lvPlaylists.SelectedItems.Count > 0 && MessageBox.Show("Are you sure you wish to delete playlist " + lvPlaylists.SelectedItems[0].Text + "?", "Delete Playlist", MessageBoxButtons.YesNo) == DialogResult.Yes)
                {
                    File.Delete(Path.Combine(playlistFolderBrowser.SelectedPath, lvPlaylists.SelectedItems[0].Text + Playlist.FileExtension));
                    lvPlaylists.SelectedItems[0].Remove();
                    if (lvPlaylists.Items.Count > 0)
                        lvPlaylists.Items[0].Selected = true;
                }
            }
            catch (Exception ex)
            {
                ShowMessage("An error occurred while deleting the selected playlist:" + Environment.NewLine + ex.Message);
                Logger.LogError(ex.ToString());
            }
        }

        private void btnAddToPlaylist_Click(object sender, EventArgs e)
        {
            AddSelectedSongToPlaylist();
        }

        private void lvSongs_DoubleClick(object sender, EventArgs e)
        {
            AddSelectedSongToPlaylist();
        }

        private void AddSelectedSongToPlaylist()
        {
            if (!LoggedIn)
                return;

            if (lvSongs.SelectedItems.Count > 0)
            {
                CurrentPlaylist.AddSong(Path.Combine(songFolderBrowser.SelectedPath, lvSongs.SelectedItems[0].Text + ".wav"));
                CurrentPlaylist.Save();
                LoadPlaylist();
            }
        }

        private void btnRemoveFromPlaylist_Click(object sender, EventArgs e)
        {
            if (!LoggedIn)
                return;

            if (lvCurPlaylistSongs.SelectedItems.Count > 0)
            {
                int selectedIndex = lvCurPlaylistSongs.SelectedIndices[0];
                CurrentPlaylist.RemoveSong(selectedIndex);
                CurrentPlaylist.Save();
                LoadPlaylist(null, selectedIndex);
            }
        }

        private void btnMoveUp_Click(object sender, EventArgs e)
        {
            if (!LoggedIn)
                return;

            if (lvCurPlaylistSongs.SelectedItems.Count > 0)
            {
                // We want to have the same item selected when they come back
                int selectedIndex = lvCurPlaylistSongs.SelectedIndices[0];
                CurrentPlaylist.MoveUp(selectedIndex);
                CurrentPlaylist.Save();
                LoadPlaylist(null, selectedIndex - 1);
            }
        }

        private void btnMoveDown_Click(object sender, EventArgs e)
        {
            if (!LoggedIn)
                return;

            if (lvCurPlaylistSongs.SelectedItems.Count > 0)
            {
                // We want to have the same item selected when they come back
                int selectedIndex = lvCurPlaylistSongs.SelectedIndices[0];
                CurrentPlaylist.MoveDown(selectedIndex);
                CurrentPlaylist.Save();
                LoadPlaylist(null, selectedIndex + 1);
            }
        }

        private void SelectSong(int songNumber)
        {
            if (InvokeRequired)
            {
                try
                {
                    BeginInvoke(new Action(() => SelectSong(songNumber)));
                }
                catch (ObjectDisposedException) { } // This means we're closing anyways, so no need to update the UI anymore
                return;
            }

            if (songNumber >= 0 && songNumber < lvCurPlaylistSongs.Items.Count)
            {
                lvCurPlaylistSongs.Items[songNumber].Selected = true;
                int songsAboveToShow = 2;
                if (songNumber - songsAboveToShow >= 0)
                    lvCurPlaylistSongs.TopItem = lvCurPlaylistSongs.Items[songNumber - songsAboveToShow];
                else
                    lvCurPlaylistSongs.TopItem = lvCurPlaylistSongs.Items[0];
            }
        }

        #endregion

        #region Announcements

        bool announcementInProgress = false;

        private void lvPlaylists_Click(object sender, EventArgs e)
        {
            lvAnnouncements.SelectedIndices.Clear();
        }

        private void lvCurPlaylistSongs_Click(object sender, EventArgs e)
        {
            lvAnnouncements.SelectedIndices.Clear();
        }

        private void btnAnnounce_Click(object sender, EventArgs e)
        {
            if (announcementInProgress)
            {
                StopAnnouncement();
            }
            else
            {
                if (lvAnnouncements.SelectedItems.Count > 0)
                {
                    string announcement = Path.Combine(Settings.CurrentSettings.AnnouncementDirectory, lvAnnouncements.SelectedItems[0].Text + ".wav");
                    new System.Threading.Thread(() => Announce(announcement)).Start();
                }
            }
        }

        private void btnAnnounce_EnabledChanged(object sender, EventArgs e)
        {
            UpdateAnnouncementButton();
        }

        private void UpdateAnnouncementButton()
        {
            bool stopped = SoundController == null || SoundController.PlaybackState == Player.PlayState.Stopped;
            if (btnAnnounce.Enabled)
            {
                btnAnnounce.Text = "PLAY SELECTED ANNOUNCEMENT";
                btnAnnounce.ForeColor = System.Drawing.Color.Yellow;
                btnAnnounce.BackColor = System.Drawing.Color.Red;
                btnAnnounce.UseVisualStyleBackColor = false;
            }
            else
            {
                if (stopped)
                {
                    btnAnnounce.Text = "PLAY ANNOUNCEMENT - SELECT ANNOUNCEMENT";
                }
                else
                {
                    btnAnnounce.Text = "PLAY ANNOUNCEMENT - STOP SHOW FIRST";
                }
                btnAnnounce.BackColor = System.Drawing.Color.DarkGray;
                btnAnnounce.UseVisualStyleBackColor = false;
            }
            if (announcementInProgress)
            {
                btnAnnounce.Text = "STOP ANNOUNCEMENT";
            }
            lvAnnouncements.Enabled = stopped;
        }

        private void lvAnnouncements_SelectedIndexChanged(object sender, EventArgs e)
        {
            SetPlaybackEnable(initComplete);

            lvAnnouncements.Items.Cast<ListViewItem>()
                .ToList().ForEach(item =>
                {
                    item.BackColor = item.Selected ? System.Drawing.SystemColors.Highlight : System.Drawing.SystemColors.Window;
                    item.ForeColor = item.Selected ? System.Drawing.SystemColors.HighlightText : System.Drawing.SystemColors.WindowText;
                });
        }

        private void Announce(string announcement)
        {
            announcementInProgress = true;
            SetPlaybackEnable(false);
            SetVolumeEnable(true);
            SoundController.Play(announcement);

            while (SoundController.PlaybackState == Player.PlayState.Playing)
                System.Threading.Thread.Sleep(RefreshRate);

            SetPlaybackEnable(true);
            SetOutput(0);
            announcementInProgress = false;
        }

        private void StopAnnouncement()
        {
            SoundController.Stop();
        }

        #endregion

        #region Playback Control

        private void StartPlayback()
        {
            stopping = false;

            if (SoundController.PlaybackState != Player.PlayState.Stopped)
                return;

            if (InvokeRequired)
            {
                try
                {
                    Invoke(new Action(() => StartPlayback()));
                }
                catch (ObjectDisposedException) { } // We're shutting down, it doesn't matter
                return;
            }

            bool goodPlaylist;
            try
            {
                goodPlaylist = CurrentPlaylist.Verify();
            }
            catch (Exception e)
            {
                ShowMessage("There was an error verifying the playlist: " + Environment.NewLine + e.Message);
                Logger.LogError(e.ToString());
                goodPlaylist = false;
            }

            if (goodPlaylist)
            {
                btnStartStop.Text = "Stop Show";
                btnPauseResume.Text = "Pause Show";
                btnPauseResume.Enabled = true;
                btnAnnounce.Enabled = btnSettings.Enabled =
                    txtManualFCW.Enabled = btnExecuteManualFCW.Enabled =
                    lvCurPlaylistSongs.Enabled = btnAddToPlaylist.Enabled = btnRemoveFromPlaylist.Enabled = btnMoveUp.Enabled = btnMoveDown.Enabled = false;
                barVolLeftChannel.Focus();

                new System.Threading.Thread(() => PlayAudio(CurrentPlaylist)) { IsBackground = true, Priority = System.Threading.ThreadPriority.AboveNormal }.Start();
            }
        }

        private void PausePlayback()
        {
            if (InvokeRequired)
            {

                try
                {
                    Invoke(new Action(() => PausePlayback()));
                }
                catch (ObjectDisposedException) { } // We're shutting down, it doesn't matter
                return;
            }

            SoundController.Pause();

            SetOutput(0);

            Logger.LogInfo("Pausing playback");

            btnPauseResume.Text = "Resume Show";
        }

        private void ResumePlayback()
        {
            if (InvokeRequired)
            {
                try
                {
                    Invoke(new Action(() => ResumePlayback()));
                }
                catch (ObjectDisposedException) { } // We're shutting down, it doesn't matter
                return;
            }

            SoundController.Play();
            barVolLeftChannel.Focus();

            Logger.LogInfo("Resuming playback");

            btnPauseResume.Text = "Pause Show";
        }

        private void StopPlayback()
        {
            stopping = true;

            if (SoundController != null) // This could be null if they close it real quick after startup
                SoundController.Stop();
            if (LightController != null)
                ResetLights();
            UpdateTimeUI(CurrentPlaylist.Name, "", TimeSpan.Zero, TimeSpan.Zero, TimeSpan.Zero, TimeSpan.Zero);

            // Make sure everyone knows we've stopped
            ExecuteCommands(new Command(99, 0));

            if (InvokeRequired)
            {
                try
                {
                    Invoke(new Action(() => StopPlayback()));
                }
                catch (ObjectDisposedException) { } // We're shutting down, it doesn't matter
                return;
            }

            Logger.LogInfo("Stopping playback");

            ForcePaint();

            SetOutput(0);

            btnStartStop.Text = "Start Show";
            btnPauseResume.Text = "Pause Show";
            btnPauseResume.Enabled = false;
            btnSettings.Enabled = txtManualFCW.Enabled = btnExecuteManualFCW.Enabled = lvCurPlaylistSongs.Enabled = true;
            btnAddToPlaylist.Enabled = btnRemoveFromPlaylist.Enabled = btnMoveUp.Enabled = btnMoveDown.Enabled = LoggedIn;
            btnAnnounce.Enabled = lvAnnouncements.SelectedItems.Count > 0;
            UpdateAnnouncementButton();
        }

        private void PlayLeader(uint lengthMilliseconds, uint numCommands)
        {
            Logger.LogInfo("Creating leader of length {0} and {1} commands", lengthMilliseconds, numCommands);

            CommandLine[] lines = new CommandLine[numCommands];
            uint timePerCommand = lengthMilliseconds / numCommands;
            uint curCommandTime = 0;
            for (int i = 0; i < lines.Length; i++)
            {
                lines[i] = new CommandLine(curCommandTime, new Command((int)numCommands - i, 0));
                curCommandTime += timePerCommand;
            }

            CommandFile file = new CommandFile(lines);

            Logger.LogInfo("Playing leader");
            TimeSpan length = TimeSpan.FromMilliseconds(lengthMilliseconds);
            PlaySong(CurrentPlaylist.Name, null, file, length, TimeSpan.Zero, length);
        }

        private void PlayAudio(Playlist playlist)
        {
            try
            {
                waterFCWsSent = waterFCWCount = 0;
                lightFCWsExecuted = lightFCWCount = 0;
                totalFCWsExecuted = totalFCWCount = 0;

                // New as of 6/1/2015: They want a 3-second "leader" to allow them a moment to see that the PLC is fully operational before the show begins
                // Also, I'm told sometimes we miss the first 3 FCWs of a show, so this ensures those are dummy FCWs
                PlayLeader(3000, 5);

                waterFCWsSent = 0;
                lightFCWsExecuted = 0;
                totalFCWsExecuted = 0;

                TimeSpan totalInShow = Player.GetTime(true, playlist.Songs);

                for (int i = playlist.StartIndex; i < playlist.Songs.Length; i++)
                {
                    if (stopping)
                        break;

                    if (Playlist.SongIsComment(playlist.Songs[i]))
                    {
                        // Move right along
                        Logger.LogInfo("Skipping song " + i + ", " + playlist.Songs[i] + " in playlist because it's a comment");
                        i++;
                        if (i >= playlist.Songs.Length) break;
                    }

                    // This first section can run pretty much as long as we want - we're not yet in a time-sensitive operation

                    TimeSpan totalInSong = Player.GetTime(false, playlist.Songs[i]);

                    TimeSpan timeInShow = Player.GetTime(true, playlist.Songs.Take(i).ToArray());
                    UpdateTimeUI(playlist.Name, playlist.Songs[i], TimeSpan.Zero, totalInSong, timeInShow, totalInShow);
                    SelectSong(i);

                    // When we start a new song, we need to be sure to unlock all the backcurtains
                    if (skipResetAfterSong)
                    {
                        Logger.LogInfo("Skipping light reset");
                    }
                    else
                    {
                        ResetLights();
                    }
                    skipResetAfterSong = false;

                    string supportingFileBase = Path.Combine(Path.GetDirectoryName(playlist.Songs[i]), Path.GetFileNameWithoutExtension(playlist.Songs[i]));
                    string controlFile = supportingFileBase + ".ctl";
                    string colorMapFile = supportingFileBase + ".map";

                    // Fall back on the default if we don't have a color scheme for this song
                    if (File.Exists(colorMapFile))
                        LoadColors(colorMapFile);
                    else
                        LoadColors();

                    CommandFile commandFile = new CommandFile(controlFile);

                    if (!File.Exists(playlist.Songs[i]))
                        throw new FileNotFoundException("Unable to find song " + playlist.Songs[i]);

                    PlaySong(playlist.Name, playlist.Songs[i], commandFile, totalInSong, timeInShow, totalInShow);

                    Logger.LogInfo("Stop time: {0}", SoundController.GetPosition());
                }
                // Make sure that we fully stop
                if (!stopping) // Already done elsewhere
                    StopPlayback();

                // Since we're at the end, go ahead and move them back to the top of the playlist
                SelectSong(0);
            }
            catch (Exception e)
            {
                ShowMessage("An error occurred during playback:" + Environment.NewLine + e.Message);
                Logger.LogError(e.ToString());
                StopPlayback();
            }
        }

        private void PlaySong(string showName, string song, CommandFile commandFile, TimeSpan totalInSong, TimeSpan timeInShow, TimeSpan totalInShow)
        {
            int cyclesPerRefresh = 5;
            int refreshNum = 0;
            totalFCWCount += CountCommands(out int tempWaterFCWCount, out int tempLightFCWCount, commandFile);
            waterFCWCount += tempWaterFCWCount;
            lightFCWCount += tempLightFCWCount;

            CommandLine currentLine;
            TimeSpan timeInSong = TimeSpan.Zero;

            // Here we start playing the song, so the time for (relatively) long-running tasks is over
            if (song != null)
            {
                Logger.LogInfo("Starting song {0}", song);
                SoundController.Play(song);
            }
            else
            {
                Logger.LogInfo("Starting blank 'song'");
                SoundController.PlayBlank((int)totalInSong.TotalMilliseconds);
                song = "Leader";
            }

            Invoke(new Action(() => { UpdateAnnouncementButton(); }));

            if (LoggedIn && TimeSpan.TryParse("00:" + txtStartPoint.Text, out TimeSpan skipToTime) && skipToTime < totalInSong)
            {
                SoundController.SkipTo(skipToTime);
                while (commandFile.GetCurrentLine() != null && commandFile.GetCurrentLine().TimeInMilliseconds < SoundController.GetPosition().TotalMilliseconds)
                    commandFile.NextLine();
                Invoke(new Action(() => { txtStartPoint.Text = ""; }));
            }

            System.Diagnostics.Stopwatch loopWatch = new System.Diagnostics.Stopwatch();
            System.Diagnostics.Stopwatch refreshWatch = new System.Diagnostics.Stopwatch();
            // Ensure we don't go past the end (that happened once on Come Together and I had to reboot the computer to get it back)
            while (SoundController.PlaybackState != Player.PlayState.Stopped && timeInSong <= totalInSong)
            {
                currentLine = commandFile.GetCurrentLine();

                // Wait until it's time to execute the command
                int timeTilNextCommand;
                do
                {
                    timeInSong = SoundController.GetPosition();
                    //Logger.LogDebug("Current position: {0}", timeInSong.ToString());
                    if (currentLine != null)
                        timeTilNextCommand = (int)(currentLine.TimeInMilliseconds - timeInSong.TotalMilliseconds);
                    else
                        timeTilNextCommand = int.MaxValue; // When we finish a file we start returning null, so we won't worry about anything til playback stops

                    // We'll sleep for up to RefreshRate ms (less if we don't have that long til it's time to command)
                    if (timeTilNextCommand > RefreshRate)
                        System.Threading.Thread.Sleep(RefreshRate);
                    else if (timeTilNextCommand > 0)
                        System.Threading.Thread.Sleep(timeTilNextCommand);

                    // Every refresh, we want to update the various time-related items in the light controller (shift and fade)
                    // Since we have commands on the 100 ms marks, we'll automatically realign with that at every command
                    // At one point I performed a 99900 ms fade - I was off by about 1500ms (1.5%) - I call that good enough
                    if (SoundController.PlaybackState == Player.PlayState.Playing)
                    {
                        refreshWatch.Restart();
                        LightController.Refresh();
                        // Refresh every X cycles to prevent flooding the UI with updates
                        if (refreshNum++ % cyclesPerRefresh == 0 || timeTilNextCommand == 0 || stopping)
                        {
                            ForcePaint();
                            UpdateTimeUI(showName, song, timeInSong, totalInSong, timeInShow.Add(timeInSong), totalInShow);
                            if (refreshWatch.ElapsedMilliseconds > Settings.CurrentSettings.RefreshWarnMS)
                                Logger.LogWarning("Refresh took {0}ms", refreshWatch.ElapsedMilliseconds);
                        }
                    }

                } while (timeTilNextCommand > 0 && SoundController.PlaybackState != Player.PlayState.Stopped && timeInSong <= totalInSong);

                if (!stopping)
                {
                    loopWatch.Restart();
                    if (currentLine != null)
                    {
                        ExecuteCommands(currentLine.Commands);
                        // If we've executed this line, we need to shift focus to the next one and wait for it
                        commandFile.NextLine();
                    }

                    // Just as a check - it shouldn't ever take this long, but we'll know if it does
                    if (loopWatch.ElapsedMilliseconds > Settings.CurrentSettings.LoopWarnMS)
                        Logger.LogWarning("Loop took {0}ms", loopWatch.ElapsedMilliseconds);
                }
            }
        }

        #endregion

        #region Commands

        private int CountCommands(out int waterCommands, out int lightCommands, params CommandFile[] commandFiles)
        {
            int totalCommands = 0;
            waterCommands = 0;
            lightCommands = 0;

            foreach (CommandFile commandFile in commandFiles)
            {
                foreach (CommandLine line in commandFile.Commands)
                {
                    foreach (Command command in line.Commands)
                    {
                        FCW fcw = FCWs[command.Address];
                        if (fcw == null) continue;

                        if ((fcw.Type & FCWType.Water) != FCWType.None)
                            waterCommands++;

                        if ((fcw.Type & FCWType.Light) != FCWType.None)
                            lightCommands++;

                        totalCommands++;
                    }
                }
            }

            return totalCommands;
        }

        private void ExecuteCommands(params Command[] commands)
        {
            // If we haven't finished loading in our FCWs this is going to be all sorts of problems
            // (That will only happen if they close it immediately after opening)
            if (!canExecute)
                return;

            string joinedCommands = string.Join(" ", (object[])commands);
            Logger.LogDebug("Executing commands {0}", joinedCommands);

            System.Collections.Generic.List<string> waterFCWs = new System.Collections.Generic.List<string>();
            System.Collections.Generic.List<string> lightFCWs = new System.Collections.Generic.List<string>();
            for (int i = 0; i < commands.Length; i++)
            {
                Command command = commands[i];
                try
                {
                    FCW fcwToExecute = null;
                    try
                    {
                        fcwToExecute = FCWs[command.Address];
                        if (fcwToExecute == null)
                            throw new NullReferenceException();
                    }
                    catch (Exception e)
                    {
                        if (e is IndexOutOfRangeException || e is NullReferenceException)
                        {
                            // Looks like they asked for an FCW beyond our ken
                            ShowMessage("Unknown FCW " + command.Address + " requested. Passing to PLC and ignoring");
                            Logger.LogError(e.ToString());
                            fcwToExecute = new FCW(command.Address, FCWType.Water, FCWLightRole.None);
                        }
                        else
                            throw;
                    }

                    // Special case (added 4/30/2015): we do care about the spout - if it's at 0, we force the voice lights to stop mirroring
                    // Addition (8/25/2017): the new helix has the same effect
                    bool change = false, lightsOff = false;
                    switch (fcwToExecute.Address)
                    {
                        case (int)SpecialFCWAddress.WaterSpout:
                            change = (command.Data & 32) != 0;
                            lightsOff = (command.Data & 0x7) == 0;
                            break;
                        case (int)SpecialFCWAddress.Voice:
                        case (int)SpecialFCWAddress.Helix:
                            change = true;
                            lightsOff = (command.Data == 0);
                            break;
                    }
                    if (change)
                    {
                        foreach (int light in FCWs[(int)SpecialFCWAddress.Voice].AffectedLights)
                        {
                            LightController.FirmlyLockLight(light, lightsOff);
                        }
                    }

                    if ((fcwToExecute.Type & FCWType.Water) != FCWType.None)
                    {
                        waterFCWs.Add(command.ToString());
                        // If it's just a water command, this is our only place to increment
                        if (fcwToExecute.Type == FCWType.Water)
                            totalFCWsExecuted++;
                        PLCController.AddToQueue(command.ToString());
                    }
                    if ((fcwToExecute.Type & FCWType.Light) != FCWType.None)
                    {
                        lightFCWs.Add(command.ToString());

                        LEDColor newColor = GetColor(command, out double intensity, out bool lockedColor);

                        // Special case: if this is a fade command, we need the following command to tell us where we're fading to
                        // Otherwise, we just up and skip
                        if (fcwToExecute.Role == FCWLightRole.Fade)
                        {
                            if (commands.Length <= i + 1)
                            {
                                ShowMessage("Fade FCW " + fcwToExecute.Address + " found without following color - ignoring");
                                continue;
                            }
                            FCW nextFCW = FCWs[commands[i + 1].Address];
                            if (!System.Linq.Enumerable.SequenceEqual(nextFCW.AffectedLights, fcwToExecute.AffectedLights))
                            {
                                ShowMessage("Fade FCW " + fcwToExecute.Address + " followed by non-matching FCW " + nextFCW.Address + " - ignoring both");
                                continue;
                            }
                            newColor = GetColor(commands[i + 1], out intensity, out lockedColor);
                            lightFCWsExecuted++;
                            totalFCWsExecuted++;
                            i++; // Skip the next command (since it's just a color setter)
                        }

                        // An individual light command takes priority over anything else
                        if (fcwToExecute.AffectedLights.Length == 1)
                        {
                            lockedColor = true;
                        }

                        // Terry has decided than an all-off command (053-000) still turns everything off, even individually-commanded lights
                        // In other words, light-wise it's exactly the same as a reset (099-000)
                        if (fcwToExecute.Address == (int)SpecialFCWAddress.All && newColor == LEDColor.Black)
                        {
                            ResetLights();
                        }
                        else
                        {
                            foreach (int light in fcwToExecute.AffectedLights)
                            {
                                switch (fcwToExecute.Role)
                                {
                                    case FCWLightRole.TurnOnOff:
                                        // Note that under some circumstances newColor might be null here
                                        // But those are undefined circumstances (that is, the choreographer screwed up and put in a nonexistent color number) so we'll just let it happen
                                        LightController.SetLightColor(light, newColor, intensity, lockedColor);
                                        LightController.SetRawValue(light, (byte)command.Data);
                                        break;
                                    case FCWLightRole.Fade:
                                        LightController.FadeLight(light, newColor, intensity, lockedColor, command.Data * 100);
                                        break;
                                    default:
                                        break;
                                }
                                if (lockedColor && newColor == LEDColor.Black)
                                    LightController.UnlockLight(light);
                            }
                        }
                        lightFCWsExecuted++;
                        totalFCWsExecuted++;
                    }
                    if ((fcwToExecute.Type & FCWType.Special) != FCWType.None)
                    {
                        ExecuteSpecialCommand(command);

                        if (fcwToExecute.Type == FCWType.Special)
                            totalFCWsExecuted++;
                    }
                }
                catch (Exception e)
                {
                    // To avoid crashing during playback, we'll catch our exceptions and throw them onscreen
                    // But non-modally so they keep control
                    ShowMessage("An error occurred during playback: " + Environment.NewLine + e.Message);
                    Logger.LogError(e.ToString());
                }
            }

            try
            {
                waterFCWsSent += PLCController.SendQueue();
            }
            catch (Exception e)
            {
                ShowMessage("An error occurred sending to the PLC: " + Environment.NewLine + e.Message + Environment.NewLine + "Attempting to reconnect...");
                Logger.LogError(e.ToString());
            }

            UpdateFCWs(string.Join(" ", waterFCWs), string.Join(" ", lightFCWs), waterFCWsSent, waterFCWCount, lightFCWsExecuted, lightFCWCount, totalFCWsExecuted, totalFCWCount);
        }

        private LEDColor GetColor(Command command, out double intensity, out bool lockedColor)
        {
            intensity = command.Data / 100 / 10d;
            // For the sake of backwards compatibility, an intensity of 0 is in fact full intensity
            // If they want to turn it off, they just tell it color 0 (black)
            if (intensity == 0)
                intensity = 1;

            int colorNumber = command.Data % 100;
            lockedColor = false;

            LEDColor newColor = null;
            if (colorNumber < Colors.Length)
                newColor = Colors[colorNumber];

            // I'm not sure where to put this so I'm doing it here
            switch (command.Address)
            {
                // The back curtain (024) is very special for a few reasons
                // For one, it has its own special color scheme (16 = green, 32 = yellow)
                // For another, if it's turned on it means none of the other FCWs may affect those lights
                case (int)SpecialFCWAddress.BackCurtain_Legacy:
                    switch (command.Data)
                    {
                        case 0:
                            newColor = LEDColor.Black;
                            break;
                        case 16:
                            newColor = LEDColor.BackCurtain16;
                            break;
                        case 32:
                            newColor = LEDColor.BackCurtain32;
                            break;
                        case 48:
                            newColor = LEDColor.BackCurtain48;
                            break;
                        default: // I guess leave whatever color we had before (or null) - this is undefined behavior
                            break;
                    }
                    lockedColor = true;
                    break;

                // All back curtain commands should lock our colors (though they use the standard coloration)
                case (int)SpecialFCWAddress.BackCurtain:
                case (int)SpecialFCWAddress.BackCurtainFade:
                    lockedColor = true;
                break;

                // To my great surprise, there's only one allowed voice color, number 1
                // Addendum, 4/30/2015: it seems the voice lights are also part of module 4, but this color overrides it

                // Note, 6/1/2016: Jason has asked to keep the voice on for "a few seconds" after turning off the voice/spout - this is where we'll do it
                // I'm envisioning something along the lines where upon turning it on we set the "real" off time to infinity
                // And upon turning it off we add X (configurable) seconds to the current time and mark that as the "real" off time
                // And, of course, once we hit the "real" off time we actually turn it to black
                // Note that it will not behave as expected when paused, but that's not a feature they use during performance (and I'd put this down as intended behavior)
                // I guess a 99/0 (or, by extension, ending a song) would just turn it straight off?
                case (int)SpecialFCWAddress.Voice:
                    switch (command.Data)
                    {
                        case 0:
                            newColor = LEDColor.Black;
                            break;
                        case 1:
                            newColor = LEDColor.VoiceSlashSpout;
                            break;
                    }
                    lockedColor = true;
                    break;
            }

            return newColor;
        }

        private void ExecuteSpecialCommand(Command command)
        {
            // If it's a special water command, it should have already been sent
            switch (command.Address)
            {
                case (int)SpecialFCWAddress.SwapAandB:
                    bool changeLights = (command.Data & 2) > 0;
                    bool AtoB = (command.Data & 16) > 0;
                    bool BtoA = (command.Data & 32) > 0;
                    if (changeLights)
                        LightController.SwapModuleLights(AtoB, BtoA);
                    break;
                case (int)SpecialFCWAddress.ShiftRotate: // Shift/rotate lights and water
                    // All 0 is stop
                    // 000000XX is direction
                    // 00XX0000 is rotate or no
                    // 0X000000 is repeat or no (1 means keep going at the rate defined by 86, 0 is one-time)
                    if (command.Data == 0)
                        LightController.EndShift();
                    else
                    {
                        // The FCL requires a bit for right, left, shift, and rotate
                        // but...if we're not shifting right we must be shifting left, and if we're not rotating we must be shifting only
                        bool right = (command.Data & 1) > 0;
                        //bool left = (command.Data & 2) > 0;
                        //bool shiftOnly = (command.Data & 16) > 0;
                        bool rotate = (command.Data & 32) > 0;
                        bool repeat = (command.Data & 64) > 0;

                        LightController.BeginShift(right, rotate, repeat);
                    }
                    break;
                case (int)SpecialFCWAddress.SetShiftRotateTimer: // For ShiftRotate - Data is in tenths of a second, the timer will want milliseconds
                    LightController.SetShiftTimer(command.Data * 100);
                    break;
                case (int)SpecialFCWAddress.Reset:
                    switch (command.Data)
                    {
                        case 0:
                            ResetLights();
                            break;
                        case 77:
                            skipResetAfterSong = true;
                            Logger.LogInfo("Will skip light reset after current song");
                            break;
                    }
                    break;
            }
        }

        private void ExecuteManualFCW()
        {
            try
            {
                if (txtManualFCW.Text == secretWord)
                {
                    DoEasterEgg();
                    return;
                }

                // Make it into a fake command line to maximize code reuse (so we can do a whole batch of commands)
                CommandLine manualCommands = new CommandLine("00:00.0" + txtManualFCW.Text);
                try
                {
                    ExecuteCommands(manualCommands.Commands);
                    ForcePaint();

                    txtManualFCW.Text = "";
                }
                catch (Exception ex)
                {
                    ShowMessage("Error running command " + txtManualFCW.Text + ". Make sure it's a valid FCW." + Environment.NewLine + ex.Message);
                    Logger.LogWarning(ex.ToString());
                }
            }
            catch (Exception ex)
            {
                ShowMessage("Error parsing command " + txtManualFCW.Text + ". Make sure it's a valid FCW.");
                Logger.LogWarning(ex.ToString());
            }
        }

        public void ResetLights()
        {
            LightController.Reset(true);
            foreach (int light in FCWs[(int)SpecialFCWAddress.Voice].AffectedLights)
                LightController.FirmlyLockLight(light, true);
        }

        #endregion

        #region UI Updates

        private void SetPlaybackEnable(bool enable)
        {
            if (InvokeRequired)
            {
                try
                {
                    BeginInvoke(new Action(() => SetPlaybackEnable(enable)));
                }
                catch (ObjectDisposedException) { } // This means we're closing anyways, so no need to update the UI anymore
                return;
            }

            btnStartStop.Enabled = barVolLeftChannel.Enabled = txtManualFCW.Enabled = btnExecuteManualFCW.Enabled = enable;
            if (!enable) btnPauseResume.Enabled = enable;
            if (announcementInProgress)
            {
                btnAnnounce.Enabled = true;
            }
            else
            {
                ResizeColumns(lvAnnouncements);
                btnAnnounce.Enabled = enable && lvAnnouncements.SelectedItems.Count > 0 && SoundController?.PlaybackState == Player.PlayState.Stopped;
            }
            UpdateAnnouncementButton();
        }

        private void SetVolumeEnable(bool enable)
        {
            if (InvokeRequired)
            {
                try
                {
                    BeginInvoke(new Action(() => SetPlaybackEnable(enable)));
                }
                catch (ObjectDisposedException) { } // This means we're closing anyways, so no need to update the UI anymore
                return;
            }

            barVolLeftChannel.Enabled = enable;
        }

        private void SetAdminControl(bool enable)
        {
            if (InvokeRequired)
            {
                try
                {
                    BeginInvoke(new Action(() => SetAdminControl(enable)));
                }
                catch (ObjectDisposedException) { } // This means we're closing anyways, so no need to update the UI anymore
                return;
            }
            tlpLists.SuspendLayout();
            if (enable)
            {
                tlpLists.RowStyles[1].Height = 25;
                tlpLists.RowStyles[2].SizeType = SizeType.Percent;
                tlpLists.RowStyles[2].Height = 50;
                tlpLists.RowStyles[12].Height = 20;
                tlpLists.RowStyles[13].Height = 20;
                tlpLists.ColumnStyles[3].SizeType = SizeType.Percent;
                tlpLists.ColumnStyles[4].SizeType = SizeType.Percent;
                tlpLists.ColumnStyles[5].SizeType = SizeType.Percent;
                tlpLists.ColumnStyles[6].SizeType = SizeType.Absolute;
                tlpLists.ColumnStyles[6].Width = 0;
                tlpLists.ColumnStyles[7].SizeType = SizeType.Absolute;
                tlpLists.ColumnStyles[7].Width = 0;
                tlpLists.ColumnStyles[8].SizeType = SizeType.Absolute;
                tlpLists.ColumnStyles[8].Width = 0;
                tlpLists.SetColumn(btnAnnounce, 9);
                tlpLists.SetColumnSpan(btnAnnounce, 4);
                tlpLists.SetColumn(lvAnnouncements, 9);
                tlpLists.SetColumnSpan(lvAnnouncements, 4);
                tlpLists.SetRow(lvAnnouncements, 2);
                tlpLists.SetRowSpan(lvAnnouncements, 1);
            }
            else
            {
                tlpLists.RowStyles[1].Height = 0;
                tlpLists.RowStyles[2].SizeType = SizeType.Absolute;
                tlpLists.RowStyles[2].Height = 0;
                tlpLists.RowStyles[12].Height = 0;
                tlpLists.RowStyles[13].Height = 0;
                tlpLists.ColumnStyles[3].SizeType = SizeType.Absolute;
                tlpLists.ColumnStyles[3].Width = 0;
                tlpLists.ColumnStyles[4].SizeType = SizeType.Absolute;
                tlpLists.ColumnStyles[4].Width = 0;
                tlpLists.ColumnStyles[5].SizeType = SizeType.Absolute;
                tlpLists.ColumnStyles[5].Width = 0;
                tlpLists.ColumnStyles[6].SizeType = SizeType.Percent;
                tlpLists.ColumnStyles[7].SizeType = SizeType.Percent;
                tlpLists.ColumnStyles[8].SizeType = SizeType.Percent;
                tlpLists.SetColumn(btnAnnounce, 6);
                tlpLists.SetColumnSpan(btnAnnounce, 3);
                tlpLists.SetColumn(lvAnnouncements, 6);
                tlpLists.SetColumnSpan(lvAnnouncements, 3);
                tlpLists.SetRow(lvAnnouncements, 3);
                tlpLists.SetRowSpan(lvAnnouncements, 6);
            }
            int numColumns = 0;
            for (int i = 0; i < tlpLists.ColumnStyles.Count; i++)
                if (tlpLists.ColumnStyles[i].SizeType == SizeType.Percent)
                    numColumns++;
            float size = 100 / (numColumns - 1);
            for (int i = 0; i < tlpLists.ColumnStyles.Count; i++)
                if (tlpLists.ColumnStyles[i].SizeType == SizeType.Percent)
                    tlpLists.ColumnStyles[i].Width = size;
            tlpLists.ResumeLayout();
            Logger.LogInfo("User logged {0}", enable ? "in" : "out");
            btnLogin.Text = enable ? "Log out" : "Log in";
            btnBrowsePlaylist.Visible = btnBrowseSong.Visible = enable;
            lblCurrentPlaylistFolder.Visible = lblCurrentSongFolder.Visible = enable;
            lvSongs.Visible = enable;
            btnNewPlaylist.Visible = btnRenamePlaylist.Visible = btnDeletePlaylist.Visible = btnAddToPlaylist.Visible = btnRemoveFromPlaylist.Visible = btnMoveUp.Visible = btnMoveDown.Visible = enable;
            btnSettings.Visible = txtManualFCW.Visible = btnExecuteManualFCW.Visible = enable;
            lvPlaylists.LabelEdit = enable;

            if (CurrentPlaylist != null)
                LoadPlaylist(CurrentPlaylist.Name, CurrentPlaylist.StartIndex, true);
        }

        private void UpdateTimeUI(string showTitle, string songTitle, TimeSpan timeInSong, TimeSpan totalInSong, TimeSpan timeInShow, TimeSpan totalInShow)
        {
            if (InvokeRequired)
            {
                try
                {
                    BeginInvoke(new Action(() => UpdateTimeUI(showTitle, songTitle, timeInSong, totalInSong, timeInShow, totalInShow)));
                }
                catch (ObjectDisposedException) { } // This means we're closing anyways, so no need to update the UI anymore
                return;
            }

            timeInSong = timeInSong.Truncate();
            totalInSong = totalInSong.Truncate();
            timeInShow = timeInShow.Truncate();
            totalInShow = totalInShow.Truncate();

            if (!string.IsNullOrWhiteSpace(songTitle) && File.Exists(songTitle))
                songTitle = Path.GetFileNameWithoutExtension(songTitle);
            lblCurrentShow.Text = "Now Playing Show: " + showTitle;
            lblCurrentSong.Text = "Now Playing Song: " + songTitle;
            lblSongProgress.Text = "Song: " + timeInSong.ToDisplayTime() + " of " + totalInSong.ToDisplayTime() + " (" + totalInSong.Subtract(timeInSong).ToDisplayTime() + " remaining)";
            lblShowProgress.Text = "Show: " + timeInShow.ToDisplayTime() + " of " + totalInShow.ToDisplayTime() + " (" + totalInShow.Subtract(timeInShow).ToDisplayTime() + " remaining)";
        }

        public void UpdateFCWs(string waterFCWs, string lightFCWs, int waterFCWsDone, int waterFCWsTotal, int lightFCWsDone, int lightFCWsTotal, int allFCWsDone, int allFCWsTotal)
        {
            if (InvokeRequired)
            {
                try
                {
                    BeginInvoke(new Action(() => UpdateFCWs(waterFCWs, lightFCWs, waterFCWsDone, waterFCWsTotal, lightFCWsDone, lightFCWsTotal, allFCWsDone, allFCWsTotal)));
                }
                catch (ObjectDisposedException) { } // This means we're closing anyways, so no need to update the UI anymore
                return;
            }

            if (waterFCWs.Length > 0)
                lblWaterFCWs.Text = "Current Water FCW(s): " + waterFCWs;
            if (lightFCWs.Length > 0)
                lblLightFCWs.Text = "Current Light FCW(s): " + lightFCWs;

            lblWaterFCWCount.Text = string.Format("{0}/{1}\nwater FCWs sent", waterFCWsDone, waterFCWsTotal);
            lblLightFCWCount.Text = string.Format("{0}/{1}\nlight FCWs executed", lightFCWsDone, lightFCWsTotal);
            lblTotalFCWCount.Text = string.Format("{0}/{1}\ntotal FCWs executed", allFCWsDone, allFCWsTotal);
        }

        void SoundController_VolumeChanged(object sender, float newVolLeft, float newVolRight)
        {
            SetUIVolume(newVolLeft, newVolRight);
        }

        private void barVolume_ValueChanged(object sender, EventArgs e)
        {
            SetSystemVolume(barVolLeftChannel.Value / 100f, null);
        }

        private void SetUIVolume(float leftChannel, float rightChannel)
        {
            if (InvokeRequired)
            {
                try
                {
                    BeginInvoke(new Action(() => SetUIVolume(leftChannel, rightChannel)));
                }
                catch (ObjectDisposedException) { } // This means we're closing anyways, so no need to update the UI anymore
                return;
            }

            barVolLeftChannel.Value = (int)Math.Round(leftChannel * 100);
            ttVolume.SetToolTip(barVolLeftChannel, barVolLeftChannel.Value.ToString());
        }

        private void SetSystemVolume(float? leftChannel, float? rightChannel)
        {
            SoundController.LeftVolume = leftChannel ?? SoundController.LeftVolume;
            SoundController.RightVolume = rightChannel ?? SoundController.RightVolume;
        }

        private void SoundController_OutputLevelChanged(object sender, float maxSample)
        {
            SetOutput(maxSample);
        }

        private void SetOutput(float output)
        {
            if (InvokeRequired)
            {
                try
                {
                    BeginInvoke(new Action(() => SetOutput(output)));
                }
                catch (ObjectDisposedException) { } // This means we're closing anyways, so no need to update the UI anymore
                return;
            }

            volMeter.Amplitude = output;
        }

        private void ForcePaint()
        {
            if (InvokeRequired)
            {
                try
                {
                    Invoke(new System.Action(() => ForcePaint()));
                }
                catch (System.ObjectDisposedException) { } // We're shutting down, it doesn't matter
                return;
            }

            foreach (LightControl light in pnlFountain.Controls.OfType<LightControl>())
            {
                light.SetColor(LightController?.GetLightColor(light.LightNumber));
            }
        }

        #endregion

        #region Form Events

        private void btnBrowsePlaylist_Click(object sender, EventArgs e)
        {
            if (playlistFolderBrowser.ShowDialog(this) == DialogResult.OK)
                LoadListView(lvPlaylists, lblCurrentPlaylistFolder, playlistFolderBrowser.SelectedPath, "*" + Playlist.FileExtension);
        }

        private void btnBrowseSong_Click(object sender, EventArgs e)
        {
            if (songFolderBrowser.ShowDialog(this) == DialogResult.OK)
                LoadListView(lvSongs, lblCurrentSongFolder, songFolderBrowser.SelectedPath, "*.wav");
        }

        private void btnStartStop_Click(object sender, EventArgs e)
        {
            switch (SoundController.PlaybackState)
            {
                case Player.PlayState.Stopped:
                    StartPlayback();
                    break;
                default: // If we're playing or paused, we want to full-stop
                    if (MessageBox.Show("Are you sure you wish to stop the show?", "Stop Show?", MessageBoxButtons.YesNo, MessageBoxIcon.Question) == DialogResult.Yes)
                    {
                        StopPlayback();
                    }
                    break;
            }
        }

        private void btnPauseResume_Click(object sender, EventArgs e)
        {
            switch (SoundController.PlaybackState)
            {
                case Player.PlayState.Playing:
                    PausePlayback();
                    break;
                case Player.PlayState.Paused:
                    ResumePlayback();
                    break;
                default: // This should always be disabled when we're stopped
                    break;
            }
        }

        private void btnExecuteManualFCW_Click(object sender, EventArgs e)
        {
            ExecuteManualFCW();
        }

        private void txtManualFCW_KeyPress(object sender, KeyPressEventArgs e)
        {
            if (e.KeyChar == (char)Keys.Return)
            {
                ExecuteManualFCW();
                e.Handled = true;
            }
        }

        private void btnLogin_Click(object sender, EventArgs e)
        {
            if (!LoggedIn)
            {
                string password = "APEX8422"; // Default
                string passwordFile = Path.Combine(@"C:\", "GHMF", "Config", "GHMFP.pas");
                if (File.Exists(passwordFile))
                {
                    using (StreamReader pwStream = File.OpenText(passwordFile))
                    {
                        string potentialPassword = "";
                        while (potentialPassword.Length == 0 && !pwStream.EndOfStream)
                            potentialPassword = pwStream.ReadLine();
                        if (potentialPassword.Length > 0)
                            password = potentialPassword;
                    }
                }

                using (LoginForm loginForm = new LoginForm(password, "APEX8373")) // Yeah, I'm adding in a backdoor, at Terry's request, but really this ain't no high-level security to begin with
                {
                    LoggedIn = (loginForm.ShowDialog(this) == DialogResult.OK);
                }
            }
            else
                LoggedIn = false;
        }

        private void btnSettings_Click(object sender, EventArgs e)
        {
            using (SettingsForm settingsForm = new SettingsForm(plcDisabled, dmxDisabled))
            {
                if (settingsForm.ShowDialog(this) == DialogResult.OK)
                {
                    if (PLCController != null && PLCController.Connected)
                        PLCController.Disconnect();
                    plcDisabled = settingsForm.PLC_Disabled;
                    dmxDisabled = settingsForm.DMX_Disabled;

                    lblStartPoint.Visible = txtStartPoint.Visible = Settings.CurrentSettings.ShowStartAt;

                    SaveState(false);
                    SetPlaybackEnable(false);
                    new System.Threading.Thread(() => InitAll()) { IsBackground = true }.Start();
                }
            }
        }

        private void btnConnectDMX_Click(object sender, EventArgs e)
        {
            if (LightController != null)
            {
                if (LightController.DMXConnected)
                    LightController.DisconnectDMX();
                else
                    LightController.ConnectDMX();
            }
        }

        private void PlaybackForm_Resize(object sender, EventArgs e)
        {
            ResizeColumns(lvPlaylists);
            ResizeColumns(lvSongs);
            ResizeColumns(lvAnnouncements);

            volMeter.Invalidate();
        }

        private void ResizeColumns(ListView listView)
        {
            // -1 to size to fit contents, but it seems just a little too much (we see horizontal scrollbars)
            listView.Columns[0].Width = -1;
            // So subtract a little!
            listView.Columns[0].Width -= 2;
        }

        #endregion

        #region Utilities

        internal static void ShowMessage(string message)
        {
            new System.Threading.Thread(() => MessageBox.Show(message)) { IsBackground = true }.Start();
        }

        internal static bool IsOnScreen(Form form)
        {
            return Screen.AllScreens.Any(s => s.WorkingArea.IntersectsWith(form.DesktopBounds));
        }

        internal static string GetArtistName(string fileName)
        {
            try
            {
                int artistHeader = 13; // Found by looping through all possibilities - I suppose on a different version of Windows this might not work

                string folder = Path.GetDirectoryName(fileName);
                string file = Path.GetFileName(fileName);

                Shell32.Folder objFolder = GetShell32NameSpaceFolder(folder);

                return objFolder.GetDetailsOf(objFolder.ParseName(file), artistHeader);
            }
            catch (Exception e)
            {
                Logger.LogError(e.ToString());
                return "<Artist Info Unavailable>";
            }
        }

        private static Shell32.Folder GetShell32NameSpaceFolder(string folder)
        {
            // This is apparently necessary when the build and target machines are not practically identical
            // Thanks, StackOverflow!
            var shellAppType = Type.GetTypeFromProgID("Shell.Application");
            var shell = Activator.CreateInstance(shellAppType);
            return (Shell32.Folder)shellAppType.InvokeMember("NameSpace", System.Reflection.BindingFlags.InvokeMethod, null, shell, new[] { folder });
        }

        #endregion

        #region Shh

        private const string secretWord = "iridescent";
        // Not what you'd call creative, but it's a classic
        private readonly Keys[] easterEgg = { Keys.Up, Keys.Up, Keys.Down, Keys.Down, Keys.Left, Keys.Right, Keys.Left, Keys.Right, Keys.B, Keys.A, Keys.Enter };
        private int easterEggIndex = 0;
        protected override bool ProcessCmdKey(ref Message msg, Keys keyData)
        {
            bool handled = false;
            // Don't let any of the plain old operators do this - accidentally (yeah right) or on purpose
            if (LoggedIn && keyData == easterEgg[easterEggIndex])
            {
                easterEggIndex++;
                // Eat the ones that could interfere with things - if they've already done the arrow directions they're clearly doing this on purpose
                if (keyData == Keys.A || keyData == Keys.B || keyData == Keys.Enter)
                    handled = true;
            }
            else
                easterEggIndex = 0;
            if (easterEggIndex == easterEgg.Length)
            {
                DoEasterEgg();
                easterEggIndex = 0;
            }

            return handled || base.ProcessCmdKey(ref msg, keyData);
        }

        private void DoEasterEgg()
        {
            new System.Threading.Thread(() => DoEasterEggThread()) { IsBackground = true }.Start();
        }

        private void DoEasterEggThread()
        {
            if (LightsInitd && SoundInitd && SoundController.PlaybackState == Player.PlayState.Stopped)
            {
                ExecuteCommands(new Command(99, 0), new Command(48, 48));
                System.Collections.Generic.List<System.Threading.Thread> threads = new System.Collections.Generic.List<System.Threading.Thread>();
                // Loop through all the individual lights and rainbow them (500 is the start of our individual-light addresses)
                foreach (FCW fcw in FCWs.Where(f => f != null && f.Address > 500 && f.Role == FCWLightRole.TurnOnOff && f.AffectedLights.Length == 1).OrderBy(f => f.AffectedLights.First()))
                {
                    System.Threading.Thread thread = new System.Threading.Thread(() => DoLightEasterEgg(fcw.Address)) { IsBackground = true };
                    threads.Add(thread);
                    thread.Start();
                    System.Threading.Thread.Sleep(50);
                }
                foreach (System.Threading.Thread thread in threads)
                    thread.Join(5000);

                ExecuteCommands(new Command(99, 0));
                ForcePaint();
            }
        }

        private void DoLightEasterEgg(int fcw)
        {
            for (int color = 1; color < Colors.Length; color++)
            {
                // Once we hit black we're done
                if (Colors[color] == LEDColor.Black)
                    break;

                ExecuteCommands(new Command(fcw, color));
                ForcePaint();
                System.Threading.Thread.Sleep(50);
            }
            ExecuteCommands(new Command(fcw, 0));
        }

        #endregion
    }
}

internal static class NativeMethods
{
    // Import SetThreadExecutionState Win32 API and necessary flags
    [System.Runtime.InteropServices.DllImport("kernel32.dll")]
    public static extern uint SetThreadExecutionState(uint esFlags);
    public const uint ES_CONTINUOUS = 0x80000000;
    public const uint ES_DISPLAY_REQUIRED = 0x00000002;
    public const uint ES_SYSTEM_REQUIRED = 0x00000001;
}