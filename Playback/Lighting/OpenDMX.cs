using System;
using System.Runtime.InteropServices;
using System.Threading;

namespace Playback
{
    // From http://www.enttec.com/download/examples/OpenDMX.cs
    // Of course, I had to fix the memory leak in the write function
    public class OpenDMX
    {

        public static byte[] buffer { get; private set; }
        public static byte[] header { get; private set; }
        public static byte[] ender { get; private set; }
        public static uint handle;
        public static volatile bool done = false;
        public static int bytesWritten = 0;
        public static FT_STATUS status
        {
            get
            {
                return m_status;
            }
            set
            {
                m_status = value;
                Logger.LogDebug("DMX Status changed to {0}", m_status.ToString());
            }
        }
        private static FT_STATUS m_status;

        public const byte BITS_8 = 8;
        public const byte STOP_BITS_2 = 2;
        public const byte PARITY_NONE = 0;
        public const UInt16 FLOW_NONE = 0;
        public const byte PURGE_RX = 1;
        public const byte PURGE_TX = 2;

        [DllImport("FTD2XX.dll")]
        public static extern FT_STATUS FT_Open(UInt32 uiPort, ref uint ftHandle);
        [DllImport("FTD2XX.dll")]
        public static extern FT_STATUS FT_Close(uint ftHandle);
        [DllImport("FTD2XX.dll")]
        public static extern FT_STATUS FT_Read(uint ftHandle, IntPtr lpBuffer, UInt32 dwBytesToRead, ref UInt32 lpdwBytesReturned);
        [DllImport("FTD2XX.dll")]
        public static extern FT_STATUS FT_Write(uint ftHandle, IntPtr lpBuffer, UInt32 dwBytesToWrite, ref UInt32 lpdwBytesWritten);
        [DllImport("FTD2XX.dll")]
        public static extern FT_STATUS FT_SetDataCharacteristics(uint ftHandle, byte uWordLength, byte uStopBits, byte uParity);
        [DllImport("FTD2XX.dll")]
        public static extern FT_STATUS FT_SetFlowControl(uint ftHandle, char usFlowControl, byte uXon, byte uXoff);
        [DllImport("FTD2XX.dll")]
        public static extern FT_STATUS FT_GetModemStatus(uint ftHandle, ref UInt32 lpdwModemStatus);
        [DllImport("FTD2XX.dll")]
        public static extern FT_STATUS FT_Purge(uint ftHandle, UInt32 dwMask);
        [DllImport("FTD2XX.dll")]
        public static extern FT_STATUS FT_ListDevices(ref int arg1, ref int arg2, UInt32 flags);
        [DllImport("FTD2XX.dll")]
        public static extern FT_STATUS FT_ClrRts(uint ftHandle);
        [DllImport("FTD2XX.dll")]
        public static extern FT_STATUS FT_SetBreakOn(uint ftHandle);
        [DllImport("FTD2XX.dll")]
        public static extern FT_STATUS FT_SetBreakOff(uint ftHandle);
        [DllImport("FTD2XX.dll")]
        public static extern FT_STATUS FT_GetStatus(uint ftHandle, ref UInt32 lpdwAmountInRxQueue, ref UInt32 lpdwAmountInTxQueue, ref UInt32 lpdwEventStatus);
        [DllImport("FTD2XX.dll")]
        public static extern FT_STATUS FT_ResetDevice(uint ftHandle);
        [DllImport("FTD2XX.dll")]
        public static extern FT_STATUS FT_SetDivisor(uint ftHandle, char usDivisor);

        static OpenDMX()
        {
            Logger.LogDebug("Creating OpenDMX");
            buffer = new byte[513];
            header = new byte[] { 0x7E, 6, (byte)(buffer.Length & 0xFF), (byte)(buffer.Length >> 8) };
            ender = new byte[] { 0xE7 };
        }

        public static void start()
        {
            Logger.LogDebug("Starting DMX");
            buffer = new byte[513];
            done = false;
            if (handle != 0)
                status = FT_Close(handle);
            handle = 0;
            status = FT_Open(0, ref handle);
            new Thread(new ThreadStart(writeData)) { IsBackground = true }.Start();
            setDmxValue(0, 0);  //Set DMX Start Code
            //initOpenDMX();
        }

        public static void stop()
        {
            Logger.LogDebug("Stopping DMX");
            done = true;
            if (handle != 0)
                status = FT_Close(handle);
            handle = 0;
        }

        public static void setDmxValue(int channel, byte value)
        {
            buffer[channel] = value;
        }

        public static void writeData()
        {
            // I'm fairly certain that we could do this only on changing a buffer value
            // But the difference in CPU usage seems to be negligible so why bother
            while (!done)
            {
                //initOpenDMX();
                //FT_SetBreakOn(handle);
                //FT_SetBreakOff(handle);
                bytesWritten = write(handle, header, header.Length);
                bytesWritten = write(handle, buffer, buffer.Length);
                bytesWritten = write(handle, ender, ender.Length);
                Thread.Sleep(20);
            }
        }

        public static int write(uint handle, byte[] data, int length)
        {
            IntPtr ptr = Marshal.AllocHGlobal(length);
            Marshal.Copy(data, 0, ptr, length);
            uint bytesWritten = 0;
            status = FT_Write(handle, ptr, (uint)length, ref bytesWritten);
            Marshal.FreeHGlobal(ptr); // The online example doesn't free this - I should probably let them know...or something
            return (int)bytesWritten;
        }

        public static void initOpenDMX()
        {
            Logger.LogDebug("Initializing DMX");
            status = FT_ResetDevice(handle);
            status = FT_SetDataCharacteristics(handle, BITS_8, STOP_BITS_2, PARITY_NONE);
            status = FT_SetFlowControl(handle, (char)FLOW_NONE, 0, 0);
            status = FT_ClrRts(handle);
            status = FT_Purge(handle, PURGE_TX);
            status = FT_Purge(handle, PURGE_RX);
        }
    }

    /// <summary>
    /// Enumaration containing the various return status for the DLL functions.
    /// </summary>
    public enum FT_STATUS
    {
        FT_OK = 0,
        FT_INVALID_HANDLE,
        FT_DEVICE_NOT_FOUND,
        FT_DEVICE_NOT_OPENED,
        FT_IO_ERROR,
        FT_INSUFFICIENT_RESOURCES,
        FT_INVALID_PARAMETER,
        FT_INVALID_BAUD_RATE,
        FT_DEVICE_NOT_OPENED_FOR_ERASE,
        FT_DEVICE_NOT_OPENED_FOR_WRITE,
        FT_FAILED_TO_WRITE_DEVICE,
        FT_EEPROM_READ_FAILED,
        FT_EEPROM_WRITE_FAILED,
        FT_EEPROM_ERASE_FAILED,
        FT_EEPROM_NOT_PRESENT,
        FT_EEPROM_NOT_PROGRAMMED,
        FT_INVALID_ARGS,
        FT_OTHER_ERROR
    };
}
