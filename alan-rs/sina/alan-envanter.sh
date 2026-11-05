#!/usr/bin/env bash
# Oyuncu'nun alan envanteri (MIMARI.md 3.6): her alanın nerede okunduğu, nerede yazıldığı (katmana göre).
# Yöntem: geçici bir kopyada Oyuncu'nun bütün alanları yeniden adlandırılır; derleyicinin her hatası bir
# kullanım yeridir (adı ortak alanlar, ör. x, başka yapılarınkiyle karışmaz). Yazma: =, +=, &mut, take() …
#
#   sina/alan-envanter.sh            özet (alan başına okuma sayısı, katmana göre yazma)
#   sina/alan-envanter.sh --yer      yazma yerleriyle
set -euo pipefail
kok="$(cd "$(dirname "$0")/.." && pwd)"
gec=$(mktemp -d)
trap 'rm -rf "$gec"' EXIT
mkdir -p "$gec/rs"
(cd "$kok" && tar --exclude=./target -cf - .) | (cd "$gec/rs" && tar -xf -)
awk '/^pub struct Oyuncu \{/{f=1;next} f&&/^\}/{exit} f&&/^    pub /{print}' "$kok/motor/src/model.rs" | sed 's/^    pub //;s/:.*//' > "$gec/adlar.txt"
python3 - "$gec/rs/motor/src/model.rs" <<'PY'
import re, sys
p = sys.argv[1]; L = open(p).read().split('\n')
i = L.index('pub struct Oyuncu {'); j = i
while L[j] != '}': j += 1
for k in range(i + 1, j):
    m = re.match(r'^(    pub )([A-Za-z_][A-Za-z0-9_]*)(:.*)$', L[k])
    if m: L[k] = m.group(1) + m.group(2) + '__Z' + m.group(3)
open(p, 'w').write('\n'.join(L))
PY
(cd "$gec/rs" && CARGO_TARGET_DIR="$kok/target/envanter" cargo check -p alan-motor --message-format=json 2>/dev/null) > "$gec/hata.json" || true
python3 "$kok/sina/alan-envanter.py" "$gec/adlar.txt" "$gec/hata.json" "$gec/rs" "${1:-}"
