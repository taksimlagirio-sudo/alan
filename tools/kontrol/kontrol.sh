#!/bin/bash
# ALAN · her zip için kontrol paketi. Yeni sürümü bir öncekiyle karşılaştırır ve OUT/RAPOR.md yazar.
# Kullanım: tools/kontrol/kontrol.sh <yeni site klasörü> <önceki site klasörü> <çıktı klasörü>
#   site klasörü: index.html, canli.html, lab.html ve alan-v5/ (sitede yayınlanan hâl)
# Ortam: PW (Playwright test klasörü, varsayılan scratchpad/pw), PAR (paralel iş, varsayılan 4)
set -u
A=$(cd "$(dirname "$0")/../.." && pwd); YENI=$(cd "$1" && pwd); ONCEKI=$(cd "$2" && pwd); mkdir -p "$3"; OUT=$(cd "$3" && pwd)
PW=${PW:-$A/../pw}; PAR=${PAR:-4}; cd "$A"
mkdir -p "$OUT/stat" "$OUT/taktik" "$OUT/donma"; echo "$YENI" > "$OUT/yeni.txt"; echo "$ONCEKI" > "$OUT/onceki.txt"; date -u +%FT%TZ > "$OUT/baslangic.txt"
# 1 · tarayıcı: oyun akışı ve laboratuvar
PORT=$((8940 + RANDOM % 50)); (cd "$YENI" && exec python3 -m http.server $PORT > /dev/null 2>&1) & SRV=$!; sleep 1
{ echo "--- oyun"; timeout 400 node "$PW/t9.js" $PORT 2>&1 | grep -v agent-proxy | tail -2; echo "--- lab"; timeout 150 node "$PW/t11.js" $PORT 2>&1 | grep -o "hatalar:.*\|console:.*\|pageerror:.*\|http [0-9].*\|[0-9]* / [0-9]* maç" | head -8; } > "$OUT/tarayici.txt"
kill $SRV 2> /dev/null
# 2 · işler: donma (4 maç), istatistik (16 kurulum × iki sürüm, aynı kuyrukta), taktik etkisi (yeni sürüm, 9 değişken × 2 tohum)
J="$OUT/isler.txt"; : > "$J"
for s in 11 12 13 14; do echo "node donma/don.js '$YENI/alan-v5' $s > '$OUT/donma/$s.txt' 2>&1" >> "$J"; done
for s in $(seq 0 15); do for e in yeni onceki; do d=$YENI; [ $e = onceki ] && d=$ONCEKI; echo "node tools/v5-savunma.js '$d/alan-v5' $s 0 > '$OUT/stat/$e-$s-0.json' 2>&1" >> "$J"; done; done
for k in 0 1; do for v in "varsayilan:" "blokDusuk:LBLOK=Düşük" "blokYuksek:LBLOK=Yüksek" "siddet0:LSID=0" "siddet1:LSID=1" "tempo02:LTEMPO=0.2" "tempo08:LTEMPO=0.8" "genDar:LGEN=Dar" "genGenis:LGEN=Geniş"; do ad=${v%%:*}; env=${v#*:}; echo "$env node tools/v5-savunma.js '$YENI/alan-v5' 2 $k > '$OUT/taktik/$ad-$k.json' 2>&1" >> "$J"; done; done
xargs -P"$PAR" -d '\n' -I{} sh -c '{}' < "$J"
date -u +%FT%TZ > "$OUT/bitis.txt"
# 3 · rapor
node tools/kontrol/rapor.js "$OUT" > "$OUT/RAPOR.md"; echo "rapor: $OUT/RAPOR.md"
