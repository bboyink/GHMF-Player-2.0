namespace Playback
{
    partial class Monitor
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
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(Monitor));
            this.lblWaterFCWs = new System.Windows.Forms.Label();
            this.lblCurrentWaterFCW = new System.Windows.Forms.Label();
            this.lblLights = new System.Windows.Forms.Label();
            this.lvLightColors = new System.Windows.Forms.ListView();
            this.lblMotion = new System.Windows.Forms.Label();
            this.lblCurrentMotion = new System.Windows.Forms.Label();
            this.lblLightFCW = new System.Windows.Forms.Label();
            this.lblCurrentLightFCW = new System.Windows.Forms.Label();
            this.lblWaterFCWCount = new System.Windows.Forms.Label();
            this.lblTotalFCWCount = new System.Windows.Forms.Label();
            this.lblLightFCWCount = new System.Windows.Forms.Label();
            this.SuspendLayout();
            // 
            // lblWaterFCWs
            // 
            this.lblWaterFCWs.AutoSize = true;
            this.lblWaterFCWs.Location = new System.Drawing.Point(12, 9);
            this.lblWaterFCWs.Name = "lblWaterFCWs";
            this.lblWaterFCWs.Size = new System.Drawing.Size(114, 13);
            this.lblWaterFCWs.TabIndex = 0;
            this.lblWaterFCWs.Text = "Current Water FCW(s):";
            // 
            // lblCurrentWaterFCW
            // 
            this.lblCurrentWaterFCW.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblCurrentWaterFCW.Location = new System.Drawing.Point(132, 9);
            this.lblCurrentWaterFCW.Name = "lblCurrentWaterFCW";
            this.lblCurrentWaterFCW.Size = new System.Drawing.Size(390, 35);
            this.lblCurrentWaterFCW.TabIndex = 2;
            // 
            // lblLights
            // 
            this.lblLights.Anchor = ((System.Windows.Forms.AnchorStyles)((((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom) 
            | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblLights.AutoSize = true;
            this.lblLights.Location = new System.Drawing.Point(12, 153);
            this.lblLights.Name = "lblLights";
            this.lblLights.Size = new System.Drawing.Size(38, 13);
            this.lblLights.TabIndex = 9;
            this.lblLights.Text = "Lights:";
            // 
            // lvLightColors
            // 
            this.lvLightColors.Anchor = ((System.Windows.Forms.AnchorStyles)((((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom) 
            | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lvLightColors.Location = new System.Drawing.Point(12, 169);
            this.lvLightColors.Name = "lvLightColors";
            this.lvLightColors.Size = new System.Drawing.Size(510, 331);
            this.lvLightColors.TabIndex = 10;
            this.lvLightColors.UseCompatibleStateImageBehavior = false;
            this.lvLightColors.View = System.Windows.Forms.View.SmallIcon;
            // 
            // lblMotion
            // 
            this.lblMotion.AutoSize = true;
            this.lblMotion.Location = new System.Drawing.Point(12, 105);
            this.lblMotion.Name = "lblMotion";
            this.lblMotion.Size = new System.Drawing.Size(79, 13);
            this.lblMotion.TabIndex = 6;
            this.lblMotion.Text = "Current Motion:";
            // 
            // lblCurrentMotion
            // 
            this.lblCurrentMotion.AutoSize = true;
            this.lblCurrentMotion.Location = new System.Drawing.Point(132, 105);
            this.lblCurrentMotion.Name = "lblCurrentMotion";
            this.lblCurrentMotion.Size = new System.Drawing.Size(33, 13);
            this.lblCurrentMotion.TabIndex = 8;
            this.lblCurrentMotion.Text = "None";
            // 
            // lblLightFCW
            // 
            this.lblLightFCW.AutoSize = true;
            this.lblLightFCW.Location = new System.Drawing.Point(12, 57);
            this.lblLightFCW.Name = "lblLightFCW";
            this.lblLightFCW.Size = new System.Drawing.Size(108, 13);
            this.lblLightFCW.TabIndex = 3;
            this.lblLightFCW.Text = "Current Light FCW(s):";
            // 
            // lblCurrentLightFCW
            // 
            this.lblCurrentLightFCW.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblCurrentLightFCW.Location = new System.Drawing.Point(132, 57);
            this.lblCurrentLightFCW.Name = "lblCurrentLightFCW";
            this.lblCurrentLightFCW.Size = new System.Drawing.Size(390, 35);
            this.lblCurrentLightFCW.TabIndex = 5;
            // 
            // lblWaterFCWCount
            // 
            this.lblWaterFCWCount.AutoSize = true;
            this.lblWaterFCWCount.Location = new System.Drawing.Point(12, 22);
            this.lblWaterFCWCount.Name = "lblWaterFCWCount";
            this.lblWaterFCWCount.Size = new System.Drawing.Size(88, 26);
            this.lblWaterFCWCount.TabIndex = 1;
            this.lblWaterFCWCount.Text = "0/0\r\nwater FCWs sent";
            // 
            // lblTotalFCWCount
            // 
            this.lblTotalFCWCount.AutoSize = true;
            this.lblTotalFCWCount.Location = new System.Drawing.Point(12, 118);
            this.lblTotalFCWCount.Name = "lblTotalFCWCount";
            this.lblTotalFCWCount.Size = new System.Drawing.Size(106, 26);
            this.lblTotalFCWCount.TabIndex = 7;
            this.lblTotalFCWCount.Text = "0/0\r\ntotal FCWs executed";
            // 
            // lblLightFCWCount
            // 
            this.lblLightFCWCount.AutoSize = true;
            this.lblLightFCWCount.Location = new System.Drawing.Point(12, 70);
            this.lblLightFCWCount.Name = "lblLightFCWCount";
            this.lblLightFCWCount.Size = new System.Drawing.Size(105, 26);
            this.lblLightFCWCount.TabIndex = 4;
            this.lblLightFCWCount.Text = "0/0\r\nlight FCWs executed";
            // 
            // Monitor
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 13F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(534, 512);
            this.Controls.Add(this.lblLightFCWCount);
            this.Controls.Add(this.lblTotalFCWCount);
            this.Controls.Add(this.lblWaterFCWCount);
            this.Controls.Add(this.lblCurrentLightFCW);
            this.Controls.Add(this.lblLightFCW);
            this.Controls.Add(this.lblCurrentMotion);
            this.Controls.Add(this.lblMotion);
            this.Controls.Add(this.lvLightColors);
            this.Controls.Add(this.lblLights);
            this.Controls.Add(this.lblCurrentWaterFCW);
            this.Controls.Add(this.lblWaterFCWs);
            this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
            this.MinimumSize = new System.Drawing.Size(300, 200);
            this.Name = "Monitor";
            this.StartPosition = System.Windows.Forms.FormStartPosition.Manual;
            this.Text = "Monitor";
            this.FormClosing += new System.Windows.Forms.FormClosingEventHandler(this.Monitor_FormClosing);
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.Label lblWaterFCWs;
        private System.Windows.Forms.Label lblCurrentWaterFCW;
        private System.Windows.Forms.Label lblLights;
        private System.Windows.Forms.ListView lvLightColors;
        private System.Windows.Forms.Label lblMotion;
        private System.Windows.Forms.Label lblCurrentMotion;
        private System.Windows.Forms.Label lblLightFCW;
        private System.Windows.Forms.Label lblCurrentLightFCW;
        private System.Windows.Forms.Label lblWaterFCWCount;
        private System.Windows.Forms.Label lblTotalFCWCount;
        private System.Windows.Forms.Label lblLightFCWCount;
    }
}