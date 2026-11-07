#!/usr/bin/env python3
# Harita ölçeği (tutma tablosunun ölçek kipi, ALAN_BILGI_TUTMA_OLCEK_IZ): kafanın sahiplik haritasının (zemin_s) taşıyıcı karar
# anlarındaki ortalaması, tutma tablosuyla aynı hücrelerde. Okuyucu: ölçek = tutma / harita (hücre içi sıralama haritadan, ölçek tablodan).
# Örnek: bir taşıyıcı kararı (HARITA izi, ALAN_SUT_SAY_IZ); n = karar sayısı.
#   harita.py <çıktı.json> <analiz.jsonl>...   (her analiz dosyasının yanında aynı adlı .iz: HARITA satırları)
import json, sys, math, os
sys.path.insert(0, os.path.dirname(__file__))
from ortak import Izgara, yaz
W, CY = 120.0, 30.0
cikti = sys.argv[1]
eksen = [("uzaklik", list(range(0, 66, 6))), ("onde", list(range(0, 7)))]
g = Izgara(eksen); mac = 0
for f in sys.argv[2:]:
    iz = f[:-len('.jsonl')] + '.iz'
    if not os.path.exists(iz): continue
    F = {}
    for l in open(f):
        r = json.loads(l); F[r['t']] = r
    if not F: continue
    mac += 1
    for l in open(iz):
        p = l.split()
        if len(p) != 5 or p[0] != 'HARITA': continue
        t, d, S = int(p[1]), float(p[3]), float(p[4])
        for tt in (t, t - 1, t + 1):
            r = F.get(tt)
            if not r or r['h'] is None or r['h'] < 0: continue
            h = r['h']; tm = 0 if h < 7 else 1; gx = W if tm == 0 else 0.0
            x, y = r['p'][h][0], r['p'][h][1]
            if abs(math.hypot(gx - x, CY - y) - d) > 1.0: continue
            rak = range(8, 14) if tm == 0 else range(1, 7)
            onde = sum(1 for q in rak if abs(gx - r['p'][q][0]) < abs(gx - x))
            g.ekle([d, onde], S); break
meta = json.loads(os.environ.get("BILGI_META", "{}")); meta["mac"] = mac
meta.setdefault("aciklama", "taşıyıcı karar anında kafanın sahiplik haritası (zemin_s) ortalaması (n: karar)")
yaz(cikti, "harita", eksen, g.yumusat(60), g.N, meta)
