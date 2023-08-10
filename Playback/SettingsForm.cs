using System;

namespace Playback
{
    public partial class SettingsForm : System.Windows.Forms.Form
    {
        public bool PLC_Disabled = false;
        public bool DMX_Disabled = false;

        public SettingsForm(bool plcDisabled, bool dmxDisabled)
        {
            InitializeComponent();

            cbAudioDevices.Items.AddRange(Player.GetAudioOutputs());

            // Load the audio output device
            string selectedDevice = Player.FriendlyNameFromUID(Settings.CurrentSettings.AudioEndpointID);
            if (cbAudioDevices.Items.Contains(selectedDevice))
                cbAudioDevices.SelectedItem = selectedDevice;
            else
                cbAudioDevices.SelectedIndex = 0;

            // Load the playback latency
            numLatency.Value = Settings.CurrentSettings.PlaybackLatency;
            numUpdateRate.Value = Settings.CurrentSettings.UpdateRate;

            // Load the PLC comms settings
            txtPLCIP.Text = Settings.CurrentSettings.PLCIPAddress;
            txtPLCPort.Text = Settings.CurrentSettings.PLCPort;

            chkShowStartAt.Checked = Settings.CurrentSettings.ShowStartAt;

            // Load the files and folders
            txtLightFCWMap.Text = Settings.CurrentSettings.FCWMap;
            txtLightDMXMap.Text = Settings.CurrentSettings.DMXMap;
            txtDefaultColorMap.Text = Settings.CurrentSettings.DefaultColorMap;
            txtAnnouncementDir.Text = Settings.CurrentSettings.AnnouncementDirectory;

            // Load the log levels
            cbLogLevel.Items.Add(LogLevel.Debug);
            cbLogLevel.Items.Add(LogLevel.Info);
            cbLogLevel.Items.Add(LogLevel.Warning);
            cbLogLevel.Items.Add(LogLevel.Error);
            cbLogLevel.SelectedItem = Settings.CurrentSettings.LogLevel;

            chkDisablePLC.Checked = plcDisabled;
            chkDisableDMX.Checked = dmxDisabled;
        }

        private void btnOK_Click(object sender, EventArgs e)
        {
            DialogResult = System.Windows.Forms.DialogResult.OK;

            // Save the audio output device
            Settings.CurrentSettings.AudioEndpointID = Player.UIDFromFriendlyName((string)cbAudioDevices.SelectedItem);

            // Save the playback latency
            Settings.CurrentSettings.PlaybackLatency = (int)numLatency.Value;
            Settings.CurrentSettings.UpdateRate = (int)numUpdateRate.Value;

            // Save the PLC comms settings
            if (!string.IsNullOrWhiteSpace(txtPLCIP.Text) && System.Net.IPAddress.TryParse(txtPLCIP.Text, out _))
                Settings.CurrentSettings.PLCIPAddress = txtPLCIP.Text;
            if (!string.IsNullOrWhiteSpace(txtPLCPort.Text) && int.TryParse(txtPLCPort.Text, out _))
                Settings.CurrentSettings.PLCPort = txtPLCPort.Text;

            Settings.CurrentSettings.ShowStartAt = chkShowStartAt.Checked;

            // Save the files and folders
            if (System.IO.File.Exists(txtLightFCWMap.Text))
                Settings.CurrentSettings.FCWMap = txtLightFCWMap.Text;
            if (System.IO.File.Exists(txtLightDMXMap.Text))
                Settings.CurrentSettings.DMXMap = txtLightDMXMap.Text;
            if (System.IO.File.Exists(txtDefaultColorMap.Text))
                Settings.CurrentSettings.DefaultColorMap = txtDefaultColorMap.Text;
            if (System.IO.Directory.Exists(txtAnnouncementDir.Text))
                Settings.CurrentSettings.AnnouncementDirectory = txtAnnouncementDir.Text;

            Settings.CurrentSettings.LogLevel = (LogLevel)cbLogLevel.SelectedItem;

            PLC_Disabled = chkDisablePLC.Checked;
            DMX_Disabled = chkDisableDMX.Checked;

            Close();
        }

        private void btnCancel_Click(object sender, EventArgs e)
        {
            DialogResult = System.Windows.Forms.DialogResult.Cancel;
            Close();
        }

        private void btnBrowseLightFCWMap_Click(object sender, EventArgs e)
        {
            if (ofdCSVs.ShowDialog() == System.Windows.Forms.DialogResult.OK)
                txtLightFCWMap.Text = ofdCSVs.FileName;
        }

        private void btnBrowseLightDMXMap_Click(object sender, EventArgs e)
        {
            if (ofdCSVs.ShowDialog() == System.Windows.Forms.DialogResult.OK)
                txtLightDMXMap.Text = ofdCSVs.FileName;
        }

        private void btnBrowseDefaultColorMap_Click(object sender, EventArgs e)
        {
            if (ofdCSVs.ShowDialog() == System.Windows.Forms.DialogResult.OK)
                txtDefaultColorMap.Text = ofdCSVs.FileName;
        }

        private void btnBrowseAnnouncementDir_Click(object sender, EventArgs e)
        {
            if (fbd.ShowDialog() == System.Windows.Forms.DialogResult.OK)
                txtAnnouncementDir.Text = fbd.SelectedPath;
        }
    }
}
