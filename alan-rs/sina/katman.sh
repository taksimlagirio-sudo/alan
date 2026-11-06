#!/usr/bin/env bash
# Katman kuralı denetimi (MIMARI.md 2.1): her katman yalnız altındakilere bağlanır.
# Aşama 7c'den beri katmanlar arası kuralı (1, 1b, 5, 6) derleyici de zorlar (her katman ayrı crate, katman/);
# burada kalmaları ucuz bir ikinci denetim. Katman içi kurallar (2, 3, 4: kafa ve şekil içindeki bölümler) yalnız burada.
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
# denetle_izinli "<açıklama>" "<dosyalar>" "<izinli örüntü>" <yasak örüntü>...: izinli örüntüye uyan satırlar sayılmaz
denetle_izinli() {
  local ad="$1" dosyalar="$2" izin="$3"; shift 3
  local bozuk=0
  for yasak in "$@"; do
    # shellcheck disable=SC2086
    if grep -nE "$yasak" $dosyalar 2>/dev/null | grep -vE "$izin" >"$gec"; then
      echo "İHLAL ($ad): '$yasak'"; sed 's/^/   /' "$gec"; hata=1; bozuk=1
    fi
  done
  [ "$bozuk" = 0 ] && echo "tutuyor: $ad"
  return 0
}
# 1. kural katmanı yalnız temele bağlı
denetle "kural → üst katmanlar yok" "$(find kural -name '*.rs')" \
  'crate::(mac|sekil|kafa|paralel|paket|oyun)\b'
# 1b. dünya katmanı yalnız temele bağlı
denetle "dunya → üst katmanlar yok" "$(find dunya -name '*.rs')" \
  'crate::(mac|sekil|kafa|kural|paralel|paket|oyun)\b'
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
# 5. temel katman (kökteki dosyalar) üst katmanlara bağlanmaz (Aşama 7a; MacEk Mac'ten çıktı, istisna yok).
temel="$(ls *.rs | grep -vE '^(lib|oyun|paket|paralel|parmak)\.rs$')"
denetle "temel → maç / yerleşim / kafa yok" "$temel" 'crate::(mac|sekil|kafa)::'
# 6. yerleşim maça bağlanmaz (Aşama 7b; kafaya maçın verdiği el ile ulaşır: Mc::kafa).
denetle "yerleşim → maç yok" "$(find sekil -name '*.rs')" 'crate::mac::'
exit $hata
