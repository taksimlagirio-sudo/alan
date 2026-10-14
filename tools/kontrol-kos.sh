#!/bin/bash
# Kontrol paketi: yeni bir motoru 16 maçta oynatır ve kontrol tablosunu tabanla yan yana koyar.
# Kullanım: tools/kontrol-kos.sh <motor klasörü (alan-v5 js dosyaları)> <ad> [taban kayıt klasörü] [senaryo sayısı, varsayılan 16]
# Örnek:   tools/kontrol-kos.sh ../site59/alan-v5 z59 lab/kayit/z58
# Kayıtlar lab/kayit/<ad>/ altına, tablo lab/kayit/<ad>-tablo.md dosyasına. Yarıda kalırsa aynı komut kaldığı yerden sürer.
set -e
SRC=$1; AD=$2; TABAN=${3:-}; S=${4:-16}
mkdir -p hiz30/$AD lab/kayit/$AD
cp "$SRC"/*.js hiz30/$AD/ && rm -f hiz30/$AD/test-*.js
J=lab/kayit/isler-$AD.jsonl; : > $J
for i in $(seq 0 $((S-1))); do echo "{\"out\":\"lab/kayit/$AD/$i.json.gz\",\"args\":[\"hiz30/$AD\",\"$i\"]}" >> $J; done
rm -f $J.BITTI
node tools/paralel.js $J 4 > lab/kayit/isler-$AD.log 2>&1
if [ -n "$TABAN" ]; then node tools/kontrol-tablosu.js $TABAN=$(basename $TABAN) lab/kayit/$AD=$AD; else node tools/kontrol-tablosu.js lab/kayit/$AD=$AD; fi | tee lab/kayit/$AD-tablo.md
