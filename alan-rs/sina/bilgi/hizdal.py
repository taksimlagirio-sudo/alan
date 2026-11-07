#!/usr/bin/env python3
# Kontra hızının nedensel değeri (analizci, A2): kontra anında koşucu N tik depar / tempoya zorlanır (alan-kos hizdal), iki kolun farkı.
#   hizdal.py sec <klasör> <an_sayısı>   : her tohumun analiz.jsonl'inden kontra anları ve koşucu → t<s>/tler, t<s>/oy
#   hizdal.py rapor <klasör>             : kollar: 12 sn içinde gol, kutuya (25 br) varış, kontra bitişi; depar − tempo farkı
# Kontra (analiz/py/kontra.py): top rakip kaleye 50+ br'de kazanılır, topun kale tarafında en çok 3 rakip saha oyuncusu.
# Koşucu: topu kazanan takımın, taşıyıcı ve Bekçi dışında, rakip kaleye en yakın saha oyuncusu.
import json, sys, math, os, glob, random, collections
W, CY, TPS = 120.0, 30.0, 30
TK = lambda i: 0 if i < 7 else 1

def kontralar(R):
    son = None; out = []
    for i, r in enumerate(R):
        h = r['h']
        if h is None or h < 0 or h in (0, 7): continue
        tm = TK(h)
        if son is not None and son != tm:
            gx = W if tm == 0 else 0.0; x, y = r['p'][h][:2]
            rak = range(8, 14) if tm == 0 else range(1, 7)
            onde = sum(1 for q in rak if abs(gx - r['p'][q][0]) < abs(gx - x))
            if abs(gx - x) > 50 and onde <= 3:
                biz = [q for q in (range(1, 7) if tm == 0 else range(8, 14)) if q != h]
                kos = min(biz, key=lambda q: math.hypot(gx - r['p'][q][0], CY - r['p'][q][1]))
                out.append((int(r['t']), kos, tm, math.hypot(gx - x, CY - y), onde, math.hypot(gx - r['p'][kos][0], CY - r['p'][kos][1])))
        son = tm
    return out

if sys.argv[1] == 'sec':
    kl, an = sys.argv[2], int(sys.argv[3])
    hep = []
    for d in sorted(glob.glob(f'{kl}/t*/')):
        R = [json.loads(l) for l in open(d + 'analiz.jsonl')]
        for k in kontralar(R): hep.append((d, k))
    random.Random(7).shuffle(hep); hep = hep[:an]
    bol = collections.defaultdict(list)
    for d, k in hep: bol[d].append(k)
    for d, ks in bol.items():
        ks.sort()
        open(d + 'tler', 'w').write(','.join(str(k[0]) for k in ks))
        open(d + 'oy', 'w').write(','.join(str(k[1]) for k in ks))
        json.dump(ks, open(d + 'kontra.json', 'w'))
    print(f'{len(hep)} kontra anı seçildi ({len(bol)} tohum)', file=sys.stderr)
    sys.exit(0)

def sonuc(f, tm):
    R = [json.loads(l) for l in open(f)] if os.path.exists(f) else []
    if not R: return None
    gx = W if tm == 0 else 0.0; s0 = R[0]['s'][tm]; gol = False; kutu = False; bitis = len(R)
    for i, r in enumerate(R[:12 * TPS]):
        if r['s'][tm] > s0: gol = True; bitis = min(bitis, i); break
        h = r['h']
        if h is not None and h >= 0 and h not in (0, 7):
            if TK(h) != tm: bitis = min(bitis, i); break
            if math.hypot(gx - r['p'][h][0], CY - r['p'][h][1]) < 25: kutu = True
    return gol, kutu, bitis

kl = sys.argv[2]; T = collections.defaultdict(lambda: [0, 0, 0, 0, 0, 0.0]); n = 0; S = [0, 0, 0, 0]
for d in sorted(glob.glob(f'{kl}/t*/')):
    if not os.path.exists(d + 'kontra.json'): continue
    for (t0, kos, tm, uz, onde, kk) in json.load(open(d + 'kontra.json')):
        a = sonuc(f'{d}{t0}/depar.jsonl', tm); b = sonuc(f'{d}{t0}/tempo.jsonl', tm)
        if a is None or b is None: continue
        n += 1; S[0] += a[0]; S[1] += b[0]; S[2] += a[1]; S[3] += b[1]
        key = (min(90, 10 * round(uz / 10)), onde, '≤40' if kk <= 40 else '>40')
        c = T[key]; c[0] += 1; c[1] += a[0]; c[2] += b[0]; c[3] += a[1]; c[4] += b[1]; c[5] += a[2] - b[2]
print(f'{n} kontra anı: 12 sn içinde gol depar %{100*S[0]/max(1,n):.1f} / tempo %{100*S[1]/max(1,n):.1f}; kutuya varış depar %{100*S[2]/max(1,n):.1f} / tempo %{100*S[3]/max(1,n):.1f}')
print('kazanım (uzaklık, önde, koşucunun kaleye kalan): n | gol depar−tempo | kutu depar−tempo | bitiş farkı (tik)')
for k in sorted(T):
    c = T[k]
    print(f'  {k}: {c[0]:3d} | {(c[1]-c[2])/c[0]:+.2f} | {(c[3]-c[4])/c[0]:+.2f} | {c[5]/c[0]:+.0f}')
