#!/usr/bin/env bash
# Şut kestiriminin denetimi: maçlarda atılan her şutta kafanın gol olasılığı (det.Pk) ile 60 tik içinde gol olup
# olmadığı. Tablo: kestirim aralığına ve kaleye uzaklığa göre ortalama kestirim → gerçek. Bir kural değil, denetim:
# kafanın şut benzetimi maçın şut / Bekçi kuralından saparsa burada görünür.
#   sina/sut.sh <kayıt.json>... [ortam değişkenleri motora geçer]  (TIK=5400)
set -euo pipefail
kok="$(cd "$(dirname "$0")/.." && pwd)"
cargo build --release -q -p alan-kos --manifest-path "$kok/Cargo.toml"
kos="$kok/target/release/alan-kos"
gec=$(mktemp -d); trap 'rm -rf "$gec"' EXIT
for f in "$@"; do "$kos" "$f" "${TIK:-5400}" sutlar > "$gec/$(basename "$f").txt" 2>/dev/null & done
wait
python3 - "$gec" <<'P'
import json, glob, math, sys
R = [json.loads(l) for f in glob.glob(sys.argv[1] + '/*.txt') for l in open(f) if l.strip()]
print('şut', len(R), 'gol', sum(r[6] for r in R))
for a, b in [(0, .1), (.1, .3), (.3, .5), (.5, .7), (.7, 1.01)]:
    x = [r for r in R if r[5] == r[5] and a <= r[5] < b]
    if x: print(f"  kestirim {a:.1f}-{b:.1f}: n {len(x):3} ort {sum(r[5] for r in x)/len(x):.2f} gerçek {sum(r[6] for r in x)/len(x):.2f}")
for a, b in [(0, 12), (12, 16), (16, 20), (20, 26), (26, 60)]:
    x = [r for r in R if a <= min(math.hypot(r[3]-120, r[4]-30), math.hypot(r[3], r[4]-30)) < b]
    if x: print(f"  uzaklık {a}-{b}: n {len(x):3} kestirim {sum(r[5] for r in x if r[5]==r[5])/len(x):.2f} gerçek {sum(r[6] for r in x)/len(x):.2f}")
P
