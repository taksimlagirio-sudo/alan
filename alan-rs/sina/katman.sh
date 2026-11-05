#!/usr/bin/env bash
# Katman kuralı denetimi (MIMARI.md 2.1): her katman yalnız altındakilere bağlanır.
# Kurallar aşamalar ilerledikçe eklenir; şimdilik modüller fiziksel olarak yerinde, katmanlar dosya düzeyinde.
#   sina/katman.sh      kural bozulmuşsa ihlalleri yazar ve 1 ile çıkar
set -euo pipefail
cd "$(dirname "$0")/../motor/src"
hata=0
gec=$(mktemp)
trap 'rm -f "$gec"' EXIT
# denetle "<açıklama>" "<dosyalar (boşlukla)>" <yasak örüntü>...
denetle() {
  local ad="$1" dosyalar="$2"; shift 2
  local bozuk=0
  for yasak in "$@"; do
    # shellcheck disable=SC2086
    if grep -nE "$yasak" $dosyalar >"$gec" 2>/dev/null; then
      echo "İHLAL ($ad): '$yasak'"; sed 's/^/   /' "$gec"; hata=1; bozuk=1
    fi
  done
  [ "$bozuk" = 0 ] && echo "tutuyor: $ad"
  return 0
}
# 1. kural katmanı yalnız temele bağlı
denetle "kural → üst katmanlar yok" "$(find kural -name '*.rs')" \
  'crate::(mac|sekil|kafa|paralel|paket|oyun)\b'
# 2. yerleşim ve değerlendirme kararın dosyalarına bağlanmaz (Aşama 2).
#    Henüz istisna: kafa2_baglam (önbellek tipleri, Aşama 3), gecici_dis1 (Dis köprüsü), kafa/mod.rs.
degerlendirme="$(find sekil -name '*.rs') $(ls kafa/kafa1_*.rs) kafa/kafa2_zemin.rs kafa/kafa2_imkan.rs kafa/kafa2_dunya.rs kafa/kafa2_bag.rs"
denetle "yerleşim + değerlendirme → karar (kafa2_karar, kafa2_tarti, kafa2_eski) yok" "$degerlendirme" \
  'kafa2_(karar|tarti|eski)\b'
# 3. değerlendirme (yerleşimin hizmetleri dahil) yerleşimin kararına (sekil::konum, position) bağlanmaz
hizmet="$(ls kafa/kafa1_*.rs) kafa/kafa2_zemin.rs kafa/kafa2_imkan.rs kafa/kafa2_dunya.rs kafa/kafa2_bag.rs sekil/yer.rs sekil/pres.rs sekil/takim.rs sekil/ucus.rs sekil/gor.rs sekil/varlik.rs sekil/sabit.rs sekil/gecici_dis.rs"
denetle "değerlendirme → yerleşimin kararı (sekil::konum) yok" "$hizmet" '(sekil|super)::(konum|position)\b|\bkonum::'
# 4. karar yerleşimin kararına bağlanmaz (yerleşimin hizmetlerini kullanabilir)
denetle "karar → yerleşimin kararı (sekil::konum) yok" "kafa/kafa2_karar.rs kafa/kafa2_tarti.rs kafa/kafa2_eski.rs" '(sekil|super)::(konum|position)\b|\bkonum::'
exit $hata
