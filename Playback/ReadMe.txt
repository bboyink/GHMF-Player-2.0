==========
What's New
==========

Version 1.3.0.0		Added support for 8 new lights (46-53). Modules 1, 2, 3, 5, 6, and 7 each get a new back center light, and the peacock gets two back lights.

					During the reading of control scripts identified by "created with GHMF", previous light designations are remapped to their new locations.

					Ensured that the light indicators stay in their proper location at all times by not using "zoom" on the fountain graphic.

					Music files are read with MediaFoundationReader, which allowed for the addition of MP3 streams.

					Stopped invalid FCWs from crashing the program. Now, it ignores them. Likewise, bad colors are ignored.

					Moved the settings file to the Config subfolder of the application's path. This makes it easier to transfer everything to a new computer intact.
					Previously, this file resided in the public shared documents folder, so it was easily overlooked. When debugging in Visual Studio, settings will
					reside at c:\ghmf\config. The file name was shortened to Playback.xml.

					For more convenient testing of the executable, compile with TEST as a compiler condition. This bypasses a lot of little annoyances that slow
					down rapid testing, such as ensuring PLC is disabled and skipping passwords.

					Properly designated City of Grand Haven as owner of all code and updated copyright information accordingly.



=============================
Notes: 9-16-2025 - Jim Swarts
=============================

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


=============================
Notes: 9-17-2025 - Jim Swarts
=============================

DMX mapping includes a new "D" format type that designates the "light" as a special-use DMX device. Similarly, FCW mapping include "DMX" as a Light Role. This
combination allows any byte to be sent to the first channel mapped to the device. FCWs that normally reset all lights will not affect these DMX channels, nor will
playing songs.

FCWs 900-902 have been mapped to DMX devices 900-902, which, in order are Madrix 1, Madrix 2, and Fireworks. As of this writing, they are mapped to channels 500, 501,
and 502. The existing raw functions were not condusive to sticky settings, which are required for the intended use.
