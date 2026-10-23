# ALAN · Yetenek ölçümü 2: dallanma testi, özellik 4'e karşı 10 (23 Ekim 2026)

> Motor: main 614496b (yetenek Aşama 1–8, ince yapı 1–7). Araçlar: `tools/dal-olc.js`, `tools/dal-ozet.js`. Veriler: `lab/yt2/sonuc` (pozisyon değeriyle), `lab/yt2/ilk` (ilk koşu, yalnız şut ve xG; dallar birebir aynı).
> İstek: yetenek ölçümünü 4 ve 10 üzerinden, daha hızlı ama sonuç verecek biçimde.

## Yöntem
- Herkes 10 (34 özellik), taktik varsayılan. 4 taban maç; her birinden 16 an (300–2550. tik, 150 tikte bir) = 64 an.
- Her anda maçın tam kopyası; 35 kol (taban ve her özellik A'da 4) aynı kopyadan 180 tik (6 sn) ilerletilir. Aynı an, aynı rastgele sayı durumu.
- Ölçü: pozisyon değeri (motorun kafasının kullandığı `AlanDecide.V`; top kimde ve nerede), dal boyunca her 10 tikte; ayrıca şut, xG, top payı, top kazanma, bar.
- 2240 dal, 4 çekirdekte ~55 dk.

## Sonuç

64 an × 34 kol; her dal 6 sn. Özellik 4'e düşünce, aynı andaki taban (10) dala göre değişim (A − rakip). Sıra: pozisyon değeri farkına göre, en çok zarar görenden.

| özellik | grup | an | pozisyon değeri farkı | top A'dayken başlayan | değilken | xG farkı | A hücumdayken | A savunmadayken | şut farkı | top kazanma farkı | top payı | bar | tabanla aynı dal |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Hız | Fiziksel | 64 | −0.0187 | −0.0101 | −0.0206 | −0.010 | −0.023 | +0.056 | −0.02 | −0.02 | −2.6% | −0.034 * | 0/64 |
| Çeviklik | Fiziksel | 64 | −0.0100 | −0.0075 | −0.0105 | −0.030 | −0.023 | −0.008 | −0.02 | +0.09 | +7.3% | −0.003 | 0/64 |
| Okuma | Zihinsel | 64 | −0.0083 | −0.0113 | −0.0076 | −0.008 | −0.023 | −0.057 | +0.00 | −0.03 | +1.4% | −0.022 | 0/64 |
| Alan kontrolü | ALAN | 64 | −0.0077 | −0.0156 | −0.0061 | −0.003 | +0.006 | −0.036 | +0.00 | +0.02 | +2.2% | −0.033 * | 4/64 |
| Alan hakimiyeti | Bekçi | 64 | −0.0037 | +0.0002 | −0.0045 | +0.013 | +0.000 | +0.043 | +0.00 | +0.02 | +2.7% | −0.014 | 44/64 |
| Aldatma | Teknik | 64 | −0.0025 | −0.0145 | +0.0000 | +0.000 | −0.002 | +0.003 | +0.02 | −0.03 | +0.9% | −0.001 | 25/64 |
| Yerleşim | Bekçi | 64 | −0.0022 | −0.0002 | −0.0026 | +0.017 | +0.000 | +0.055 | +0.06 * | −0.03 | −0.6% | −0.002 | 34/64 |
| Markaj | Savunma | 64 | +0.0000 | +0.0000 | +0.0000 | +0.000 | +0.000 | +0.000 | +0.00 | +0.00 | +0.0% | +0.000 | 64/64 |
| Refleks | Bekçi | 64 | +0.0001 | −0.0022 | +0.0005 | −0.027 | +0.000 | −0.036 | −0.02 | +0.02 | +4.2% | −0.006 | 27/64 |
| Uzaktan şut | Teknik | 64 | +0.0001 | −0.0017 | +0.0005 | −0.004 | −0.023 | +0.000 | +0.02 | +0.00 | +0.1% | +0.001 | 52/64 |
| Birebir | Bekçi | 64 | +0.0007 | +0.0000 | +0.0008 | +0.000 | +0.000 | +0.000 | +0.02 | +0.00 | +0.6% | +0.001 | 61/64 |
| Uzanma | Fiziksel | 64 | +0.0012 | −0.0099 | +0.0035 | −0.002 | −0.023 | +0.055 | +0.05 | +0.00 | +1.9% | −0.001 | 5/64 |
| Savunmada yer alma | Yer alma | 64 | +0.0013 | +0.0042 | +0.0007 | −0.035 * | −0.023 | −0.076 | −0.03 | −0.03 | −0.2% | −0.003 | 16/64 |
| Yaratıcılık | Zihinsel | 64 | +0.0017 | +0.0009 | +0.0019 | +0.005 | −0.023 | +0.000 | +0.02 | +0.00 | +2.4% | −0.002 | 38/64 |
| Vizyon | Zihinsel | 64 | +0.0024 | +0.0031 | +0.0022 | −0.020 | −0.023 | −0.012 | −0.05 | +0.09 | +6.1% | +0.004 | 8/64 |
| Bitiricilik | Teknik | 64 | +0.0025 | −0.0005 | +0.0032 | +0.000 | +0.000 | +0.000 | +0.02 | +0.02 | +1.6% | +0.008 * | 53/64 |
| Karar | Zihinsel | 64 | +0.0031 | −0.0017 | +0.0041 | +0.003 | +0.000 | +0.018 | +0.00 | +0.05 | +2.1% | −0.002 | 24/64 |
| İlk kontrol | Teknik | 64 | +0.0036 | +0.0134 | +0.0016 | −0.004 | −0.023 | +0.000 | +0.00 | +0.05 | +4.3% | −0.011 | 35/64 |
| Teknik | Teknik | 64 | +0.0038 | −0.0106 | +0.0068 | −0.006 | +0.002 | −0.007 | +0.02 | +0.06 | +2.3% | −0.008 | 22/64 |
| Kesme | Savunma | 64 | +0.0040 | +0.0007 | +0.0047 | −0.021 | +0.000 | +0.015 | −0.02 | −0.02 | +1.5% | −0.006 | 33/64 |
| Yükleme | ALAN | 64 | +0.0040 | +0.0010 | +0.0046 | −0.004 | −0.023 | +0.000 | −0.02 | +0.06 | +4.0 *% | −0.002 | 36/64 |
| Patlayıcılık | Fiziksel | 64 | +0.0045 | +0.0044 | +0.0045 | +0.007 | −0.023 | +0.006 | +0.05 | +0.09 | +6.9% | +0.012 | 0/64 |
| Gövde | Fiziksel | 64 | +0.0051 | −0.0232 | +0.0110 | −0.003 | −0.023 | −0.006 | −0.03 | +0.02 | +0.6% | −0.018 * | 4/64 |
| Çalışkanlık | Zihinsel | 64 | +0.0052 | +0.0082 | +0.0046 | +0.007 | +0.019 | +0.029 | +0.02 | −0.02 | +0.1% | +0.004 | 2/64 |
| Pas | Teknik | 64 | +0.0055 | +0.0134 | +0.0038 | −0.004 | −0.023 | +0.000 | +0.00 | +0.11 * | +6.4 *% | −0.013 | 26/64 |
| Toparlanma | Fiziksel | 64 | +0.0062 | −0.0081 | +0.0092 | −0.008 | −0.023 | −0.001 | +0.00 | −0.02 | +0.0% | −0.038 * | 1/64 |
| Hücumda yer alma | Yer alma | 64 | +0.0063 | +0.0072 | +0.0062 | +0.006 | −0.023 | +0.000 | +0.03 | +0.00 | +1.6% | +0.004 | 32/64 |
| Güç | Fiziksel | 64 | +0.0065 | −0.0335 | +0.0148 | −0.016 | −0.023 | +0.021 | +0.02 | −0.05 | +0.5% | −0.010 | 0/64 |
| Dayanıklılık | Fiziksel | 64 | +0.0067 | +0.0093 | +0.0062 | +0.005 | +0.024 | −0.017 | +0.03 | −0.05 | +0.5% | −0.061 * | 0/64 |
| Tepki | Zihinsel | 64 | +0.0071 | +0.0156 | +0.0053 | −0.011 | −0.023 | +0.060 | +0.05 | +0.06 | +1.0% | −0.014 | 1/64 |
| Soğukkanlılık | Zihinsel | 64 | +0.0072 | +0.0070 | +0.0072 | −0.012 | −0.016 | −0.001 | −0.02 | +0.05 | +2.9% | −0.003 | 20/64 |
| Sürme | Teknik | 64 | +0.0083 | +0.0042 | +0.0091 | +0.002 | −0.023 | +0.000 | +0.03 | +0.00 | +1.2% | −0.010 | 29/64 |
| Denge | Fiziksel | 64 | +0.0115 | −0.0148 | +0.0170 | −0.024 | −0.023 | −0.034 | +0.02 | +0.11 | +7.3 *% | −0.018 | 0/64 |
| Alan gücü | ALAN | 64 | +0.0124 | +0.0137 | +0.0122 | −0.003 | −0.023 | +0.055 | +0.03 | +0.00 | +6.3% | +0.012 | 1/64 |

Pozisyon değeri: motorun kafasının pozisyon tartısı (AlanDecide.V), dal boyunca her 10 tikte; A topa sahipken artı, rakip sahipken eksi.
`*`: fark standart hatanın iki katından büyük. "A hücumdayken": dalın başladığı anda top A'da; "savunmadayken": rakipte. "Tabanla aynı dal": özellik değişse de dalın bütün sayıları birebir aynı (özellik o anda hiç iş görmedi).

## Okuma (dürüst)
1. **Yöntem beklenen gücü vermedi.** Motor kaotik: iki dal 1–2 saniyede ayrışıyor, eşleşmenin kazancı küçük. Taban dalların pozisyon değerinin dağılımı sd 0,119; eşli farkınki sd 0,076 (yalnız 1,6 kat daha az gürültü). 6 saniyelik dalda şut çok nadir (dal başına 0,14), xG ölçüsü işe yaramıyor (ilk koşu).
2. Bu güçle **hiçbir özelliğin pozisyon değerine etkisi anlamlı değil.** En büyüğü Hız (4'te −0,019 ± 0,010, iki standart hata sınırında); Çeviklik, Okuma, Alan kontrolü de eksi yönde. Alan gücü ve Denge 4'te artı (gürültü olası).
3. **Güvenilir çıkanlar:**
   - **Bar:** Dayanıklılık (−0,061 \*), Toparlanma (−0,038 \*), Hız (−0,034 \*), Alan kontrolü (−0,033 \*), Gövde (−0,018 \*) 4'te barı düşürüyor. Beklenen yönde.
   - **Ne kadar iş gördüğü** ("tabanla aynı dal"): Markaj 64 dalın 64'ünde hiç iş görmüyor (varsayılan taktikte adam markajı yok); Birebir 61, Bitiricilik 53, Uzaktan şut 52, Alan hakimiyeti 44 dalda hiç iş görmüyor (yalnız şut ve Bekçi anlarında devreye giriyor). Hız, Çeviklik, Denge, Güç, Patlayıcılık, Okuma, Dayanıklılık her dalda iş görüyor.
4. **Ters işaret kuşkusu:** Pas 4'te top kazanma +0,11 \*, top payı +%6,4 \*; Denge 4'te top payı +%7,3 \*; Yükleme 4'te top payı +%4,0 \*. Kötü özellikle daha çok top. Tam maç ölçümüyle doğrulanmalı.

## Öneri
Bu motor için hızlı yol dallanma değil. Sonuç veren yol tam maç ölçümü (yetenek ölçümü 1'in düzeni): 34 özellik 4'te + taban, 12 tohum × 90 sn = 420 maç, bugünkü hızla ~2 saat. Eşli referans (aynı tohumda bütün kolların ortalaması) gürültüyü azaltır.
