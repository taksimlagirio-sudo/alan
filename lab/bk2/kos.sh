#!/bin/sh
# motor: main d72cef5 (bk1 ile aynı), 9000 tik. A'nın en öndeki oyuncusu Hız 16
o=/home/user/alan-arsiv/lab/bk2/sonuc/$1-$2-$3.json; [ -s $o ] && exit 0
cd /home/user/alan-arsiv && ONDE_OZ=hiz:16 timeout 1500 bun tools/blok-olc.js /tmp/claude-0/-home-user-alan/5b8226d7-bb61-5e57-822b-7f77f7501f4e/scratchpad/motor/d72c $1 $2 $3 9000 > $o.tmp 2>&1 && mv $o.tmp $o
