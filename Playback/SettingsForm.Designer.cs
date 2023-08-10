namespace Playback
{
    partial class SettingsForm
    {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(SettingsForm));
            this.btnOK = new System.Windows.Forms.Button();
            this.btnCancel = new System.Windows.Forms.Button();
            this.cbAudioDevices = new System.Windows.Forms.ComboBox();
            this.txtPLCIP = new System.Windows.Forms.TextBox();
            this.txtPLCPort = new System.Windows.Forms.TextBox();
            this.btnBrowseLightFCWMap = new System.Windows.Forms.Button();
            this.ofdCSVs = new System.Windows.Forms.OpenFileDialog();
            this.txtLightFCWMap = new System.Windows.Forms.TextBox();
            this.txtLightDMXMap = new System.Windows.Forms.TextBox();
            this.btnBrowseLightDMXMap = new System.Windows.Forms.Button();
            this.txtDefaultColorMap = new System.Windows.Forms.TextBox();
            this.btnBrowseDefaultColorMap = new System.Windows.Forms.Button();
            this.txtAnnouncementDir = new System.Windows.Forms.TextBox();
            this.btnBrowseAnnouncementDir = new System.Windows.Forms.Button();
            this.fbd = new System.Windows.Forms.FolderBrowserDialog();
            this.lblAudioDevices = new System.Windows.Forms.Label();
            this.lblPLCIP = new System.Windows.Forms.Label();
            this.lblPLCPort = new System.Windows.Forms.Label();
            this.lblFCWMap = new System.Windows.Forms.Label();
            this.lblDMXMap = new System.Windows.Forms.Label();
            this.lblDefaultColorMap = new System.Windows.Forms.Label();
            this.lblAnnouncementDir = new System.Windows.Forms.Label();
            this.lblLogLevel = new System.Windows.Forms.Label();
            this.cbLogLevel = new System.Windows.Forms.ComboBox();
            this.chkDisablePLC = new System.Windows.Forms.CheckBox();
            this.chkDisableDMX = new System.Windows.Forms.CheckBox();
            this.numLatency = new System.Windows.Forms.NumericUpDown();
            this.lblLatency = new System.Windows.Forms.Label();
            this.lblUpdateRate = new System.Windows.Forms.Label();
            this.numUpdateRate = new System.Windows.Forms.NumericUpDown();
            this.chkShowStartAt = new System.Windows.Forms.CheckBox();
            ((System.ComponentModel.ISupportInitialize)(this.numLatency)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.numUpdateRate)).BeginInit();
            this.SuspendLayout();
            // 
            // btnOK
            // 
            this.btnOK.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right)));
            this.btnOK.Location = new System.Drawing.Point(266, 302);
            this.btnOK.Name = "btnOK";
            this.btnOK.Size = new System.Drawing.Size(75, 23);
            this.btnOK.TabIndex = 26;
            this.btnOK.Text = "OK";
            this.btnOK.UseVisualStyleBackColor = true;
            this.btnOK.Click += new System.EventHandler(this.btnOK_Click);
            // 
            // btnCancel
            // 
            this.btnCancel.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right)));
            this.btnCancel.DialogResult = System.Windows.Forms.DialogResult.Cancel;
            this.btnCancel.Location = new System.Drawing.Point(347, 302);
            this.btnCancel.Name = "btnCancel";
            this.btnCancel.Size = new System.Drawing.Size(75, 23);
            this.btnCancel.TabIndex = 27;
            this.btnCancel.Text = "Cancel";
            this.btnCancel.UseVisualStyleBackColor = true;
            this.btnCancel.Click += new System.EventHandler(this.btnCancel_Click);
            // 
            // cbAudioDevices
            // 
            this.cbAudioDevices.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.cbAudioDevices.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
            this.cbAudioDevices.FormattingEnabled = true;
            this.cbAudioDevices.Location = new System.Drawing.Point(12, 29);
            this.cbAudioDevices.Name = "cbAudioDevices";
            this.cbAudioDevices.Size = new System.Drawing.Size(410, 21);
            this.cbAudioDevices.TabIndex = 1;
            // 
            // txtPLCIP
            // 
            this.txtPLCIP.Location = new System.Drawing.Point(12, 119);
            this.txtPLCIP.Name = "txtPLCIP";
            this.txtPLCIP.Size = new System.Drawing.Size(100, 20);
            this.txtPLCIP.TabIndex = 8;
            // 
            // txtPLCPort
            // 
            this.txtPLCPort.Location = new System.Drawing.Point(128, 119);
            this.txtPLCPort.Name = "txtPLCPort";
            this.txtPLCPort.Size = new System.Drawing.Size(65, 20);
            this.txtPLCPort.TabIndex = 9;
            // 
            // btnBrowseLightFCWMap
            // 
            this.btnBrowseLightFCWMap.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.btnBrowseLightFCWMap.Location = new System.Drawing.Point(347, 156);
            this.btnBrowseLightFCWMap.Name = "btnBrowseLightFCWMap";
            this.btnBrowseLightFCWMap.Size = new System.Drawing.Size(75, 23);
            this.btnBrowseLightFCWMap.TabIndex = 14;
            this.btnBrowseLightFCWMap.Text = "Browse...";
            this.btnBrowseLightFCWMap.UseVisualStyleBackColor = true;
            this.btnBrowseLightFCWMap.Click += new System.EventHandler(this.btnBrowseLightFCWMap_Click);
            // 
            // ofdCSVs
            // 
            this.ofdCSVs.DefaultExt = "csv";
            this.ofdCSVs.Filter = "CSV files|*.csv";
            // 
            // txtLightFCWMap
            // 
            this.txtLightFCWMap.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.txtLightFCWMap.Location = new System.Drawing.Point(12, 158);
            this.txtLightFCWMap.Name = "txtLightFCWMap";
            this.txtLightFCWMap.Size = new System.Drawing.Size(329, 20);
            this.txtLightFCWMap.TabIndex = 13;
            // 
            // txtLightDMXMap
            // 
            this.txtLightDMXMap.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.txtLightDMXMap.Location = new System.Drawing.Point(12, 197);
            this.txtLightDMXMap.Name = "txtLightDMXMap";
            this.txtLightDMXMap.Size = new System.Drawing.Size(329, 20);
            this.txtLightDMXMap.TabIndex = 16;
            // 
            // btnBrowseLightDMXMap
            // 
            this.btnBrowseLightDMXMap.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.btnBrowseLightDMXMap.Location = new System.Drawing.Point(347, 195);
            this.btnBrowseLightDMXMap.Name = "btnBrowseLightDMXMap";
            this.btnBrowseLightDMXMap.Size = new System.Drawing.Size(75, 23);
            this.btnBrowseLightDMXMap.TabIndex = 17;
            this.btnBrowseLightDMXMap.Text = "Browse...";
            this.btnBrowseLightDMXMap.UseVisualStyleBackColor = true;
            this.btnBrowseLightDMXMap.Click += new System.EventHandler(this.btnBrowseLightDMXMap_Click);
            // 
            // txtDefaultColorMap
            // 
            this.txtDefaultColorMap.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.txtDefaultColorMap.Location = new System.Drawing.Point(12, 236);
            this.txtDefaultColorMap.Name = "txtDefaultColorMap";
            this.txtDefaultColorMap.Size = new System.Drawing.Size(329, 20);
            this.txtDefaultColorMap.TabIndex = 19;
            // 
            // btnBrowseDefaultColorMap
            // 
            this.btnBrowseDefaultColorMap.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.btnBrowseDefaultColorMap.Location = new System.Drawing.Point(347, 234);
            this.btnBrowseDefaultColorMap.Name = "btnBrowseDefaultColorMap";
            this.btnBrowseDefaultColorMap.Size = new System.Drawing.Size(75, 23);
            this.btnBrowseDefaultColorMap.TabIndex = 20;
            this.btnBrowseDefaultColorMap.Text = "Browse...";
            this.btnBrowseDefaultColorMap.UseVisualStyleBackColor = true;
            this.btnBrowseDefaultColorMap.Click += new System.EventHandler(this.btnBrowseDefaultColorMap_Click);
            // 
            // txtAnnouncementDir
            // 
            this.txtAnnouncementDir.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.txtAnnouncementDir.Location = new System.Drawing.Point(12, 275);
            this.txtAnnouncementDir.Name = "txtAnnouncementDir";
            this.txtAnnouncementDir.Size = new System.Drawing.Size(329, 20);
            this.txtAnnouncementDir.TabIndex = 22;
            // 
            // btnBrowseAnnouncementDir
            // 
            this.btnBrowseAnnouncementDir.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.btnBrowseAnnouncementDir.Location = new System.Drawing.Point(347, 273);
            this.btnBrowseAnnouncementDir.Name = "btnBrowseAnnouncementDir";
            this.btnBrowseAnnouncementDir.Size = new System.Drawing.Size(75, 23);
            this.btnBrowseAnnouncementDir.TabIndex = 23;
            this.btnBrowseAnnouncementDir.Text = "Browse...";
            this.btnBrowseAnnouncementDir.UseVisualStyleBackColor = true;
            this.btnBrowseAnnouncementDir.Click += new System.EventHandler(this.btnBrowseAnnouncementDir_Click);
            // 
            // lblAudioDevices
            // 
            this.lblAudioDevices.AutoSize = true;
            this.lblAudioDevices.Location = new System.Drawing.Point(9, 9);
            this.lblAudioDevices.Name = "lblAudioDevices";
            this.lblAudioDevices.Size = new System.Drawing.Size(109, 13);
            this.lblAudioDevices.TabIndex = 0;
            this.lblAudioDevices.Text = "Audio Output Device:";
            // 
            // lblPLCIP
            // 
            this.lblPLCIP.AutoSize = true;
            this.lblPLCIP.Location = new System.Drawing.Point(12, 103);
            this.lblPLCIP.Name = "lblPLCIP";
            this.lblPLCIP.Size = new System.Drawing.Size(84, 13);
            this.lblPLCIP.TabIndex = 6;
            this.lblPLCIP.Text = "PLC IP Address:";
            // 
            // lblPLCPort
            // 
            this.lblPLCPort.AutoSize = true;
            this.lblPLCPort.Location = new System.Drawing.Point(125, 103);
            this.lblPLCPort.Name = "lblPLCPort";
            this.lblPLCPort.Size = new System.Drawing.Size(29, 13);
            this.lblPLCPort.TabIndex = 7;
            this.lblPLCPort.Text = "Port:";
            // 
            // lblFCWMap
            // 
            this.lblFCWMap.AutoSize = true;
            this.lblFCWMap.Location = new System.Drawing.Point(12, 142);
            this.lblFCWMap.Name = "lblFCWMap";
            this.lblFCWMap.Size = new System.Drawing.Size(55, 13);
            this.lblFCWMap.TabIndex = 12;
            this.lblFCWMap.Text = "FCW Map";
            // 
            // lblDMXMap
            // 
            this.lblDMXMap.AutoSize = true;
            this.lblDMXMap.Location = new System.Drawing.Point(12, 181);
            this.lblDMXMap.Name = "lblDMXMap";
            this.lblDMXMap.Size = new System.Drawing.Size(55, 13);
            this.lblDMXMap.TabIndex = 15;
            this.lblDMXMap.Text = "DMX Map";
            // 
            // lblDefaultColorMap
            // 
            this.lblDefaultColorMap.AutoSize = true;
            this.lblDefaultColorMap.Location = new System.Drawing.Point(12, 220);
            this.lblDefaultColorMap.Name = "lblDefaultColorMap";
            this.lblDefaultColorMap.Size = new System.Drawing.Size(92, 13);
            this.lblDefaultColorMap.TabIndex = 18;
            this.lblDefaultColorMap.Text = "Default Color Map";
            // 
            // lblAnnouncementDir
            // 
            this.lblAnnouncementDir.AutoSize = true;
            this.lblAnnouncementDir.Location = new System.Drawing.Point(12, 259);
            this.lblAnnouncementDir.Name = "lblAnnouncementDir";
            this.lblAnnouncementDir.Size = new System.Drawing.Size(132, 13);
            this.lblAnnouncementDir.TabIndex = 21;
            this.lblAnnouncementDir.Text = "Announcements Directory:";
            // 
            // lblLogLevel
            // 
            this.lblLogLevel.AutoSize = true;
            this.lblLogLevel.Location = new System.Drawing.Point(12, 307);
            this.lblLogLevel.Name = "lblLogLevel";
            this.lblLogLevel.Size = new System.Drawing.Size(57, 13);
            this.lblLogLevel.TabIndex = 24;
            this.lblLogLevel.Text = "Log Level:";
            // 
            // cbLogLevel
            // 
            this.cbLogLevel.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
            this.cbLogLevel.FormattingEnabled = true;
            this.cbLogLevel.Location = new System.Drawing.Point(75, 304);
            this.cbLogLevel.Name = "cbLogLevel";
            this.cbLogLevel.Size = new System.Drawing.Size(79, 21);
            this.cbLogLevel.TabIndex = 25;
            // 
            // chkDisablePLC
            // 
            this.chkDisablePLC.AutoSize = true;
            this.chkDisablePLC.Location = new System.Drawing.Point(199, 106);
            this.chkDisablePLC.Name = "chkDisablePLC";
            this.chkDisablePLC.Size = new System.Drawing.Size(139, 17);
            this.chkDisablePLC.TabIndex = 10;
            this.chkDisablePLC.Text = "Disable PLC (for testing)";
            this.chkDisablePLC.UseVisualStyleBackColor = true;
            // 
            // chkDisableDMX
            // 
            this.chkDisableDMX.AutoSize = true;
            this.chkDisableDMX.Location = new System.Drawing.Point(199, 129);
            this.chkDisableDMX.Name = "chkDisableDMX";
            this.chkDisableDMX.Size = new System.Drawing.Size(143, 17);
            this.chkDisableDMX.TabIndex = 11;
            this.chkDisableDMX.Text = "Disable DMX (for testing)";
            this.chkDisableDMX.UseVisualStyleBackColor = true;
            // 
            // numLatency
            // 
            this.numLatency.Location = new System.Drawing.Point(12, 71);
            this.numLatency.Maximum = new decimal(new int[] {
            630,
            0,
            0,
            0});
            this.numLatency.Name = "numLatency";
            this.numLatency.Size = new System.Drawing.Size(120, 20);
            this.numLatency.TabIndex = 3;
            // 
            // lblLatency
            // 
            this.lblLatency.AutoSize = true;
            this.lblLatency.Location = new System.Drawing.Point(12, 53);
            this.lblLatency.Name = "lblLatency";
            this.lblLatency.Size = new System.Drawing.Size(114, 13);
            this.lblLatency.TabIndex = 2;
            this.lblLatency.Text = "Playback Latency (ms)";
            // 
            // lblUpdateRate
            // 
            this.lblUpdateRate.AutoSize = true;
            this.lblUpdateRate.Location = new System.Drawing.Point(279, 53);
            this.lblUpdateRate.Name = "lblUpdateRate";
            this.lblUpdateRate.Size = new System.Drawing.Size(143, 13);
            this.lblUpdateRate.TabIndex = 4;
            this.lblUpdateRate.Text = "VU Update Rate (updates/s)";
            // 
            // numUpdateRate
            // 
            this.numUpdateRate.Location = new System.Drawing.Point(282, 71);
            this.numUpdateRate.Minimum = new decimal(new int[] {
            1,
            0,
            0,
            0});
            this.numUpdateRate.Name = "numUpdateRate";
            this.numUpdateRate.Size = new System.Drawing.Size(140, 20);
            this.numUpdateRate.TabIndex = 5;
            this.numUpdateRate.Value = new decimal(new int[] {
            1,
            0,
            0,
            0});
            // 
            // chkShowStartAt
            // 
            this.chkShowStartAt.AutoSize = true;
            this.chkShowStartAt.Location = new System.Drawing.Point(149, 72);
            this.chkShowStartAt.Name = "chkShowStartAt";
            this.chkShowStartAt.Size = new System.Drawing.Size(111, 17);
            this.chkShowStartAt.TabIndex = 28;
            this.chkShowStartAt.Text = "Show Start At box";
            this.chkShowStartAt.UseVisualStyleBackColor = true;
            // 
            // SettingsForm
            // 
            this.AcceptButton = this.btnOK;
            this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 13F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.CancelButton = this.btnCancel;
            this.ClientSize = new System.Drawing.Size(434, 337);
            this.Controls.Add(this.chkShowStartAt);
            this.Controls.Add(this.lblUpdateRate);
            this.Controls.Add(this.numUpdateRate);
            this.Controls.Add(this.lblLatency);
            this.Controls.Add(this.numLatency);
            this.Controls.Add(this.chkDisableDMX);
            this.Controls.Add(this.chkDisablePLC);
            this.Controls.Add(this.lblLogLevel);
            this.Controls.Add(this.cbLogLevel);
            this.Controls.Add(this.lblAnnouncementDir);
            this.Controls.Add(this.lblDefaultColorMap);
            this.Controls.Add(this.lblDMXMap);
            this.Controls.Add(this.lblFCWMap);
            this.Controls.Add(this.lblPLCPort);
            this.Controls.Add(this.lblPLCIP);
            this.Controls.Add(this.lblAudioDevices);
            this.Controls.Add(this.txtAnnouncementDir);
            this.Controls.Add(this.btnBrowseAnnouncementDir);
            this.Controls.Add(this.txtDefaultColorMap);
            this.Controls.Add(this.btnBrowseDefaultColorMap);
            this.Controls.Add(this.txtLightDMXMap);
            this.Controls.Add(this.btnBrowseLightDMXMap);
            this.Controls.Add(this.txtLightFCWMap);
            this.Controls.Add(this.btnBrowseLightFCWMap);
            this.Controls.Add(this.txtPLCPort);
            this.Controls.Add(this.txtPLCIP);
            this.Controls.Add(this.cbAudioDevices);
            this.Controls.Add(this.btnCancel);
            this.Controls.Add(this.btnOK);
            this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
            this.MinimumSize = new System.Drawing.Size(450, 375);
            this.Name = "SettingsForm";
            this.Text = "SettingsForm";
            ((System.ComponentModel.ISupportInitialize)(this.numLatency)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.numUpdateRate)).EndInit();
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.Button btnOK;
        private System.Windows.Forms.Button btnCancel;
        private System.Windows.Forms.ComboBox cbAudioDevices;
        private System.Windows.Forms.TextBox txtPLCIP;
        private System.Windows.Forms.TextBox txtPLCPort;
        private System.Windows.Forms.Button btnBrowseLightFCWMap;
        private System.Windows.Forms.OpenFileDialog ofdCSVs;
        private System.Windows.Forms.TextBox txtLightFCWMap;
        private System.Windows.Forms.TextBox txtLightDMXMap;
        private System.Windows.Forms.Button btnBrowseLightDMXMap;
        private System.Windows.Forms.TextBox txtDefaultColorMap;
        private System.Windows.Forms.Button btnBrowseDefaultColorMap;
        private System.Windows.Forms.TextBox txtAnnouncementDir;
        private System.Windows.Forms.Button btnBrowseAnnouncementDir;
        private System.Windows.Forms.FolderBrowserDialog fbd;
        private System.Windows.Forms.Label lblAudioDevices;
        private System.Windows.Forms.Label lblPLCIP;
        private System.Windows.Forms.Label lblPLCPort;
        private System.Windows.Forms.Label lblFCWMap;
        private System.Windows.Forms.Label lblDMXMap;
        private System.Windows.Forms.Label lblDefaultColorMap;
        private System.Windows.Forms.Label lblAnnouncementDir;
        private System.Windows.Forms.Label lblLogLevel;
        private System.Windows.Forms.ComboBox cbLogLevel;
        private System.Windows.Forms.CheckBox chkDisablePLC;
        private System.Windows.Forms.CheckBox chkDisableDMX;
        private System.Windows.Forms.NumericUpDown numLatency;
        private System.Windows.Forms.Label lblLatency;
        private System.Windows.Forms.Label lblUpdateRate;
        private System.Windows.Forms.NumericUpDown numUpdateRate;
        private System.Windows.Forms.CheckBox chkShowStartAt;
    }
}