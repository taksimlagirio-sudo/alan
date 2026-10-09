#!/bin/sh
# Pasiflik teşhisi için dört motor: taban (alan-v3f), valtab (ölçülmüş değer tablosu açık),
# kayipegri (kaybın bedeli de elle yazılmış eğriden — SADECE TEŞHİS, kalıcı değil), tuteski (yeni "tut" değeri geri alınmış).
set -e; cd "$(dirname "$0")/.."; D=deney/pasiflik; rm -rf $D; mkdir -p $D
for v in taban valtab kayipegri tuteski; do cp -r alan-v3f $D/$v; done
sed -i 's/useValTab: false/useValTab: true/' $D/valtab/decide.js
sed -i 's/const lossAt = (x, y) => Vo(src, x, y) \* risk;/const lossAt = (x, y) => (D.b0 + D.b1 * threat(1 - team, x, y)) * risk;/' $D/kayipegri/decide.js
python3 - "$D/tuteski/decide.js" <<'PY'
import sys; f=sys.argv[1]; s=open(f).read()
a="{ const tu = opts.find(o => o.kind === 'tut' && o._w), gs = opts.filter(o => o.kind === 'gönder' && o.v != null); if (tu && gs.length) { const bestS = Math.max(...gs.map(o => o.v)), P = tu.det.P; tu.v = P * Math.max(build(tu._w, team, h.x, h.y), bestS) - (1 - P) * lossAt(h.x, h.y); } }"
assert s.count(a)==1; open(f,'w').write(s.replace(a,''))
PY
for v in valtab kayipegri tuteski; do cmp -s alan-v3f/decide.js $D/$v/decide.js && { echo "$v değişmedi!"; exit 1; }; done; echo kuruldu
