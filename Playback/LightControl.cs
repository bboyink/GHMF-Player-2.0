using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Windows.Forms;

namespace Playback
{
    public partial class LightControl : UserControl
    {
        [Category("Behavior")]
        public int LightNumber { get; set; }

        public LightControl()
        {
            InitializeComponent();
        }

        public void SetColor(LEDColor color)
        {
            Color newColor = Color.FromArgb(color?.R ?? 0, color?.G ?? 0, color?.B ?? 0);
            if (ForeColor != newColor)
            {
                ForeColor = newColor;
                Refresh();
            }
        }

        private void LightControl_Paint(object sender, PaintEventArgs e)
        {
            LightNumberLabel.Text = LightNumber.ToString();

            int yiqSpace = ((ForeColor.R * 299) + (ForeColor.G * 587) + (ForeColor.B * 114)) / 1000;
            if (yiqSpace > 131)
            {
                LightNumberLabel.ForeColor = Color.Black;
            }
            else
            {
                LightNumberLabel.ForeColor = Color.White;
            }
            e.Graphics.FillEllipse(new SolidBrush(ForeColor), 0, 0, Width - 1, Height - 1);
        }
    }
}
