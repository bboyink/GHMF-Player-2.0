namespace Playback
{
    public partial class InitStatus : System.Windows.Forms.Form
    {
        private System.Windows.Forms.Label[] labels;

        public enum InitStep
        {
            LoadFCWs,
            LoadColors,
            LoadPlaylists,
            LoadSongs,
            LoadAnnouncements,
            InitSound,
            InitLights,
            InitPLCComms,
            LoadCurrentPlaylist
        }

        public InitStatus()
        {
            InitializeComponent();

            labels = new System.Windows.Forms.Label[] {
                lblFCWsLoaded,
                lblColorsLoaded,
                lblPlaylistsLoaded,
                lblSongsLoaded,
                lblAnnouncementsLoaded,
                lblSoundInitialized,
                lblLightsInitialized,
                lblPLCCommsInitialized,
                lblCurrentPlaylistLoaded };

            Reset();
        }

        public void Reset()
        {
            if (InvokeRequired)
            {
                try
                {
                    Invoke(new System.Action(() => Reset()));
                }
                catch (System.ObjectDisposedException) { } // We're shutting down, it doesn't matter
                return;
            }

            foreach (System.Windows.Forms.Label label in labels)
            {
                label.Tag = false;
                label.ForeColor = System.Drawing.SystemColors.ControlText;
                label.Text = "Initializing...";
            }
        }

        public void Complete(InitStep step, bool success)
        {
            if (InvokeRequired)
            {
                try
                {
                    Invoke(new System.Action(() => Complete(step, success)));
                }
                catch (System.ObjectDisposedException) { } // We're shutting down, it doesn't matter
                return;
            }

            System.Windows.Forms.Label labelToChange = null;
            switch (step)
            {
                case InitStep.LoadFCWs:
                    labelToChange = lblFCWsLoaded;
                    break;
                case InitStep.LoadColors:
                    labelToChange = lblColorsLoaded;
                    break;
                case InitStep.LoadPlaylists:
                    labelToChange = lblPlaylistsLoaded;
                    break;
                case InitStep.LoadSongs:
                    labelToChange = lblSongsLoaded;
                    break;
                case InitStep.LoadAnnouncements:
                    labelToChange = lblAnnouncementsLoaded;
                    break;
                case InitStep.InitSound:
                    labelToChange = lblSoundInitialized;
                    break;
                case InitStep.InitLights:
                    labelToChange = lblLightsInitialized;
                    break;
                case InitStep.InitPLCComms:
                    labelToChange = lblPLCCommsInitialized;
                    break;
                case InitStep.LoadCurrentPlaylist:
                    labelToChange = lblCurrentPlaylistLoaded;
                    break;
                default:
                    return;
            }
            if (success)
            {
                labelToChange.Tag = true;
                labelToChange.Text = "Complete";
                labelToChange.ForeColor = System.Drawing.Color.DarkGreen;

                bool allComplete = true;
                foreach (System.Windows.Forms.Label label in labels)
                    allComplete &= (bool)label.Tag;

                if (allComplete)
                    Close();
            }
            else
            {
                labelToChange.Text = "Failed";
                labelToChange.ForeColor = System.Drawing.Color.DarkRed;
            }
        }
    }
}
