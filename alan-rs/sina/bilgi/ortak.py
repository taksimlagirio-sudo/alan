# Bilgi tablolarının ortak kodu (analiz/TASARIM-BILGI.md): hücre sayacı, komşu yumuşatma, eskiyle harmanlama, dosya biçimi.
import json, os, itertools, datetime

class Izgara:
    """Eksenler: [(ad, düğümler)]. Hücreye değer biriktirir (toplam, n)."""
    def __init__(self, eksen):
        self.eksen = eksen
        self.boy = [len(d) for _, d in eksen]
        self.T = [0.0] * self.hucre_sayisi(); self.N = [0.0] * self.hucre_sayisi()
    def hucre_sayisi(self):
        p = 1
        for b in self.boy: p *= b
        return p
    def indeks(self, v):
        idx = 0
        for (ad, d), x in zip(self.eksen, v):
            i = min(range(len(d)), key=lambda k: abs(d[k] - x))
            idx = idx * len(d) + i
        return idx
    def ekle(self, v, deger, w=1.0):
        """v: koordinat listesi ya da (indeks,) demeti."""
        i = v[0] if isinstance(v, tuple) else self.indeks(v)
        self.T[i] += w * deger; self.N[i] += w
    def koordinat(self, idx):
        c = []
        for b in reversed(self.boy):
            c.append(idx % b); idx //= b
        return list(reversed(c))
    def yumusat(self, en_az):
        """Az örnekli hücre: halka halka komşuları katar (en az `en_az` örneğe ya da 4 halkaya dek). n ölçülen kalır."""
        D = []
        for idx in range(self.hucre_sayisi()):
            c = self.koordinat(idx)
            for r in range(0, 5):
                t = n = 0.0
                for d in itertools.product(*[range(max(0, ci - r), min(b, ci + r + 1)) for ci, b in zip(c, self.boy)]):
                    j = 0
                    for di, b in zip(d, self.boy): j = j * b + di
                    t += self.T[j]; n += self.N[j]
                if n >= en_az: break
            D.append(t / n if n > 0 else 0.0)
        return D

def harmanla(eski_yol, yeni_deger, yeni_n, oran=0.5):
    """Sabit nokta adımı: yeni = oran·eski + (1−oran)·ölçüm (eksenler aynıysa). Döner: (değer, en büyük değişim)."""
    if not eski_yol or not os.path.exists(eski_yol):
        return yeni_deger, None
    e = json.load(open(eski_yol))
    if len(e.get('deger', [])) != len(yeni_deger):
        return yeni_deger, None
    d = [round(oran * a + (1 - oran) * b, 4) for a, b in zip(e['deger'], yeni_deger)]
    # en büyük değişim yalnız örneği olan hücrelerde
    fark = max((abs(a - b) for a, b, n in zip(e['deger'], d, yeni_n) if n > 0), default=0.0)
    return d, fark

def yaz(yol, ad, eksen, deger, n, meta):
    j = {"bicim": 1, "ad": ad, "surum": meta.get("surum", 1),
         "uretim": datetime.datetime.utcnow().strftime("%Y-%m-%dT%H:%MZ"),
         "kural_izi": meta.get("kural_izi", ""), "davranis_izi": meta.get("davranis_izi", ""),
         "bayraklar": meta.get("bayraklar", []), "mac": meta.get("mac", 0), "tohumlar": meta.get("tohumlar", []),
         "k": meta.get("k", 30), "aciklama": meta.get("aciklama", ""),
         "eksen": [{"ad": a, "dugum": d} for a, d in eksen],
         "deger": [round(x, 4) for x in deger], "n": [int(x) for x in n]}
    with open(yol, "w") as f:
        json.dump(j, f, ensure_ascii=False)
