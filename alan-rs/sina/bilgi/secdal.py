#!/usr/bin/env python3
# Evre 3 ölçümü (analizcinin tanımı): kaleye ≤25 br taşıyıcı karar anlarında seçenek dallanması.
#   secdal.py sec <klasör> <an_sayısı>     her tohum için T listesini yazar (<klasör>/t<s>/tler)
#   secdal.py rapor <klasör>               katman × kol raporu
import json, sys, os, glob, math, random, collections
W, CY = 120.0, 30.0
def seg(px, py, ax, ay, bx, by):
    dx, dy = bx - ax, by - ay; L = dx * dx + dy * dy
    t = max(0, min(1, ((px - ax) * dx + (py - ay) * dy) / L)) if L else 0
    return math.hypot(px - ax - t * dx, py - ay - t * dy)
def sahiplik(A):
    """tik → sahiplik no (takım, no); top boştayken son tutanın sahipliği sürer, gol ya da rakip tutuşu bitirir."""
    sid = {}; sah = None; no = 0; s0 = None
    for r in A:
        if s0 is not None and r['s'] != s0: sah = None
        s0 = r['s']; h = r['h']
        if h is not None and h >= 0:
            tm = 0 if h < 7 else 1
            if tm != sah: no += 1; sah = tm
        if sah is not None: sid[r['t']] = (sah, no)
    return sid
def katman(fr, tm, x, y):
    gx = W if tm == 0 else 0.0
    d = math.hypot(gx - x, CY - y)
    rak = range(8, 14) if tm == 0 else range(1, 7)
    hat = any(seg(fr['p'][q][0], fr['p'][q][1], x, y, gx, CY) < 3 and abs(gx - fr['p'][q][0]) < abs(gx - x) for q in rak)
    onde = sum(1 for q in rak if abs(gx - fr['p'][q][0]) < abs(gx - x))
    if 35 <= d <= 45 and (y < 8 or y > CY * 2 - 8): return ('kenar', '35–45', 'uzak'), d
    return ('kesik' if hat else 'açık', '≤15' if d <= 15 else '15–25', '0–2' if onde <= 2 else '3+'), d
