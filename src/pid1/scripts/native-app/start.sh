#!/bin/sh
set -eu
grep -q 'androidboot.serialno=4C0490290FA05A0C0' /proc/cmdline
mkdir -p /doom /share/tiles /share/rpg /bin/lean
for name in ytplay slides doomtv rpgtv flighttv snaketv ytplay-lean doomtv-lean; do
    if [ -f "/tmp/app-next-$name" ]; then
        mv "/tmp/app-next-$name" "/tmp/app-$name"
        chmod 755 "/tmp/app-$name"
    fi
done
gzip -dc /tmp/app-wad.gz > /doom/freedoom1.wad
cp /tmp/demo-video.rgb /share/tiles/demo-video.rgb
cp /tmp/flight.rgb /share/tiles/flight.rgb
cp /tmp/youtube.rgb /share/tiles/youtube.rgb
cp /tmp/doom.rgb /share/tiles/doom.rgb
cp /tmp/rpg-tile.rgb /share/tiles/rpg.rgb
for picture in /tmp/rpg-picture-*.rgb; do
    name=${picture##*/rpg-picture-}
    cp "$picture" "/share/rpg/$name"
done
for name in ytplay slides doomtv rpgtv flighttv snaketv; do ln -sf "/tmp/app-$name" "/bin/$name"; done
for name in ytplay doomtv; do ln -sf "/tmp/app-$name-lean" "/bin/lean/$name"; done
[ -f /tmp/app-volume ] || echo 0 > /tmp/app-volume
test -f /tmp/pid1-volume.ready
