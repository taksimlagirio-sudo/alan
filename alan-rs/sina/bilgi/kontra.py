#!/usr/bin/env python3
# A2 kontra değeri (analiz/TASARIM-BILGI.md, tanım analiz/py/kontra.py ve analiz/ZAMAN-DEGERI.md): top rakip kaleye 50+ br'de
# kazanılır ve topun kale tarafında en çok 3 rakip saha oyuncusu vardır. Kontra boyunca (12 sn'ye ya da rakip topu alana dek)
# her 5 tikte durum örneklenir; sonuç: o andan N sn içinde gol.
# Hücre: uzaklik (top / taşıyıcının rakip kale merkezine, br) × onde (rakip saha oyuncularından kale çizgisine toptan yakın
# olanlar) × sure (top kazanılalı geçen süre, sn). Okuyucu: mac/kontra_tablo.rs (hiz2'de kontra koşusunun değeri).
#   kontra.py <çıktı.json> <N_sn> <analiz.jsonl>...   (meta ortamdan: BILGI_META json)
import json, sys, math, os
sys.path.insert(0, os.path.dirname(__file__))
from ortak import Izgara, yaz
W, CY, TPS = 120.0, 30.0, 30
KAZAN_D, KAZAN_ONDE, SURE, ADIM = 50.0, 3, 12.0, 5
cikti, N = sys.argv[1], float(sys.argv[2]); H = int(N * TPS)
eksen = [("uzaklik", list(range(10, 91, 10))), ("onde", list(range(0, 6))), ("sure", list(range(0, 13, 2)))]
g = Izgara(eksen); mac = 0; kontra = 0
TK = lambda i: 0 if i < 7 else 1
for f in sys.argv[3:]:
    R = [json.loads(l) for l in open(f)]
    if not R: continue
    mac += 1
    gol = []; s0 = list(R[0]['s'])
    for r in R:
        for tm in (0, 1):
            if r['s'][tm] > s0[tm]: gol.append((r['t'], tm))
        s0 = list(r['s'])
    son = None; i = 0
    while i < len(R):
        r = R[i]; h = r['h']
        if h is None or h < 0 or h in (0, 7):
            i += 1; continue
        tm = TK(h)
        if son is not None and son != tm:
            gx = W if tm == 0 else 0.0; x = r['p'][h][0]
            rak = range(8, 14) if tm == 0 else range(1, 7)
            onde0 = sum(1 for q in rak if abs(gx - r['p'][q][0]) < abs(gx - x))
            if abs(gx - x) > KAZAN_D and onde0 <= KAZAN_ONDE:
                kontra += 1; t0 = r['t']
                ornek = []                                       # bu kontranın örnekleri: (durum, sonuç)
                j = i
                while j < len(R) and R[j]['t'] - t0 <= SURE * TPS:
                    q = R[j]; hq = q['h']
                    if hq is not None and hq >= 0 and TK(hq) != tm and hq not in (0, 7):
                        break                                    # rakip topu aldı: kontra bitti
                    if (q['t'] - t0) % ADIM == 0:
                        if hq is not None and hq >= 0 and TK(hq) == tm:
                            bx, by = q['p'][hq][0], q['p'][hq][1]
                        elif q.get('b'):
                            bx, by = q['b'][0], q['b'][1]
                        else:
                            j += 1; continue
                        onde = sum(1 for k in rak if abs(gx - q['p'][k][0]) < abs(gx - bx))
                        oldu = any(q['t'] < t <= q['t'] + H and t2 == tm for t, t2 in gol)
                        ornek.append(([math.hypot(gx - bx, CY - by), onde, (q['t'] - t0) / TPS], 1.0 if oldu else 0.0))
                    if any(t == q['t'] for t, _ in gol): break
                    j += 1
                # bağımsızlık: bir kontra bir hücreye en çok 1 ağırlıkla katılır (aynı hücredeki örnekleri ortalanır); n = bağımsız kontra
                say = {}
                for v, _ in ornek: say[g.indeks(v)] = say.get(g.indeks(v), 0) + 1
                for v, d in ornek: g.ekle(v, d, 1.0 / say[g.indeks(v)])
        son = tm; i += 1
meta = json.loads(os.environ.get("BILGI_META", "{}")); meta["mac"] = mac
meta.setdefault("aciklama", f"kontra (top rakip kaleye {KAZAN_D:g}+ br'de, önünde en çok {KAZAN_ONDE} rakip kazanıldı): bu durumdan {N:g} sn içinde gol; {kontra} kontra; n = hücreye katılan bağımsız kontra sayısı (bir kontra bir hücreye en çok 1). Not: tek kurulumla (217468, Liman – Kale Duvarı, tohum değiştirerek) üretildi, bu iki takımın taktiğine uyar; ileride başka kurulumlarla da üretilmeli")
yaz(cikti, "kontra", eksen, g.yumusat(20), g.N, meta)
dolu = sorted(x for x in g.N if x > 0)
print(f"kontra: {mac} maç, {kontra} kontra; örnekli hücre {len(dolu)}/{len(g.N)}, hücre başına bağımsız kontra en az {dolu[0] if dolu else 0:.1f} / medyan {dolu[len(dolu)//2] if dolu else 0:.1f} / en çok {dolu[-1] if dolu else 0:.1f}", file=sys.stderr)
