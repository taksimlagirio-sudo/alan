#!/usr/bin/env bash
# Bir taşıyıcı kararının bütün seçeneklerini gerçekte dener (alan-kos dene, 4 süreç): kafanın değeri ve tutma kestirimi
# yanında N farklı zarla süre tik sonra gol / gol yeme / top kaybı oranları. Kafanın seçimleri gerçekle tutarlı mı, gözle.
#   sina/dene.sh <kayıt.json> <tik> [N=24] [süre=90]     (ALAN_DENE_KAFA=kal|mek kopyaların kafası)
set -euo pipefail
kok="$(cd "$(dirname "$0")/.." && pwd)"
kos="$kok/target/release/alan-kos"
k="$1"; t="$2"; n="${3:-24}"; s="${4:-90}"
say=$(ALAN_DENE_LISTE=1 "$kos" "$k" "$t" dene 2>/dev/null)
gec=$(mktemp -d); trap 'rm -rf "$gec"' EXIT
for p in 0 1 2 3; do
  sec=$(seq $p 4 $((say-1)) | paste -sd,)
  [ -z "$sec" ] && continue
  ALAN_DENE_SEC="$sec" "$kos" "$k" "$t" dene "$n" "$s" > "$gec/$p" 2>/dev/null &
done
wait
cat "$gec"/* | python3 -c "import sys,re; L=sys.stdin.read().splitlines(); L.sort(key=lambda l:-float(re.search(r' v +(-?[0-9.]+)',l).group(1))); print(chr(10).join(L))"
