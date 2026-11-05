#!/usr/bin/env bash
# Birebir aynılık denetimi: motoru derler, sina/ayni/liste.txt'teki maçları oynatır ve her tikin parmak
# izini (motor/src/parmak.rs) kayıtlı başvuruyla (sina/ayni/<maç>.pk) karşılaştırır.
#
#   sina/ayni.sh          denetle: hepsi aynıysa 0 ile çıkar, değilse ilk ayrışan tiki yazar ve 1 ile çıkar
#   sina/ayni.sh --yaz    başvuruları yeniden yaz (yalnız davranışı bilerek değiştiren bir değişiklikten sonra)
#
# "Davranış aynı" diyen her değişiklik bu denetimden geçmeli.
set -euo pipefail
kok="$(cd "$(dirname "$0")/.." && pwd)"
cd "$kok"
yaz=0
[ "${1:-}" = "--yaz" ] && yaz=1
gec=$(mktemp -d)
trap 'rm -rf "$gec"' EXIT
if ! cargo build --release -p alan-kos >"$gec/derleme.txt" 2>&1; then
  cat "$gec/derleme.txt"; echo "DERLENMEDİ"; exit 2
fi
kos="$kok/target/release/alan-kos"
mapfile -t maclar < <(grep -v '^\s*#' sina/ayni/liste.txt | grep -v '^\s*$')
for satir in "${maclar[@]}"; do
  read -r ad tik <<<"$satir"
  "$kos" "../anlar/$ad.json" "$tik" parmak > "$gec/$ad.pk" 2>/dev/null &
done
wait
hata=0
for satir in "${maclar[@]}"; do
  read -r ad tik <<<"$satir"
  if [ "$yaz" = 1 ]; then
    cp "$gec/$ad.pk" "sina/ayni/$ad.pk"
    echo "yazıldı: $ad ($tik tik)"
  elif [ ! -f "sina/ayni/$ad.pk" ]; then
    echo "BAŞVURU YOK: $ad (önce: sina/ayni.sh --yaz)"; hata=1
  elif cmp -s "$gec/$ad.pk" "sina/ayni/$ad.pk"; then
    echo "aynı: $ad ($tik tik)"
  else
    # diff fark bulunca 1 döner, head boruyu erken kapatır: pipefail altında betik burada sessizce çıkmasın
    ilk=$( (diff "sina/ayni/$ad.pk" "$gec/$ad.pk" || true) | awk '/^</ && !f {print $2; f=1}')
    echo "AYRIŞTI: $ad, ilk farklı tik $ilk"; hata=1
  fi
done
exit $hata
