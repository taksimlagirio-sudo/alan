#!/usr/bin/env bash
# Oyuncu'nun alan envanteri (MIMARI.md 3.6): her alanın hangi parçada durduğu (yapı / gövde / maç durumu),
# nerede okunduğu, nerede yazıldığı (katmana göre).
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
# üç parça (MIMARI.md 3.6): yapının kendisi, paylaşımlı gövde (p.govde) ve maç durumu (p.hal)
for y in Oyuncu OyuncuGovde OyuncuMac; do
  awk -v s="$y" '$0=="pub struct "s" {"{f=1;next} f&&/^\}/{exit} f&&/^    pub /{sub(/^    pub /,""); sub(/:.*/,""); print s" "$0}' "$kok/motor/src/model.rs"
done | grep -v ' hal$\| govde$' > "$gec/adlar.txt"
python3 - "$gec/rs/motor/src/model.rs" <<'PY'
import re, sys
p = sys.argv[1]; L = open(p).read().split('\n')
for yapi in ('Oyuncu', 'OyuncuGovde', 'OyuncuMac'):
    i = L.index('pub struct %s {' % yapi); j = i
    while L[j] != '}': j += 1
    for k in range(i + 1, j):
        m = re.match(r'^(    pub )([A-Za-z_][A-Za-z0-9_]*)(:.*)$', L[k])
        if m and m.group(2) not in ('hal', 'govde'): L[k] = m.group(1) + m.group(2) + '__Z' + m.group(3)
open(p, 'w').write('\n'.join(L))
PY
(cd "$gec/rs" && CARGO_TARGET_DIR="$kok/target/envanter" cargo check -p alan-motor --message-format=json 2>/dev/null) > "$gec/hata.json" || true
python3 "$kok/sina/alan-envanter.py" "$gec/adlar.txt" "$gec/hata.json" "$gec/rs" "${1:-}"
