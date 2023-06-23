namespace Playback
{
    public partial class LoginForm : System.Windows.Forms.Form
    {
        // This was never intended to be particularly high security
        // Heck, we're actually loading the password in from a file
        // It's just to keep curious operators out
        private string[] passwords;

        public LoginForm(params string[] passwords)
        {
            InitializeComponent();

            this.passwords = passwords;
        }

        private void btnOK_Click(object sender, System.EventArgs e)
        {
            if (ValidatePassword())
            {
                DialogResult = System.Windows.Forms.DialogResult.OK;
                Close();
            }
            else
            {
                System.Windows.Forms.MessageBox.Show("Password not recognized.");
                tbPassword.Clear();
                tbPassword.Focus();
            }
        }

        private void btnCancel_Click(object sender, System.EventArgs e)
        {
            DialogResult = System.Windows.Forms.DialogResult.Cancel;
            Close();
        }

        private bool ValidatePassword()
        {
            return (System.Array.Find(passwords, pw => pw == tbPassword.Text) != null);
        }
    }
}
