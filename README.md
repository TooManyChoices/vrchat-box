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
CONTENT_CACHE="/tmp/VRCCache" # don't ask why it's in tmp :3
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
### but what if i'm a windows user, which is literally the only platform vrchat supports
uhhhh

maybe i'll make a gui one day idk
there's also using operands instead, `vrchat -- {whatever chatbox input}`
___
*ahem*

Neither "vrchatbox" or this repository are endorsed by VRChat and do not reflect the views or opinions of VRChat or anyone officially involved in producing or managing VRChat properties. VRChat and all associated properties are trademarks or registered trademarks of VRChat Inc. VRChat © VRChat Inc.
