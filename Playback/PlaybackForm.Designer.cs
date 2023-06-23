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
            this.btnMonitor = new System.Windows.Forms.Button();
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
            this.tlpLists.SetColumnSpan(this.btnStartStop, 2);
            this.btnStartStop.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnStartStop.Location = new System.Drawing.Point(459, 216);
            this.btnStartStop.Name = "btnStartStop";
            this.btnStartStop.Size = new System.Drawing.Size(107, 24);
            this.btnStartStop.TabIndex = 0;
            this.btnStartStop.Text = "Start Show";
            this.btnStartStop.UseVisualStyleBackColor = true;
            this.btnStartStop.Click += new System.EventHandler(this.btnStartStop_Click);
            // 
            // btnPauseResume
            // 
            this.tlpLists.SetColumnSpan(this.btnPauseResume, 2);
            this.btnPauseResume.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnPauseResume.Location = new System.Drawing.Point(572, 216);
            this.btnPauseResume.Name = "btnPauseResume";
            this.btnPauseResume.Size = new System.Drawing.Size(107, 24);
            this.btnPauseResume.TabIndex = 1;
            this.btnPauseResume.Text = "Pause Show";
            this.btnPauseResume.UseVisualStyleBackColor = true;
            this.btnPauseResume.Click += new System.EventHandler(this.btnPauseResume_Click);
            // 
            // barVolLeftChannel
            // 
            this.barVolLeftChannel.Dock = System.Windows.Forms.DockStyle.Fill;
            this.barVolLeftChannel.LargeChange = 0;
            this.barVolLeftChannel.Location = new System.Drawing.Point(701, 33);
            this.barVolLeftChannel.Maximum = 100;
            this.barVolLeftChannel.Name = "barVolLeftChannel";
            this.barVolLeftChannel.Orientation = System.Windows.Forms.Orientation.Vertical;
            this.tlpLists.SetRowSpan(this.barVolLeftChannel, 4);
            this.barVolLeftChannel.Size = new System.Drawing.Size(180, 365);
            this.barVolLeftChannel.TabIndex = 7;
            this.barVolLeftChannel.TickFrequency = 5;
            this.barVolLeftChannel.ValueChanged += new System.EventHandler(this.barVolume_ValueChanged);
            // 
            // btnSettings
            // 
            this.btnSettings.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnSettings.Location = new System.Drawing.Point(155, 434);
            this.btnSettings.Name = "btnSettings";
            this.btnSettings.Size = new System.Drawing.Size(70, 24);
            this.btnSettings.TabIndex = 11;
            this.btnSettings.Text = "Settings";
            this.btnSettings.UseVisualStyleBackColor = true;
            this.btnSettings.Click += new System.EventHandler(this.btnSettings_Click);
            // 
            // btnLogin
            // 
            this.btnLogin.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnLogin.Location = new System.Drawing.Point(3, 434);
            this.btnLogin.Name = "btnLogin";
            this.btnLogin.Size = new System.Drawing.Size(70, 24);
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
            this.lvPlaylists.Size = new System.Drawing.Size(222, 340);
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
            this.lvSongs.Location = new System.Drawing.Point(231, 58);
            this.lvSongs.MultiSelect = false;
            this.lvSongs.Name = "lvSongs";
            this.tlpLists.SetRowSpan(this.lvSongs, 3);
            this.lvSongs.Size = new System.Drawing.Size(222, 340);
            this.lvSongs.TabIndex = 3;
            this.lvSongs.UseCompatibleStateImageBehavior = false;
            this.lvSongs.View = System.Windows.Forms.View.Details;
            this.lvSongs.DoubleClick += new System.EventHandler(this.lvSongs_DoubleClick);
            // 
            // btnConnectDMX
            // 
            this.btnConnectDMX.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnConnectDMX.Location = new System.Drawing.Point(609, 434);
            this.btnConnectDMX.Name = "btnConnectDMX";
            this.btnConnectDMX.Size = new System.Drawing.Size(70, 24);
            this.btnConnectDMX.TabIndex = 14;
            this.btnConnectDMX.Text = "Connect DMX";
            this.btnConnectDMX.UseVisualStyleBackColor = true;
            this.btnConnectDMX.Click += new System.EventHandler(this.btnConnectDMX_Click);
            // 
            // txtManualFCW
            // 
            this.tlpLists.SetColumnSpan(this.txtManualFCW, 2);
            this.txtManualFCW.Dock = System.Windows.Forms.DockStyle.Fill;
            this.txtManualFCW.Location = new System.Drawing.Point(231, 434);
            this.txtManualFCW.Name = "txtManualFCW";
            this.txtManualFCW.Size = new System.Drawing.Size(146, 20);
            this.txtManualFCW.TabIndex = 12;
            this.txtManualFCW.KeyPress += new System.Windows.Forms.KeyPressEventHandler(this.txtManualFCW_KeyPress);
            // 
            // btnExecuteManualFCW
            // 
            this.btnExecuteManualFCW.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnExecuteManualFCW.Location = new System.Drawing.Point(383, 434);
            this.btnExecuteManualFCW.Name = "btnExecuteManualFCW";
            this.btnExecuteManualFCW.Size = new System.Drawing.Size(70, 24);
            this.btnExecuteManualFCW.TabIndex = 13;
            this.btnExecuteManualFCW.Text = "Execute";
            this.btnExecuteManualFCW.UseVisualStyleBackColor = true;
            this.btnExecuteManualFCW.Click += new System.EventHandler(this.btnExecuteManualFCW_Click);
            // 
            // tlpLists
            // 
            this.tlpLists.ColumnCount = 12;
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 11.11234F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 11.11235F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 11.11235F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 11.11235F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 11.11235F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 11.11235F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 11.11235F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 5.550613F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 5.550613F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 11.11235F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Absolute, 16F));
            this.tlpLists.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Absolute, 184F));
            this.tlpLists.Controls.Add(this.lblCurrentSongFolder, 3, 1);
            this.tlpLists.Controls.Add(this.lblCurrentPlaylistFolder, 0, 1);
            this.tlpLists.Controls.Add(this.btnMonitor, 1, 6);
            this.tlpLists.Controls.Add(this.btnExecuteManualFCW, 5, 6);
            this.tlpLists.Controls.Add(this.btnPauseResume, 8, 3);
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
            this.tlpLists.Controls.Add(this.btnMoveUp, 7, 5);
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
            this.tlpLists.Controls.Add(this.txtStartPoint, 9, 1);
            this.tlpLists.Controls.Add(this.lblStartPoint, 7, 1);
            this.tlpLists.Controls.Add(this.volMeter, 10, 1);
            this.tlpLists.Dock = System.Windows.Forms.DockStyle.Fill;
            this.tlpLists.Location = new System.Drawing.Point(0, 0);
            this.tlpLists.Margin = new System.Windows.Forms.Padding(0);
            this.tlpLists.Name = "tlpLists";
            this.tlpLists.RowCount = 7;
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 30F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 25F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 50F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 30F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 50F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 30F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 30F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.tlpLists.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.tlpLists.Size = new System.Drawing.Size(884, 461);
            this.tlpLists.TabIndex = 0;
            // 
            // lblCurrentSongFolder
            // 
            this.lblCurrentSongFolder.AutoEllipsis = true;
            this.tlpLists.SetColumnSpan(this.lblCurrentSongFolder, 3);
            this.lblCurrentSongFolder.Dock = System.Windows.Forms.DockStyle.Bottom;
            this.lblCurrentSongFolder.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblCurrentSongFolder.Location = new System.Drawing.Point(231, 35);
            this.lblCurrentSongFolder.Name = "lblCurrentSongFolder";
            this.lblCurrentSongFolder.Size = new System.Drawing.Size(222, 20);
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
            this.lblCurrentPlaylistFolder.Size = new System.Drawing.Size(222, 20);
            this.lblCurrentPlaylistFolder.TabIndex = 27;
            // 
            // btnMonitor
            // 
            this.btnMonitor.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnMonitor.Enabled = false;
            this.btnMonitor.Location = new System.Drawing.Point(79, 434);
            this.btnMonitor.Name = "btnMonitor";
            this.btnMonitor.Size = new System.Drawing.Size(70, 24);
            this.btnMonitor.TabIndex = 10;
            this.btnMonitor.Text = "Monitor";
            this.btnMonitor.UseVisualStyleBackColor = true;
            this.btnMonitor.Click += new System.EventHandler(this.btnMonitor_Click);
            // 
            // lblVolume
            // 
            this.lblVolume.AutoSize = true;
            this.tlpLists.SetColumnSpan(this.lblVolume, 2);
            this.lblVolume.Dock = System.Windows.Forms.DockStyle.Bottom;
            this.lblVolume.Font = new System.Drawing.Font("Microsoft Sans Serif", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblVolume.Location = new System.Drawing.Point(685, 15);
            this.lblVolume.Name = "lblVolume";
            this.lblVolume.Size = new System.Drawing.Size(196, 15);
            this.lblVolume.TabIndex = 26;
            this.lblVolume.Text = "Audience Volume";
            this.lblVolume.TextAlign = System.Drawing.ContentAlignment.BottomCenter;
            // 
            // btnMoveDown
            // 
            this.btnMoveDown.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnMoveDown.Location = new System.Drawing.Point(609, 404);
            this.btnMoveDown.Name = "btnMoveDown";
            this.btnMoveDown.Size = new System.Drawing.Size(70, 24);
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
            this.btnAnnounce.Location = new System.Drawing.Point(459, 3);
            this.btnAnnounce.Name = "btnAnnounce";
            this.btnAnnounce.Size = new System.Drawing.Size(220, 24);
            this.btnAnnounce.TabIndex = 6;
            this.btnAnnounce.Text = "PLAY ANNOUNCEMENT";
            this.btnAnnounce.UseVisualStyleBackColor = true;
            this.btnAnnounce.EnabledChanged += new System.EventHandler(this.btnAnnounce_EnabledChanged);
            this.btnAnnounce.Click += new System.EventHandler(this.btnAnnounce_Click);
            // 
            // btnBrowseSong
            // 
            this.btnBrowseSong.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnBrowseSong.Location = new System.Drawing.Point(383, 3);
            this.btnBrowseSong.Name = "btnBrowseSong";
            this.btnBrowseSong.Size = new System.Drawing.Size(70, 24);
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
            this.lblSongSelection.Location = new System.Drawing.Point(231, 10);
            this.lblSongSelection.Name = "lblSongSelection";
            this.lblSongSelection.Size = new System.Drawing.Size(146, 20);
            this.lblSongSelection.TabIndex = 25;
            this.lblSongSelection.Text = "Song Selection";
            // 
            // btnRemoveFromPlaylist
            // 
            this.btnRemoveFromPlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnRemoveFromPlaylist.Location = new System.Drawing.Point(459, 404);
            this.btnRemoveFromPlaylist.Name = "btnRemoveFromPlaylist";
            this.btnRemoveFromPlaylist.Size = new System.Drawing.Size(70, 24);
            this.btnRemoveFromPlaylist.TabIndex = 19;
            this.btnRemoveFromPlaylist.Text = "<-- Remove";
            this.btnRemoveFromPlaylist.UseVisualStyleBackColor = true;
            this.btnRemoveFromPlaylist.Click += new System.EventHandler(this.btnRemoveFromPlaylist_Click);
            // 
            // btnMoveUp
            // 
            this.tlpLists.SetColumnSpan(this.btnMoveUp, 2);
            this.btnMoveUp.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnMoveUp.Location = new System.Drawing.Point(535, 404);
            this.btnMoveUp.Name = "btnMoveUp";
            this.btnMoveUp.Size = new System.Drawing.Size(68, 24);
            this.btnMoveUp.TabIndex = 20;
            this.btnMoveUp.Text = "Move Up";
            this.btnMoveUp.UseVisualStyleBackColor = true;
            this.btnMoveUp.Click += new System.EventHandler(this.btnMoveUp_Click);
            // 
            // btnBrowsePlaylist
            // 
            this.btnBrowsePlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnBrowsePlaylist.Location = new System.Drawing.Point(155, 3);
            this.btnBrowsePlaylist.Name = "btnBrowsePlaylist";
            this.btnBrowsePlaylist.Size = new System.Drawing.Size(70, 24);
            this.btnBrowsePlaylist.TabIndex = 22;
            this.btnBrowsePlaylist.Text = "Browse...";
            this.btnBrowsePlaylist.UseVisualStyleBackColor = true;
            this.btnBrowsePlaylist.Click += new System.EventHandler(this.btnBrowsePlaylist_Click);
            // 
            // btnRenamePlaylist
            // 
            this.btnRenamePlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnRenamePlaylist.Location = new System.Drawing.Point(155, 404);
            this.btnRenamePlaylist.Name = "btnRenamePlaylist";
            this.btnRenamePlaylist.Size = new System.Drawing.Size(70, 24);
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
            this.lvCurPlaylistSongs.Location = new System.Drawing.Point(459, 246);
            this.lvCurPlaylistSongs.MultiSelect = false;
            this.lvCurPlaylistSongs.Name = "lvCurPlaylistSongs";
            this.lvCurPlaylistSongs.Size = new System.Drawing.Size(220, 152);
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
            this.pnlPlaceholder.Location = new System.Drawing.Point(456, 55);
            this.pnlPlaceholder.Margin = new System.Windows.Forms.Padding(0);
            this.pnlPlaceholder.Name = "pnlPlaceholder";
            this.pnlPlaceholder.Size = new System.Drawing.Size(226, 158);
            this.pnlPlaceholder.TabIndex = 5;
            // 
            // lblCurrentShow
            // 
            this.lblCurrentShow.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblCurrentShow.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblCurrentShow.Location = new System.Drawing.Point(3, 89);
            this.lblCurrentShow.Name = "lblCurrentShow";
            this.lblCurrentShow.Size = new System.Drawing.Size(220, 17);
            this.lblCurrentShow.TabIndex = 4;
            this.lblCurrentShow.Text = "Now Playing Show: ";
            this.lblCurrentShow.TextAlign = System.Drawing.ContentAlignment.TopCenter;
            // 
            // lblCurrentSong
            // 
            this.lblCurrentSong.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblCurrentSong.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblCurrentSong.Location = new System.Drawing.Point(3, 106);
            this.lblCurrentSong.Name = "lblCurrentSong";
            this.lblCurrentSong.Size = new System.Drawing.Size(220, 17);
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
            this.lvAnnouncements.Size = new System.Drawing.Size(220, 83);
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
            this.lblShowProgress.Location = new System.Drawing.Point(3, 140);
            this.lblShowProgress.Name = "lblShowProgress";
            this.lblShowProgress.Size = new System.Drawing.Size(220, 17);
            this.lblShowProgress.TabIndex = 2;
            this.lblShowProgress.Text = "Show Now : Show Total";
            this.lblShowProgress.TextAlign = System.Drawing.ContentAlignment.TopCenter;
            // 
            // lblSongProgress
            // 
            this.lblSongProgress.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblSongProgress.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblSongProgress.Location = new System.Drawing.Point(3, 123);
            this.lblSongProgress.Name = "lblSongProgress";
            this.lblSongProgress.Size = new System.Drawing.Size(220, 17);
            this.lblSongProgress.TabIndex = 1;
            this.lblSongProgress.Text = "Song Now : Song Total";
            this.lblSongProgress.TextAlign = System.Drawing.ContentAlignment.TopCenter;
            // 
            // btnDeletePlaylist
            // 
            this.btnDeletePlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnDeletePlaylist.Location = new System.Drawing.Point(79, 404);
            this.btnDeletePlaylist.Name = "btnDeletePlaylist";
            this.btnDeletePlaylist.Size = new System.Drawing.Size(70, 24);
            this.btnDeletePlaylist.TabIndex = 16;
            this.btnDeletePlaylist.Text = "Delete";
            this.btnDeletePlaylist.UseVisualStyleBackColor = true;
            this.btnDeletePlaylist.Click += new System.EventHandler(this.btnDeletePlaylist_Click);
            // 
            // btnNewPlaylist
            // 
            this.btnNewPlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnNewPlaylist.Location = new System.Drawing.Point(3, 404);
            this.btnNewPlaylist.Name = "btnNewPlaylist";
            this.btnNewPlaylist.Size = new System.Drawing.Size(70, 24);
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
            this.lblPlaylistSelection.Size = new System.Drawing.Size(146, 20);
            this.lblPlaylistSelection.TabIndex = 24;
            this.lblPlaylistSelection.Text = "Playlist Selection";
            // 
            // btnAddToPlaylist
            // 
            this.btnAddToPlaylist.Dock = System.Windows.Forms.DockStyle.Fill;
            this.btnAddToPlaylist.Location = new System.Drawing.Point(383, 404);
            this.btnAddToPlaylist.Name = "btnAddToPlaylist";
            this.btnAddToPlaylist.Size = new System.Drawing.Size(70, 24);
            this.btnAddToPlaylist.TabIndex = 18;
            this.btnAddToPlaylist.Text = "Add -->";
            this.btnAddToPlaylist.UseVisualStyleBackColor = true;
            this.btnAddToPlaylist.Click += new System.EventHandler(this.btnAddToPlaylist_Click);
            // 
            // txtStartPoint
            // 
            this.txtStartPoint.Dock = System.Windows.Forms.DockStyle.Fill;
            this.txtStartPoint.Location = new System.Drawing.Point(609, 33);
            this.txtStartPoint.Name = "txtStartPoint";
            this.txtStartPoint.Size = new System.Drawing.Size(70, 20);
            this.txtStartPoint.TabIndex = 30;
            // 
            // lblStartPoint
            // 
            this.tlpLists.SetColumnSpan(this.lblStartPoint, 2);
            this.lblStartPoint.Dock = System.Windows.Forms.DockStyle.Fill;
            this.lblStartPoint.Font = new System.Drawing.Font("Microsoft Sans Serif", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblStartPoint.Location = new System.Drawing.Point(535, 30);
            this.lblStartPoint.Name = "lblStartPoint";
            this.lblStartPoint.Size = new System.Drawing.Size(68, 25);
            this.lblStartPoint.TabIndex = 29;
            this.lblStartPoint.Text = "Start at:";
            this.lblStartPoint.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // volMeter
            // 
            this.volMeter.Amplitude = 0F;
            this.volMeter.Dock = System.Windows.Forms.DockStyle.Fill;
            this.volMeter.ForeColor = System.Drawing.Color.Lime;
            this.volMeter.Location = new System.Drawing.Point(685, 33);
            this.volMeter.MaxDb = 18F;
            this.volMeter.MinDb = -60F;
            this.volMeter.Name = "volMeter";
            this.tlpLists.SetRowSpan(this.volMeter, 4);
            this.volMeter.Size = new System.Drawing.Size(10, 365);
            this.volMeter.TabIndex = 31;
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
            this.ClientSize = new System.Drawing.Size(884, 461);
            this.Controls.Add(this.tlpLists);
            this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
            this.KeyPreview = true;
            this.MinimumSize = new System.Drawing.Size(896, 490);
            this.Name = "PlaybackForm";
            this.StartPosition = System.Windows.Forms.FormStartPosition.Manual;
            this.Text = "Grand Haven Musical Fountain Playback Control";
            this.FormClosing += new System.Windows.Forms.FormClosingEventHandler(this.PlaybackForm_FormClosing);
            this.Resize += new System.EventHandler(this.PlaybackForm_Resize);
            ((System.ComponentModel.ISupportInitialize)(this.barVolLeftChannel)).EndInit();
            this.tlpLists.ResumeLayout(false);
            this.tlpLists.PerformLayout();
            this.pnlPlaceholder.ResumeLayout(false);
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
        private System.Windows.Forms.Button btnMonitor;
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
    }
}

