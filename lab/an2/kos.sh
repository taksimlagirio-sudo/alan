#!/bin/sh
# motor: main 3d8475b, 9000 tik. eski: ALAN_SUT_KAPAT_ESKI + ALAN_KENAR_YON_ESKI (50e4b1b'nin iki kuralı kapalı)
o=/home/user/alan-arsiv/lab/an2/sonuc/$1-$2-T_dikine-$3.json; [ -s $o ] && exit 0
B=; [ $1 = eski ] && B=ALAN_SUT_KAPAT_ESKI,ALAN_KENAR_YON_ESKI
cd /home/user/alan-arsiv && BAYRAK=$B timeout 1500 bun tools/sut-anatomi.js /tmp/claude-0/-home-user-alan/5b8226d7-bb61-5e57-822b-7f77f7501f4e/scratchpad/motor/3d84 $2 T_dikine $3 9000 > $o.tmp 2>&1 && mv $o.tmp $o
