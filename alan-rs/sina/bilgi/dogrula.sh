#!/usr/bin/env bash
# Savunma deneylerinin doğrulaması: kollar × tohumlar, her maç ayrı süreç; ölçüler gidip.py / ters.py / hat.py / tehlike_yuru.py.
#   sina/bilgi/dogrula.sh <klasör> <tik> "<tohumlar>" "<kol1>=<ENV…>" "<kol2>=<ENV…>" …   (ör. "s7q=ALAN_S7Q_IZ=1")
# Kolların ortak bayrakları <klasör>/bayraklar.env (yoksa analiz/bayraklar.env kopyalanır; BAYRAK=<yol> ile başka dosya).
set -euo pipefail
kok="$(cd "$(dirname "$0")/../.." && pwd)"; cd "$kok"
kl="$1"; tik="$2"; TOH="$3"; shift 3
kos="${KOS:-$kok/target/release/alan-kos}"
mkdir -p "$kl"; [ -f "$kl/bayraklar.env" ] || cp "${BAYRAK:-analiz/bayraklar.env}" "$kl/bayraklar.env"
BAY="$(grep -v '^#' "$kl/bayraklar.env" | tr '\n' ' ')"
for s in $TOH; do python3 -c "import json,sys; v=json.load(open('analiz/mac-217468.json')); v['yeni']['tohum']=int(sys.argv[1]); json.dump(v,open(sys.argv[2],'w'),ensure_ascii=False)" "$s" "$kl/kayit-$s.json"; done
for k in "$@"; do ad="${k%%=*}"; ek="${k#*=}"; [ "$ek" = "$k" ] && ek=""; for s in $TOH; do echo "$ad $s $ek"; done; done \
  | xargs -P4 -L1 sh -c "ad=\$0; s=\$1; shift 2; [ -s \"$kl/\$ad-\$s.jsonl\" ] || env $BAY \"\$@\" '$kos' \"$kl/kayit-\$s.json\" $tik analiz > \"$kl/\$ad-\$s.jsonl\" 2>/dev/null"
{ for k in "$@"; do ad="${k%%=*}"; echo "== $ad ($k)";
  python3 analiz/py/ters.py "$kl/$ad-*.jsonl"; python3 analiz/py/gidip.py "$kl/$ad-*.jsonl"; python3 analiz/py/hat.py "$kl/$ad-*.jsonl"; python3 analiz/py/tehlike_yuru.py "$kl/$ad-*.jsonl"; done; } | tee "$kl/ozet.txt"
