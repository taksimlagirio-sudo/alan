#!/usr/bin/env python3
# Maç kayıtlarında (analiz.jsonl) şut sayısı uzaklık dilimine göre (taşıyıcının kale merkezine uzaklığı; şut uçuşunun ilk tiki).
#   uzaksut.py <analiz.jsonl>...   → "≤18 a 18–30 b 30+ c (maç m)"
import json, sys, math
W, CY = 120.0, 30.0
S = {'≤18': 0, '18–30': 0, '30+': 0}; m = 0
for f in sys.argv[1:]:
    R = [json.loads(l) for l in open(f)]
    if not R: continue
    m += 1
    for j in range(1, len(R)):
        fl = R[j].get('fl'); p = R[j - 1]
        if not fl or not fl[4]: continue
        if p.get('fl') and p['fl'][4] and p['fl'][1] == fl[1]: continue
        h = p['h']
        if h is None or h < 0: continue
        gx = W if h < 7 else 0.0; d = math.hypot(gx - p['p'][h][0], CY - p['p'][h][1])
        S['≤18' if d <= 18 else '18–30' if d <= 30 else '30+'] += 1
print(' '.join(f'{k} {v}' for k, v in S.items()) + f' (maç {m})')
