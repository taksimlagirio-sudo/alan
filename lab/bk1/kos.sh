#!/bin/sh
# motor: main d72cef5 (scratchpad kopyası), 9000 tik. Önde bekleyen: tac.onde = 1
o=/home/user/alan-arsiv/lab/bk1/sonuc/$1-$2-$3.json; [ -s $o ] && exit 0
cd /home/user/alan-arsiv && timeout 1500 bun tools/blok-olc.js /tmp/claude-0/-home-user-alan/5b8226d7-bb61-5e57-822b-7f77f7501f4e/scratchpad/motor/d72c $1 $2 $3 9000 > $o.tmp 2>&1 && mv $o.tmp $o
