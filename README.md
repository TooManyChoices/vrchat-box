# vrchat-box
## *formerly.. vrchatbox*

*gui app coming in 2050*

`vrchat-box` is a simple cli tool that sends data to VRChat's OSC port at /chatbox/input, enabling you to say anything a linux command can output (that is less than 144 characters and is properly formatted text) 

## TO USE
`vrchat-box --help` shows the few options you have regarding configuration:
```
--help: show this message
--version: show a different message
--client-port: set port of osc client, or OS will choose randomly
--server-address: set address of vrchat osc server, or default to ({VRCHAT_OSC_ADDR})
--enable-sfx: every message sent makes the chatbox notification sound, probably don't enable
--show-keyboard: instead of instantly becoming a message, outputs it to client keyboard
--append-mode: if taking from stdin, add onto a growing buffer of a message instead of completely replacing the previous messages
--typing-indicator: enable in-game typing indicator for lifetime of this program
```

You may pipe text from another program into `vrchat-box` by running it as ``echo "savhjkerfbkhea" | vrchat-box`` or having anything after a "--" will send that instead, example: ``vrchat-box -- savhjkerfbkhea``

The intended use case is in sh scripts, which is like running the command normally but in a text file.

## TO INSTALL

### linux

You need to have rust/cargo installed on your system. Should be easy enough. Right.

Run ``cargo install --git "https://github.com/TooManyChoices/vrchat-box.git"`` which will place a compiled binary in ~/.cargo/bin/ which you may add to your $PATH.

To update, you just run that command again whenever you feel that there's been an update.

### android

You CAN build and run this on an android device, like a phone or a ~~meta~~ oculus quest. I just don't know enough about android to give proper instructions.

If you don't already have a terminal shell on your device, install [Termux](https://f-droid.org/en/packages/com.termux/), and in it run ``pkg install rust``

Follow the linux instructions, the directory that the binary goes will probably be different though.

### windows

So the regular Windows shell doesn't support the one thing that makes this program any useful, so you're gonna need to install [cygwin](https://cygwin.com/) first.

Then run ``cargo install --git "https://github.com/TooManyChoices/vrchat-box.git"`` and the binary should be built and put... somewhere man idk.

___
*ahem*

This repository is not endorsed by VRChat and does not reflect the views or opinions of VRChat or anyone officially involved in producing or managing VRChat properties. VRChat and all associated properties are trademarks or registered trademarks of VRChat Inc. VRChat © VRChat Inc.
