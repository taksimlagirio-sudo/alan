#!/bin/bash
set -- $1
O=/home/user/alan-arsiv/lab/en1; f=$O/sonuc/${1/:/-}-$2-$3-$4.json; [ -s $f ] && exit 0
if [ "$1" = "-" ]; then OZ=; else OZ=$1; fi
OZ=$OZ bun /home/user/alan-arsiv/tools/enerji-olc.js /tmp/claude-0/-home-user-alan/5b8226d7-bb61-5e57-822b-7f77f7501f4e/scratchpad/motor/yeni $2 $3 $4 2700 > $f.tmp 2>&1 && mv $f.tmp $f
