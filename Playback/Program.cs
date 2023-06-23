using System;
using System.Runtime.InteropServices;
using System.Windows.Forms;

namespace Playback
{
    static class Program
    {
        [DllImport("user32.dll")]
        [return: MarshalAs(UnmanagedType.Bool)]
        static extern bool SetForegroundWindow(IntPtr hWnd);
        [DllImport("user32.dll")]
        [return: MarshalAs(UnmanagedType.Bool)]
        static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);

        /// <summary>
        /// The main entry point for the application.
        /// </summary>
        [STAThread]
        static void Main()
        {
            bool createdNew = true;
            using (System.Threading.Mutex mutex = new System.Threading.Mutex(true, "ApexPlayback", out createdNew))
            {
                if (createdNew)
                {
                    // Since this is *the* program running on the PC (other stuff like Solitaire can be slowed down if need be) we're bumping up our priority
                    System.Diagnostics.Process.GetCurrentProcess().PriorityClass = System.Diagnostics.ProcessPriorityClass.RealTime;

                    // Set up our exception handlers
                    AppDomain.CurrentDomain.UnhandledException += CurrentDomain_UnhandledException;
                    Application.SetUnhandledExceptionMode(UnhandledExceptionMode.CatchException);
                    Application.ThreadException += Application_ThreadException;

                    // Begin
                    Application.EnableVisualStyles();
                    Application.SetCompatibleTextRenderingDefault(false);
                    Application.Run(new PlaybackForm());
                }
                else
                {
                    System.Diagnostics.Process current = System.Diagnostics.Process.GetCurrentProcess();
                    foreach (System.Diagnostics.Process process in System.Diagnostics.Process.GetProcessesByName(current.ProcessName))
                    {
                        if (process.Id != current.Id)
                        {
                            SetForegroundWindow(process.MainWindowHandle);
                            ShowWindow(process.MainWindowHandle, 9); // 9 = restore
                            break;
                        }
                    }
                }
            }
        }

        /// <summary>
        /// Unhandled Exception Handler
        /// </summary>
        static void CurrentDomain_UnhandledException(object sender, UnhandledExceptionEventArgs e)
        {
            try
            {
                // Get the Instance of the Exception
                Exception ex = (Exception)e.ExceptionObject;

                // Display the Error to the User
                MessageBox.Show(ex.Message, "Unhandled Exception", MessageBoxButtons.OK, MessageBoxIcon.Error);

                Logger.LogError(ex.ToString());
            }
            finally { }
        }

        /// <summary>
        /// Unhandled Exception Handler
        /// </summary>
        static void Application_ThreadException(object sender, System.Threading.ThreadExceptionEventArgs e)
        {
            try
            {
                // Display the Error to the User
                MessageBox.Show(e.Exception.Message, "Unhandled Exception", MessageBoxButtons.OK, MessageBoxIcon.Error);

                Logger.LogError(e.Exception.ToString());
            }
            finally { }
        }
    }
}
