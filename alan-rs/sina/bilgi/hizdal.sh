#!/usr/bin/env bash
# Kontra hızının nedensel ölçümü (analizci, A2): kontra anlarında koşucu N tik depar / tempo (alan-kos hizdal), iki kolun farkı.
#   sina/bilgi/hizdal.sh <klasör> [an_sayısı=200] [tohumlar="301 … 316"] [N=90]   (ALAN_* ortamdan; varsayılan analiz/bayraklar.env)
# secdal.sh'in kalıbı: her an ayrı süreçte (iş parçacığı önbellekleri anlar arasında taşınmasın).
set -euo pipefail
kok="$(cd "$(dirname "$0")/../.." && pwd)"; cd "$kok"
kl="$1"; an="${2:-200}"; TOH="${3:-$(seq -s ' ' 301 316)}"; N="${4:-90}"
kos="${KOS:-$kok/target/release/alan-kos}"
mkdir -p "$kl"
# bayrak dosyasının kopyası: bütün maçlar ve kollar bununla (koşu sırasında analiz/bayraklar.env değişse de); varsa onu kullan
[ -f "$kl/bayraklar.env" ] || cp "${BAYRAK:-analiz/bayraklar.env}" "$kl/bayraklar.env"
BAY="$(grep -v '^#' "$kl/bayraklar.env" | tr '\n' ' ')"
for s in $TOH; do
  mkdir -p "$kl/t$s"
  python3 -c "import json,sys; v=json.load(open('analiz/mac-217468.json')); v['yeni']['tohum']=int(sys.argv[1]); json.dump(v,open(sys.argv[2],'w'),ensure_ascii=False)" "$s" "$kl/t$s/kayit.json"
done
printf '%s\n' $TOH | xargs -P4 -I{} sh -c "[ -s '$kl/t{}/analiz.jsonl' ] || env $BAY '$kos' '$kl/t{}/kayit.json' 9000 analiz > '$kl/t{}/analiz.jsonl' 2>/dev/null"
python3 sina/bilgi/hizdal.py sec "$kl" "$an"
for s in $TOH; do [ -s "$kl/t$s/tler" ] && paste -d' ' <(tr ',' '\n' < "$kl/t$s/tler") <(tr ',' '\n' < "$kl/t$s/oy") | sed "s|^|$s |"; done \
  | xargs -P4 -n3 sh -c "env $BAY '$kos' \"$kl/t\$0/kayit.json\" \$1 hizdal \"$kl/t\$0\" 360 \$2 $N 2>/dev/null < /dev/null; :" 2>/dev/null || true
python3 sina/bilgi/hizdal.py rapor "$kl"
python3 sina/bilgi/hizdal.py ozet "$kl" "$kl/ozet.json"
