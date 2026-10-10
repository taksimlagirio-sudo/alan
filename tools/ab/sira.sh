#!/bin/sh
# 8. madde iş sırası (yeniden başlatılabilir: biten işler atlanır). Çalıştırma: sh tools/ab/sira.sh (depo kökünden)
set -e; cd "$(dirname "$0")/../.."; L=lab/ab; mkdir -p $L/e1 $L/e2 $L/ogren $L/mac
while pgrep -f "tools/ab/topla.js" > /dev/null; do sleep 20; done
echo "$(date +%T) aşama 1: değerlendirme"
[ -f $L/anlar.txt ] || node tools/ab/sec-anlar.js alan-v4g $L/anlar 24 > $L/anlar.txt
cat $L/anlar.txt | xargs -P4 -I{} sh -c "[ -s $L/e1/{}.jsonl ] || node tools/ab/degerlendir.js alan-v4g $L/anlar/{} $L/e1/{}.jsonl 6 30 6 >> $L/e1.log 2>&1"
echo "$(date +%T) aşama 2: hafıza (4 tur × 26 maç)"
for r in 1 2 3 4; do a=$(( (r-1)*26+1 )); b=$(( r*26 )); p=$((r-1)); T=""; [ $p -gt 0 ] && T="$L/tablo-r$p.json A"
  [ -f $L/tablo-r$r.json ] && continue
  seq $a $b | xargs -P4 -I{} sh -c "[ -s $L/ogren/o{}.json ] || node tools/ab/ogren-ab.js alan-v4g {} $L/ogren/o{}.json $T >> $L/ogren.log 2>&1"
  node tools/ab/tablo-kur.js $L/tablo-r$r.json $L/ogren/o*.json > $L/tablo-r$r.txt; echo "$(date +%T) tur $r bitti"; done
echo "$(date +%T) aşama 3: A seçimleri"
cat $L/anlar.txt | xargs -P4 -I{} sh -c "[ -s $L/e2/{}.jsonl ] || ASAMA2=$L/e1/{}.jsonl node tools/ab/degerlendir.js alan-v4g $L/anlar/{} $L/e2/{}.jsonl 6 30 6 $L/tablo-r4.json >> $L/e2.log 2>&1"
echo "$(date +%T) aşama 4: maçlar"
( for i in $(seq 1 10); do echo "B $i"; done; for y in A A0 C; do for i in $(seq 1 10); do echo "$y $i"; done; done ) | xargs -P4 -L1 sh -c "[ -s $L/mac/\$0_\$1.json ] || node tools/ab/mac.js alan-v4g \$1 \$0 $L/tablo-r4.json 4 > $L/mac/\$0_\$1.json 2>> $L/mac.log"
echo "$(date +%T) bitti"
