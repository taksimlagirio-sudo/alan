#!/usr/bin/env bash
# Şut tablosunun denetimi: kafanın şut modelinin bugünkü parmak izi (alan-kos sutizi) tablonun üretildiği izle
# (motor/src/kafa/kafa2_imk_sut.bin.iz) aynı mı. Değilse tablo eskidir: şut ya da Bekçi kuralı değişti, tablo bu
# kuraldan yeniden üretilmeli. Ortam değişkenleri (ALAN_*) motora geçer: deney bayraklarıyla modelin izi değişir.
#   sina/tablo.sh                  denetler (eskiyse 1 ile çıkar)
#   sina/tablo.sh --uret <çıktı>   tabloyu bugünkü modelle üretir (4 süreç, ~3 dk) ve yanına izini yazar
set -euo pipefail
kok="$(cd "$(dirname "$0")/.." && pwd)"
cd "$kok"
cargo build --release -q -p alan-kos
kos="$kok/target/release/alan-kos"
kayit="../anlar/a-107-1-3464-kontra.json"
if [ "${1:-}" = "--uret" ]; then
  cikti="$2"; gec=$(mktemp -d); trap 'rm -rf "$gec"' EXIT
  for p in 0 1 2 3; do "$kos" "$kayit" 2 imktablo "$gec/$p.bin" $((p*4)) $((p*4+4)) 2>/dev/null & done
  wait
  cat "$gec"/0.bin "$gec"/1.bin "$gec"/2.bin "$gec"/3.bin > "$cikti"
  "$kos" "$kayit" 2 sutizi 2>/dev/null > "$cikti.iz"
  echo "üretildi: $cikti (iz $(cat "$cikti.iz"))"
  exit 0
fi
simdi=$("$kos" "$kayit" 2 sutizi 2>/dev/null)
izf="motor/src/kafa/kafa2_imk_sut.bin.iz"
tablo=$( [ -f "$izf" ] && cat "$izf" || echo "izsiz")
if [ "$simdi" = "$tablo" ]; then
  echo "şut tablosu güncel (iz $simdi)"
else
  echo "ŞUT TABLOSU ESKİ: tablo $tablo, şut modeli $simdi — sina/tablo.sh --uret ile yeniden üretilmeli"
  exit 1
fi
