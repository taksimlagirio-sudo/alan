#!/bin/bash
# bir iş: <özellik> <değer> <tohum>; çıktısı varsa atlanır
set -- $1
f=/home/user/alan-arsiv/lab/oz1/sonuc/$1-$2-$3.json; [ -s $f ] && exit 0
bun /home/user/alan-arsiv/tools/oz-olc.js /tmp/claude-0/-home-user-alan/5b8226d7-bb61-5e57-822b-7f77f7501f4e/scratchpad/motor/bbd5ead $1 $2 $3 > $f.tmp 2>&1 && mv $f.tmp $f
