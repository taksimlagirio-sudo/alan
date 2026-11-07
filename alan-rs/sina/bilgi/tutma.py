#!/usr/bin/env python3
# A1 tutma değeri: topa sahip takım bu durumda N sn içinde gol atar mı (motorun kendi maçlarından).
# Hücre: uzaklik (taşıyıcının rakip kale merkezine, br) × onde (rakip saha oyuncularından kale çizgisine toptan yakın olanlar).
#   tutma.py <çıktı.json> <N_sn> <analiz.jsonl>...   (meta ortamdan: BILGI_META json)
import json, sys, math, os
sys.path.insert(0, os.path.dirname(__file__))
from ortak import Izgara, yaz
W, CY, TPS = 120.0, 30.0, 30
cikti, N = sys.argv[1], float(sys.argv[2]); H = int(N * TPS)
eksen = [("uzaklik", list(range(0, 66, 6))), ("onde", list(range(0, 7)))]
g = Izgara(eksen); mac = 0
for f in sys.argv[3:]:
    R = [json.loads(l) for l in open(f)]
    if not R: continue
    mac += 1
    gol = []; s0 = list(R[0]['s'])
    for r in R:
        for tm in (0, 1):
            if r['s'][tm] > s0[tm]: gol.append((r['t'], tm))
        s0 = list(r['s'])
    for r in R:
        h = r['h']
        if h is None or h < 0 or h in (0, 7): continue          # Bekçi'nin tutuşu hariç
        tm = 0 if h < 7 else 1; gx = W if tm == 0 else 0.0
        x, y = r['p'][h][0], r['p'][h][1]
        rak = range(8, 14) if tm == 0 else range(1, 7)
        onde = sum(1 for q in rak if abs(gx - r['p'][q][0]) < abs(gx - x))
        oldu = any(r['t'] < t <= r['t'] + H and t2 == tm for t, t2 in gol)
        g.ekle([math.hypot(gx - x, CY - y), onde], 1.0 if oldu else 0.0)
meta = json.loads(os.environ.get("BILGI_META", "{}")); meta["mac"] = mac
meta.setdefault("aciklama", f"topa sahip takım bu durumda {N:g} sn içinde gol atar mı")
yaz(cikti, "tutma", eksen, g.yumusat(300), g.N, meta)