def sec(kl, toplam):
    aday = collections.defaultdict(list)
    for t in sorted(glob.glob(kl + '/t*/')):
        A = [json.loads(l) for l in open(t + 'analiz.jsonl')]
        F = {r['t']: r for r in A}; sid = sahiplik(A)
        for l in open(t + 'kararlar.txt'):
            r = json.loads(l)
            if not r[5]: continue
            T = int(r[0]) - 1; fr = F.get(T); si = sid.get(T)
            if not fr or not si or si[0] != r[2]: continue
            k, d = katman(fr, r[2], r[3], r[4])
            if d > 25 and k[0] != 'kenar': continue
            aday[k].append((t, T, si))
    rnd = random.Random(7); her = max(1, toplam // 8); secilen = collections.defaultdict(list); say = collections.Counter()
    for k in sorted(aday):
        L = aday[k][:]; rnd.shuffle(L); n = 0
        for (t, T, si) in L:
            if n >= her: break
            ay = [T2 for (T2, s2) in secilen[t] if s2 == si]
            if len(ay) >= 2 or any(abs(T - T2) < 60 for T2 in ay): continue
            secilen[t].append((T, si)); n += 1; say[k] += 1
    for t, L in secilen.items():
        open(t + 'tler', 'w').write(','.join(str(T) for T, _ in sorted(L)))
    print('katman başına an:', dict(say), 'toplam', sum(say.values()))
def sonuc(R, tm):
    """kolun satırları (T+1'den): (gol mü, kayıp tiki, kayıp x, rakip gol mü (kayıptan sonra, ufuk içinde), sahiplik süresi)"""
    s0 = R[0]['s'] if R else [0, 0]; t0 = R[0]['t'] if R else 0
    for r in R:
        if r['s'][tm] > s0[tm]: return (True, None, None, False, r['t'] - t0)
        if r['s'][1 - tm] > s0[1 - tm]: return (False, r['t'], None, True, r['t'] - t0)
        h = r['h']
        if h is not None and h >= 0 and (0 if h < 7 else 1) != tm:
            tk, xk = r['t'], r['p'][h][0]
            rg = any(q['s'][1 - tm] > s0[1 - tm] for q in R if q['t'] > tk)
            return (False, tk, xk, rg, tk - t0)
    return (False, None, None, False, len(R))
def rapor(kl):
    T = collections.defaultdict(lambda: collections.defaultdict(list)); yok = collections.Counter(); kontrol = [0, 0]
    PAS = []; TUT = []; ONCE = collections.defaultdict(list); SUT = []
    for t in sorted(glob.glob(kl + '/t*/')):
        A = {}; Al = []
        for l in open(t + 'analiz.jsonl'):
            A[json.loads(l)['t']] = l.strip()
        Ar = [json.loads(v) for v in A.values()]
        F = {r['t']: r for r in Ar}
        Z = collections.defaultdict(dict)   # T → kol → (v ya da None)
        ZD = {}                              # (T, kol) → ZORLA ayrıntısı
        try: AD = json.load(open(t + 'ad.json'))['ad']
        except Exception: AD = []
        SEC = {}
        for l in open(t + 'kararlar.txt'):
            q = json.loads(l)
            if q[5]: SEC[int(q[0]) - 1] = q[5][0][0].split()[0]
        cur = None
        for l in open(t + 'secdal.err'):
            p = l.split()
            if not p: continue
            if p[0] == 'SECDAL': cur = int(p[1])
            elif p[0] == 'ZORLA' and cur is not None:
                kol = p[3]
                if 'yok' in l: Z[cur][kol] = None; continue
                kv = {p[i]: p[i + 1] for i in range(5, len(p) - 1, 2)}
                Z[cur][kol] = float(kv.get('v', 'nan')); ZD[(cur, kol)] = kv
        for d in sorted(glob.glob(t + '[0-9]*/')):
            Tn = int(os.path.basename(d.rstrip('/'))); fr = F.get(Tn)
            if not fr or fr['h'] is None or fr['h'] < 0: continue
            h = fr['h']; tm = 0 if h < 7 else 1; x, y = fr['p'][h][0], fr['p'][h][1]
            k, _ = katman(fr, tm, x, y)
            once = [l.strip() for l in open(d + 'once.jsonl') if l.strip()]
            ayni = all(A.get(json.loads(l)['t']) == l for l in once[:300])
            kontrol[0] += 1; kontrol[1] += ayni
            for kol in ['once', 'sut', 'pas', 'sur', 'tut']:
                if kol != 'once' and kol in Z[Tn] and Z[Tn][kol] is None: yok[(k, kol)] += 1; continue
                if not os.path.exists(d + kol + '.jsonl'): continue
                R = [json.loads(l) for l in open(d + kol + '.jsonl') if l.strip()]
                g, tk, xk, rg, sure = sonuc(R, tm)
                T[k][kol].append((g, rg, Z[Tn].get(kol), sure))
                kv = ZD.get((Tn, kol))
                if kol == 'once': ONCE[k].append(SEC.get(Tn, '?'))
                if kol == 'sut' and kv:
                    # şutun kendi sonucu: şut uçuşu başladıktan sonra kimse topu tutmadan gol (dönen top / ikinci şut sayılmaz)
                    s0 = R[0]['s'] if R else [0, 0]; uc = False; kg = False
                    for r in R:
                        f = r.get('fl')
                        if f and f[4]: uc = True
                        if r['s'] != s0: kg = uc and r['s'][tm] > s0[tm]; break
                        if uc and r['h'] is not None and r['h'] >= 0 and not (f and f[4]): break
                    SUT.append((k, float(kv.get('Pk', 'nan')), g, kg))
                if kol == 'pas' and kv:
                    # pasın sonucu: topu ilk tutan (taşıyıcı dışında) ya da gol
                    s0 = R[0]['s'] if R else [0, 0]; son = ('yok', None); ucti = False
                    for r in R:
                        if r.get('fl'): ucti = True
                        if r['s'] != s0: son = ('gol' if r['s'][tm] > s0[tm] else 'rakip gol', r['t'] - Tn); break
                        hh = r['h']
                        if hh is not None and hh >= 0 and hh != h:
                            nm = AD[hh] if hh < len(AD) else str(hh)
                            rk = (0 if hh < 7 else 1) != tm
                            son = ('alıcı' if nm == kv.get('q') else (('rakip uçuşta' if ucti else 'rakip bırakmadan') if rk else 'başka arkadaş'), r['t'] - Tn); break
                    PAS.append((k, kv, son))
                if kol == 'tut' and kv:
                    th = float(kv.get('sure', 'nan')); kept = True
                    for r in R:
                        if r['t'] - Tn > th: break
                        hh = r['h']
                        if r['s'] != (R[0]['s'] if R else r['s']) or (hh is not None and hh >= 0 and (0 if hh < 7 else 1) != tm): kept = False; break
                    TUT.append((k, float(kv.get('tutar', 'nan')), th, kept))
    print(f'kontrol: once kolu asıl maçla aynı {kontrol[1]}/{kontrol[0]}')
    for k in sorted(T):
        print('==', ' × '.join(k))
        b = T[k].get('once', []); go = sum(x[0] for x in b) / max(1, len(b)); ro = sum(x[1] for x in b) / max(1, len(b))
        for kol in ['once', 'sut', 'pas', 'sur', 'tut']:
            L = T[k].get(kol, [])
            if not L and not yok[(k, kol)]: continue
            n = len(L); g = sum(x[0] for x in L) / max(1, n); r = sum(x[1] for x in L) / max(1, n)
            vs = [x[2] for x in L if x[2] is not None]; v = sum(vs) / len(vs) if vs else float('nan')
            s = sum(x[3] for x in L) / max(1, n)
            print(f"   {kol:5} n {n:3} (seçenek yok {yok[(k, kol)]:2})  gol %{100*g:4.0f}  rakip gol %{100*r:4.0f}  fark %{100*(g-r):+4.0f}  kol−once %{100*((g-r)-(go-ro)):+4.0f}  kafa v {v:6.3f}  sahiplik {s:5.0f} tik")
    rapor_ek(PAS, TUT)
    print('== once kolunda kafanın asıl seçimi (katman başına; gönder = şut)')
    for k in sorted(ONCE):
        c = collections.Counter(ONCE[k]); n = len(ONCE[k])
        print('   ' + ' × '.join(k) + f'  n {n:3}  ' + '  '.join(f'{a} %{100*b/n:.0f}' for a, b in c.most_common()))
    print('== sut kolu: kafanın Pk ortalaması ile zorlanan şutun gerçek gol oranı (katman başına)')
    B = collections.defaultdict(list)
    for k, pk, g, kg in SUT: B[k].append((pk, g, kg))
    for k in sorted(B):
        X = B[k]; n = len(X)
        print('   ' + ' × '.join(k) + f'  n {n:3}  kafa Pk {sum(x[0] for x in X)/n:.3f}  şutun kendi golü %{100*sum(x[2] for x in X)/n:.0f}  sahiplikte gol (dönen top dahil) %{100*sum(x[1] for x in X)/n:.0f}')
    # tür başına kafa v ile gerçek (bütün katmanlar)
    print('== tür başına: kafa v ortalaması ile gerçek (gol − rakip gol)')
    for kol in ['sut', 'pas', 'sur', 'tut']:
        L = [x for k in T for x in T[k].get(kol, []) if x[2] is not None]
        if L: print(f"   {kol:5} n {len(L):3}  kafa v {sum(x[2] for x in L)/len(L):.3f}  gerçek gol %{100*sum(x[0] for x in L)/len(L):.0f}  rakip gol %{100*sum(x[1] for x in L)/len(L):.0f}")
def rapor_ek(PAS, TUT):
    print('== pas kolu (B1, seçilimsiz): kafanın Pd dilimine göre gerçek sonuç')
    B = collections.defaultdict(list)
    for k, kv, son in PAS:
        try: P, Ps = float(kv['P']), float(kv['Ps'])
        except Exception: continue
        Pd = max(0.0, P - Ps); B[min(4, int(Pd * 5))].append((kv, son))
    for b in sorted(B):
        X = B[b]; c = collections.Counter(s[0] for _, s in X); n = len(X)
        ort = lambda a: sum(float(kv.get(a, 'nan')) for kv, _ in X) / n
        print(f"   Pd {b/5:.1f}–{(b+1)/5:.1f}: n {n:3}  alıcıya %{100*c['alıcı']/n:3.0f}  başka arkadaş %{100*c['başka arkadaş']/n:3.0f}  rakibe: bırakmadan %{100*c['rakip bırakmadan']/n:3.0f} uçuşta %{100*c['rakip uçuşta']/n:3.0f}  gol %{100*c['gol']/n:3.0f} | kafa Pl %{100*ort('Pl'):3.0f}  pre {ort('pre'):.2f}  v {ort('v'):.3f}")
    print('== tut kolu (B2): kafanın keep_p dilimine göre topun bekleme süresince gerçekten tutulması')
    B = collections.defaultdict(list)
    for k, kp, th, kept in TUT:
        if kp == kp: B[min(4, int(kp * 5))].append((th, kept))
    for b in sorted(B):
        X = B[b]; n = len(X)
        print(f"   keep_p {b/5:.1f}–{(b+1)/5:.1f}: n {n:3}  gerçekte tuttu %{100*sum(x[1] for x in X)/n:3.0f}  bekleme ort {sum(x[0] for x in X)/n:.0f} tik")
if __name__ == '__main__':
    {'sec': lambda: sec(sys.argv[2], int(sys.argv[3])), 'rapor': lambda: rapor(sys.argv[2])}[sys.argv[1]]()
