# ALAN · zip46 ölçümü: sabitlerin üç sınıfı ve saha boyu

## 1 · Birebir aynılık (100×50): 16 / 16 ✓
zip45 ile zip46, 16 maç baştan sona: her tikte bütün oyuncuların konumu, hızı, hedefi, görevi, Çekirdek, şarj, geçiş ve karşı pres durumu, paslar, olaylar, skor ve istatistikler eşit. Sabitlerin üç sınıfa ayrılması oyunu değiştirmedi. zip46 yayında.

## 2 · Üç saha boyu (herkes 10; Liman'ın 4 maçı; Alan gücü 10 / 6 / 14)

Alan gücü 10 kolu:
| Saha | Gol | Pas tutma | Şut | Son bölgede topun ortalama süresi | Hesap |
|---|---|---|---|---|---|
| 100×50 | 7,0 | %87 | 11,5 | 23 tik | 136 sn |
| 120×60 | 5,8 | %93 | 12,0 | 29 tik | 141 sn |
| 140×70 | 5,0 | %95 | 8,5 | 31 tik | 121 sn |

Üç kolun ortalamasında da aynı yön: son bölgede kalma 23 → 28 → 33 tik, pas tutma %89 → %92 → %93. Motor büyük sahada sorunsuz (36 maçta yığılma, takılma yok); hesap süresi artmıyor.

**Net çıkmayanlar:**
- **Geçiş:** kollar arasında dağınık (110–350 tik/maç), saha boyuyla yön yok.
- **Alan gücü:** hiçbir boyda 14 ile 6 arasında düzenli bir gol / kapma / rakip pas tutma farkı yok; 4 maçta sonuç gürültüde. Saha büyüdükçe Alan gücünün değerinin artıp artmadığı gol üzerinden ölçülemiyor. Öneri: mekaniği doğrudan ölçmek (rakip pasının Liman alanlarında ne kadar yavaşladığı ve ne sıklıkla söndüğü). Claude Code bunu kayıtlardan hesaplayabilir; motorun Çekirdeğin söndüğü anı ve o andaki alanı (kimin alanı) olay olarak yazması işi kolaylaştırır.

## 3 · Gerçek kadroyla doğrulama: 100×50 ile 120×60 (herkes 20, 16 maç, ayarsız takımlar)

| | 100×50 | 120×60 |
|---|---|---|
| Sayı / maç | 4,31 | **3,19** |
| Gönderme / maç | 16,9 | 12,3 |
| Pas / maç (tutma) | 96 (%91,5) | 88 (%91,4) |
| Top kapma / maç | 20,6 | 16,8 |
| Top el değiştirme / maç | 24,8 | 19,9 |
| Her an duran hücumcu / savunmacı | 0,44 / 0,21 | 0,37 / 0,16 |
| Uzun tutuşta 1 sn+ duran hücumcu | 1,30 | **0,51** |
| Son bölgede top: pay · ortalama süre | %10 · 16 tik | %12 · 20 tik |
| Geçiş / "git" (tik/maç) | 373 / 36 | 315 / 24 |
| Koşu / takım-maç (çizgiyi geçen) | 32,7 (%20) | 29,1 (%19) |
| Aklın pas tahmini → gerçek | %95 → %91 | %96 → %92 (kendine pas %82 → %90) |
| Hesap süresi (sn / maç) | 173 | **157** |

- Oyun karakteri korunuyor; sahiplikler uzuyor (el değiştirme −%20), top son bölgede daha uzun kalıyor, durma azalıyor, akıl aynı isabette (kendine pasta daha iyi).
- Maç 90 saniye kaldığı için mesafeler uzayınca olay sayısı düşüyor: pas −%8, gönderme −%27, gol −%26 (4,3 → 3,2).
- Geçiş büyük sahada artmıyor (beklenenin tersine biraz azalıyor): geçişin seyrekliği sahadan değil kuraldan (sayıca üstünlük şartı).
- Hesap süresi düşüyor.

**Karar kullanıcıda.** 120×60'a geçilirse gol sayısının düşüşü bilinçli bir seçim olmalı (istenirse maç süresi ya da pas hızı ayrı ayrı düşünülür; tek bir sayı değiştirip bakmak kolay).

## 4 · Büyük sahaya geçiş için Design'ın işi
1. Taktik, Canlı ve Lab ekranları saha ölçüsünü motordan okusun (bugün 100×50 çiziyorlar).
2. Analiz işçileri (analiz-isci, analiz-kalabalik, lab-isci) 100 birim varsayımını bırakmalı.
3. Hazır tablolar (gönderme ve değer tabloları) her saha boyu için doğru okunuyor mu: Design'ın notunda "tek kaynak" deniyor; 120×60'ta gönderme tablosunun sahaya göre yeniden üretilip üretilmediği (`SHT_SURUM`) kontrol edilmeli.
4. Kayıtlı taktikler ve duruşlar 100×50 ölçeğinde kalıyor, motor ölçekliyor: ekranda da aynı ölçekle gösterilmeli.
5. Varsayılan saha kararı verilince oyuncu kaydına saha ölçüsü yazılsın (eski kayıt 100×50 sayılır).
