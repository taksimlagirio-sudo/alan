#!/usr/bin/env bash
# Maç 217468'i (Liman – Kale Duvarı) güncel motorla oynatır ve davranış ölçümlerini yazar.
# Kullanım: alan-rs/analiz/olc.sh [çıktı klasörü]   (varsayılan alan-rs/analiz/out, git'e girmez)
set -e
cd "$(dirname "$0")/.."
OUT=${1:-analiz/out}; mkdir -p "$OUT"
cargo build --release -q -p alan-kos
./target/release/alan-kos analiz/mac-217468.json 9000 analiz > "$OUT/analiz.jsonl" 2> "$OUT/analiz.err" &
./target/release/alan-kos analiz/mac-217468.json 9000 kararlar > "$OUT/kararlar.txt" 2> "$OUT/kararlar.err" &
wait
cat "$OUT/analiz.err"
export ANALIZ_OUT="$(cd "$OUT" && pwd)"
for b in zincir pres sut pas oyuncu sekil firsat topsuz; do python3 analiz/py/$b.py > "$OUT/$b.txt"; done
echo "--- baskı";  cat "$OUT/pres.txt"
echo "--- şekil";  cat "$OUT/sekil.txt"
echo "--- topsuz"; cat "$OUT/topsuz.txt"
echo "--- donma (değeri ≈0 kararlar)"
python3 - "$OUT/kararlar.txt" <<'P'
import json,sys,collections
c=collections.Counter();n=0
for l in open(sys.argv[1]):
  r=json.loads(l);n+=1;v=r[5][0][2]
  if v is not None and abs(v)<1e-6: c[r[1]]+=1
print(sum(c.values()),'/',n,dict(c))
P
