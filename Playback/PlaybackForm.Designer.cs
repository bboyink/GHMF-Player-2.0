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
            this.lblCurrentSongFolder = new System.Windows.Forms.Label();
            this.lblCurrentPlaylistFolder = new System.Windows.Forms.Label();
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
            this.pnlPlaceholder = new System.Windows.Forms.Panel();
            this.lblCurrentShow = new System.Windows.Forms.Label();
            this.lblCurrentSong = new System.Windows.Forms.Label();
            this.lvAnnouncements = new System.Windows.Forms.ListView();
            this.lblShowProgress = new System.Windows.Forms.Label();
            this.lblSongProgress = new System.Windows.Forms.Label();
            this.btnDeletePlaylist = new System.Windows.Forms.Button();
            this.btnNewPlaylist = new System.Windows.Forms.Button();
            this.lblPlaylistSelection = new System.Windows.Forms.Label();
            this.btnAddToPlaylist = new System.Windows.Forms.Button();
            this.txtStartPoint = new System.Windows.Forms.TextBox();
            this.lblStartPoint = new System.Windows.Forms.Label();
            this.volMeter = new NAudio.Gui.VolumeMeter();
            this.pnlFountain = new System.Windows.Forms.Panel();
            this.light41 = new System.Windows.Forms.Panel();
            this.light40 = new System.Windows.Forms.Panel();
            this.light39 = new System.Windows.Forms.Panel();
            this.light38 = new System.Windows.Forms.Panel();
            this.light37 = new System.Windows.Forms.Panel();
            this.light36 = new System.Windows.Forms.Panel();
            this.light43 = new System.Windows.Forms.Panel();
            this.light42 = new System.Windows.Forms.Panel();
            this.light33 = new System.Windows.Forms.Panel();
            this.light28 = new System.Windows.Forms.Panel();
            this.light23 = new System.Windows.Forms.Panel();
            this.light18 = new System.Windows.Forms.Panel();
            this.light13 = new System.Windows.Forms.Panel();
            this.light8 = new System.Windows.Forms.Panel();
            this.light3 = new System.Windows.Forms.Panel();
            this.light20 = new System.Windows.Forms.Panel();
            this.light17 = new System.Windows.Forms.Panel();
            this.light16 = new System.Windows.Forms.Panel();
            this.light19 = new System.Windows.Forms.Panel();
            this.light32 = new System.Windows.Forms.Panel();
            this.light27 = new System.Windows.Forms.Panel();
            this.light31 = new System.Windows.Forms.Panel();
            this.light22 = new System.Windows.Forms.Panel();
            this.light26 = new System.Windows.Forms.Panel();
            this.light12 = new System.Windows.Forms.Panel();
            this.light21 = new System.Windows.Forms.Panel();
            this.light7 = new System.Windows.Forms.Panel();
            this.light11 = new System.Windows.Forms.Panel();
            this.light2 = new System.Windows.Forms.Panel();
            this.light6 = new System.Windows.Forms.Panel();
            this.light1 = new System.Windows.Forms.Panel();
            this.light35 = new System.Windows.Forms.Panel();
            this.light30 = new System.Windows.Forms.Panel();
            this.light34 = new System.Windows.Forms.Panel();
            this.light25 = new System.Windows.Forms.Panel();
            this.light29 = new System.Windows.Forms.Panel();
            this.light15 = new System.Windows.Forms.Panel();
            this.light24 = new System.Windows.Forms.Panel();
            this.light10 = new System.Windows.Forms.Panel();
            this.light14 = new System.Windows.Forms.Panel();
            this.light9 = new System.Windows.Forms.Panel();
            this.light5 = new System.Windows.Forms.Panel();
            this.light4 = new System.Windows.Forms.Panel();
            this.pbFountain = new System.Windows.Forms.PictureBox();
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
            this.pnlPlaceholder.SuspendLayout();
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
            this.btnStartStop.Location = new System.Drawing.Point(567, 326);
            this.btnStartStop.Name = "btnStartStop";
            this.btnStartStop.Size = new System.Drawing.Size(88, 24);
            this.btnStartStop.TabIndex = 0;
            this.btnStartStop.Text = "Start Show";
            this.btnStartStop.UseVisualStyleBackColor = true;
            this.btnStartStop.Click += new System.EventHandler(this.btnStartStop_Click);
            // 
            // btnPauseResume
            // 
            this.btnPauseResume.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnPauseResume.Location = new System.Drawing.Point(849, 326);
            this.btnPauseResume.Name = "btnPauseResume";
            this.btnPauseResume.Size = new System.Drawing.Size(88, 24);
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
            this.tlpLists.SetRowSpan(this.barVolLeftChannel, 4);
            this.barVolLeftChannel.Size = new System.Drawing.Size(46, 585);
            this.barVolLeftChannel.TabIndex = 7;
            this.barVolLeftChannel.TickFrequency = 5;
            this.barVolLeftChannel.ValueChanged += new System.EventHandler(this.barVolume_ValueChanged);
            // 
            // btnSettings
            // 
            this.btnSettings.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnSettings.Location = new System.Drawing.Point(191, 654);
            this.btnSettings.Name = "btnSettings";
            this.btnSettings.Size = new System.Drawing.Size(88, 24);
            this.btnSettings.TabIndex = 11;
            this.btnSettings.Text = "Settings";
            this.btnSettings.UseVisualStyleBackColor = true;
            this.btnSettings.Click += new System.EventHandler(this.btnSettings_Click);
            // 
            // btnLogin
            // 
            this.btnLogin.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnLogin.Location = new System.Drawing.Point(3, 654);
            this.btnLogin.Name = "btnLogin";
            this.btnLogin.Size = new System.Drawing.Size(88, 24);
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
            this.tlpLists.SetRowSpan(this.lvPlaylists, 3);
            this.lvPlaylists.Size = new System.Drawing.Size(276, 560);
            this.lvPlaylists.TabIndex = 2;
            this.lvPlaylists.UseCompatibleStateImageBehavior = false;
            this.lvPlaylists.View = System.Windows.Forms.View.Details;
            this.lvPlaylists.AfterLabelEdit += new System.Windows.Forms.LabelEditEventHandler(this.lvPlaylists_AfterLabelEdit);
            this.lvPlaylists.SelectedIndexChanged += new System.EventHandler(this.lvPlaylists_SelectedIndexChanged);
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
            this.lvSongs.Location = new System.Drawing.Point(285, 58);
            this.lvSongs.MultiSelect = false;
            this.lvSongs.Name = "lvSongs";
            this.tlpLists.SetRowSpan(this.lvSongs, 3);
            this.lvSongs.Size = new System.Drawing.Size(276, 560);
            this.lvSongs.TabIndex = 3;
            this.lvSongs.UseCompatibleStateImageBehavior = false;
            this.lvSongs.View = System.Windows.Forms.View.Details;
            this.lvSongs.DoubleClick += new System.EventHandler(this.lvSongs_DoubleClick);
            // 
            // btnConnectDMX
            // 
            this.btnConnectDMX.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnConnectDMX.Location = new System.Drawing.Point(849, 654);
            this.btnConnectDMX.Name = "btnConnectDMX";
            this.btnConnectDMX.Size = new System.Drawing.Size(88, 24);
            this.btnConnectDMX.TabIndex = 14;
            this.btnConnectDMX.Text = "Connect DMX";
            this.btnConnectDMX.UseVisualStyleBackColor = true;
            this.btnConnectDMX.Click += new System.EventHandler(this.btnConnectDMX_Click);
            // 
            // txtManualFCW
            // 
            this.tlpLists.SetColumnSpan(this.txtManualFCW, 2);
            this.txtManualFCW.Dock = System.Windows.Forms.DockStyle.Fill;
            this.txtManualFCW.Location = new System.Drawing.Point(285, 654);
            this.txtManualFCW.Name = "txtManualFCW";
            this.txtManualFCW.Size = new System.Drawing.Size(182, 20);
            this.txtManualFCW.TabIndex = 12;
            this.txtManualFCW.KeyPress += new System.Windows.Forms.KeyPressEventHandler(this.txtManualFCW_KeyPress);
            // 
            // btnExecuteManualFCW
            // 
            this.btnExecuteManualFCW.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnExecuteManualFCW.Location = new System.Drawing.Point(473, 654);
            this.btnExecuteManualFCW.Name = "btnExecuteManualFCW";
            this.btnExecuteManualFCW.Size = new System.Drawing.Size(88, 24);
            this.btnExecuteManualFCW.TabIndex = 13;
            this.btnExecuteManualFCW.Text = "Execute";
            this.btnExecuteManualFCW.UseVisualStyleBackColor = true;
            this.btnExecuteManualFCW.Click += new System.EventHandler(this.btnExecuteManualFCW_Click);
            // 
            // tlpLists
            // 
            this.tlpLists.ColumnCount = 12;
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 10F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Absolute, 16F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Absolute, 52F));
            this.tlpLists.Controls.Add(this.lblCurrentSongFolder, 3, 1);
            this.tlpLists.Controls.Add(this.lblCurrentPlaylistFolder, 0, 1);
            this.tlpLists.Controls.Add(this.btnExecuteManualFCW, 5, 6);
            this.tlpLists.Controls.Add(this.btnPauseResume, 9, 3);
            this.tlpLists.Controls.Add(this.txtManualFCW, 3, 6);
            this.tlpLists.Controls.Add(this.lblVolume, 10, 0);
            this.tlpLists.Controls.Add(this.btnMoveDown, 9, 5);
            this.tlpLists.Controls.Add(this.btnSettings, 2, 6);
            this.tlpLists.Controls.Add(this.btnAnnounce, 6, 0);
            this.tlpLists.Controls.Add(this.btnBrowseSong, 5, 0);
            this.tlpLists.Controls.Add(this.btnLogin, 0, 6);
            this.tlpLists.Controls.Add(this.btnStartStop, 6, 3);
            this.tlpLists.Controls.Add(this.lblSongSelection, 3, 0);
            this.tlpLists.Controls.Add(this.barVolLeftChannel, 11, 1);
            this.tlpLists.Controls.Add(this.btnRemoveFromPlaylist, 6, 5);
            this.tlpLists.Controls.Add(this.btnMoveUp, 8, 5);
            this.tlpLists.Controls.Add(this.btnBrowsePlaylist, 2, 0);
            this.tlpLists.Controls.Add(this.btnRenamePlaylist, 2, 5);
            this.tlpLists.Controls.Add(this.lvCurPlaylistSongs, 6, 4);
            this.tlpLists.Controls.Add(this.pnlPlaceholder, 6, 2);
            this.tlpLists.Controls.Add(this.lvPlaylists, 0, 2);
            this.tlpLists.Controls.Add(this.lvSongs, 3, 2);
            this.tlpLists.Controls.Add(this.btnDeletePlaylist, 1, 5);
            this.tlpLists.Controls.Add(this.btnNewPlaylist, 0, 5);
            this.tlpLists.Controls.Add(this.lblPlaylistSelection, 0, 0);
            this.tlpLists.Controls.Add(this.btnAddToPlaylist, 5, 5);
            this.tlpLists.Controls.Add(this.btnConnectDMX, 9, 6);
            this.tlpLists.Controls.Add(this.txtStartPoint, 8, 3);
            this.tlpLists.Controls.Add(this.lblStartPoint, 7, 3);
            this.tlpLists.Controls.Add(this.volMeter, 10, 1);
            this.tlpLists.Controls.Add(this.pnlFountain, 0, 7);
            this.tlpLists.Dock = System.Windows.Forms.DockStyle.Fill;
            this.tlpLists.Location = new System.Drawing.Point(0, 0);
            this.tlpLists.Margin = new System.Windows.Forms.Padding(0);
            this.tlpLists.Name = "tlpLists";
            this.tlpLists.RowCount = 8;
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 30F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 25F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 50F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 30F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 50F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 30F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 30F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 279F));
            this.tlpLists.Size = new System.Drawing.Size(1008, 961);
            this.tlpLists.TabIndex = 0;
            // 
            // lblCurrentSongFolder
            // 
            this.lblCurrentSongFolder.AutoEllipsis = true;
            this.tlpLists.SetColumnSpan(this.lblCurrentSongFolder, 3);
            this.lblCurrentSongFolder.Dock = System.Windows.Forms.DockStyle.Bottom;
            this.lblCurrentSongFolder.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblCurrentSongFolder.Location = new System.Drawing.Point(285, 35);
            this.lblCurrentSongFolder.Name = "lblCurrentSongFolder";
            this.lblCurrentSongFolder.Size = new System.Drawing.Size(276, 20);
            this.lblCurrentSongFolder.TabIndex = 28;
            // 
            // lblCurrentPlaylistFolder
            // 
            this.lblCurrentPlaylistFolder.AutoEllipsis = true;
            this.tlpLists.SetColumnSpan(this.lblCurrentPlaylistFolder, 3);
            this.lblCurrentPlaylistFolder.Dock = System.Windows.Forms.DockStyle.Bottom;
            this.lblCurrentPlaylistFolder.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblCurrentPlaylistFolder.Location = new System.Drawing.Point(3, 35);
            this.lblCurrentPlaylistFolder.Name = "lblCurrentPlaylistFolder";
            this.lblCurrentPlaylistFolder.Size = new System.Drawing.Size(276, 20);
            this.lblCurrentPlaylistFolder.TabIndex = 27;
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
            this.btnMoveDown.Location = new System.Drawing.Point(849, 624);
            this.btnMoveDown.Name = "btnMoveDown";
            this.btnMoveDown.Size = new System.Drawing.Size(88, 24);
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
            this.btnAnnounce.Location = new System.Drawing.Point(567, 3);
            this.btnAnnounce.Name = "btnAnnounce";
            this.btnAnnounce.Size = new System.Drawing.Size(370, 24);
            this.btnAnnounce.TabIndex = 6;
            this.btnAnnounce.Text = "PLAY ANNOUNCEMENT";
            this.btnAnnounce.UseVisualStyleBackColor = true;
            this.btnAnnounce.EnabledChanged += new System.EventHandler(this.btnAnnounce_EnabledChanged);
            this.btnAnnounce.Click += new System.EventHandler(this.btnAnnounce_Click);
            // 
            // btnBrowseSong
            // 
            this.btnBrowseSong.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnBrowseSong.Location = new System.Drawing.Point(473, 3);
            this.btnBrowseSong.Name = "btnBrowseSong";
            this.btnBrowseSong.Size = new System.Drawing.Size(88, 24);
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
            this.lblSongSelection.Location = new System.Drawing.Point(285, 10);
            this.lblSongSelection.Name = "lblSongSelection";
            this.lblSongSelection.Size = new System.Drawing.Size(182, 20);
            this.lblSongSelection.TabIndex = 25;
            this.lblSongSelection.Text = "Song Selection";
            // 
            // btnRemoveFromPlaylist
            // 
            this.btnRemoveFromPlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnRemoveFromPlaylist.Location = new System.Drawing.Point(567, 624);
            this.btnRemoveFromPlaylist.Name = "btnRemoveFromPlaylist";
            this.btnRemoveFromPlaylist.Size = new System.Drawing.Size(88, 24);
            this.btnRemoveFromPlaylist.TabIndex = 19;
            this.btnRemoveFromPlaylist.Text = "<-- Remove";
            this.btnRemoveFromPlaylist.UseVisualStyleBackColor = true;
            this.btnRemoveFromPlaylist.Click += new System.EventHandler(this.btnRemoveFromPlaylist_Click);
            // 
            // btnMoveUp
            // 
            this.btnMoveUp.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnMoveUp.Location = new System.Drawing.Point(755, 624);
            this.btnMoveUp.Name = "btnMoveUp";
            this.btnMoveUp.Size = new System.Drawing.Size(88, 24);
            this.btnMoveUp.TabIndex = 20;
            this.btnMoveUp.Text = "Move Up";
            this.btnMoveUp.UseVisualStyleBackColor = true;
            this.btnMoveUp.Click += new System.EventHandler(this.btnMoveUp_Click);
            // 
            // btnBrowsePlaylist
            // 
            this.btnBrowsePlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnBrowsePlaylist.Location = new System.Drawing.Point(191, 3);
            this.btnBrowsePlaylist.Name = "btnBrowsePlaylist";
            this.btnBrowsePlaylist.Size = new System.Drawing.Size(88, 24);
            this.btnBrowsePlaylist.TabIndex = 22;
            this.btnBrowsePlaylist.Text = "Browse...";
            this.btnBrowsePlaylist.UseVisualStyleBackColor = true;
            this.btnBrowsePlaylist.Click += new System.EventHandler(this.btnBrowsePlaylist_Click);
            // 
            // btnRenamePlaylist
            // 
            this.btnRenamePlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnRenamePlaylist.Location = new System.Drawing.Point(191, 624);
            this.btnRenamePlaylist.Name = "btnRenamePlaylist";
            this.btnRenamePlaylist.Size = new System.Drawing.Size(88, 24);
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
            this.lvCurPlaylistSongs.Location = new System.Drawing.Point(567, 356);
            this.lvCurPlaylistSongs.MultiSelect = false;
            this.lvCurPlaylistSongs.Name = "lvCurPlaylistSongs";
            this.lvCurPlaylistSongs.Size = new System.Drawing.Size(370, 262);
            this.lvCurPlaylistSongs.TabIndex = 4;
            this.lvCurPlaylistSongs.UseCompatibleStateImageBehavior = false;
            this.lvCurPlaylistSongs.View = System.Windows.Forms.View.Details;
            this.lvCurPlaylistSongs.SelectedIndexChanged += new System.EventHandler(this.lvCurPlaylistSongs_SelectedIndexChanged);
            // 
            // pnlPlaceholder
            // 
            this.tlpLists.SetColumnSpan(this.pnlPlaceholder, 4);
            this.pnlPlaceholder.Controls.Add(this.lblCurrentShow);
            this.pnlPlaceholder.Controls.Add(this.lblCurrentSong);
            this.pnlPlaceholder.Controls.Add(this.lvAnnouncements);
            this.pnlPlaceholder.Controls.Add(this.lblShowProgress);
            this.pnlPlaceholder.Controls.Add(this.lblSongProgress);
            this.pnlPlaceholder.Dock = System.Windows.Forms.DockStyle.Fill;
            this.pnlPlaceholder.Location = new System.Drawing.Point(564, 55);
            this.pnlPlaceholder.Margin = new System.Windows.Forms.Padding(0);
            this.pnlPlaceholder.Name = "pnlPlaceholder";
            this.pnlPlaceholder.Size = new System.Drawing.Size(376, 268);
            this.pnlPlaceholder.TabIndex = 5;
            // 
            // lblCurrentShow
            // 
            this.lblCurrentShow.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblCurrentShow.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblCurrentShow.Location = new System.Drawing.Point(3, 200);
            this.lblCurrentShow.Name = "lblCurrentShow";
            this.lblCurrentShow.Size = new System.Drawing.Size(370, 17);
            this.lblCurrentShow.TabIndex = 4;
            this.lblCurrentShow.Text = "Now Playing Show: ";
            this.lblCurrentShow.TextAlign = System.Drawing.ContentAlignment.TopCenter;
            // 
            // lblCurrentSong
            // 
            this.lblCurrentSong.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblCurrentSong.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblCurrentSong.Location = new System.Drawing.Point(3, 217);
            this.lblCurrentSong.Name = "lblCurrentSong";
            this.lblCurrentSong.Size = new System.Drawing.Size(370, 17);
            this.lblCurrentSong.TabIndex = 0;
            this.lblCurrentSong.Text = "Now Playing Song: ";
            this.lblCurrentSong.TextAlign = System.Drawing.ContentAlignment.TopCenter;
            // 
            // lvAnnouncements
            // 
            this.lvAnnouncements.Anchor = ((System.Windows.Forms.AnchorStyles)((((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom) 
            | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lvAnnouncements.Columns.AddRange(new System.Windows.Forms.ColumnHeader[] {
            colName});
            this.lvAnnouncements.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lvAnnouncements.HeaderStyle = System.Windows.Forms.ColumnHeaderStyle.None;
            this.lvAnnouncements.HideSelection = false;
            this.lvAnnouncements.Location = new System.Drawing.Point(3, 3);
            this.lvAnnouncements.MultiSelect = false;
            this.lvAnnouncements.Name = "lvAnnouncements";
            this.lvAnnouncements.Size = new System.Drawing.Size(370, 194);
            this.lvAnnouncements.TabIndex = 3;
            this.lvAnnouncements.UseCompatibleStateImageBehavior = false;
            this.lvAnnouncements.View = System.Windows.Forms.View.Details;
            this.lvAnnouncements.SelectedIndexChanged += new System.EventHandler(this.lvAnnouncements_SelectedIndexChanged);
            // 
            // lblShowProgress
            // 
            this.lblShowProgress.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblShowProgress.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblShowProgress.Location = new System.Drawing.Point(3, 251);
            this.lblShowProgress.Name = "lblShowProgress";
            this.lblShowProgress.Size = new System.Drawing.Size(370, 17);
            this.lblShowProgress.TabIndex = 2;
            this.lblShowProgress.Text = "Show Now : Show Total";
            this.lblShowProgress.TextAlign = System.Drawing.ContentAlignment.TopCenter;
            // 
            // lblSongProgress
            // 
            this.lblSongProgress.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblSongProgress.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblSongProgress.Location = new System.Drawing.Point(3, 234);
            this.lblSongProgress.Name = "lblSongProgress";
            this.lblSongProgress.Size = new System.Drawing.Size(370, 17);
            this.lblSongProgress.TabIndex = 1;
            this.lblSongProgress.Text = "Song Now : Song Total";
            this.lblSongProgress.TextAlign = System.Drawing.ContentAlignment.TopCenter;
            // 
            // btnDeletePlaylist
            // 
            this.btnDeletePlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnDeletePlaylist.Location = new System.Drawing.Point(97, 624);
            this.btnDeletePlaylist.Name = "btnDeletePlaylist";
            this.btnDeletePlaylist.Size = new System.Drawing.Size(88, 24);
            this.btnDeletePlaylist.TabIndex = 16;
            this.btnDeletePlaylist.Text = "Delete";
            this.btnDeletePlaylist.UseVisualStyleBackColor = true;
            this.btnDeletePlaylist.Click += new System.EventHandler(this.btnDeletePlaylist_Click);
            // 
            // btnNewPlaylist
            // 
            this.btnNewPlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnNewPlaylist.Location = new System.Drawing.Point(3, 624);
            this.btnNewPlaylist.Name = "btnNewPlaylist";
            this.btnNewPlaylist.Size = new System.Drawing.Size(88, 24);
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
            this.lblPlaylistSelection.Size = new System.Drawing.Size(182, 20);
            this.lblPlaylistSelection.TabIndex = 24;
            this.lblPlaylistSelection.Text = "Playlist Selection";
            // 
            // btnAddToPlaylist
            // 
            this.btnAddToPlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnAddToPlaylist.Location = new System.Drawing.Point(473, 624);
            this.btnAddToPlaylist.Name = "btnAddToPlaylist";
            this.btnAddToPlaylist.Size = new System.Drawing.Size(88, 24);
            this.btnAddToPlaylist.TabIndex = 18;
            this.btnAddToPlaylist.Text = "Add -->";
            this.btnAddToPlaylist.UseVisualStyleBackColor = true;
            this.btnAddToPlaylist.Click += new System.EventHandler(this.btnAddToPlaylist_Click);
            // 
            // txtStartPoint
            // 
            this.txtStartPoint.Dock = System.Windows.Forms.DockStyle.Fill;
            this.txtStartPoint.Location = new System.Drawing.Point(755, 326);
            this.txtStartPoint.Name = "txtStartPoint";
            this.txtStartPoint.Size = new System.Drawing.Size(88, 20);
            this.txtStartPoint.TabIndex = 30;
            this.txtStartPoint.Text = "0:00";
            // 
            // lblStartPoint
            // 
            this.lblStartPoint.Dock = System.Windows.Forms.DockStyle.Fill;
            this.lblStartPoint.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblStartPoint.Location = new System.Drawing.Point(661, 323);
            this.lblStartPoint.Name = "lblStartPoint";
            this.lblStartPoint.Size = new System.Drawing.Size(88, 30);
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
            this.tlpLists.SetRowSpan(this.volMeter, 4);
            this.volMeter.Size = new System.Drawing.Size(10, 585);
            this.volMeter.TabIndex = 31;
            // 
            // pnlFountain
            // 
            this.pnlFountain.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.tlpLists.SetColumnSpan(this.pnlFountain, 12);
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
            this.pnlFountain.Location = new System.Drawing.Point(3, 684);
            this.pnlFountain.Name = "pnlFountain";
            this.pnlFountain.Size = new System.Drawing.Size(1002, 274);
            this.pnlFountain.TabIndex = 33;
            // 
            // light41
            // 
            this.light41.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light41.Location = new System.Drawing.Point(572, 17);
            this.light41.Name = "light41";
            this.light41.Size = new System.Drawing.Size(20, 20);
            this.light41.TabIndex = 34;
            this.light41.Tag = "41";
            this.light41.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light40
            // 
            this.light40.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light40.Location = new System.Drawing.Point(540, 17);
            this.light40.Name = "light40";
            this.light40.Size = new System.Drawing.Size(20, 20);
            this.light40.TabIndex = 45;
            this.light40.Tag = "40";
            this.light40.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light39
            // 
            this.light39.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light39.Location = new System.Drawing.Point(508, 17);
            this.light39.Name = "light39";
            this.light39.Size = new System.Drawing.Size(20, 20);
            this.light39.TabIndex = 44;
            this.light39.Tag = "39";
            this.light39.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light38
            // 
            this.light38.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light38.Location = new System.Drawing.Point(474, 17);
            this.light38.Name = "light38";
            this.light38.Size = new System.Drawing.Size(20, 20);
            this.light38.TabIndex = 34;
            this.light38.Tag = "38";
            this.light38.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light37
            // 
            this.light37.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light37.Location = new System.Drawing.Point(442, 17);
            this.light37.Name = "light37";
            this.light37.Size = new System.Drawing.Size(20, 20);
            this.light37.TabIndex = 34;
            this.light37.Tag = "37";
            this.light37.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light36
            // 
            this.light36.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light36.Location = new System.Drawing.Point(411, 17);
            this.light36.Name = "light36";
            this.light36.Size = new System.Drawing.Size(20, 20);
            this.light36.TabIndex = 43;
            this.light36.Tag = "36";
            this.light36.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light43
            // 
            this.light43.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light43.Location = new System.Drawing.Point(514, 74);
            this.light43.Name = "light43";
            this.light43.Size = new System.Drawing.Size(20, 20);
            this.light43.TabIndex = 34;
            this.light43.Tag = "43";
            this.light43.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light42
            // 
            this.light42.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light42.Location = new System.Drawing.Point(467, 74);
            this.light42.Name = "light42";
            this.light42.Size = new System.Drawing.Size(20, 20);
            this.light42.TabIndex = 42;
            this.light42.Tag = "42";
            this.light42.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light33
            // 
            this.light33.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light33.Location = new System.Drawing.Point(889, 130);
            this.light33.Name = "light33";
            this.light33.Size = new System.Drawing.Size(20, 20);
            this.light33.TabIndex = 41;
            this.light33.Tag = "33";
            this.light33.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light28
            // 
            this.light28.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light28.Location = new System.Drawing.Point(761, 130);
            this.light28.Name = "light28";
            this.light28.Size = new System.Drawing.Size(20, 20);
            this.light28.TabIndex = 40;
            this.light28.Tag = "28";
            this.light28.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light23
            // 
            this.light23.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light23.Location = new System.Drawing.Point(632, 130);
            this.light23.Name = "light23";
            this.light23.Size = new System.Drawing.Size(20, 20);
            this.light23.TabIndex = 34;
            this.light23.Tag = "23";
            this.light23.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light18
            // 
            this.light18.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light18.Location = new System.Drawing.Point(504, 130);
            this.light18.Name = "light18";
            this.light18.Size = new System.Drawing.Size(20, 20);
            this.light18.TabIndex = 34;
            this.light18.Tag = "18";
            this.light18.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light13
            // 
            this.light13.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light13.Location = new System.Drawing.Point(376, 130);
            this.light13.Name = "light13";
            this.light13.Size = new System.Drawing.Size(20, 20);
            this.light13.TabIndex = 34;
            this.light13.Tag = "13";
            this.light13.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light8
            // 
            this.light8.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light8.Location = new System.Drawing.Point(248, 130);
            this.light8.Name = "light8";
            this.light8.Size = new System.Drawing.Size(20, 20);
            this.light8.TabIndex = 7;
            this.light8.Tag = "8";
            this.light8.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light3
            // 
            this.light3.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light3.Location = new System.Drawing.Point(120, 130);
            this.light3.Name = "light3";
            this.light3.Size = new System.Drawing.Size(20, 20);
            this.light3.TabIndex = 2;
            this.light3.Tag = "3";
            this.light3.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light20
            // 
            this.light20.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light20.Location = new System.Drawing.Point(540, 219);
            this.light20.Name = "light20";
            this.light20.Size = new System.Drawing.Size(20, 20);
            this.light20.TabIndex = 34;
            this.light20.Tag = "20";
            this.light20.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light17
            // 
            this.light17.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light17.Location = new System.Drawing.Point(514, 219);
            this.light17.Name = "light17";
            this.light17.Size = new System.Drawing.Size(20, 20);
            this.light17.TabIndex = 34;
            this.light17.Tag = "17";
            this.light17.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light16
            // 
            this.light16.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light16.Location = new System.Drawing.Point(466, 219);
            this.light16.Name = "light16";
            this.light16.Size = new System.Drawing.Size(20, 20);
            this.light16.TabIndex = 34;
            this.light16.Tag = "16";
            this.light16.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light19
            // 
            this.light19.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light19.Location = new System.Drawing.Point(442, 219);
            this.light19.Name = "light19";
            this.light19.Size = new System.Drawing.Size(20, 20);
            this.light19.TabIndex = 34;
            this.light19.Tag = "19";
            this.light19.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light32
            // 
            this.light32.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light32.Location = new System.Drawing.Point(900, 219);
            this.light32.Name = "light32";
            this.light32.Size = new System.Drawing.Size(20, 20);
            this.light32.TabIndex = 37;
            this.light32.Tag = "32";
            this.light32.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light27
            // 
            this.light27.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light27.Location = new System.Drawing.Point(771, 219);
            this.light27.Name = "light27";
            this.light27.Size = new System.Drawing.Size(20, 20);
            this.light27.TabIndex = 37;
            this.light27.Tag = "27";
            this.light27.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light31
            // 
            this.light31.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light31.Location = new System.Drawing.Point(852, 219);
            this.light31.Name = "light31";
            this.light31.Size = new System.Drawing.Size(20, 20);
            this.light31.TabIndex = 36;
            this.light31.Tag = "31";
            this.light31.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light22
            // 
            this.light22.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light22.Location = new System.Drawing.Point(643, 219);
            this.light22.Name = "light22";
            this.light22.Size = new System.Drawing.Size(20, 20);
            this.light22.TabIndex = 37;
            this.light22.Tag = "22";
            this.light22.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light26
            // 
            this.light26.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light26.Location = new System.Drawing.Point(724, 219);
            this.light26.Name = "light26";
            this.light26.Size = new System.Drawing.Size(20, 20);
            this.light26.TabIndex = 36;
            this.light26.Tag = "26";
            this.light26.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light12
            // 
            this.light12.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light12.Location = new System.Drawing.Point(388, 219);
            this.light12.Name = "light12";
            this.light12.Size = new System.Drawing.Size(20, 20);
            this.light12.TabIndex = 37;
            this.light12.Tag = "12";
            this.light12.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light21
            // 
            this.light21.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light21.Location = new System.Drawing.Point(595, 219);
            this.light21.Name = "light21";
            this.light21.Size = new System.Drawing.Size(20, 20);
            this.light21.TabIndex = 36;
            this.light21.Tag = "21";
            this.light21.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light7
            // 
            this.light7.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light7.Location = new System.Drawing.Point(258, 219);
            this.light7.Name = "light7";
            this.light7.Size = new System.Drawing.Size(20, 20);
            this.light7.TabIndex = 6;
            this.light7.Tag = "7";
            this.light7.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light11
            // 
            this.light11.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light11.Location = new System.Drawing.Point(338, 219);
            this.light11.Name = "light11";
            this.light11.Size = new System.Drawing.Size(20, 20);
            this.light11.TabIndex = 10;
            this.light11.Tag = "11";
            this.light11.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light2
            // 
            this.light2.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light2.Location = new System.Drawing.Point(130, 219);
            this.light2.Name = "light2";
            this.light2.Size = new System.Drawing.Size(20, 20);
            this.light2.TabIndex = 1;
            this.light2.Tag = "2";
            this.light2.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light6
            // 
            this.light6.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light6.Location = new System.Drawing.Point(211, 219);
            this.light6.Name = "light6";
            this.light6.Size = new System.Drawing.Size(20, 20);
            this.light6.TabIndex = 5;
            this.light6.Tag = "6";
            this.light6.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light1
            // 
            this.light1.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light1.Location = new System.Drawing.Point(80, 219);
            this.light1.Name = "light1";
            this.light1.Size = new System.Drawing.Size(20, 20);
            this.light1.TabIndex = 0;
            this.light1.Tag = "1";
            this.light1.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light35
            // 
            this.light35.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light35.Location = new System.Drawing.Point(925, 42);
            this.light35.Name = "light35";
            this.light35.Size = new System.Drawing.Size(20, 20);
            this.light35.TabIndex = 35;
            this.light35.Tag = "35";
            this.light35.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light30
            // 
            this.light30.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light30.Location = new System.Drawing.Point(795, 42);
            this.light30.Name = "light30";
            this.light30.Size = new System.Drawing.Size(20, 20);
            this.light30.TabIndex = 35;
            this.light30.Tag = "30";
            this.light30.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light34
            // 
            this.light34.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light34.Location = new System.Drawing.Point(829, 42);
            this.light34.Name = "light34";
            this.light34.Size = new System.Drawing.Size(20, 20);
            this.light34.TabIndex = 36;
            this.light34.Tag = "34";
            this.light34.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light25
            // 
            this.light25.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light25.Location = new System.Drawing.Point(668, 42);
            this.light25.Name = "light25";
            this.light25.Size = new System.Drawing.Size(20, 20);
            this.light25.TabIndex = 35;
            this.light25.Tag = "25";
            this.light25.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light29
            // 
            this.light29.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light29.Location = new System.Drawing.Point(699, 42);
            this.light29.Name = "light29";
            this.light29.Size = new System.Drawing.Size(20, 20);
            this.light29.TabIndex = 36;
            this.light29.Tag = "29";
            this.light29.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light15
            // 
            this.light15.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light15.Location = new System.Drawing.Point(411, 42);
            this.light15.Name = "light15";
            this.light15.Size = new System.Drawing.Size(20, 20);
            this.light15.TabIndex = 35;
            this.light15.Tag = "15";
            this.light15.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light24
            // 
            this.light24.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light24.Location = new System.Drawing.Point(572, 42);
            this.light24.Name = "light24";
            this.light24.Size = new System.Drawing.Size(20, 20);
            this.light24.TabIndex = 36;
            this.light24.Tag = "24";
            this.light24.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light10
            // 
            this.light10.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light10.Location = new System.Drawing.Point(282, 42);
            this.light10.Name = "light10";
            this.light10.Size = new System.Drawing.Size(20, 20);
            this.light10.TabIndex = 9;
            this.light10.Tag = "10";
            this.light10.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light14
            // 
            this.light14.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light14.Location = new System.Drawing.Point(315, 42);
            this.light14.Name = "light14";
            this.light14.Size = new System.Drawing.Size(20, 20);
            this.light14.TabIndex = 36;
            this.light14.Tag = "14";
            this.light14.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light9
            // 
            this.light9.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light9.Location = new System.Drawing.Point(186, 42);
            this.light9.Name = "light9";
            this.light9.Size = new System.Drawing.Size(20, 20);
            this.light9.TabIndex = 8;
            this.light9.Tag = "9";
            this.light9.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light5
            // 
            this.light5.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light5.Location = new System.Drawing.Point(154, 42);
            this.light5.Name = "light5";
            this.light5.Size = new System.Drawing.Size(20, 20);
            this.light5.TabIndex = 4;
            this.light5.Tag = "5";
            this.light5.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // light4
            // 
            this.light4.Anchor = System.Windows.Forms.AnchorStyles.None;
            this.light4.Location = new System.Drawing.Point(57, 42);
            this.light4.Name = "light4";
            this.light4.Size = new System.Drawing.Size(20, 20);
            this.light4.TabIndex = 3;
            this.light4.Tag = "4";
            this.light4.Paint += new System.Windows.Forms.PaintEventHandler(this.light_Paint);
            // 
            // pbFountain
            // 
            this.pbFountain.Anchor = ((System.Windows.Forms.AnchorStyles)((((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom) 
            | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.pbFountain.Image = ((System.Drawing.Image)(resources.GetObject("pbFountain.Image")));
            this.pbFountain.Location = new System.Drawing.Point(0, 3);
            this.pbFountain.Name = "pbFountain";
            this.pbFountain.Size = new System.Drawing.Size(1002, 271);
            this.pbFountain.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.pbFountain.TabIndex = 32;
            this.pbFountain.TabStop = false;
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
            this.MinimumSize = new System.Drawing.Size(1024, 1000);
            this.Name = "PlaybackForm";
            this.StartPosition = System.Windows.Forms.FormStartPosition.Manual;
            this.Text = "Grand Haven Musical Fountain Playback Control";
            this.FormClosing += new System.Windows.Forms.FormClosingEventHandler(this.PlaybackForm_FormClosing);
            this.Resize += new System.EventHandler(this.PlaybackForm_Resize);
            ((System.ComponentModel.ISupportInitialize)(this.barVolLeftChannel)).EndInit();
            this.tlpLists.ResumeLayout(false);
            this.tlpLists.PerformLayout();
            this.pnlPlaceholder.ResumeLayout(false);
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
        private System.Windows.Forms.Panel pnlPlaceholder;
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
        private System.Windows.Forms.Panel light4;
        private System.Windows.Forms.Panel light10;
        private System.Windows.Forms.Panel light9;
        private System.Windows.Forms.Panel light5;
        private System.Windows.Forms.Panel light3;
        private System.Windows.Forms.Panel light20;
        private System.Windows.Forms.Panel light17;
        private System.Windows.Forms.Panel light16;
        private System.Windows.Forms.Panel light19;
        private System.Windows.Forms.Panel light32;
        private System.Windows.Forms.Panel light27;
        private System.Windows.Forms.Panel light31;
        private System.Windows.Forms.Panel light22;
        private System.Windows.Forms.Panel light26;
        private System.Windows.Forms.Panel light12;
        private System.Windows.Forms.Panel light21;
        private System.Windows.Forms.Panel light7;
        private System.Windows.Forms.Panel light11;
        private System.Windows.Forms.Panel light2;
        private System.Windows.Forms.Panel light6;
        private System.Windows.Forms.Panel light1;
        private System.Windows.Forms.Panel light35;
        private System.Windows.Forms.Panel light30;
        private System.Windows.Forms.Panel light34;
        private System.Windows.Forms.Panel light25;
        private System.Windows.Forms.Panel light29;
        private System.Windows.Forms.Panel light15;
        private System.Windows.Forms.Panel light24;
        private System.Windows.Forms.Panel light14;
        private System.Windows.Forms.Panel light41;
        private System.Windows.Forms.Panel light40;
        private System.Windows.Forms.Panel light39;
        private System.Windows.Forms.Panel light38;
        private System.Windows.Forms.Panel light37;
        private System.Windows.Forms.Panel light36;
        private System.Windows.Forms.Panel light43;
        private System.Windows.Forms.Panel light42;
        private System.Windows.Forms.Panel light33;
        private System.Windows.Forms.Panel light28;
        private System.Windows.Forms.Panel light23;
        private System.Windows.Forms.Panel light18;
        private System.Windows.Forms.Panel light13;
        private System.Windows.Forms.Panel light8;
    }
}

