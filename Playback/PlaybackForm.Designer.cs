namespace Playback
{
    partial class PlaybackForm
    {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            this.components = new System.ComponentModel.Container();
            System.Windows.Forms.ColumnHeader colName;
            System.Windows.Forms.ColumnHeader colCurPlaylistSongName;
            System.Windows.Forms.ColumnHeader colPlaylistName;
            System.Windows.Forms.ColumnHeader colSongName;
            System.Windows.Forms.ColumnHeader colCurPlaylistSongArtist;
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(PlaybackForm));
            this.btnStartStop = new System.Windows.Forms.Button();
            this.btnPauseResume = new System.Windows.Forms.Button();
            this.barVolLeftChannel = new System.Windows.Forms.TrackBar();
            this.btnSettings = new System.Windows.Forms.Button();
            this.btnLogin = new System.Windows.Forms.Button();
            this.lvPlaylists = new System.Windows.Forms.ListView();
            this.lvSongs = new System.Windows.Forms.ListView();
            this.btnConnectDMX = new System.Windows.Forms.Button();
            this.txtManualFCW = new System.Windows.Forms.TextBox();
            this.btnExecuteManualFCW = new System.Windows.Forms.Button();
            this.tlpLists = new System.Windows.Forms.TableLayoutPanel();
            this.lblLightFCWCount = new System.Windows.Forms.Label();
            this.lblTotalFCWCount = new System.Windows.Forms.Label();
            this.lblWaterFCWCount = new System.Windows.Forms.Label();
            this.lblLightFCWs = new System.Windows.Forms.Label();
            this.lblWaterFCWs = new System.Windows.Forms.Label();
            this.lblCurrentShow = new System.Windows.Forms.Label();
            this.lblCurrentSongFolder = new System.Windows.Forms.Label();
            this.lblCurrentSong = new System.Windows.Forms.Label();
            this.lblCurrentPlaylistFolder = new System.Windows.Forms.Label();
            this.lblSongProgress = new System.Windows.Forms.Label();
            this.lblVolume = new System.Windows.Forms.Label();
            this.btnMoveDown = new System.Windows.Forms.Button();
            this.btnAnnounce = new System.Windows.Forms.Button();
            this.btnBrowseSong = new System.Windows.Forms.Button();
            this.lblSongSelection = new System.Windows.Forms.Label();
            this.btnRemoveFromPlaylist = new System.Windows.Forms.Button();
            this.btnMoveUp = new System.Windows.Forms.Button();
            this.btnBrowsePlaylist = new System.Windows.Forms.Button();
            this.btnRenamePlaylist = new System.Windows.Forms.Button();
            this.lvCurPlaylistSongs = new System.Windows.Forms.ListView();
            this.btnDeletePlaylist = new System.Windows.Forms.Button();
            this.btnNewPlaylist = new System.Windows.Forms.Button();
            this.lblPlaylistSelection = new System.Windows.Forms.Label();
            this.btnAddToPlaylist = new System.Windows.Forms.Button();
            this.txtStartPoint = new System.Windows.Forms.TextBox();
            this.lblStartPoint = new System.Windows.Forms.Label();
            this.volMeter = new NAudio.Gui.VolumeMeter();
            this.pnlFountain = new System.Windows.Forms.Panel();
            this.light45 = new Playback.LightControl();
            this.light44 = new Playback.LightControl();
            this.light41 = new Playback.LightControl();
            this.light40 = new Playback.LightControl();
            this.light39 = new Playback.LightControl();
            this.light38 = new Playback.LightControl();
            this.light37 = new Playback.LightControl();
            this.light36 = new Playback.LightControl();
            this.light43 = new Playback.LightControl();
            this.light42 = new Playback.LightControl();
            this.light33 = new Playback.LightControl();
            this.light28 = new Playback.LightControl();
            this.light23 = new Playback.LightControl();
            this.light18 = new Playback.LightControl();
            this.light13 = new Playback.LightControl();
            this.light8 = new Playback.LightControl();
            this.light3 = new Playback.LightControl();
            this.light20 = new Playback.LightControl();
            this.light17 = new Playback.LightControl();
            this.light16 = new Playback.LightControl();
            this.light19 = new Playback.LightControl();
            this.light32 = new Playback.LightControl();
            this.light27 = new Playback.LightControl();
            this.light31 = new Playback.LightControl();
            this.light22 = new Playback.LightControl();
            this.light26 = new Playback.LightControl();
            this.light12 = new Playback.LightControl();
            this.light21 = new Playback.LightControl();
            this.light7 = new Playback.LightControl();
            this.light11 = new Playback.LightControl();
            this.light2 = new Playback.LightControl();
            this.light6 = new Playback.LightControl();
            this.light1 = new Playback.LightControl();
            this.light35 = new Playback.LightControl();
            this.light30 = new Playback.LightControl();
            this.light34 = new Playback.LightControl();
            this.light25 = new Playback.LightControl();
            this.light29 = new Playback.LightControl();
            this.light15 = new Playback.LightControl();
            this.light24 = new Playback.LightControl();
            this.light10 = new Playback.LightControl();
            this.light14 = new Playback.LightControl();
            this.light9 = new Playback.LightControl();
            this.light5 = new Playback.LightControl();
            this.light4 = new Playback.LightControl();
            this.pbFountain = new System.Windows.Forms.PictureBox();
            this.lblShowProgress = new System.Windows.Forms.Label();
            this.lvAnnouncements = new System.Windows.Forms.ListView();
            this.ttVolume = new System.Windows.Forms.ToolTip(this.components);
            this.playlistFolderBrowser = new System.Windows.Forms.FolderBrowserDialog();
            this.songFolderBrowser = new System.Windows.Forms.FolderBrowserDialog();
            colName = ((System.Windows.Forms.ColumnHeader)(new System.Windows.Forms.ColumnHeader()));
            colCurPlaylistSongName = ((System.Windows.Forms.ColumnHeader)(new System.Windows.Forms.ColumnHeader()));
            colPlaylistName = ((System.Windows.Forms.ColumnHeader)(new System.Windows.Forms.ColumnHeader()));
            colSongName = ((System.Windows.Forms.ColumnHeader)(new System.Windows.Forms.ColumnHeader()));
            colCurPlaylistSongArtist = ((System.Windows.Forms.ColumnHeader)(new System.Windows.Forms.ColumnHeader()));
            ((System.ComponentModel.ISupportInitialize)(this.barVolLeftChannel)).BeginInit();
            this.tlpLists.SuspendLayout();
            this.pnlFountain.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.pbFountain)).BeginInit();
            this.SuspendLayout();
            // 
            // colName
            // 
            colName.Text = "Name";
            colName.Width = 190;
            // 
            // colCurPlaylistSongName
            // 
            colCurPlaylistSongName.Text = "Song Name";
            // 
            // colPlaylistName
            // 
            colPlaylistName.Text = "Playlist";
            colPlaylistName.Width = 0;
            // 
            // colSongName
            // 
            colSongName.Width = 0;
            // 
            // colCurPlaylistSongArtist
            // 
            colCurPlaylistSongArtist.Text = "Artist Name";
            // 
            // btnStartStop
            // 
            this.btnStartStop.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnStartStop.Location = new System.Drawing.Point(591, 346);
            this.btnStartStop.Name = "btnStartStop";
            this.btnStartStop.Size = new System.Drawing.Size(82, 24);
            this.btnStartStop.TabIndex = 0;
            this.btnStartStop.Text = "Start Show";
            this.btnStartStop.UseVisualStyleBackColor = true;
            this.btnStartStop.Click += new System.EventHandler(this.btnStartStop_Click);
            // 
            // btnPauseResume
            // 
            this.btnPauseResume.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnPauseResume.Location = new System.Drawing.Point(855, 346);
            this.btnPauseResume.Name = "btnPauseResume";
            this.btnPauseResume.Size = new System.Drawing.Size(82, 24);
            this.btnPauseResume.TabIndex = 1;
            this.btnPauseResume.Text = "Pause Show";
            this.btnPauseResume.UseVisualStyleBackColor = true;
            this.btnPauseResume.Click += new System.EventHandler(this.btnPauseResume_Click);
            // 
            // barVolLeftChannel
            // 
            this.barVolLeftChannel.Dock = System.Windows.Forms.DockStyle.Fill;
            this.barVolLeftChannel.LargeChange = 0;
            this.barVolLeftChannel.Location = new System.Drawing.Point(959, 33);
            this.barVolLeftChannel.Maximum = 100;
            this.barVolLeftChannel.Name = "barVolLeftChannel";
            this.barVolLeftChannel.Orientation = System.Windows.Forms.Orientation.Vertical;
            this.tlpLists.SetRowSpan(this.barVolLeftChannel, 8);
            this.barVolLeftChannel.Size = new System.Drawing.Size(46, 545);
            this.barVolLeftChannel.TabIndex = 7;
            this.barVolLeftChannel.TickFrequency = 5;
            this.barVolLeftChannel.ValueChanged += new System.EventHandler(this.barVolume_ValueChanged);
            // 
            // btnSettings
            // 
            this.btnSettings.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnSettings.Location = new System.Drawing.Point(179, 614);
            this.btnSettings.Name = "btnSettings";
            this.btnSettings.Size = new System.Drawing.Size(82, 24);
            this.btnSettings.TabIndex = 11;
            this.btnSettings.Text = "Settings";
            this.btnSettings.UseVisualStyleBackColor = true;
            this.btnSettings.Click += new System.EventHandler(this.btnSettings_Click);
            // 
            // btnLogin
            // 
            this.btnLogin.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnLogin.Location = new System.Drawing.Point(3, 614);
            this.btnLogin.Name = "btnLogin";
            this.btnLogin.Size = new System.Drawing.Size(82, 24);
            this.btnLogin.TabIndex = 9;
            this.btnLogin.Text = "Log in";
            this.btnLogin.UseVisualStyleBackColor = true;
            this.btnLogin.Click += new System.EventHandler(this.btnLogin_Click);
            // 
            // lvPlaylists
            // 
            this.lvPlaylists.Columns.AddRange(new System.Windows.Forms.ColumnHeader[] {
            colPlaylistName});
            this.tlpLists.SetColumnSpan(this.lvPlaylists, 3);
            this.lvPlaylists.Dock = System.Windows.Forms.DockStyle.Fill;
            this.lvPlaylists.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lvPlaylists.HeaderStyle = System.Windows.Forms.ColumnHeaderStyle.None;
            this.lvPlaylists.HideSelection = false;
            this.lvPlaylists.Location = new System.Drawing.Point(3, 58);
            this.lvPlaylists.MultiSelect = false;
            this.lvPlaylists.Name = "lvPlaylists";
            this.tlpLists.SetRowSpan(this.lvPlaylists, 7);
            this.lvPlaylists.Size = new System.Drawing.Size(258, 520);
            this.lvPlaylists.TabIndex = 2;
            this.lvPlaylists.UseCompatibleStateImageBehavior = false;
            this.lvPlaylists.View = System.Windows.Forms.View.Details;
            this.lvPlaylists.AfterLabelEdit += new System.Windows.Forms.LabelEditEventHandler(this.lvPlaylists_AfterLabelEdit);
            this.lvPlaylists.SelectedIndexChanged += new System.EventHandler(this.lvPlaylists_SelectedIndexChanged);
            this.lvPlaylists.Click += new System.EventHandler(this.lvPlaylists_Click);
            this.lvPlaylists.KeyUp += new System.Windows.Forms.KeyEventHandler(this.lvPlaylists_KeyUp);
            // 
            // lvSongs
            // 
            this.lvSongs.Columns.AddRange(new System.Windows.Forms.ColumnHeader[] {
            colSongName});
            this.tlpLists.SetColumnSpan(this.lvSongs, 3);
            this.lvSongs.Dock = System.Windows.Forms.DockStyle.Fill;
            this.lvSongs.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lvSongs.HeaderStyle = System.Windows.Forms.ColumnHeaderStyle.None;
            this.lvSongs.HideSelection = false;
            this.lvSongs.Location = new System.Drawing.Point(267, 58);
            this.lvSongs.MultiSelect = false;
            this.lvSongs.Name = "lvSongs";
            this.tlpLists.SetRowSpan(this.lvSongs, 7);
            this.lvSongs.Size = new System.Drawing.Size(258, 520);
            this.lvSongs.TabIndex = 3;
            this.lvSongs.UseCompatibleStateImageBehavior = false;
            this.lvSongs.View = System.Windows.Forms.View.Details;
            this.lvSongs.DoubleClick += new System.EventHandler(this.lvSongs_DoubleClick);
            // 
            // btnConnectDMX
            // 
            this.btnConnectDMX.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnConnectDMX.Location = new System.Drawing.Point(855, 614);
            this.btnConnectDMX.Name = "btnConnectDMX";
            this.btnConnectDMX.Size = new System.Drawing.Size(82, 24);
            this.btnConnectDMX.TabIndex = 14;
            this.btnConnectDMX.Text = "Connect DMX";
            this.btnConnectDMX.UseVisualStyleBackColor = true;
            this.btnConnectDMX.Click += new System.EventHandler(this.btnConnectDMX_Click);
            // 
            // txtManualFCW
            // 
            this.tlpLists.SetColumnSpan(this.txtManualFCW, 2);
            this.txtManualFCW.Dock = System.Windows.Forms.DockStyle.Fill;
            this.txtManualFCW.Location = new System.Drawing.Point(267, 614);
            this.txtManualFCW.Name = "txtManualFCW";
            this.txtManualFCW.Size = new System.Drawing.Size(170, 20);
            this.txtManualFCW.TabIndex = 12;
            this.txtManualFCW.KeyPress += new System.Windows.Forms.KeyPressEventHandler(this.txtManualFCW_KeyPress);
            // 
            // btnExecuteManualFCW
            // 
            this.btnExecuteManualFCW.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnExecuteManualFCW.Location = new System.Drawing.Point(443, 614);
            this.btnExecuteManualFCW.Name = "btnExecuteManualFCW";
            this.btnExecuteManualFCW.Size = new System.Drawing.Size(82, 24);
            this.btnExecuteManualFCW.TabIndex = 13;
            this.btnExecuteManualFCW.Text = "Execute";
            this.btnExecuteManualFCW.UseVisualStyleBackColor = true;
            this.btnExecuteManualFCW.Click += new System.EventHandler(this.btnExecuteManualFCW_Click);
            // 
            // tlpLists
            // 
            this.tlpLists.ColumnCount = 15;
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Absolute, 16F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Absolute, 52F));
            this.tlpLists.Controls.Add(this.lblLightFCWCount, 1, 12);
            this.tlpLists.Controls.Add(this.lblTotalFCWCount, 2, 12);
            this.tlpLists.Controls.Add(this.lblWaterFCWCount, 0, 12);
            this.tlpLists.Controls.Add(this.lblLightFCWs, 4, 12);
            this.tlpLists.Controls.Add(this.lblWaterFCWs, 4, 13);
            this.tlpLists.Controls.Add(this.lblCurrentShow, 9, 3);
            this.tlpLists.Controls.Add(this.lblCurrentSongFolder, 3, 1);
            this.tlpLists.Controls.Add(this.lblCurrentSong, 9, 4);
            this.tlpLists.Controls.Add(this.lblCurrentPlaylistFolder, 0, 1);
            this.tlpLists.Controls.Add(this.btnExecuteManualFCW, 5, 10);
            this.tlpLists.Controls.Add(this.lblSongProgress, 9, 5);
            this.tlpLists.Controls.Add(this.btnPauseResume, 12, 7);
            this.tlpLists.Controls.Add(this.txtManualFCW, 3, 10);
            this.tlpLists.Controls.Add(this.lblVolume, 13, 0);
            this.tlpLists.Controls.Add(this.btnMoveDown, 12, 9);
            this.tlpLists.Controls.Add(this.btnSettings, 2, 10);
            this.tlpLists.Controls.Add(this.btnAnnounce, 9, 0);
            this.tlpLists.Controls.Add(this.btnBrowseSong, 5, 0);
            this.tlpLists.Controls.Add(this.btnLogin, 0, 10);
            this.tlpLists.Controls.Add(this.btnStartStop, 9, 7);
            this.tlpLists.Controls.Add(this.lblSongSelection, 3, 0);
            this.tlpLists.Controls.Add(this.barVolLeftChannel, 14, 1);
            this.tlpLists.Controls.Add(this.btnRemoveFromPlaylist, 9, 9);
            this.tlpLists.Controls.Add(this.btnMoveUp, 11, 9);
            this.tlpLists.Controls.Add(this.btnBrowsePlaylist, 2, 0);
            this.tlpLists.Controls.Add(this.btnRenamePlaylist, 2, 9);
            this.tlpLists.Controls.Add(this.lvCurPlaylistSongs, 9, 8);
            this.tlpLists.Controls.Add(this.lvPlaylists, 0, 2);
            this.tlpLists.Controls.Add(this.lvSongs, 3, 2);
            this.tlpLists.Controls.Add(this.btnDeletePlaylist, 1, 9);
            this.tlpLists.Controls.Add(this.btnNewPlaylist, 0, 9);
            this.tlpLists.Controls.Add(this.lblPlaylistSelection, 0, 0);
            this.tlpLists.Controls.Add(this.btnAddToPlaylist, 5, 9);
            this.tlpLists.Controls.Add(this.btnConnectDMX, 12, 10);
            this.tlpLists.Controls.Add(this.txtStartPoint, 11, 7);
            this.tlpLists.Controls.Add(this.lblStartPoint, 10, 7);
            this.tlpLists.Controls.Add(this.volMeter, 13, 1);
            this.tlpLists.Controls.Add(this.pnlFountain, 0, 11);
            this.tlpLists.Controls.Add(this.lblShowProgress, 9, 6);
            this.tlpLists.Controls.Add(this.lvAnnouncements, 9, 2);
            this.tlpLists.Dock = System.Windows.Forms.DockStyle.Fill;
            this.tlpLists.Location = new System.Drawing.Point(0, 0);
            this.tlpLists.Margin = new System.Windows.Forms.Padding(0);
            this.tlpLists.Name = "tlpLists";
            this.tlpLists.RowCount = 14;
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 30F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 25F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 50F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 30F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 50F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 30F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 30F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 279F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.tlpLists.Size = new System.Drawing.Size(1008, 961);
            this.tlpLists.TabIndex = 0;
            // 
            // lblLightFCWCount
            // 
            this.lblLightFCWCount.AutoSize = true;
            this.lblLightFCWCount.Location = new System.Drawing.Point(92, 920);
            this.lblLightFCWCount.Margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
            this.lblLightFCWCount.Name = "lblLightFCWCount";
            this.tlpLists.SetRowSpan(this.lblLightFCWCount, 2);
            this.lblLightFCWCount.Size = new System.Drawing.Size(61, 39);
            this.lblLightFCWCount.TabIndex = 37;
            this.lblLightFCWCount.Text = "0/0\r\nlight FCWs executed";
            // 
            // lblTotalFCWCount
            // 
            this.lblTotalFCWCount.AutoSize = true;
            this.lblTotalFCWCount.Location = new System.Drawing.Point(180, 920);
            this.lblTotalFCWCount.Margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
            this.lblTotalFCWCount.Name = "lblTotalFCWCount";
            this.tlpLists.SetRowSpan(this.lblTotalFCWCount, 2);
            this.lblTotalFCWCount.Size = new System.Drawing.Size(62, 39);
            this.lblTotalFCWCount.TabIndex = 39;
            this.lblTotalFCWCount.Text = "0/0\r\ntotal FCWs executed";
            // 
            // lblWaterFCWCount
            // 
            this.lblWaterFCWCount.AutoSize = true;
            this.lblWaterFCWCount.Location = new System.Drawing.Point(4, 920);
            this.lblWaterFCWCount.Margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
            this.lblWaterFCWCount.Name = "lblWaterFCWCount";
            this.tlpLists.SetRowSpan(this.lblWaterFCWCount, 2);
            this.lblWaterFCWCount.Size = new System.Drawing.Size(68, 39);
            this.lblWaterFCWCount.TabIndex = 35;
            this.lblWaterFCWCount.Text = "0/0\r\nwater FCWs sent";
            // 
            // lblLightFCWs
            // 
            this.lblLightFCWs.AutoSize = true;
            this.tlpLists.SetColumnSpan(this.lblLightFCWs, 11);
            this.lblLightFCWs.Location = new System.Drawing.Point(356, 920);
            this.lblLightFCWs.Margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
            this.lblLightFCWs.Name = "lblLightFCWs";
            this.lblLightFCWs.Size = new System.Drawing.Size(108, 13);
            this.lblLightFCWs.TabIndex = 36;
            this.lblLightFCWs.Text = "Current Light FCW(s):";
            // 
            // lblWaterFCWs
            // 
            this.lblWaterFCWs.AutoSize = true;
            this.tlpLists.SetColumnSpan(this.lblWaterFCWs, 11);
            this.lblWaterFCWs.Location = new System.Drawing.Point(356, 940);
            this.lblWaterFCWs.Margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
            this.lblWaterFCWs.Name = "lblWaterFCWs";
            this.lblWaterFCWs.Size = new System.Drawing.Size(114, 13);
            this.lblWaterFCWs.TabIndex = 34;
            this.lblWaterFCWs.Text = "Current Water FCW(s):";
            // 
            // lblCurrentShow
            // 
            this.tlpLists.SetColumnSpan(this.lblCurrentShow, 4);
            this.lblCurrentShow.Dock = System.Windows.Forms.DockStyle.Fill;
            this.lblCurrentShow.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblCurrentShow.Location = new System.Drawing.Point(591, 263);
            this.lblCurrentShow.Name = "lblCurrentShow";
            this.lblCurrentShow.Size = new System.Drawing.Size(346, 20);
            this.lblCurrentShow.TabIndex = 4;
            this.lblCurrentShow.Text = "Now Playing Show: ";
            this.lblCurrentShow.TextAlign = System.Drawing.ContentAlignment.TopCenter;
            // 
            // lblCurrentSongFolder
            // 
            this.lblCurrentSongFolder.AutoEllipsis = true;
            this.tlpLists.SetColumnSpan(this.lblCurrentSongFolder, 3);
            this.lblCurrentSongFolder.Dock = System.Windows.Forms.DockStyle.Bottom;
            this.lblCurrentSongFolder.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblCurrentSongFolder.Location = new System.Drawing.Point(267, 35);
            this.lblCurrentSongFolder.Name = "lblCurrentSongFolder";
            this.lblCurrentSongFolder.Size = new System.Drawing.Size(258, 20);
            this.lblCurrentSongFolder.TabIndex = 28;
            // 
            // lblCurrentSong
            // 
            this.tlpLists.SetColumnSpan(this.lblCurrentSong, 4);
            this.lblCurrentSong.Dock = System.Windows.Forms.DockStyle.Fill;
            this.lblCurrentSong.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblCurrentSong.Location = new System.Drawing.Point(591, 283);
            this.lblCurrentSong.Name = "lblCurrentSong";
            this.lblCurrentSong.Size = new System.Drawing.Size(346, 20);
            this.lblCurrentSong.TabIndex = 0;
            this.lblCurrentSong.Text = "Now Playing Song: ";
            this.lblCurrentSong.TextAlign = System.Drawing.ContentAlignment.TopCenter;
            // 
            // lblCurrentPlaylistFolder
            // 
            this.lblCurrentPlaylistFolder.AutoEllipsis = true;
            this.tlpLists.SetColumnSpan(this.lblCurrentPlaylistFolder, 3);
            this.lblCurrentPlaylistFolder.Dock = System.Windows.Forms.DockStyle.Bottom;
            this.lblCurrentPlaylistFolder.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblCurrentPlaylistFolder.Location = new System.Drawing.Point(3, 35);
            this.lblCurrentPlaylistFolder.Name = "lblCurrentPlaylistFolder";
            this.lblCurrentPlaylistFolder.Size = new System.Drawing.Size(258, 20);
            this.lblCurrentPlaylistFolder.TabIndex = 27;
            // 
            // lblSongProgress
            // 
            this.tlpLists.SetColumnSpan(this.lblSongProgress, 4);
            this.lblSongProgress.Dock = System.Windows.Forms.DockStyle.Fill;
            this.lblSongProgress.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblSongProgress.Location = new System.Drawing.Point(591, 303);
            this.lblSongProgress.Name = "lblSongProgress";
            this.lblSongProgress.Size = new System.Drawing.Size(346, 20);
            this.lblSongProgress.TabIndex = 1;
            this.lblSongProgress.Text = "Song Now : Song Total";
            this.lblSongProgress.TextAlign = System.Drawing.ContentAlignment.TopCenter;
            // 
            // lblVolume
            // 
            this.lblVolume.AutoSize = true;
            this.tlpLists.SetColumnSpan(this.lblVolume, 2);
            this.lblVolume.Dock = System.Windows.Forms.DockStyle.Bottom;
            this.lblVolume.Font = new System.Drawing.Font("Microsoft Sans Serif", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblVolume.Location = new System.Drawing.Point(943, 0);
            this.lblVolume.Name = "lblVolume";
            this.lblVolume.Size = new System.Drawing.Size(62, 30);
            this.lblVolume.TabIndex = 26;
            this.lblVolume.Text = "Audience Volume";
            this.lblVolume.TextAlign = System.Drawing.ContentAlignment.BottomCenter;
            // 
            // btnMoveDown
            // 
            this.btnMoveDown.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnMoveDown.Location = new System.Drawing.Point(855, 584);
            this.btnMoveDown.Name = "btnMoveDown";
            this.btnMoveDown.Size = new System.Drawing.Size(82, 24);
            this.btnMoveDown.TabIndex = 21;
            this.btnMoveDown.Text = "Move Down";
            this.btnMoveDown.UseVisualStyleBackColor = true;
            this.btnMoveDown.Click += new System.EventHandler(this.btnMoveDown_Click);
            // 
            // btnAnnounce
            // 
            this.tlpLists.SetColumnSpan(this.btnAnnounce, 4);
            this.btnAnnounce.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnAnnounce.Enabled = false;
            this.btnAnnounce.Location = new System.Drawing.Point(591, 3);
            this.btnAnnounce.Name = "btnAnnounce";
            this.btnAnnounce.Size = new System.Drawing.Size(346, 24);
            this.btnAnnounce.TabIndex = 6;
            this.btnAnnounce.Text = "PLAY ANNOUNCEMENT";
            this.btnAnnounce.UseVisualStyleBackColor = true;
            this.btnAnnounce.EnabledChanged += new System.EventHandler(this.btnAnnounce_EnabledChanged);
            this.btnAnnounce.Click += new System.EventHandler(this.btnAnnounce_Click);
            // 
            // btnBrowseSong
            // 
            this.btnBrowseSong.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnBrowseSong.Location = new System.Drawing.Point(443, 3);
            this.btnBrowseSong.Name = "btnBrowseSong";
            this.btnBrowseSong.Size = new System.Drawing.Size(82, 24);
            this.btnBrowseSong.TabIndex = 23;
            this.btnBrowseSong.Text = "Browse...";
            this.btnBrowseSong.UseVisualStyleBackColor = true;
            this.btnBrowseSong.Click += new System.EventHandler(this.btnBrowseSong_Click);
            // 
            // lblSongSelection
            // 
            this.lblSongSelection.AutoSize = true;
            this.tlpLists.SetColumnSpan(this.lblSongSelection, 2);
            this.lblSongSelection.Dock = System.Windows.Forms.DockStyle.Bottom;
            this.lblSongSelection.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblSongSelection.Location = new System.Drawing.Point(267, 10);
            this.lblSongSelection.Name = "lblSongSelection";
            this.lblSongSelection.Size = new System.Drawing.Size(170, 20);
            this.lblSongSelection.TabIndex = 25;
            this.lblSongSelection.Text = "Song Selection";
            // 
            // btnRemoveFromPlaylist
            // 
            this.btnRemoveFromPlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnRemoveFromPlaylist.Location = new System.Drawing.Point(591, 584);
            this.btnRemoveFromPlaylist.Name = "btnRemoveFromPlaylist";
            this.btnRemoveFromPlaylist.Size = new System.Drawing.Size(82, 24);
            this.btnRemoveFromPlaylist.TabIndex = 19;
            this.btnRemoveFromPlaylist.Text = "<-- Remove";
            this.btnRemoveFromPlaylist.UseVisualStyleBackColor = true;
            this.btnRemoveFromPlaylist.Click += new System.EventHandler(this.btnRemoveFromPlaylist_Click);
            // 
            // btnMoveUp
            // 
            this.btnMoveUp.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnMoveUp.Location = new System.Drawing.Point(767, 584);
            this.btnMoveUp.Name = "btnMoveUp";
            this.btnMoveUp.Size = new System.Drawing.Size(82, 24);
            this.btnMoveUp.TabIndex = 20;
            this.btnMoveUp.Text = "Move Up";
            this.btnMoveUp.UseVisualStyleBackColor = true;
            this.btnMoveUp.Click += new System.EventHandler(this.btnMoveUp_Click);
            // 
            // btnBrowsePlaylist
            // 
            this.btnBrowsePlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnBrowsePlaylist.Location = new System.Drawing.Point(179, 3);
            this.btnBrowsePlaylist.Name = "btnBrowsePlaylist";
            this.btnBrowsePlaylist.Size = new System.Drawing.Size(82, 24);
            this.btnBrowsePlaylist.TabIndex = 22;
            this.btnBrowsePlaylist.Text = "Browse...";
            this.btnBrowsePlaylist.UseVisualStyleBackColor = true;
            this.btnBrowsePlaylist.Click += new System.EventHandler(this.btnBrowsePlaylist_Click);
            // 
            // btnRenamePlaylist
            // 
            this.btnRenamePlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnRenamePlaylist.Location = new System.Drawing.Point(179, 584);
            this.btnRenamePlaylist.Name = "btnRenamePlaylist";
            this.btnRenamePlaylist.Size = new System.Drawing.Size(82, 24);
            this.btnRenamePlaylist.TabIndex = 17;
            this.btnRenamePlaylist.Text = "Rename";
            this.btnRenamePlaylist.UseVisualStyleBackColor = true;
            this.btnRenamePlaylist.Click += new System.EventHandler(this.btnRenamePlaylist_Click);
            // 
            // lvCurPlaylistSongs
            // 
            this.lvCurPlaylistSongs.Columns.AddRange(new System.Windows.Forms.ColumnHeader[] {
            colCurPlaylistSongName,
            colCurPlaylistSongArtist});
            this.tlpLists.SetColumnSpan(this.lvCurPlaylistSongs, 4);
            this.lvCurPlaylistSongs.Dock = System.Windows.Forms.DockStyle.Fill;
            this.lvCurPlaylistSongs.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lvCurPlaylistSongs.FullRowSelect = true;
            this.lvCurPlaylistSongs.HeaderStyle = System.Windows.Forms.ColumnHeaderStyle.Nonclickable;
            this.lvCurPlaylistSongs.HideSelection = false;
            this.lvCurPlaylistSongs.Location = new System.Drawing.Point(591, 376);
            this.lvCurPlaylistSongs.MultiSelect = false;
            this.lvCurPlaylistSongs.Name = "lvCurPlaylistSongs";
            this.lvCurPlaylistSongs.Size = new System.Drawing.Size(346, 202);
            this.lvCurPlaylistSongs.TabIndex = 4;
            this.lvCurPlaylistSongs.UseCompatibleStateImageBehavior = false;
            this.lvCurPlaylistSongs.View = System.Windows.Forms.View.Details;
            this.lvCurPlaylistSongs.SelectedIndexChanged += new System.EventHandler(this.lvCurPlaylistSongs_SelectedIndexChanged);
            this.lvCurPlaylistSongs.Click += new System.EventHandler(this.lvCurPlaylistSongs_Click);
            // 
            // btnDeletePlaylist
            // 
            this.btnDeletePlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnDeletePlaylist.Location = new System.Drawing.Point(91, 584);
            this.btnDeletePlaylist.Name = "btnDeletePlaylist";
            this.btnDeletePlaylist.Size = new System.Drawing.Size(82, 24);
            this.btnDeletePlaylist.TabIndex = 16;
            this.btnDeletePlaylist.Text = "Delete";
            this.btnDeletePlaylist.UseVisualStyleBackColor = true;
            this.btnDeletePlaylist.Click += new System.EventHandler(this.btnDeletePlaylist_Click);
            // 
            // btnNewPlaylist
            // 
            this.btnNewPlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnNewPlaylist.Location = new System.Drawing.Point(3, 584);
            this.btnNewPlaylist.Name = "btnNewPlaylist";
            this.btnNewPlaylist.Size = new System.Drawing.Size(82, 24);
            this.btnNewPlaylist.TabIndex = 15;
            this.btnNewPlaylist.Text = "New";
            this.btnNewPlaylist.UseVisualStyleBackColor = true;
            this.btnNewPlaylist.Click += new System.EventHandler(this.btnNewPlaylist_Click);
            // 
            // lblPlaylistSelection
            // 
            this.lblPlaylistSelection.AutoSize = true;
            this.tlpLists.SetColumnSpan(this.lblPlaylistSelection, 2);
            this.lblPlaylistSelection.Dock = System.Windows.Forms.DockStyle.Bottom;
            this.lblPlaylistSelection.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblPlaylistSelection.Location = new System.Drawing.Point(3, 10);
            this.lblPlaylistSelection.Name = "lblPlaylistSelection";
            this.lblPlaylistSelection.Size = new System.Drawing.Size(170, 20);
            this.lblPlaylistSelection.TabIndex = 24;
            this.lblPlaylistSelection.Text = "Playlist Selection";
            // 
            // btnAddToPlaylist
            // 
            this.btnAddToPlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnAddToPlaylist.Location = new System.Drawing.Point(443, 584);
            this.btnAddToPlaylist.Name = "btnAddToPlaylist";
            this.btnAddToPlaylist.Size = new System.Drawing.Size(82, 24);
            this.btnAddToPlaylist.TabIndex = 18;
            this.btnAddToPlaylist.Text = "Add -->";
            this.btnAddToPlaylist.UseVisualStyleBackColor = true;
            this.btnAddToPlaylist.Click += new System.EventHandler(this.btnAddToPlaylist_Click);
            // 
            // txtStartPoint
            // 
            this.txtStartPoint.Dock = System.Windows.Forms.DockStyle.Fill;
            this.txtStartPoint.Location = new System.Drawing.Point(767, 346);
            this.txtStartPoint.Name = "txtStartPoint";
            this.txtStartPoint.Size = new System.Drawing.Size(82, 20);
            this.txtStartPoint.TabIndex = 30;
            this.txtStartPoint.Text = "0:00";
            // 
            // lblStartPoint
            // 
            this.lblStartPoint.Dock = System.Windows.Forms.DockStyle.Fill;
            this.lblStartPoint.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblStartPoint.Location = new System.Drawing.Point(679, 343);
            this.lblStartPoint.Name = "lblStartPoint";
            this.lblStartPoint.Size = new System.Drawing.Size(82, 30);
            this.lblStartPoint.TabIndex = 29;
            this.lblStartPoint.Text = "Start at:";
            this.lblStartPoint.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // volMeter
            // 
            this.volMeter.Amplitude = 0F;
            this.volMeter.Dock = System.Windows.Forms.DockStyle.Fill;
            this.volMeter.ForeColor = System.Drawing.Color.Lime;
            this.volMeter.Location = new System.Drawing.Point(943, 33);
            this.volMeter.MaxDb = 18F;
            this.volMeter.MinDb = -60F;
            this.volMeter.Name = "volMeter";
            this.tlpLists.SetRowSpan(this.volMeter, 8);
            this.volMeter.Size = new System.Drawing.Size(10, 545);
            this.volMeter.TabIndex = 31;
            // 
            // pnlFountain
            // 
            this.pnlFountain.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.tlpLists.SetColumnSpan(this.pnlFountain, 15);
            this.pnlFountain.Controls.Add(this.light45);
            this.pnlFountain.Controls.Add(this.light44);
            this.pnlFountain.Controls.Add(this.light41);
            this.pnlFountain.Controls.Add(this.light40);
            this.pnlFountain.Controls.Add(this.light39);
            this.pnlFountain.Controls.Add(this.light38);
            this.pnlFountain.Controls.Add(this.light37);
            this.pnlFountain.Controls.Add(this.light36);
            this.pnlFountain.Controls.Add(this.light43);
            this.pnlFountain.Controls.Add(this.light42);
            this.pnlFountain.Controls.Add(this.light33);
            this.pnlFountain.Controls.Add(this.light28);
            this.pnlFountain.Controls.Add(this.light23);
            this.pnlFountain.Controls.Add(this.light18);
            this.pnlFountain.Controls.Add(this.light13);
            this.pnlFountain.Controls.Add(this.light8);
            this.pnlFountain.Controls.Add(this.light3);
            this.pnlFountain.Controls.Add(this.light20);
            this.pnlFountain.Controls.Add(this.light17);
            this.pnlFountain.Controls.Add(this.light16);
            this.pnlFountain.Controls.Add(this.light19);
            this.pnlFountain.Controls.Add(this.light32);
            this.pnlFountain.Controls.Add(this.light27);
            this.pnlFountain.Controls.Add(this.light31);
            this.pnlFountain.Controls.Add(this.light22);
            this.pnlFountain.Controls.Add(this.light26);
            this.pnlFountain.Controls.Add(this.light12);
            this.pnlFountain.Controls.Add(this.light21);
            this.pnlFountain.Controls.Add(this.light7);
            this.pnlFountain.Controls.Add(this.light11);
            this.pnlFountain.Controls.Add(this.light2);
            this.pnlFountain.Controls.Add(this.light6);
            this.pnlFountain.Controls.Add(this.light1);
            this.pnlFountain.Controls.Add(this.light35);
            this.pnlFountain.Controls.Add(this.light30);
            this.pnlFountain.Controls.Add(this.light34);
            this.pnlFountain.Controls.Add(this.light25);
            this.pnlFountain.Controls.Add(this.light29);
            this.pnlFountain.Controls.Add(this.light15);
            this.pnlFountain.Controls.Add(this.light24);
            this.pnlFountain.Controls.Add(this.light10);
            this.pnlFountain.Controls.Add(this.light14);
            this.pnlFountain.Controls.Add(this.light9);
            this.pnlFountain.Controls.Add(this.light5);
            this.pnlFountain.Controls.Add(this.light4);
            this.pnlFountain.Controls.Add(this.pbFountain);
            this.pnlFountain.Location = new System.Drawing.Point(3, 644);
            this.pnlFountain.Name = "pnlFountain";
            this.pnlFountain.Size = new System.Drawing.Size(1002, 272);
            this.pnlFountain.TabIndex = 33;
            // 
            // light45
            // 
            this.light45.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light45.LightNumber = 45;
            this.light45.Location = new System.Drawing.Point(683, 106);
            this.light45.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light45.Name = "light45";
            this.light45.Size = new System.Drawing.Size(20, 20);
            this.light45.TabIndex = 45;
            // 
            // light44
            // 
            this.light44.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light44.LightNumber = 44;
            this.light44.Location = new System.Drawing.Point(298, 106);
            this.light44.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light44.Name = "light44";
            this.light44.Size = new System.Drawing.Size(20, 20);
            this.light44.TabIndex = 44;
            // 
            // light41
            // 
            this.light41.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light41.LightNumber = 41;
            this.light41.Location = new System.Drawing.Point(538, 12);
            this.light41.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light41.Name = "light41";
            this.light41.Size = new System.Drawing.Size(20, 20);
            this.light41.TabIndex = 41;
            // 
            // light40
            // 
            this.light40.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light40.LightNumber = 40;
            this.light40.Location = new System.Drawing.Point(519, 12);
            this.light40.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light40.Name = "light40";
            this.light40.Size = new System.Drawing.Size(20, 20);
            this.light40.TabIndex = 40;
            // 
            // light39
            // 
            this.light39.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light39.LightNumber = 39;
            this.light39.Location = new System.Drawing.Point(500, 12);
            this.light39.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light39.Name = "light39";
            this.light39.Size = new System.Drawing.Size(20, 20);
            this.light39.TabIndex = 39;
            // 
            // light38
            // 
            this.light38.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light38.LightNumber = 38;
            this.light38.Location = new System.Drawing.Point(481, 12);
            this.light38.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light38.Name = "light38";
            this.light38.Size = new System.Drawing.Size(20, 20);
            this.light38.TabIndex = 38;
            // 
            // light37
            // 
            this.light37.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light37.LightNumber = 37;
            this.light37.Location = new System.Drawing.Point(462, 12);
            this.light37.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light37.Name = "light37";
            this.light37.Size = new System.Drawing.Size(20, 20);
            this.light37.TabIndex = 37;
            // 
            // light36
            // 
            this.light36.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light36.LightNumber = 36;
            this.light36.Location = new System.Drawing.Point(443, 12);
            this.light36.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light36.Name = "light36";
            this.light36.Size = new System.Drawing.Size(20, 20);
            this.light36.TabIndex = 36;
            // 
            // light43
            // 
            this.light43.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light43.LightNumber = 43;
            this.light43.Location = new System.Drawing.Point(514, 73);
            this.light43.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light43.Name = "light43";
            this.light43.Size = new System.Drawing.Size(20, 20);
            this.light43.TabIndex = 43;
            // 
            // light42
            // 
            this.light42.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light42.LightNumber = 42;
            this.light42.Location = new System.Drawing.Point(467, 73);
            this.light42.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light42.Name = "light42";
            this.light42.Size = new System.Drawing.Size(20, 20);
            this.light42.TabIndex = 42;
            // 
            // light33
            // 
            this.light33.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light33.LightNumber = 33;
            this.light33.Location = new System.Drawing.Point(889, 129);
            this.light33.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light33.Name = "light33";
            this.light33.Size = new System.Drawing.Size(20, 20);
            this.light33.TabIndex = 33;
            // 
            // light28
            // 
            this.light28.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light28.LightNumber = 28;
            this.light28.Location = new System.Drawing.Point(761, 129);
            this.light28.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light28.Name = "light28";
            this.light28.Size = new System.Drawing.Size(20, 20);
            this.light28.TabIndex = 28;
            // 
            // light23
            // 
            this.light23.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light23.LightNumber = 23;
            this.light23.Location = new System.Drawing.Point(632, 129);
            this.light23.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light23.Name = "light23";
            this.light23.Size = new System.Drawing.Size(20, 20);
            this.light23.TabIndex = 23;
            // 
            // light18
            // 
            this.light18.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light18.LightNumber = 18;
            this.light18.Location = new System.Drawing.Point(504, 129);
            this.light18.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light18.Name = "light18";
            this.light18.Size = new System.Drawing.Size(20, 20);
            this.light18.TabIndex = 18;
            // 
            // light13
            // 
            this.light13.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light13.LightNumber = 13;
            this.light13.Location = new System.Drawing.Point(376, 129);
            this.light13.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light13.Name = "light13";
            this.light13.Size = new System.Drawing.Size(20, 20);
            this.light13.TabIndex = 13;
            // 
            // light8
            // 
            this.light8.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light8.LightNumber = 8;
            this.light8.Location = new System.Drawing.Point(248, 129);
            this.light8.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light8.Name = "light8";
            this.light8.Size = new System.Drawing.Size(20, 20);
            this.light8.TabIndex = 8;
            // 
            // light3
            // 
            this.light3.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light3.LightNumber = 3;
            this.light3.Location = new System.Drawing.Point(120, 129);
            this.light3.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light3.Name = "light3";
            this.light3.Size = new System.Drawing.Size(20, 20);
            this.light3.TabIndex = 3;
            // 
            // light20
            // 
            this.light20.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light20.LightNumber = 20;
            this.light20.Location = new System.Drawing.Point(540, 218);
            this.light20.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light20.Name = "light20";
            this.light20.Size = new System.Drawing.Size(20, 20);
            this.light20.TabIndex = 20;
            // 
            // light17
            // 
            this.light17.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light17.LightNumber = 17;
            this.light17.Location = new System.Drawing.Point(514, 218);
            this.light17.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light17.Name = "light17";
            this.light17.Size = new System.Drawing.Size(20, 20);
            this.light17.TabIndex = 17;
            // 
            // light16
            // 
            this.light16.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light16.LightNumber = 16;
            this.light16.Location = new System.Drawing.Point(466, 218);
            this.light16.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light16.Name = "light16";
            this.light16.Size = new System.Drawing.Size(20, 20);
            this.light16.TabIndex = 16;
            // 
            // light19
            // 
            this.light19.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light19.LightNumber = 19;
            this.light19.Location = new System.Drawing.Point(442, 218);
            this.light19.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light19.Name = "light19";
            this.light19.Size = new System.Drawing.Size(20, 20);
            this.light19.TabIndex = 19;
            // 
            // light32
            // 
            this.light32.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light32.LightNumber = 32;
            this.light32.Location = new System.Drawing.Point(900, 218);
            this.light32.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light32.Name = "light32";
            this.light32.Size = new System.Drawing.Size(20, 20);
            this.light32.TabIndex = 32;
            // 
            // light27
            // 
            this.light27.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light27.LightNumber = 27;
            this.light27.Location = new System.Drawing.Point(771, 218);
            this.light27.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light27.Name = "light27";
            this.light27.Size = new System.Drawing.Size(20, 20);
            this.light27.TabIndex = 27;
            // 
            // light31
            // 
            this.light31.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light31.LightNumber = 31;
            this.light31.Location = new System.Drawing.Point(852, 218);
            this.light31.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light31.Name = "light31";
            this.light31.Size = new System.Drawing.Size(20, 20);
            this.light31.TabIndex = 31;
            // 
            // light22
            // 
            this.light22.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light22.LightNumber = 22;
            this.light22.Location = new System.Drawing.Point(643, 218);
            this.light22.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light22.Name = "light22";
            this.light22.Size = new System.Drawing.Size(20, 20);
            this.light22.TabIndex = 22;
            // 
            // light26
            // 
            this.light26.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light26.LightNumber = 26;
            this.light26.Location = new System.Drawing.Point(724, 218);
            this.light26.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light26.Name = "light26";
            this.light26.Size = new System.Drawing.Size(20, 20);
            this.light26.TabIndex = 26;
            // 
            // light12
            // 
            this.light12.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light12.LightNumber = 12;
            this.light12.Location = new System.Drawing.Point(388, 218);
            this.light12.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light12.Name = "light12";
            this.light12.Size = new System.Drawing.Size(20, 20);
            this.light12.TabIndex = 12;
            // 
            // light21
            // 
            this.light21.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light21.LightNumber = 21;
            this.light21.Location = new System.Drawing.Point(595, 218);
            this.light21.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light21.Name = "light21";
            this.light21.Size = new System.Drawing.Size(20, 20);
            this.light21.TabIndex = 21;
            // 
            // light7
            // 
            this.light7.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light7.LightNumber = 7;
            this.light7.Location = new System.Drawing.Point(258, 218);
            this.light7.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light7.Name = "light7";
            this.light7.Size = new System.Drawing.Size(20, 20);
            this.light7.TabIndex = 7;
            // 
            // light11
            // 
            this.light11.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light11.LightNumber = 11;
            this.light11.Location = new System.Drawing.Point(338, 218);
            this.light11.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light11.Name = "light11";
            this.light11.Size = new System.Drawing.Size(20, 20);
            this.light11.TabIndex = 11;
            // 
            // light2
            // 
            this.light2.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light2.LightNumber = 2;
            this.light2.Location = new System.Drawing.Point(130, 218);
            this.light2.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light2.Name = "light2";
            this.light2.Size = new System.Drawing.Size(20, 20);
            this.light2.TabIndex = 2;
            // 
            // light6
            // 
            this.light6.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light6.LightNumber = 6;
            this.light6.Location = new System.Drawing.Point(211, 218);
            this.light6.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light6.Name = "light6";
            this.light6.Size = new System.Drawing.Size(20, 20);
            this.light6.TabIndex = 6;
            // 
            // light1
            // 
            this.light1.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light1.LightNumber = 1;
            this.light1.Location = new System.Drawing.Point(80, 218);
            this.light1.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light1.Name = "light1";
            this.light1.Size = new System.Drawing.Size(20, 20);
            this.light1.TabIndex = 1;
            // 
            // light35
            // 
            this.light35.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light35.LightNumber = 35;
            this.light35.Location = new System.Drawing.Point(925, 41);
            this.light35.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light35.Name = "light35";
            this.light35.Size = new System.Drawing.Size(20, 20);
            this.light35.TabIndex = 35;
            // 
            // light30
            // 
            this.light30.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light30.LightNumber = 30;
            this.light30.Location = new System.Drawing.Point(795, 41);
            this.light30.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light30.Name = "light30";
            this.light30.Size = new System.Drawing.Size(20, 20);
            this.light30.TabIndex = 30;
            // 
            // light34
            // 
            this.light34.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light34.LightNumber = 34;
            this.light34.Location = new System.Drawing.Point(829, 41);
            this.light34.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light34.Name = "light34";
            this.light34.Size = new System.Drawing.Size(20, 20);
            this.light34.TabIndex = 34;
            // 
            // light25
            // 
            this.light25.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light25.LightNumber = 25;
            this.light25.Location = new System.Drawing.Point(668, 41);
            this.light25.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light25.Name = "light25";
            this.light25.Size = new System.Drawing.Size(20, 20);
            this.light25.TabIndex = 25;
            // 
            // light29
            // 
            this.light29.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light29.LightNumber = 29;
            this.light29.Location = new System.Drawing.Point(699, 41);
            this.light29.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light29.Name = "light29";
            this.light29.Size = new System.Drawing.Size(20, 20);
            this.light29.TabIndex = 29;
            // 
            // light15
            // 
            this.light15.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light15.LightNumber = 15;
            this.light15.Location = new System.Drawing.Point(411, 41);
            this.light15.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light15.Name = "light15";
            this.light15.Size = new System.Drawing.Size(20, 20);
            this.light15.TabIndex = 15;
            // 
            // light24
            // 
            this.light24.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light24.LightNumber = 24;
            this.light24.Location = new System.Drawing.Point(572, 41);
            this.light24.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light24.Name = "light24";
            this.light24.Size = new System.Drawing.Size(20, 20);
            this.light24.TabIndex = 24;
            // 
            // light10
            // 
            this.light10.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light10.LightNumber = 10;
            this.light10.Location = new System.Drawing.Point(282, 41);
            this.light10.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light10.Name = "light10";
            this.light10.Size = new System.Drawing.Size(20, 20);
            this.light10.TabIndex = 10;
            // 
            // light14
            // 
            this.light14.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light14.LightNumber = 14;
            this.light14.Location = new System.Drawing.Point(315, 41);
            this.light14.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light14.Name = "light14";
            this.light14.Size = new System.Drawing.Size(20, 20);
            this.light14.TabIndex = 14;
            // 
            // light9
            // 
            this.light9.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light9.LightNumber = 9;
            this.light9.Location = new System.Drawing.Point(186, 41);
            this.light9.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light9.Name = "light9";
            this.light9.Size = new System.Drawing.Size(20, 20);
            this.light9.TabIndex = 9;
            // 
            // light5
            // 
            this.light5.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light5.LightNumber = 5;
            this.light5.Location = new System.Drawing.Point(154, 41);
            this.light5.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light5.Name = "light5";
            this.light5.Size = new System.Drawing.Size(20, 20);
            this.light5.TabIndex = 5;
            // 
            // light4
            // 
            this.light4.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light4.LightNumber = 4;
            this.light4.Location = new System.Drawing.Point(57, 41);
            this.light4.Margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
            this.light4.Name = "light4";
            this.light4.Size = new System.Drawing.Size(20, 20);
            this.light4.TabIndex = 4;
            // 
            // pbFountain
            // 
            this.pbFountain.Anchor = ((System.Windows.Forms.AnchorStyles)((((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom) 
            | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.pbFountain.Image = ((System.Drawing.Image)(resources.GetObject("pbFountain.Image")));
            this.pbFountain.Location = new System.Drawing.Point(0, 3);
            this.pbFountain.Name = "pbFountain";
            this.pbFountain.Size = new System.Drawing.Size(1002, 269);
            this.pbFountain.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.pbFountain.TabIndex = 32;
            this.pbFountain.TabStop = false;
            // 
            // lblShowProgress
            // 
            this.tlpLists.SetColumnSpan(this.lblShowProgress, 4);
            this.lblShowProgress.Dock = System.Windows.Forms.DockStyle.Fill;
            this.lblShowProgress.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblShowProgress.Location = new System.Drawing.Point(591, 323);
            this.lblShowProgress.Name = "lblShowProgress";
            this.lblShowProgress.Size = new System.Drawing.Size(346, 20);
            this.lblShowProgress.TabIndex = 2;
            this.lblShowProgress.Text = "Show Now : Show Total";
            this.lblShowProgress.TextAlign = System.Drawing.ContentAlignment.TopCenter;
            // 
            // lvAnnouncements
            // 
            this.lvAnnouncements.Columns.AddRange(new System.Windows.Forms.ColumnHeader[] {
            colName});
            this.tlpLists.SetColumnSpan(this.lvAnnouncements, 4);
            this.lvAnnouncements.Dock = System.Windows.Forms.DockStyle.Fill;
            this.lvAnnouncements.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lvAnnouncements.HeaderStyle = System.Windows.Forms.ColumnHeaderStyle.None;
            this.lvAnnouncements.HideSelection = false;
            this.lvAnnouncements.Location = new System.Drawing.Point(591, 58);
            this.lvAnnouncements.MultiSelect = false;
            this.lvAnnouncements.Name = "lvAnnouncements";
            this.lvAnnouncements.Size = new System.Drawing.Size(346, 202);
            this.lvAnnouncements.TabIndex = 3;
            this.lvAnnouncements.UseCompatibleStateImageBehavior = false;
            this.lvAnnouncements.View = System.Windows.Forms.View.Details;
            this.lvAnnouncements.SelectedIndexChanged += new System.EventHandler(this.lvAnnouncements_SelectedIndexChanged);
            // 
            // playlistFolderBrowser
            // 
            this.playlistFolderBrowser.Description = "Select a Playlist Folder";
            this.playlistFolderBrowser.RootFolder = System.Environment.SpecialFolder.MyComputer;
            this.playlistFolderBrowser.ShowNewFolderButton = false;
            // 
            // songFolderBrowser
            // 
            this.songFolderBrowser.Description = "Select a Song Folder";
            this.songFolderBrowser.RootFolder = System.Environment.SpecialFolder.MyComputer;
            this.songFolderBrowser.ShowNewFolderButton = false;
            // 
            // PlaybackForm
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 13F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(1008, 961);
            this.Controls.Add(this.tlpLists);
            this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
            this.KeyPreview = true;
            this.MinimizeBox = false;
            this.MinimumSize = new System.Drawing.Size(1022, 917);
            this.Name = "PlaybackForm";
            this.StartPosition = System.Windows.Forms.FormStartPosition.Manual;
            this.Text = "Grand Haven Musical Fountain Playback Control";
            this.FormClosing += new System.Windows.Forms.FormClosingEventHandler(this.PlaybackForm_FormClosing);
            this.Resize += new System.EventHandler(this.PlaybackForm_Resize);
            ((System.ComponentModel.ISupportInitialize)(this.barVolLeftChannel)).EndInit();
            this.tlpLists.ResumeLayout(false);
            this.tlpLists.PerformLayout();
            this.pnlFountain.ResumeLayout(false);
            ((System.ComponentModel.ISupportInitialize)(this.pbFountain)).EndInit();
            this.ResumeLayout(false);

        }

        #endregion

        private System.Windows.Forms.Button btnStartStop;
        private System.Windows.Forms.Button btnPauseResume;
        private System.Windows.Forms.TrackBar barVolLeftChannel;
        private System.Windows.Forms.Button btnSettings;
        private System.Windows.Forms.Button btnLogin;
        private System.Windows.Forms.ListView lvPlaylists;
        private System.Windows.Forms.ListView lvSongs;
        private System.Windows.Forms.Button btnConnectDMX;
        private System.Windows.Forms.TextBox txtManualFCW;
        private System.Windows.Forms.Button btnExecuteManualFCW;
        private System.Windows.Forms.TableLayoutPanel tlpLists;
        private System.Windows.Forms.ListView lvCurPlaylistSongs;
        private System.Windows.Forms.Label lblShowProgress;
        private System.Windows.Forms.Label lblSongProgress;
        private System.Windows.Forms.Button btnAnnounce;
        private System.Windows.Forms.ListView lvAnnouncements;
        private System.Windows.Forms.Label lblVolume;
        private System.Windows.Forms.Button btnRenamePlaylist;
        private System.Windows.Forms.Button btnDeletePlaylist;
        private System.Windows.Forms.Button btnMoveDown;
        private System.Windows.Forms.Button btnRemoveFromPlaylist;
        private System.Windows.Forms.Button btnMoveUp;
        private System.Windows.Forms.ToolTip ttVolume;
        private System.Windows.Forms.Label lblCurrentSong;
        private System.Windows.Forms.Label lblPlaylistSelection;
        private System.Windows.Forms.Label lblSongSelection;
        private System.Windows.Forms.Button btnBrowseSong;
        private System.Windows.Forms.FolderBrowserDialog playlistFolderBrowser;
        private System.Windows.Forms.FolderBrowserDialog songFolderBrowser;
        private System.Windows.Forms.Button btnNewPlaylist;
        private System.Windows.Forms.Button btnAddToPlaylist;
        private System.Windows.Forms.Label lblCurrentSongFolder;
        private System.Windows.Forms.Label lblCurrentPlaylistFolder;
        private System.Windows.Forms.Button btnBrowsePlaylist;
        private System.Windows.Forms.TextBox txtStartPoint;
        private System.Windows.Forms.Label lblStartPoint;
        private NAudio.Gui.VolumeMeter volMeter;
        private System.Windows.Forms.Label lblCurrentShow;
        private System.Windows.Forms.PictureBox pbFountain;
        private System.Windows.Forms.Panel pnlFountain;
        private LightControl light4;
        private LightControl light10;
        private LightControl light9;
        private LightControl light5;
        private LightControl light3;
        private LightControl light20;
        private LightControl light17;
        private LightControl light16;
        private LightControl light19;
        private LightControl light32;
        private LightControl light27;
        private LightControl light31;
        private LightControl light22;
        private LightControl light26;
        private LightControl light12;
        private LightControl light21;
        private LightControl light7;
        private LightControl light11;
        private LightControl light2;
        private LightControl light6;
        private LightControl light1;
        private LightControl light35;
        private LightControl light30;
        private LightControl light34;
        private LightControl light25;
        private LightControl light29;
        private LightControl light15;
        private LightControl light24;
        private LightControl light14;
        private LightControl light41;
        private LightControl light40;
        private LightControl light39;
        private LightControl light38;
        private LightControl light37;
        private LightControl light36;
        private LightControl light43;
        private LightControl light42;
        private LightControl light33;
        private LightControl light28;
        private LightControl light23;
        private LightControl light18;
        private LightControl light13;
        private LightControl light8;
        private LightControl light45;
        private LightControl light44;
        private System.Windows.Forms.Label lblLightFCWCount;
        private System.Windows.Forms.Label lblTotalFCWCount;
        private System.Windows.Forms.Label lblWaterFCWCount;
        private System.Windows.Forms.Label lblLightFCWs;
        private System.Windows.Forms.Label lblWaterFCWs;
    }
}

