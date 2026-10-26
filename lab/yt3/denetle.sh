#!/bin/sh
# iş çalışmıyorsa ve bitmediyse yeniden başlatır; durumu yazar
n=$(ls /home/user/alan-arsiv/lab/yt3/sonuc/*.json 2>/dev/null | wc -l)
if [ ! -f /home/user/alan-arsiv/lab/yt3/BITTI ] && ! ps -eo args | grep -v grep | grep -q "xargs -P4 -L1 sh /home/user/alan-arsiv/lab/yt3/kos.sh"; then cd /home/user/alan-arsiv/lab/yt3 && nohup ./hepsi.sh >> /home/user/alan-arsiv/lab/yt3/kos.log 2>&1 & echo "YENİDEN BAŞLATILDI"; fi
echo "$(date +%H:%M) $n/432"
