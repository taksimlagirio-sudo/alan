#!/usr/bin/env python3
# A1 tutma değeri: topa sahip takımın bu sahipliği, bu durumdan sonraki N sn içinde golle biter mi (motorun kendi maçlarından).
# Hücre: uzaklik (taşıyıcının rakip kale merkezine, br) × onde (rakip saha oyuncularından kale çizgisine toptan yakın olanlar).
# Örnek: bir sahiplik bir hücreye en çok bir kez (ilk girdiği tik) sayılır; n = hücreyi gören bağımsız sahiplik sayısı.
# Sahiplik: aynı takımın top tutması; top boştayken son tutanın sahipliği sürer, rakip tutunca ya da gol olunca biter.
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
    sah = None; ilk = {}   # hücre → ilk tik (bu sahiplikte)
    s0 = list(R[0]['s'])
    def bitir(gol_t):
        for c, t0 in ilk.items():
            g.ekle(c, 1.0 if (gol_t is not None and gol_t - t0 <= H) else 0.0)
        ilk.clear()
    for r in R:
        s = r['s']
        if sah is not None and s[sah] > s0[sah]:
            bitir(r['t']); sah = None
        elif s != s0:
            bitir(None); sah = None
        s0 = list(s)
        h = r['h']
        if h is None or h < 0: continue
        tm = 0 if h < 7 else 1
        if sah is not None and tm != sah: bitir(None)
        sah = tm
        if h in (0, 7): continue          # Bekçi'nin tutuşu sahipliği sürdürür, hücre sayılmaz
        gx = W if tm == 0 else 0.0
        x, y = r['p'][h][0], r['p'][h][1]
        rak = range(8, 14) if tm == 0 else range(1, 7)
        onde = sum(1 for q in rak if abs(gx - r['p'][q][0]) < abs(gx - x))
        c = (g.indeks([math.hypot(gx - x, CY - y), onde]),)
        if c not in ilk: ilk[c] = r['t']
    bitir(None)
# Izgara.ekle koordinatla çalışır; burada indeksle doğrudan
meta = json.loads(os.environ.get("BILGI_META", "{}")); meta["mac"] = mac
meta.setdefault("aciklama", f"topa sahip takımın bu sahipliği bu durumdan sonraki {N:g} sn içinde golle biter mi (n: bağımsız sahiplik)")
yaz(cikti, "tutma", eksen, g.yumusat(60), g.N, meta)
