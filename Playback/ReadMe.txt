9-16-2025 - Jim Swarts

This is a legacy project that depends on .NET Framework 4.0. Try getting that for a newer machine! What an adventure! I had to install Visual Studio 2019 to get
the older frameworks, because Microsoft has pulled most of them, they do not come with VS 2022, and some are only handled as references (confusing).

Advancing to a new framework breaks the project! For example, Framework 4.5.2 causes issues with configuring lights (DMX) to the point the code confused DMX and PLC
setup. Unless you're willing to rewrite a LOT of code, leave well-enough alone.

Ultimately, with the Framework issue in mind, I feel it is best not to update NAudio or System.ValueTruplet either.

To implement support for MP3, I changed from WavFileReader to MediaFoundationReader. At first, I attempted to implement NAudio's MP3FileReader along side the Wave
reader, but the MP3 reader had issues with mismatched sampling sizes, and the InterWebs suggested MFR. Of course, MFR didn't want to work properly with the framework.
It did on my development system, but kept throwing errors on an independent test system. It took an entire day of ChatGPT conversations to finally discover that
the reader was trying to get exclusive use of the audio stream, but it needed to share it.

I now have dabbled in things I don't care to need to know, such as dumpbin, corflags, 32-bit preference when targeting AnyCPU, and other dark, scary things that are
best left to brainiacs beyond my abilities. It's no wonder everyone wants web apps these days!
