#!/usr/bin/env bash
# Katman kuralı denetimi (MIMARI.md 2.1): her katman yalnız altındakilere bağlanır.
# Şimdilik yalnız tamamlanan katmanlar denetlenir; aşamalar ilerledikçe kural eklenir.
#   sina/katman.sh      kural bozulmuşsa ihlalleri yazar ve 1 ile çıkar
set -euo pipefail
cd "$(dirname "$0")/../motor/src"
hata=0
gec=$(mktemp)
trap 'rm -f "$gec"' EXIT
# katman: dizin  yasak başvurular (üst katmanlar)
denetle() {
  local dizin="$1"; shift
  for yasak in "$@"; do
    if grep -rnE "crate::$yasak\b|super::super::$yasak\b" "$dizin" --include=*.rs >"$gec" 2>/dev/null; then
      echo "İHLAL: $dizin → $yasak"; sed 's/^/   /' "$gec"; hata=1
    fi
  done
}
denetle kural mac sekil kafa paralel paket oyun
[ "$hata" = 0 ] && echo "katman kuralı tutuyor: kural → (mac, sekil, kafa, …) başvurusu yok"
exit $hata
