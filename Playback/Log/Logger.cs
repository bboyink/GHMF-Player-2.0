using System;
using System.Collections.Generic;
using System.IO;
using System.Threading;

namespace Playback
{
    public enum LogLevel
    {
        Error,
        Warning,
        Info,
        Debug
    }

    // A heavily modified version of a logger written by StackOverflow user Sam Saffron (used by permission)
    public static class Logger
    {
        public static LogLevel LogLevel;
        private static Queue<LogEntry> logEntries = new Queue<LogEntry>();
        private static AutoResetEvent hasNewItems = new AutoResetEvent(false);

        static Logger()
        {
            new Thread(() => ProcessQueue()) { IsBackground = true }.Start();
        }

        private static void ProcessQueue()
        {
            while (true) {
                hasNewItems.WaitOne(10000, true);

                Queue<LogEntry> queueCopy;
                lock (logEntries)
                {
                    queueCopy = new Queue<LogEntry>(logEntries);
                    logEntries.Clear();
                }

                Log(queueCopy);
            }
        }

        public static void LogError(string logFormat, params object[] logParameters)
        {
            EnQueue(new LogEntry(LogLevel.Error, logFormat, logParameters));
        }

        public static void LogWarning(string logFormat, params object[] logParameters)
        {
            EnQueue(new LogEntry(LogLevel.Warning, logFormat, logParameters));
        }

        public static void LogInfo(string logFormat, params object[] logParameters)
        {
            EnQueue(new LogEntry(LogLevel.Info, logFormat, logParameters));
        }

        public static void LogDebug(string logFormat, params object[] logParameters)
        {
            EnQueue(new LogEntry(LogLevel.Debug, logFormat, logParameters));
        }

        private static void EnQueue(LogEntry entry)
        {
            if (LogLevel >= entry.Level)
            {
                lock (logEntries)
                {
                    logEntries.Enqueue(entry);
                }
                hasNewItems.Set();
            }
        }

        private static void Log(Queue<LogEntry> entries)
        {
            string logFilePath = TodaysLogFile();

            using (FileStream fs = new FileStream(logFilePath, FileMode.OpenOrCreate, System.Security.AccessControl.FileSystemRights.AppendData, FileShare.Write, 4096, FileOptions.None))
            using (StreamWriter logFile = new StreamWriter(fs))
            {
                logFile.AutoFlush = true;
                foreach (LogEntry entry in entries)
                {
                    // Remember, the date is already logged in the filename
                    logFile.WriteLine(entry.Created.ToString("HH:mm:ss.fff") + " " + entry.Level.ToString());
                    logFile.WriteLine(string.Format(entry.Format, entry.Parameters));
                }
            }
        }

        private static string TodaysLogFile()
        {
            return System.IO.Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.MyDocuments), "Playback " + DateTime.Now.ToString("yyyyMMdd") + ".txt");
        }
    }

    class LogEntry
    {
        public DateTime Created { get; }
        public LogLevel Level { get; }
        public string Format { get; }
        public object[] Parameters { get; }

        public LogEntry(LogLevel logLevel, string logFormat, params object[] logParameters)
        {
            Created = DateTime.Now;
            Level = logLevel;
            Format = logFormat;
            Parameters = logParameters;
        }
    }
}
