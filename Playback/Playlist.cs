using System.IO;

namespace Playback
{
    public class Playlist
    {
        public const string FileExtension = ".fple";

        public const string SongComment = "(COMMENT)";

        public string Filename { get; }
        public string Name { get; }
        public string[] Songs { get; private set; }
        public int StartIndex { get; set; }

        private static byte[] decoder = { 0x0F, 0x19, 0xF2, 0x37, 0x05, 0x8A, 0xFA, 0xA8, 0xAE, 0xA6, 0x77, 0x97, 0xAC, 0x68, 0x07, 0xC7, 0x7E, 0x31, 0xC2, 0xE7, 0x50, 0x3F, 0x85, 0xF0, 0x41, 0xEF, 0xAF, 0x5E, 0xC1, 0xF2, 0x82, 0x91, 0x0D, 0xB5, 0xF7, 0x56, 0x78, 0x01, 0x69, 0x82, 0xDC, 0x6A, 0xCC, 0x32, 0x00, 0x55, 0x72, 0x50, 0xDA, 0x11, 0xFB, 0x83, 0x75, 0x14, 0x27, 0x96, 0xD5, 0x15, 0xF4, 0x2E, 0xBC, 0x30, 0x4F, 0x89, 0x52, 0x45, 0x6A, 0x2A, 0x8F, 0x3D, 0x29, 0xA8, 0xAE, 0x13, 0xB8, 0x46, 0x44, 0x03, 0x59, 0x65, 0xAD, 0xF1, 0x81, 0x1E, 0x8F, 0xAA, 0x99, 0x47, 0x12, 0xFB, 0xCE, 0x7D, 0xFB, 0xF9, 0xD9, 0x28, 0xDF, 0x82, 0x62, 0xA5, 0x3D, 0x04, 0xB8, 0xFB, 0xE0, 0x66, 0x79, 0xC9, 0x6E, 0xC2, 0x6F, 0x79, 0xDF, 0x7C, 0xDD, 0x71, 0x74, 0x3B, 0xCD, 0xAE, 0x94, 0x76, 0x65, 0xEE, 0xBD, 0x22, 0x0B, 0x17, 0xC8, 0x25, 0x81, 0x82, 0x4C, 0x96, 0x7E, 0xEC, 0x34, 0xD7, 0x24, 0x90, 0xFE, 0x97, 0x5A, 0xB2, 0x1D, 0x39, 0x79, 0xA6, 0x26, 0x9C, 0xB3, 0x2D, 0x06, 0xAF, 0xC3, 0x61, 0x8B, 0x50, 0x28, 0x2B, 0x39, 0xBE, 0xC6, 0xA6, 0x4B, 0xA8, 0xE4, 0x6A, 0xBB, 0x6D, 0x61, 0x5E, 0x00, 0x6B, 0x56, 0x9D, 0x59, 0xE7, 0xBF, 0x86, 0x8E, 0x45, 0x7D, 0xE6, 0xB2, 0x14, 0x4F, 0x30, 0xB9, 0xCD, 0x69, 0x2C, 0x14, 0x13, 0x00, 0xDC, 0x4F, 0xF7, 0xD2, 0x8A, 0x17, 0x9F, 0xDC, 0xBE, 0xEF, 0x09, 0x8B, 0xC5, 0x77, 0xC3, 0x93, 0x27, 0x5F, 0x87, 0x6C, 0x47, 0xFE, 0x27, 0x28, 0x28, 0x97, 0xEA, 0xB3, 0x78, 0x40, 0xD9, 0xA7, 0x1C, 0x76, 0x38, 0x93, 0xBD, 0x0C, 0xB9, 0xA9, 0x8E, 0x71, 0x99, 0xB9, 0x57, 0xDA, 0xD2, 0x2F, 0x61, 0xAF, 0x74, 0x16, 0x96, 0xA8, 0x30, 0xBF, 0x6D, 0xC1, 0x6B, 0xB5, 0xFF, 0x94, 0xd1, 0x3c, 0x9d, 0xc6, 0xe3, 0x37, 0xdd, 0xda, 0xcb, 0xb0, 0x2f, 0xae, 0xf3, 0x5b, 0xfd, 0xa0, 0x7b, 0xf2, 0x00, 0x6a, 0x98, 0x30, 0xf2, 0xdc, 0x16, 0x49, 0x23, 0x9a, 0x50, 0x80, 0xb3, 0xea, 0x72 };

