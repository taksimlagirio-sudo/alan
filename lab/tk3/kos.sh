#!/bin/bash
set -- $1
O=/home/user/alan-arsiv/lab/tk3; f=$O/sonuc/$1-$2-$3.json; [ -s $f ] && exit 0
bun /home/user/alan-arsiv/tools/taktik-iz.js /tmp/claude-0/-home-user-alan/5b8226d7-bb61-5e57-822b-7f77f7501f4e/scratchpad/motor/6a73024 $1 $2 $3 3600 > $f.tmp 2>&1 && mv $f.tmp $f
