# vrchatbox
simple program to send unix stdin or operands as vrchat chatbox input.
## building
1. clone this repo `git clone this-repo`
2. cargo build `cargo build`
3. profit `vrchatbox`
### (installing)
1. `cargo install --path .`
2. now u can use anywhere as the current user
## how to use
do `vrchat -h` for all flags you can use
you can use this in bash/zsh/whatever scripts to chatbox abuse because you're just sooo cool and above whatever other people (the people you are talking to/are trying to talk to you) feel about it.

as an example, here's a sh script i made to show my content cache size whenever it changes, clearing it after a measly 5 seconds too.
```sh
CONTENT_CACHE="path_to_your_content_cache"
if [ ! -d "$CONTENT_CACHE" ]; then
	echo nothing at "$CONTENT_CACHE" stopping
	exit
fi

PREV_SIZE=$(du -hs "$CONTENT_CACHE" | awk '{print $1}')
while true; do
	if [ ! -d "$CONTENT_CACHE" ]; then
		echo nothing at "$CONTENT_CACHE" stopping
		exit
	fi
	SIZE=$(du -hs "$CONTENT_CACHE" | awk '{print $1}')
	if [[ "$PREV_SIZE" != "$SIZE" ]] then
		echo "my cache rn: $(echo $PREV_SIZE)b > $(echo $SIZE)b" | vrchatbox
		PREV_SIZE=$SIZE
	fi
	sleep 5
	echo "" | vrchatbox
done
```
## how to get it atleast working on every platform vrchat actually supports (except for ios)
### android
first you'll need [termux](https://termux.dev/) which is a terminal/shell/package manager without needing to root your device.

in termux, setup your package repository mirrors with `termux-change-repo` and run `pkg install git rust` to install git and rust to your device.

clone this repo by running `git clone github.com/TooManyChoices/vrchatbox` and go into it with `cd vrchatbox`

finally you can run `cargo build --release` and a binary should be made as `target/release/vrchatbox`

you could instead run `cargo install --path .` to have the binary go into a directory that any cargo thing you install goes to, but then you'd have to add that directoy to your $PATH to run it and that's out of the scope of this README.md.

good luck writing a bash script on mobile ;-;

### quest 3
it's actually the same process as on android, because, quest 3 runs on android, but you need developer mode on to install termux.

### windows
i haven't tried this but i'm assuming it'll work. the issue is that the windows shell doesn't have stdin (as far as i know, if it does post an issue calling me an ignorant dumbass) but you could probably use [cygwin](https://cygwin.com/) as it has a posix compatible shell. you could also try wsl if osc works through that.

## plans
- have a gui app because this is completely unreasonable outside of usage in scripts

___
*ahem*

Neither "vrchatbox" or this repository are endorsed by VRChat and do not reflect the views or opinions of VRChat or anyone officially involved in producing or managing VRChat properties. VRChat and all associated properties are trademarks or registered trademarks of VRChat Inc. VRChat © VRChat Inc.
