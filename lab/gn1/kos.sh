#!/bin/sh
# motor: main 33c66f8, 9000 tik. Tek fark hücum genişliği (taktik-ayar: genGenis, genDar, varsayilan = Normal)
o=/home/user/alan-arsiv/lab/gn1/sonuc/$1-$2-$3.json; [ -s $o ] && exit 0
cd /home/user/alan-arsiv && timeout 1500 bun tools/blok-olc.js /tmp/claude-0/-home-user-alan/5b8226d7-bb61-5e57-822b-7f77f7501f4e/scratchpad/motor/33c66f8 $1 $2 $3 9000 > $o.tmp 2>&1 && mv $o.tmp $o
