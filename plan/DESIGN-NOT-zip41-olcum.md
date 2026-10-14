# ALAN · zip41 ölçümü (kısa tur)

Yöntem: birleşik kayıt. Her hücre 4 maç (senaryo 0–3, Liman = takım 0): yön gösterir, kesin değil.
- Özellik: iki takımda bütün özellikler 10; Liman'ın saha oyuncularında tek özellik.
- Taktik: Liman'ın taktiğine tek ayar; varsayılanla karşılaştırma.

## 1 · Düzelenler

| | Atılan | Yenen | Şut | Gol/şut |
|---|---|---|---|---|
| taban (herkes 10) | 3,25 | 3,75 | 5,3 | 0,62 |
| Gönderme 14 | 4,25 | 4,75 | 5,8 | **0,73** |
| Güç 14 | **4,50** | **2,50** | 7,0 | 0,65 |
| Güç 6 | 3,25 | 3,00 | 7,5 | **0,44** |
| Uzanma 14 | 3,25 | **2,25** | 5,3 | 0,57 |

- **Gönderme:** artık maçta da işliyor; gol/şut 0,62 → 0,73. ✓
- **Güç:** artık bir şey yapıyor (zip40'ta 14 ile 10 birebir aynı maçtı). 14'te gol farkı +2, 6'da gol/şut düşüyor. ✓
- **Uzanma:** ters etki kalktı; atılan artık düşmüyor (3,25 → 3,25). Savunmada etkili: yenen 3,75 → 2,25, top kapma 2,8 → 7,3. Teşhis (arkadaşın kendi göndermesini kesmesi) doğruydu. ✓
- **Karşı preste basan sayısı:** ortalama 1,0 → 1,9 (varsayılan), 2,1 ("her yerde"). ✓

## 2 · Kısmen düzelen: duruş

Son bölge duruşunda iki kanadı çizgiye çekmek (y 3 / 47; ortadan uzaklık hedefi 22):

| | Son bölgede kanatların ortadan uzaklığı | Ortada |
|---|---|---|
| zip40 varsayılan → duruş | 13,9 → 14,5 (+0,6) | 15,1 → 14,1 |
| zip41 varsayılan → duruş | 14,2 → 15,9 (**+1,7**) | 14,4 → 14,1 |

Üç kat iyileşme, etki doğru bölgede (ortada değişmiyor). Ama oyuncular istenen yolun ancak ~beşte birini gidiyor (14 → 22 yerine 14 → 16). Kabul ölçütü (ellenen oyuncu kendi bölgesinde evine en az yarı yarıya uysun) henüz tutmuyor. Elle konan oyuncunun aday tartımında evden uzaklığın bedeli bir kademe daha artmalı, ya da ellenen oyuncu için adaylar evin çevresinden üretilmeli (bugün aday ızgarası eski taban yerinden de üretiliyor olabilir: kontrol edilmeli).

## 3 · Yeni hata: karşı pres neredeyse anında bitiyor

"Her yerde, 3 sn" kuralında maç başına ~13 top kaybı var; her birinde 180 tik pres beklenir. Ölçülen: maç başına 39 tik. Tek tek izledim (senaryo 0, ilk 2500 tik):

```
takım 0 · 2 kişi · 4 tik sürdü · plan 180 · top hâlâ rakipte
takım 0 · 1 kişi · 14 tik sürdü · plan 180 · top hâlâ rakipte
takım 0 · 3 kişi · 4 tik sürdü · plan 180 · top hâlâ rakipte
takım 0 · 3 kişi · 2 tik sürdü · plan 180 · top hâlâ rakipte
```

Top geri kazanılmadan ve süre dolmadan bitiyor. Sebep `match.js` `kpAkt(m, t)`:

```js
if (m.tick >= k.bitis || bt === t) { m.kp = null; return null; }
```

`kpAkt` iki takım için de çağrılıyor (ör. `kpEff` → `tacEff` her iki takım için). Topu tutan takım için çağrıldığında `bt === t` doğru oluyor ve karşı pres siliniyor. Bitiş koşulu çağıranın takımına değil, **basan takıma** bakmalı:

```js
if (m.tick >= k.bitis || bt === k.team) { m.kp = null; return null; }
```

(Bu yüzden karşı pres kuralının etkisi küçük görünüyor: 3 sn içinde topu geri kazanma "hiç" %47, "her yerde" %52.)

## 4 · Açık kalanlar
- **Sürme'nin ters etkisi** (yükseldikçe daha az şut ve gol): kopyalı ölçüm gerekiyor (aynı an, Sürme 10 ve 14). Claude Code yapabilir (~15 dk, küçük).
- **Refleks'in üst tarafta etkisizliği.**
- **Geçiş eşiği:** Design haklı, 3 geçişi seyrekleştirir; 5 denenmeli (Claude Code ölçebilir).
- **Hız:** Design'ın kısa denemesinde 11,39 → 9,1 ms/tik; kontrol paketiyle doğrulanmalı.
