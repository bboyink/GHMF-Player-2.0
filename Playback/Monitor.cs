using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using System.Windows.Forms;

namespace Playback
{
    public partial class Monitor : Form
    {
        public bool Ready = false;

        private Dictionary<int, ListViewItem> lightIndexToItemMap = new Dictionary<int, ListViewItem>();

        public Monitor()
        {
            InitializeComponent();

            lvLightColors.SmallImageList = new ImageList
            {
                ColorDepth = ColorDepth.Depth32Bit
            };
        }

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            Ready = false;

            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        public void Initialize(LightModule[] lightModules, string[] lightNames)
        {
            if (InvokeRequired)
            {
                try
                {
                    BeginInvoke(new Action(() => Initialize(lightModules, lightNames)));
                }
                catch (ObjectDisposedException) { } // This means we're closing anyways, so no need to update the UI anymore
                return;
            }

            lvLightColors.Items.Clear();
            lightIndexToItemMap.Clear();

            List<int> usedLights = new List<int>();
            List<ListViewGroup> groups = new List<ListViewGroup>();
            foreach (LightModule module in lightModules.Reverse())
            {
                groups.Add(new ListViewGroup(module.Name, module.Name));
                foreach (int light in module.LightIndices)
                {
                    if (!usedLights.Contains(light))
                    {
                        usedLights.Add(light);
                        ListViewItem newItem = new ListViewItem(
                            System.Text.RegularExpressions.Regex.Replace(
                            System.Text.RegularExpressions.Regex.Replace(lightNames[light], @"MOD.*?\d", "", System.Text.RegularExpressions.RegexOptions.IgnoreCase),
                            @"PEACOCK *", "", System.Text.RegularExpressions.RegexOptions.IgnoreCase),
                            0, groups[groups.Count - 1]);
                        lightIndexToItemMap.Add(light, newItem);
                    }
                }
            }

            groups.Reverse();
            lvLightColors.Groups.AddRange(groups.ToArray());
            foreach (ListViewGroup group in lvLightColors.Groups)
            {
                foreach (ListViewItem item in group.Items)
                {
                    lvLightColors.Items.Add(item);
                }
            }
            
            Ready = true;
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
                lblCurrentWaterFCW.Text = waterFCWs;
            if (lightFCWs.Length > 0)
                lblCurrentLightFCW.Text = lightFCWs;

            lblWaterFCWCount.Text = string.Format("{0}/{1}\nwater FCWs sent", waterFCWsDone, waterFCWsTotal);
            lblLightFCWCount.Text = string.Format("{0}/{1}\nlight FCWs executed", lightFCWsDone, lightFCWsTotal);
            lblTotalFCWCount.Text = string.Format("{0}/{1}\ntotal FCWs executed", allFCWsDone, allFCWsTotal);
        }

        public void SetColors(LEDColor[] colors)
        {
            if (InvokeRequired)
            {
                try
                {
                    BeginInvoke(new Action(() => SetColors(colors)));
                }
                catch (ObjectDisposedException) { } // This means we're closing anyways, so no need to update the UI anymore
                return;
            }

            lvLightColors.SmallImageList.Images.Clear();
            for (int i = 0; i < colors.Length; i++)
            {
                Image newImage = CreateImage(colors[i]);
                lvLightColors.SmallImageList.Images.Add(colors[i].ToString(), newImage);
            }

            // Make sure we can use the special colors
            if (!lvLightColors.SmallImageList.Images.ContainsKey(LEDColor.BackCurtain16.ToString()))
                lvLightColors.SmallImageList.Images.Add(LEDColor.BackCurtain16.ToString(), CreateImage(LEDColor.BackCurtain16));
            if (!lvLightColors.SmallImageList.Images.ContainsKey(LEDColor.BackCurtain32.ToString()))
                lvLightColors.SmallImageList.Images.Add(LEDColor.BackCurtain32.ToString(), CreateImage(LEDColor.BackCurtain32));
            if (!lvLightColors.SmallImageList.Images.ContainsKey(LEDColor.BackCurtain48.ToString()))
                lvLightColors.SmallImageList.Images.Add(LEDColor.BackCurtain48.ToString(), CreateImage(LEDColor.BackCurtain48));
            if (!lvLightColors.SmallImageList.Images.ContainsKey(LEDColor.VoiceSlashSpout.ToString().Replace("Slash", "/")))
                lvLightColors.SmallImageList.Images.Add(LEDColor.VoiceSlashSpout.ToString().Replace("Slash", "/"), CreateImage(LEDColor.VoiceSlashSpout));

            lvLightColors.Refresh();
        }

        public void UpdateLights(LEDColor[] colors)
        {
            if (InvokeRequired)
            {
                try
                {
                    BeginInvoke(new Action(() => UpdateLights(colors)));
                }
                catch (ObjectDisposedException) { } // This means we're closing anyways, so no need to update the UI anymore
                return;
            }

            for (int i = 1; i < colors.Length; i++)
            {
                bool present = lightIndexToItemMap.TryGetValue(i, out ListViewItem lvi);
                if (!present) continue;

                string targetImage = colors[i].ToString();
                if (lvi != null)
                {
                    if (lvLightColors.SmallImageList.Images.ContainsKey(targetImage))
                        lvi.ImageKey = targetImage;
                    else
                    {
                        Image newImage = CreateImage(colors[i]);
                        lvLightColors.SmallImageList.Images.Add(colors[i].ToString(), newImage);
                        lvi.ImageKey = colors[i].ToString();
                    }
                }
            }
        }

        public void UpdateMotion(string motion)
        {
            if (InvokeRequired)
            {
                try
                {
                    BeginInvoke(new Action(() => UpdateMotion(motion)));
                }
                catch (ObjectDisposedException) { } // This means we're closing anyways, so no need to update the UI anymore
                return;
            }

            lblCurrentMotion.Text = motion;
        }

        private Image CreateImage(LEDColor col)
        {
            Size imageSize = lvLightColors.SmallImageList.ImageSize;

            Bitmap image = new Bitmap(imageSize.Width, imageSize.Height);
            using (Graphics gfx = Graphics.FromImage(image))
            using (SolidBrush brush = new SolidBrush(Color.FromArgb(col.R, col.G, col.B))) // No plans to handle amber (or white) yet
            {
                gfx.FillRectangle(brush, 0, 0, imageSize.Width, imageSize.Height);
            }

            return image;
        }

        private void Monitor_FormClosing(object sender, FormClosingEventArgs e)
        {
            // I'd like to always have a monitor around that I can update
            if (e.CloseReason == CloseReason.UserClosing)
            {
                Hide();
                e.Cancel = true;
            }
        }
    }
}
