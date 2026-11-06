#!/usr/bin/env bash
# Çok tohumlu ölçüm: aynı taktik/kadro, N tohum; her tohumun ozet.json'u ve ortalaması. Kullanım: analiz/cok.sh <çıktı> [tohumlar...]
# Varsayılan tohumlar: 217468 1 2 3 4 5. Ortam: ALAN_* bayrakları geçer (ör. ALAN_SAH_YAKINSA_IZ=1 analiz/cok.sh out/sahyak).
set -e; cd "$(dirname "$0")/.."
OUT=$1; shift; T=${*:-217468 1 2 3 4 5}; mkdir -p "$OUT"
for t in $T; do TOHUM=$t analiz/olc.sh "$OUT/t$t" > "$OUT/t$t.log" 2>&1; python3 analiz/py/ozet.py "$OUT/t$t" | head -1 > "$OUT/t$t.json"; echo "tohum $t: $(cat $OUT/t$t.json)"; done
python3 - "$OUT" $T <<'P'
import json,sys,re
out=sys.argv[1]; R=[json.load(open(f'{out}/t{t}.json')) for t in sys.argv[2:]]
print('ORTALAMA (%d tohum):'%len(R))
for k in R[0]:
  v=[r[k] for r in R]
  if all(isinstance(x,(int,float)) for x in v): print(f'  {k}: {sum(v)/len(v):.1f}')
  else: print(f'  {k}: {v}')
P
