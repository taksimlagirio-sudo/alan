#!/bin/sh
# $1 = özellik:değer ya da - (taban), $2 = M (adam markajı) ya da -, $3 = tohum. Motor: main b121ca4 (kopyası lab/yt3/motor), 2700 tik
o=/home/user/alan-arsiv/lab/yt3/sonuc/$(echo $1 | tr ':' '-')-$2-$3.json; [ -s $o ] && exit 0
OZ=$1; [ "$OZ" = - ] && OZ=; MJ=; [ "$2" = M ] && MJ=1
cd /home/user/alan-arsiv && OZ=$OZ MARKAJ=$MJ timeout 1200 bun tools/yet3-olc.js /home/user/alan-arsiv/lab/yt3/motor $3 2700 > $o.tmp 2>&1 && mv $o.tmp $o
