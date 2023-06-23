namespace Playback
{
    public class PLCComms : System.IDisposable
    {
        public bool Connected { get { return !enabled || (socket != null && socket.Connected); } }

        private System.Net.Sockets.TcpClient socket;
        private bool enabled;
        private string ip;
        private int port;
        private bool reconnecting;

        private System.Collections.Generic.List<string> queue;

        public PLCComms(bool disabled)
        {
            enabled = !disabled;
            queue = new System.Collections.Generic.List<string>();
        }

        public void Dispose()
        {
            Disconnect();
        }

        public void Connect(string ipAddress, int portNumber, int timeoutMilliseconds, bool synchronous = true)
        {
            ip = ipAddress;
            port = portNumber;
            reconnecting = false;

            if (socket != null && socket.Connected)
                Disconnect();

            socket = new System.Net.Sockets.TcpClient();

            if (enabled)
            {
                Logger.LogDebug("Beginning PLC connection");
                System.IAsyncResult result = socket.BeginConnect(ip, port, new System.AsyncCallback(ConnectComplete), socket.Client);

                if (synchronous)
                {
                    Logger.LogDebug("Synchronous PLC connection waiting...");
                    bool success = result.AsyncWaitHandle.WaitOne(timeoutMilliseconds, true);

                    if (!success)
                    {
                        socket.Close();
                        socket = null;
                        throw new System.Net.Sockets.SocketException((int)System.Net.Sockets.SocketError.TimedOut);
                    }
                    Logger.LogDebug("Synchronous PLC Connection succeeded");
                }
            }
        }

        private void ConnectComplete(System.IAsyncResult result)
        {
            reconnecting = false;
            if (result.AsyncState != null)
            {
                System.Net.Sockets.Socket resultSocket = (System.Net.Sockets.Socket)result.AsyncState;
                if (resultSocket.Connected)
                    resultSocket.EndConnect(result);
                Logger.LogDebug("Asynchronous PLC Connection succeeded");
            }
            else
                socket = null;
        }

        public void Disconnect()
        {
            if (socket != null)
                socket.Close();
        }

        public void AddToQueue(string stringToAdd)
        {
            lock (queue)
            {
                if (!string.IsNullOrWhiteSpace(stringToAdd))
                    queue.Add(stringToAdd);
            }
        }

        public int SendQueue()
        {
            lock (queue)
            {
                int count = queue.Count;
                if (queue.Count > 0 && Send(string.Join(" ", queue.ToArray()) + "\r\n"))
                    queue.Clear();
                return count;
            }
        }

        public bool Send(string outString)
        {
            if (!outString.EndsWith("\r\n"))
                outString += "\r\n";

            if (enabled)
            {
                if (socket.Connected)
                {
                    Logger.LogDebug("Sending string to PLC: " + outString);
                    byte[] sendBytes = System.Text.Encoding.ASCII.GetBytes(outString);

                    try
                    {
                        socket.GetStream().Write(sendBytes, 0, sendBytes.Length);
                        return true;
                    }
                    catch (System.IO.IOException e)
                    {
                        if (e.InnerException.GetType() == typeof(System.Net.Sockets.SocketException))
                        {
                            socket.Close();
                            Logger.LogWarning("PLC disconnected. Trying to reconnect...");
                            return false;
                        }
                        else
                            throw;
                    }
                }
                else
                {
                    // Keep trying til we get it back
                    if (!reconnecting)
                    {
                        Logger.LogInfo("Attempting reconnect to PLC");
                        Connect(ip, port, 1000, false);
                        reconnecting = true;
                    }
                    return false;
                }
            }
            else
            {
                Logger.LogDebug("Would have sent string to PLC: " + outString);
                return true;
            }
        }
    }
}