        public Playlist(string fple)
        {
            Filename = fple;
            Name = Path.GetFileNameWithoutExtension(fple);
            Songs = DecodePlaylist(fple);
            StartIndex = 0;
        }

        private string[] DecodePlaylist(string playlistFilePath)
        {
            string[] entries = File.ReadAllLines(playlistFilePath);
            for (int i = 0; i < entries.Length; i++)
                entries[i] = DecodePlaylistEntry(entries[i]);
            return entries;
        }

        private string DecodePlaylistEntry(string playlistEntry)
        {
            // The new playlists aren't encoded
            if (File.Exists(playlistEntry) || SongIsComment(playlistEntry))
                return playlistEntry;

            try
            {
                byte[] encodedBytes = System.Runtime.Remoting.Metadata.W3cXsd2001.SoapHexBinary.Parse(playlistEntry).Value;
                byte[] decodedBytes = new byte[encodedBytes.Length];
                for (int i = 0; i < decodedBytes.Length; i++)
                {
                    // It seems like the encoding must have skipped anything if both plaintext and key matched (otherwise we get nulls in the middle of our string)
                    if (encodedBytes[i] != decoder[i])
                        decodedBytes[i] = (byte)(encodedBytes[i] ^ decoder[i]);
                    else
                        decodedBytes[i] = encodedBytes[i];
                }

                return System.Text.Encoding.UTF8.GetString(decodedBytes);
            }
            catch (System.Exception e)
            {
                PlaybackForm.ShowMessage("There was a problem decoding the playlist: " + System.Environment.NewLine + e.Message);
                Logger.LogError(e.ToString());

                return playlistEntry;
            }
        }

        public void AddSong(string songToAdd)
        {
            // Maybe a list would have been better, but adding and removing songs is not a time-critical or repeating action
            // And this lets me use things like the params keyword with songs without calling ToArray all the time
            System.Collections.Generic.List<string> newSongs = new System.Collections.Generic.List<string>(Songs);
            newSongs.Add(songToAdd);
            Songs = newSongs.ToArray();
        }

        public void RemoveSong(int indexToRemove)
        {
            // Again, if this were a list I could just call RemoveAt() but then I'd have to use ToArray for that params function
            // And I think I'm a little too attached to using params there
            System.Collections.Generic.List<string> newSongs = new System.Collections.Generic.List<string>(Songs);
            newSongs.RemoveAt(indexToRemove);
            Songs = newSongs.ToArray();
        }

        public void MoveUp(int indexToMove)
        {
            SwapSongs(indexToMove - 1, indexToMove);
        }

        public void MoveDown(int indexToMove)
        {
            SwapSongs(indexToMove, indexToMove + 1);
        }

        private void SwapSongs(int firstIndex, int secondIndex)
        {
            if (firstIndex >= 0 && firstIndex < Songs.Length && secondIndex >= 0 && secondIndex < Songs.Length)
            {
                string tempSong = Songs[firstIndex];
                Songs[firstIndex] = Songs[secondIndex];
                Songs[secondIndex] = tempSong;
            }
        }

        public bool Verify()
        {
            if (StartIndex >= Songs.Length)
                throw new System.IndexOutOfRangeException("Playlist start index (" + StartIndex + ") was out of range");

            foreach (string song in Songs)
            {
                if (SongIsComment(song)) continue;
                if (!File.Exists(song))
                    throw new FileNotFoundException("Song file " + song + " not found");
                string ctlFile = Path.Combine(Path.GetDirectoryName(song), Path.GetFileNameWithoutExtension(song)) + ".ctl";
                if (!File.Exists(ctlFile))
                    throw new FileNotFoundException("Control file " + ctlFile + " not found");
            }

            return true;
        }

        public static bool SongIsComment(string song)
        {
            return song.Trim().StartsWith("(");
        }

        public void Save()
        {
            // We don't encode the new playlists (for now?)
            if (!string.IsNullOrWhiteSpace(Filename) && Songs != null && File.Exists(Filename))
                File.WriteAllText(Filename, string.Join(System.Environment.NewLine, Songs));
        }
    }
}
