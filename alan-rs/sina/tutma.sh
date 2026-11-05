#!/usr/bin/env bash
# Kafanın sürmede tutma kestirimini gerçekleşenle karşılaştırır (alan-kos tutma, kilitli: taşıyıcı sürme boyunca
# yeniden karar vermez). sina/ayni/liste.txt'teki maçlar, kaleye 35 br içindeki kararlar, her karar için kafanın ilk
# dört sürme seçeneği kopya maçta zorla oynatılır. Tablo: kestirim aralığı, sayı, ortalama kestirim → gerçek tutma.
#
#   sina/tutma.sh [tik]        varsayılan 2700 tik; ortam değişkenleri (ALAN_*) motora geçer
#   ESIK=0.15 sina/tutma.sh    ortalama |kestirim − gerçek| eşiği aşarsa 1 ile çıkar
#
# Bu bir kural değil, denetimdir: motorun içine kalibrasyon sayısı konmaz; kafanın benzetimi maçın kurallarından
# saparsa (ör. savunma ya da temas kuralı değişip benzetim değişmezse) burada görünür.
set -euo pipefail
kok="$(cd "$(dirname "$0")/.." && pwd)"
cd "$kok"
tik="${1:-2700}"
gec=$(mktemp -d)
trap 'rm -rf "$gec"' EXIT
cargo build --release -p alan-kos >"$gec/derleme.txt" 2>&1 || { cat "$gec/derleme.txt"; echo "DERLENMEDİ"; exit 2; }
kos="$kok/target/release/alan-kos"
mapfile -t maclar < <(grep -v '^\s*#' sina/ayni/liste.txt | grep -v '^\s*$' | awk '{print $1}')
for ad in "${maclar[@]}"; do
  ALAN_TUTMA_KILIT=1 "$kos" "../anlar/$ad.json" "$tik" tutma > "$gec/$ad.tut" 2>/dev/null &
done
wait
python3 - "$gec" "${ESIK:-}" <<'P'
import json, glob, collections, sys
gec, esik = sys.argv[1], sys.argv[2]
k = collections.defaultdict(collections.Counter); ps = collections.defaultdict(float)
for f in glob.glob(gec + '/*.tut'):
    for l in open(f):
        r = json.loads(l); P, son = r[5], r[9]
        if P is None: continue
        b = min(int(P * 5), 4); k[b][son] += 1; ps[b] += P
N = sum(sum(c.values()) for c in k.values())
if not N: print('ölçülecek sürme yok'); sys.exit(2)
e = 0.0
print('kestirim   sayı  kestirim → gerçek tutma')
for b in range(5):
    n = sum(k[b].values())
    if not n: continue
    g = 1 - k[b]['kayip'] / n; e += n * abs(ps[b] / n - g)
    print(f'{b*0.2:.1f}–{b*0.2+0.2:.1f}  {n:5d}    {ps[b]/n:.2f} → {g:.2f}')
e /= N
print(f'ortalama |kestirim − gerçek| = {e:.3f} ({N} sürme)')
if esik and e > float(esik): print(f'EŞİK AŞILDI ({esik})'); sys.exit(1)
P
