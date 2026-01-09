using System;

namespace Playback
{
    public class Command
    {
        public int Address { get; private set; }
        public int Data { get; private set; }
        public bool HexColor {  get; private set; }

        public Command(string unparsedCommandString)
        {
            Parse(unparsedCommandString);
        }

        public Command(int address, int data)
        {
            Address = address;
            Data = data;
            HexColor = false;
        }

        public void Parse(string unparsedCommandString)
        {
            // Format:
            // AAA-DDD or AAA-DDDDDD
            // AAA: address of device (see FCW documentation)
            // DDD: data of device (see FCW documentation)
            // DDDDDD: hex data for color control

            // Really it's quite simple: before the dash is our address, after is our data
            // I made this looser (making the separator any non-numeric item) for easier input in the manual field

            // using anyt non-numeric character as a separator is no longer a viable option due to hex color values
            // string[] pieces = System.Text.RegularExpressions.Regex.Split(unparsedCommandString, @"\D");

            // we're requiring dashes now to follow FCW documentation and allow hex color values
            string[] pieces = unparsedCommandString.Split('-');
 
            Address = int.Parse(pieces[0]);

            if (pieces[1].Length < 6)
            {
                Data = int.Parse(pieces[1]);
                HexColor = false;
            }
            else
            {
                Data = Convert.ToInt32(pieces[1], 16);
                HexColor = true;
            }
        }

        public override string ToString()
        {
            return Address.ToString("000") + (HexColor ? Data.ToString("X6") : Data.ToString("000"));
        }
    }
}
