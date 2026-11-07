#!/usr/bin/env bash
# Evre 3 ölçümü: kaleye ≤25 br taşıyıcı anlarında seçenek dallanması (once, sut, pas, sur, tut; 300 tik).
#   sina/bilgi/secdal.sh <klasör> [an_sayısı=240] [tohumlar="201 … 212"]   (ALAN_* ortamdan; varsayılan analiz/bayraklar.env)
set -euo pipefail
kok="$(cd "$(dirname "$0")/../.." && pwd)"; cd "$kok"
kl="$1"; an="${2:-240}"; TOH="${3:-$(seq -s ' ' 201 212)}"
kos="${KOS:-$kok/target/release/alan-kos}"
BAY="$(grep -v '^#' analiz/bayraklar.env | tr '\n' ' ')"
mkdir -p "$kl"
for s in $TOH; do
  mkdir -p "$kl/t$s"
  python3 -c "import json,sys; v=json.load(open('analiz/mac-217468.json')); v['yeni']['tohum']=int(sys.argv[1]); json.dump(v,open(sys.argv[2],'w'),ensure_ascii=False)" "$s" "$kl/t$s/kayit.json"
done
printf '%s\n' $TOH | xargs -P4 -I{} sh -c "env $BAY '$kos' '$kl/t{}/kayit.json' 9000 analiz > '$kl/t{}/analiz.jsonl' 2>/dev/null; env $BAY '$kos' '$kl/t{}/kayit.json' 9000 kararlar > '$kl/t{}/kararlar.txt' 2>/dev/null"
for s in $TOH; do "$kos" "$kl/t$s/kayit.json" 1 kare "$kl/t$s/ad.json" > /dev/null 2>&1 || true; done
python3 sina/bilgi/secdal.py sec "$kl" "$an"
# her an ayrı süreçte (iş parçacığı önbellekleri anlar arasında taşınmasın: once kolu asıl maçla birebir aynı kalsın)
for s in $TOH; do [ -s "$kl/t$s/tler" ] && tr ',' '\n' < "$kl/t$s/tler" | sed "s|^|$s |"; done | env $BAY KOS="$kos" xargs -P4 -n2 sina/bilgi/secdal_an.sh "$kl"
for s in $TOH; do cat "$kl/t$s"/secdal-*.err > "$kl/t$s/secdal.err" 2>/dev/null || true; done
python3 sina/bilgi/secdal.py rapor "$kl"
