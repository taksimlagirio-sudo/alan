#!/usr/bin/env bash
# Eşli dal: aynı andan zorlamasız (once) ve S7 bitli kollar, her kolda en geri savunmacının arkasına geçen rakip oranı.
#   sina/bilgi/dalarka.sh <klasör> "<tohumlar>" "<çatallar>" <süre> "<bitler,virgüllü>"
# Ortak bayraklar <klasör>/bayraklar.env (yoksa analiz/bayraklar.env kopyalanır). Her tohumun taban maçı da koşulur:
# once kolu taban maçtan ayrışırsa o çatal geçersiz (dalarka.py sayar).
set -euo pipefail
kok="$(cd "$(dirname "$0")/../.." && pwd)"; cd "$kok"
kl="$1"; TOH="$2"; CAT="$3"; sure="$4"; bit="$5"
kos="${KOS:-$kok/target/release/alan-kos}"
mkdir -p "$kl"; [ -f "$kl/bayraklar.env" ] || cp analiz/bayraklar.env "$kl/bayraklar.env"
BAY="$(grep -v '^#' "$kl/bayraklar.env" | tr '\n' ' ')"
son=0; for t in $CAT; do [ "$t" -gt "$son" ] && son=$t; done
for s in $TOH; do python3 -c "import json,sys; v=json.load(open('analiz/mac-217468.json')); v['yeni']['tohum']=int(sys.argv[1]); json.dump(v,open(sys.argv[2],'w'),ensure_ascii=False)" "$s" "$kl/kayit-$s.json"; done
{ for s in $TOH; do echo "taban $s"; for t in $CAT; do echo "$s $t"; done; done; } | xargs -P4 -L1 sh -c '
  if [ "$0" = taban ]; then [ -s "'"$kl"'/taban-$1.jsonl" ] || env '"$BAY"' "'"$kos"'" "'"$kl"'/kayit-$1.json" '"$((son + sure))"' analiz > "'"$kl"'/taban-$1.jsonl" 2>/dev/null
  else [ -s "'"$kl"'/d-$0-$1/once.jsonl" ] || env '"$BAY"' "'"$kos"'" "'"$kl"'/kayit-$0.json" $1 dal "'"$kl"'/d-$0-$1" '"$sure"' "'"$bit"'" >/dev/null 2>&1; fi'
python3 analiz/py/dalarka.py "$kl" | tee "$kl/ozet.txt"
