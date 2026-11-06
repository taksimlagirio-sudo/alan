#!/usr/bin/env python3
"""cok.sh çıktısında Kale Duvarı (KAL) şutları: şut, gol, hattı boş şut (sut.txt "hatta: []"), hattı boş gol; skorlar.
kullanım: python3 analiz/py/hatta.py <cok.sh çıktı dizini>..."""
import sys, re, glob, json, os
for D in sys.argv[1:]:
    tot = [0] * 4; sk = []
    for f in sorted(glob.glob(f'{D}/t*/sut.txt')):
        t = os.path.basename(os.path.dirname(f))
        bl = open(f).read().split('\n== ')[1:]
        n = [0] * 4
        for b in bl:
            if '(KAL)' not in b.split('\n')[0]:
                continue
            bos = 'hatta: []' in b.split('\n')[0]
            gol = 'SAYI' in b
            for i, c in enumerate((1, gol, bos, gol and bos)):
                n[i] += int(bool(c))
        tot = [a + b for a, b in zip(tot, n)]
        j = json.load(open(f'{D}/{t}.json'))
        sk.append(f"{t[1:]}:{j['skor']}")
    lw = sum(1 for s in sk if (lambda a, b: a > b)(*map(int, s.split(':')[1].split('-'))))
    kw = sum(1 for s in sk if (lambda a, b: a < b)(*map(int, s.split(':')[1].split('-'))))
    print(f"{os.path.basename(D):10} {' '.join(sk)} | Liman {lw}G/{kw}M | KAL şut {tot[0]} gol {tot[1]} gol/şut {tot[1]/max(1,tot[0]):.2f} | hattı boş şut {tot[2]} gol {tot[3]}")
