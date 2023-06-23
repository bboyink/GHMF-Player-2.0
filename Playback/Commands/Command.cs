namespace Playback
{
    public class Command
    {
        public int Address { get; private set; }
        public int Data { get; private set; }

        public Command(string unparsedCommandString)
        {
            Parse(unparsedCommandString);
        }

        public Command(int address, int data)
        {
            Address = address;
            Data = data;
        }

        public void Parse(string unparsedCommandString)
        {
            // Format:
            // AAA-DDD
            // AAA: address of device (see FCW documentation)
            // DDD: data of device (see FCW documentation)

            // Really it's quite simple: before the dash is our address, after is our data
            // I made this looser (making the separator any non-numeric item) for easier input in the manual field
            string[] pieces = System.Text.RegularExpressions.Regex.Split(unparsedCommandString, @"\D");
            
            Address = int.Parse(pieces[0]);
            Data = int.Parse(pieces[1]);
        }

        public override string ToString()
        {
            return Address.ToString("000") + Data.ToString("000");
        }
    }
}
