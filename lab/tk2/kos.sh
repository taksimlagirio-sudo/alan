#!/bin/bash
set -- $1
O=/home/user/alan-arsiv/lab/tk2; f=$O/sonuc/$1-$2-$3.json; [ -s $f ] && exit 0
bun /home/user/alan-arsiv/tools/taktik-olc.js /tmp/claude-0/-home-user-alan/5b8226d7-bb61-5e57-822b-7f77f7501f4e/scratchpad/motor/7ce2527 $1 $2 $3 2400 > $f.tmp 2>&1 && mv $f.tmp $f
