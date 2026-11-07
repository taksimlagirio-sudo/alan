#!/usr/bin/env bash
# Bilgi tabloları (analiz/TASARIM-BILGI.md): motor/veri/bilgi/<ad>.json, tarifleri sina/bilgi/<ad>.conf.
#   sina/bilgi.sh                       denetler: kural ya da davranış izi değişen tablo "ESKİ" (1 ile çıkar); kendiliğinden üretmez
#   sina/bilgi.sh --uret <ad> [tur]     üretir: tohumlarla maç oynatır, ölçer, eskisiyle %50 harmanlar, değişim ESIK'in
#                                       altına inene dek (en çok `tur`, varsayılan 4) yineler; her turda en büyük değişimi yazar
# Üretim maçları analiz/bayraklar.env + tablonun kendi bayrağıyla (ALAN_BILGI_<AD>_IZ) oynar: tablo davranışı, davranış tabloyu
# değiştirir (sabit nokta). Ortamdaki ALAN_* da geçer.
set -euo pipefail
kok="$(cd "$(dirname "$0")/.." && pwd)"; cd "$kok"
kos="$kok/target/release/alan-kos"
bayrak_ad() { echo "ALAN_BILGI_$(echo "$1" | tr '[:lower:]' '[:upper:]')_IZ"; }
kural_izi() { ( for f in $KURAL; do cat "$f"; done ) | sha256sum | cut -c1-12; }
davranis_izi() { env $(grep -v '^#' analiz/bayraklar.env) "$(bayrak_ad "$1")=1" ${EK:-} "$kos" ../anlar/a-107-1-3464-kontra.json 900 parmak 2>/dev/null | sha256sum | cut -c1-12; }
alan() { python3 -c "import json,sys; print(json.load(open(sys.argv[1])).get(sys.argv[2],''))" "$1" "$2"; }

if [ "${1:-}" = "--uret" ]; then
  ad="$2"; tur="${3:-4}"; EK=""; IZ=""; source "sina/bilgi/$ad.conf"
  tablo="motor/veri/bilgi/$ad.json"; gec=$(mktemp -d); trap 'rm -rf "$gec"' EXIT
  for t in $(seq 1 "$tur"); do
    cargo build --release -q -p alan-kos
    ls=()
    for s in $TOHUMLAR; do
      k="$gec/kayit-$s.json"
      python3 -c "import json,sys; v=json.load(open('analiz/mac-217468.json')); v['yeni']['tohum']=int(sys.argv[1]); json.dump(v,open(sys.argv[2],'w'),ensure_ascii=False)" "$s" "$k"
      ls+=("$gec/a-$s.jsonl")
    done
    printf '%s\n' $TOHUMLAR | xargs -P4 -I{} sh -c "env $(grep -v '^#' analiz/bayraklar.env | tr '\n' ' ') $(bayrak_ad "$ad")=1 ${EK:-} '$kos' '$gec/kayit-{}.json' 9000 analiz > '$gec/a-{}.jsonl' 2>\$( [ -n '${IZ:-}' ] && echo '$gec/a-{}.iz' || echo /dev/null )"
    meta=$(python3 -c "import json,sys; print(json.dumps({'k':float(sys.argv[1]),'tohumlar':[int(x) for x in sys.argv[2].split()],'bayraklar':sys.argv[3].split()}))" "$K" "$TOHUMLAR" "$(grep -v '^#' analiz/bayraklar.env | tr '\n' ' ') $(bayrak_ad "$ad")=1 ${EK:-}")
    BILGI_META="$meta" python3 "$URETICI" "$gec/yeni.json" $ARG "${ls[@]}"
    fark=$(python3 - "$tablo" "$gec/yeni.json" <<'P'
import json,sys,os
sys.path.insert(0,'sina/bilgi')
from ortak import harmanla
eski,yeni=sys.argv[1],sys.argv[2]
y=json.load(open(yeni))
d,f=harmanla(eski,y['deger'],y['n'])
y['deger']=d
if os.path.exists(eski): y['surum']=json.load(open(eski)).get('surum',0)+1
json.dump(y,open(eski,'w'),ensure_ascii=False)
print('ilk' if f is None else f'{f:.4f}')
P
)
    echo "tur $t: $ad en büyük değişim $fark ($(alan "$tablo" mac) maç)"
    if [ "$fark" != "ilk" ] && python3 -c "import sys; sys.exit(0 if float(sys.argv[1])<float(sys.argv[2]) else 1)" "$fark" "$ESIK"; then break; fi
  done
  cargo build --release -q -p alan-kos
  python3 - "$tablo" "$(kural_izi)" "$(davranis_izi "$ad")" <<'P'
import json,sys
j=json.load(open(sys.argv[1])); j['kural_izi']=sys.argv[2]; j['davranis_izi']=sys.argv[3]
json.dump(j,open(sys.argv[1],'w'),ensure_ascii=False)
P
  cargo build --release -q -p alan-kos
  echo "üretildi: $tablo (kural izi $(alan "$tablo" kural_izi), davranış izi $(alan "$tablo" davranis_izi))"
  exit 0
fi

cargo build --release -q -p alan-kos
hata=0
for c in sina/bilgi/*.conf; do
  ad=$(basename "$c" .conf); EK=""; source "$c"; tablo="motor/veri/bilgi/$ad.json"
  if [ ! -f "$tablo" ]; then echo "$ad: TABLO YOK"; hata=1; continue; fi
  ki=$(kural_izi); di=$(davranis_izi "$ad")
  tk=$(alan "$tablo" kural_izi); td=$(alan "$tablo" davranis_izi)
  if [ "$ki" = "$tk" ] && [ "$di" = "$td" ]; then
    echo "$ad: güncel (sürüm $(alan "$tablo" surum), $(alan "$tablo" mac) maç)"
  else
    neden=""; [ "$ki" != "$tk" ] && neden="kural izi $tk → $ki"; [ "$di" != "$td" ] && neden="$neden davranış izi $td → $di"
    echo "$ad: ESKİ ($neden) — sina/bilgi.sh --uret $ad"; hata=1
  fi
done
exit $hata
