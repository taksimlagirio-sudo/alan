#!/bin/sh
# Tarz matrisi: altı tarzın bütün çiftleri, her çift için N tohum, paralel (P iş). Çıktı: tarz-matrisi.jsonl, sonra özet.
# Kullanım: sh tools/tarz-matrisi-kos.sh <motor klasörü> [N=4] [P=4]   (bayrak: BAYRAK=ALAN_SPRINT_DEGER_ESKI sh ...)
D=$1; N=${2:-4}; P=${3:-4}; OUT=${OUT:-tarz-matrisi.jsonl}; TD=$(cd "$(dirname "$0")" && pwd); T="T_kale T_kontra T_sahip T_presci T_dikine T_kanat"; : > "$OUT"
for a in $T; do for b in $T; do [ "$a" \< "$b" ] || continue; s=1; while [ $s -le $N ]; do echo "$a $b $s"; s=$((s+1)); done; done; done |
  xargs -P "$P" -L 1 sh -c 'node "$1/tarz-matrisi.js" "$2" "$4" "$5" "$6" >> "$3"' sh "$TD" "$D" "$OUT" 2>/dev/null || true
node "$TD/tarz-matrisi-ozet.js" "$OUT"
