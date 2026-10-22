# ALAN · Yetenek ölçümü 1: 34 özelliğin etkisi (22 Ekim 2026)

> Motor: main 67dcc28 (yetenek Aşama 1–7; Bekçi aşaması 8 bağlanmadı). Araçlar: `tools/yet-olc.js` (yükleme sırası motorun `mac-isci.js`'inden), `tools/yet-ozet.js`. Veriler: `lab/yt1/`.

## Düzen
- İki takım aynı: 34 özelliğin hepsi 10, taktik varsayılan (2-2-2 / 2-1-3). A'da tek özellik **16**: saha özellikleri A'nın 6 saha oyuncusunda, Bekçi özellikleri (Refleks, Yerleşim, Alan hakimiyeti, Birebir) A'nın Bekçi'sinde.
- 34 özellik + taban = 35 kol × 8 tohum = 280 maç, her maç 2700 tik (90 sn). Taraf tohumla dengeli.
- Referans: aynı tohumdaki bütün kolların ortalaması (tek taban maçtan daha az gürültülü; taban maça göre sonuçlar da aynı sırayı veriyor).
- Sınırlar: tek seviye (16), tek taktik, 90 sn. Kol başına 8 maç: ancak büyük etkiler (~0,3 xG üstü) güvenilir; tablonun ortası gürültü.

## Sonuç

taban 8 maç; kol başına eşli tohum 8

| özellik | grup | maç | xG farkı | gol farkı | şut farkı | kapma farkı | top payı | pas isabeti | bar | dipte | şut şarjı | xG etkisi / SH |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Hız 16 | Fiziksel | 8 | +0.84 * | +0.35 | +1.8 * | +0.5 | −7 *% | −5% | +0.02 | −0% | −0.02 | 2.4 |
| Tepki 16 | Zihinsel | 8 | +0.49 | +0.23 | +0.8 | +0.2 | +12 *% | +3% | −0.03 | −0% | −0.03 * | 1.6 |
| Alan gücü 16 | ALAN | 8 | +0.34 * | +0.10 | +0.6 | +0.2 | +3% | +5 *% | −0.03 | +1% | −0.01 | 2.7 |
| Dayanıklılık 16 | Fiziksel | 8 | +0.28 | +0.35 * | +0.5 | +0.6 | +5% | +4 *% | +0.06 * | −2 *% | +0.00 | 1.4 |
| Güç 16 | Fiziksel | 8 | +0.26 | +0.35 | +0.3 | +0.5 | +8% | −1% | −0.01 | +1% | −0.00 | 0.9 |
| Denge 16 | Fiziksel | 8 | +0.21 | +0.10 | +0.6 | −0.0 | −0% | −0% | −0.01 | +0% | −0.02 | 0.9 |
| Uzaktan şut 16 | Teknik | 8 | +0.20 | −0.02 | +0.2 | −0.2 | +3% | −0% | −0.00 | −1 *% | −0.03 * | 0.8 |
| Vizyon 16 | Zihinsel | 8 | +0.19 | +0.10 | +0.6 | +0.2 | −1% | −1% | +0.01 | −0% | +0.02 | 0.8 |
| Refleks 16 | Bekçi | 8 | +0.17 | −0.02 | +0.3 | +0.1 | −0% | +4% | −0.01 | +0% | +0.01 | 0.9 |
| Birebir 16 | Bekçi | 8 | +0.15 | −0.02 | +0.5 | +0.1 | +0% | +1% | −0.00 | +0% | −0.00 | 0.5 |
| Markaj 16 | Savunma | 8 | +0.15 | −0.02 | +0.5 | +0.1 | +0% | +1% | −0.00 | +0% | −0.00 | 0.5 |
| Uzanma 16 | Fiziksel | 8 | +0.10 | −0.02 | −0.2 | +0.1 | −0% | −2% | −0.01 | +0% | −0.05 * | 0.6 |
| Pas 16 | Teknik | 8 | +0.08 | +0.10 | +0.5 | +0.2 | +2% | +3% | +0.01 | −0% | −0.00 | 0.3 |
| Soğukkanlılık 16 | Zihinsel | 8 | +0.07 | +0.23 | +0.3 | −0.2 | +1% | +2% | +0.00 | +0% | +0.01 | 0.3 |
| Çeviklik 16 | Fiziksel | 8 | +0.07 | +0.23 | +0.1 | +0.1 | +2% | +1% | +0.00 | −0% | +0.03 | 0.2 |
| Alan hakimiyeti 16 | Bekçi | 8 | +0.02 | −0.27 * | +0.2 | −0.3 | −9% | −6% | +0.00 | +0% | −0.03 | 0.1 |
| Kesme 16 | Savunma | 8 | −0.02 | −0.02 | +0.3 | −0.5 | −5% | −0% | +0.00 | −0% | +0.00 | -0.1 |
| İlk kontrol 16 | Teknik | 8 | −0.05 | −0.02 | +0.2 | +0.2 | −2% | −1% | +0.00 | −0% | −0.01 | -0.2 |
| Hücumda yer alma 16 | Yer alma | 8 | −0.09 | −0.15 | −0.0 | −0.2 | −7 *% | +3% | +0.00 | +0% | −0.06 * | -0.9 |
| Yerleşim 16 | Bekçi | 8 | −0.09 | −0.02 | −0.4 | −0.4 | −1% | −6% | −0.01 | −0% | −0.01 | -0.4 |
| Toparlanma 16 | Fiziksel | 8 | −0.12 | −0.27 * | +0.5 | −0.0 | +5% | +1% | +0.04 * | −1 *% | +0.07 | -0.8 |
| Aldatma 16 | Teknik | 8 | −0.13 | −0.15 | −0.8 | −0.4 | −1% | +1% | +0.01 | −0% | +0.05 | -0.6 |
| Savunmada yer alma 16 | Yer alma | 8 | −0.13 | −0.02 | −0.0 | +0.2 | +12 *% | +4% | −0.02 | +0% | +0.01 | -0.4 |
| Teknik 16 | Teknik | 8 | −0.14 | −0.15 | −0.2 | −0.4 | +2% | +0% | −0.01 | −0% | +0.02 | -0.4 |
| Yaratıcılık 16 | Zihinsel | 8 | −0.19 | −0.02 | −0.5 * | −0.0 | −6% | −1% | +0.01 | +1% | +0.09 | -0.9 |
| Yükleme 16 | ALAN | 8 | −0.19 | −0.15 | −0.5 | −0.0 | +2% | −0% | −0.01 | +0% | +0.07 | -0.9 |
| Okuma 16 | Zihinsel | 8 | −0.21 | −0.15 | −0.4 | −0.4 | −4% | −9% | −0.01 | +0% | −0.05 * | -0.9 |
| Gövde 16 | Fiziksel | 8 | −0.23 | −0.02 | −0.9 | +0.2 | −2% | +1% | −0.00 | +1% | +0.02 | -0.6 |
| Karar 16 | Zihinsel | 8 | −0.28 | −0.15 | −0.4 | −0.0 | −4% | +3% | −0.03 | +1% | −0.03 | -1.4 |
| Bitiricilik 16 | Teknik | 8 | −0.28 | +0.10 | −0.7 | +0.2 | +1% | +4% | +0.02 | −1% | +0.01 | -1.3 |
| Çalışkanlık 16 | Zihinsel | 8 | −0.31 | −0.40 | −0.8 | −0.0 | +0% | −6% | +0.02 | +0% | +0.07 | -1.3 |
| Patlayıcılık 16 | Fiziksel | 8 | −0.39 | −0.27 | −0.7 | +0.1 | +2% | +0% | −0.01 | +0% | −0.04 * | -1.1 |
| Alan kontrolü 16 | ALAN | 8 | −0.40 * | −0.15 | −1.3 * | −0.0 | −5% | −2% | +0.01 | +0% | −0.06 * | -2.9 |
| Sürme 16 | Teknik | 8 | −0.53 * | +0.23 | −1.7 * | −0.2 | −5 *% | −1% | −0.02 | +1% | −0.07 * | -2.3 |

Referans: aynı tohumdaki bütün kolların ortalaması (taban dahil). Her sayı referansa göre değişim (A − rakip farkının değişimi; top payı, pas isabeti, bar, dipte ve şut şarjı A'nın kendi değerinin değişimi). `*`: standart hatanın iki katından büyük.

## Okuma

1. **Hız hâlâ en etkili özellik ve tek açık baskın.** +0,84 xG, +1,8 şut. Yetenek sistemi Hız'ı sulandırmadı: mekanik tablosunda toplam ağırlığı 2,3 (34 içinde orta sıra) ama temel koşu hızının tamamı ve sprint katı ondan. Hız her mekaniğin önünde: kim önce varırsa o kazanıyor.
2. **Tepki ikinci** (+0,49 xG, top payı +%12): algı gecikmesi, hazırlık, karar süresi. Önceki ölçümde de Tepki öndeydi.
3. **Alan gücü artık işe yarıyor** (+0,34 xG \*): Aşama 7'den sonra ALAN grubunun ilk anlamlı etkisi. Dayanıklılık barı büyütüyor (bar +0,06 \*, dipte −%2 \*), gol farkı +0,35 \*.
4. **Ters çalışan özellikler (16 daha kötü):**
   - **Sürme** −0,53 xG \*, şut −1,7 \*, top payı −%5 \*: iyi süren oyuncu daha çok sürüyor, daha az şut atıyor, topu kaybediyor. Kafa sürmenin değerini kendi becerisiyle fazla tartıyor olabilir.
   - **Alan kontrolü** −0,40 xG \*, şut −1,3 \*, şut şarjı −0,06 \*.
   - Patlayıcılık, Çalışkanlık, Bitiricilik, Karar, Okuma da eksi (−0,2 ile −0,4; tek tek anlamlı değil ama hepsi aynı yönde). **Okuma** 16'da pas isabeti −%9. Bitiricilik şutu iyileştirmesi gerekirken xG −0,28.
   - Bu, eski ölçümdeki "Görüş ters çalışıyor" sorununun yeni hâli: özellik becerisi kararı değiştiriyor (daha riskli seçenek, daha çok sürme), kazanç bedeli karşılamıyor.
5. **Hiç etkisi olmayanlar:** **Birebir** ve **Markaj** 16'da maçlar tabanla **bit bit aynı**:
   - Birebir: Bekçi aşaması (8) bağlanmadı; beklenen.
   - Markaj: varsayılan taktikte adam markajı yok (savunma talimatı `adam` 0). Markaj yalnız adam adama oynayan takımda işe yarar; bu tasarım gereği olabilir ama kadro kurarken menajere bunun görünmesi gerekir.
6. Teknik grubunun çoğu (Teknik, İlk kontrol, Pas, Aldatma) etkisiz ya da hafif eksi: en büyük ağırlığı taşıyan Teknik (mekaniklerde toplam 9,1) sahada fark yaratmıyor.

## Öneriler (motor oturumu için)
- Sürme, Alan kontrolü ve Okuma'nın neden ters çalıştığı tek tek izlenmeli (kafanın bu özelliklerle hangi kararı değiştirdiği). Bitiricilik 16'nın xG'yi düşürmesi de.
- Hız'ın payı: temel koşu hızı ve sprint katı %100 Hız. Bu iki mekaniğe başka bileşen (Patlayıcılık, Denge) katmak baskınlığı azaltır.
- Bekçi aşaması bağlanınca Bekçi özellikleri ayrıca ölçülmeli.
- Bu ölçüm 4'e düşen özelliklerle ve daha uzun maçlarla tekrarlanmalı (Dayanıklılık ve Toparlanma ancak uzun maçta görünür).
