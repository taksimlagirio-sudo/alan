#!/usr/bin/env python3
# Üretim geçmişinde salınım mı gürültü mü: ardışık turların ölçüm farklarının işaret ilişkisi.
#   salinim.py analiz/bilgi-gecmis/<ad>/<zaman>/
# Her hücre için d_t = ölçüm_t − ölçüm_{t−1}; ardışık d'lerin çarpımı negatifse ters işaret. n ağırlıklı ters işaret payı
# ~%50 ise gürültü, belirgin yüksek ve ardışık d'lerin ilintisi negatifse salınım (iki turluk döngü).
import json, sys, glob, os
k = sorted(glob.glob(os.path.join(sys.argv[1], 'tur*-olcum.json')), key=lambda p: int(os.path.basename(p)[3:].split('-')[0]))
O = [json.load(open(p)) for p in k]
print('tur', len(O))
for i in range(2, len(O)):
    a, b, c = O[i - 2], O[i - 1], O[i]
    ters = top = 0.0; sxy = sxx = syy = 0.0
    for j in range(len(c['deger'])):
        w = min(a['n'][j], b['n'][j], c['n'][j])
        if w <= 0: continue
        d1 = b['deger'][j] - a['deger'][j]; d2 = c['deger'][j] - b['deger'][j]
        top += w; ters += w * (d1 * d2 < 0)
        sxy += w * d1 * d2; sxx += w * d1 * d1; syy += w * d2 * d2
    r = sxy / (sxx * syy) ** 0.5 if sxx and syy else float('nan')
    print(f'tur {i-1}→{i} ile {i}→{i+1}: n ağırlıklı ters işaret %{100*ters/max(top,1):.0f}, ilinti {r:+.2f} (gürültüde ~%50, ilinti ~−0.5; salınımda ters işaret yüksek ve aynı hücrelerde tekrar)')
