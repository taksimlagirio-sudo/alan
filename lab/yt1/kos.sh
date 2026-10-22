#!/bin/sh
# motor: main 67dcc28 (yetenek Aşama 7), 2700 tik. $1 = özellik:değer ya da - (taban), $2 = tohum
o=/home/user/alan-arsiv/lab/yt1/sonuc/$(echo $1 | tr ':' '-')-$2.json; [ -s $o ] && exit 0
OZ=$1; [ "$OZ" = - ] && OZ=
cd /home/user/alan-arsiv && OZ=$OZ timeout 1500 bun tools/yet-olc.js /tmp/claude-0/-home-user-alan/5b8226d7-bb61-5e57-822b-7f77f7501f4e/scratchpad/motor/67dcc28 $2 2700 > $o.tmp 2>&1 && mv $o.tmp $o
