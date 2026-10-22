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

## Özellik özellik ayrıntı

Her özellik için: hangi mekaniklere hangi ağırlıkla girdiği (`yetenek.js`, parantezde aşama; Aşama 8 Bekçi bağlanmadı), ölçülen ham değerler (8 maç ortalaması) ve yorum. Yorumlar doğrulanmadı; kol başına 8 maç gürültülü.

**Taban (herkes 10):** A maç başına 18,4 pas, isabet %91,8; şut 1,5 / 0,9; xG 0,61 / 0,38; top kazanma 2,8 / 2,6; ikili 2,8; bar 0,76; şut şarjı 0,26; top payı %51.

**Gürültü ölçüsü:** rakip hiçbir kolda değişmiyor, ama rakibin değerleri kollar arasında oynuyor: xG 0,07–0,79, pas isabeti %87–95, şut 0,25–2,0. Tek bir kolun bu aralıkta kalan farkı tek başına bir şey söylemez.

### Fiziksel

**Hız** — Temel koşu hızı 100, Sprint katı 100 (A4), Tehlike tahmini 30 (A3).
- xG 1,01 / 0,08 (+0,84 \*), şut 2,3 / 0,3, top kazanma 3,4, top payı %44 (−7 \*), pas isabeti %84,8.
- Hızlı takım topu daha az tutuyor ama daha çok ve daha iyi şut buluyor; pas isabeti düşüyor çünkü daha ileri, riskli paslar seçiliyor (koşu yarışlarını kazanacağını biliyor). Hız, motordaki her "kim önce varır" hesabının girdisi: pas, kesme, boş top, pres, şut kapatma. İki mekaniğin tamamı onda.
- Öneri: temel koşu hızına ve sprint katına başka bileşen (Patlayıcılık, Denge, Dayanıklılık) katmak; ya da sprint bedelini Hız'la büyütmek (hızlı oyuncu daha çabuk yorulsun).

**Patlayıcılık** — Hızlanma 60, İvme bedeli 40, Kenardan çalım 30 (A4), Markajdan kurtulma ve Adamına yetişme 20 (A6); Dalış ve Yana kayma (A8, bağlı değil).
- xG 0,50 / 0,79 (−0,39), şut 1,5 / 2,0, ikili 1,5.
- Beklenen artı, çıkan eksi; ama eksinin kaynağı rakibin xG'sinin yükselmesi (0,79, aralığın en üstü). Gürültü olabilir. Hızlanmanın getirisi küçük: 16'da hızlanma çarpanı ~1,18.
- Öneri: tekrar ölç; Hız'ın bir kısmını buraya taşımak dengeyi kurar.

**Çeviklik** — 13 mekanik: Fren ve yön değiştirme 60, İvme bedeli 60, Fake süresi 50, Hamle 50, Yana kayma 50 (A8)…
- xG 0,57 / 0,41 (+0,07), ikili 1,8 (taban 2,8).
- Etkisiz. Payı çok mekaniğe dağılmış; her birinde küçük fark, toplamda iz yok. Dönüş çarpanı 16'da ~1,1.

**Güç** — 12 mekanik: Pasın en sert hali 70, Şutun gücü 45, Omuz 50, Gövde teması bedeli 50, Vuruşun taban enerjisi 50 (A7), Kütle 40…
- xG 0,66 / 0,31 (+0,26), gol 0,50 / 0,13, top kazanma 3,1, top payı %59 (+8).
- Yönü doğru: topu tutmaya ve korumaya yarıyor (kalkan, omuz), şut ve pas sertleşiyor. Anlamlı değil ama tutarlı.

**Denge** — 15 mekanik: Sendeleme 100, İkili sonrası sersemleme 70, Dengenin geri gelmesi 60, Dengeye direnç 50…
- xG 0,56 / 0,26 (+0,21), ikili 1,1.
- Dengenin işi ikili ve temas; maç başına 1–3 ikili var, o yüzden etkisi sınırlı. İkili sayısı artmadıkça Denge az değer taşır.

**Gövde** — Kütle 60, İtilme payı 60, Omuz 30, Sızma 20, Temas bedeli 20.
- xG 0,52 / 0,66 (−0,23), ikili 2,5.
- Etkisiz; eksi işaret rakibin yüksek pas isabetinden (%94,7, aralığın üstü). Gövdenin işi de ikili; ikili az.

**Uzanma** — Uzanma menzili 70, Dürtme 25, Pas arası kesme 15 (A3–A4); Dalış 40 (A8).
- xG 0,37 / 0,18 (+0,10), top kazanma 2,8 (değişmedi), şut şarjı 0,21 (−0,05 \*).
- Etkisiz: menzil büyüyünce kesme artması beklenirdi, artmadı. Kesme kararları menzilden çok seçimden geliyor.

**Dayanıklılık** — Bar büyüklüğü 100, Sprint bedeli 100 (A4–A7), Barın dolması 30, Alan ve şarj bedeli 30.
- Bar 0,83 (+0,06 \*), dipte −%2 \*, gol 0,38 / 0 (+0,35 \*), xG +0,28.
- Beklendiği gibi: büyük bar daha çok sprint ve iş demek. 90 sn'de bile görünüyor; uzun maçta daha büyük olmalı.

**Toparlanma** — Barın dolması 70 (A7), Dengenin geri gelmesi 20.
- Bar 0,80 (+0,04 \*), dipte −%1 \*, ama gol 0,13 / 0,38 (−0,27 \*), xG −0,12.
- Bar doğru yönde, sonuç ters. 90 sn'lik maçta barın dolmasının değeri az; gol farkı muhtemelen gürültü. Uzun maçta ölçülmeli.

### Teknik

**İlk kontrol** — İlk kontrol kapasitesi 65, Alışta şarjı korumak 60 (A7), Yeniden dokunma 50, Topu döndürme bedeli 30.
- xG 0,48 / 0,44 (−0,05), şut şarjı 0,26 (değişmedi).
- Etkisiz. Kontrol hatası zaten nadir; şarjı korumak şut şarjına yansımamış.

**Teknik** — 26 mekanik, hepsinde %15–70: Topu döndürme bedeli 70, Bırakma zamanlaması 60, Fiske 60, Topu döndürme hızı 50, Dolaş 50, Vuruşun taban enerjisi 50 (A7), Plase 40…
- xG 0,45 / 0,50 (−0,14), şut 1,3 / 1,3.
- Etkisiz. "Her şeye biraz" çalışan özellik: her mekanikte küçük pay, toplamda iz yok. Mekanik tablosunda en çok ağırlık taşıyan ikinci özellik ama sahada sıfır.
- Öneri: Teknik'in payını birkaç mekanikte yoğunlaştırmak (örneğin dolaş, fiske, bırakma) ki bir şeye gerçekten yarasın.

**Sürme** — Topla koşu hızı 60, Sürme bedeli 60, Kenardan çalım 30, Dürtmeye karşı korumak 30, Fiskede kaçırma 30.
- xG 0,25 / 0,69 (−0,53 \*), şut 0,5 / 2,0 (−1,7 \*), top payı %46 (−5 \*), pas isabeti %87,3, şut şarjı 0,20.
- Açıkça ters: iyi süren takım daha çok sürüyor, neredeyse şut atmıyor, topu kaybedip rakibe pozisyon veriyor. Sürme hızlanıp ucuzlayınca kafa sürmeyi fazla seçiyor; sürmenin sonu (kayıp, rakibin yerleşmesi) yeterince tartılmıyor.
- Öneri: kafanın sürme seçeneğinin değerini izlemek: sürmeden sonra ne oluyor, kafa ne bekliyordu.

**Pas** — Pasın yön isabeti 70, Kenardan pasın isabeti 60, Pasın hız isabeti 50, Bırakma 40, Tek dokunuş isabeti 40.
- A isabeti %93,3 (taban %91,8), maç başına pas 20,4 (18,4); xG +0,08, gol +0,10. Rakibin isabeti de %88,8 → %91,3 oynadı (rakip değişmediği hâlde): fark gürültü sınırında.
- Hâlâ etkisiz (19 Ekim ölçümünde de etkisizdi). Paslar Pas 10'da bile %92 tutuyor; kaybedilen paslar nişan hatasından değil, rakibin araya girdiği seçimlerden. Kafa iyi pasçıya daha zor pas seçtirmiyor: Pas 16'lık oyuncu 10'lukla neredeyse aynı pasları atıyor, yalnız biraz daha iyi vuruyor.
- Öneri: Pas'ın değeri koridordan gelsin: dar yoldan, rakibin uzanma menzilinden geçen topu daha az yakalatmak (yön ve hız isabeti), ve kafa iyi pasçı için dar koridoru açık görsün. O zaman Pas 16 daha ileri ve riskli pas atar, paslar tutar.

**Bitiricilik** — Plase 60, Şutun yön isabeti 40, Şut hedefi seçimi 50 (A5), Tehlike tahmini 40.
- xG 0,48 / 0,67 (−0,28), şut 1,4 / 1,9, gol 0,25 / 0,13.
- xG düşüyor ama gol artıyor (az sayı). xG pozisyondan (shotPk), isabetten değil: Bitiricilik şutun isabetini artırıyor ama xG ölçüsü bunu görmüyor. Bu özellik için doğru ölçü gol / xG; 8 maçta gol çok az.
- Öneri: gol / xG ile daha uzun ölçüm.

**Uzaktan şut** — Şutun gücü 35, Şutun yön isabeti 35, Tehlike tahmini 30.
- xG 0,76 / 0,47 (+0,20), şut 1,5.
- Yön doğru: A'nın xG'si kollar içinde yüksek. Uzak şutların zaten fazla değerli sayıldığı motorda bu özellik kolayca baskın olabilir; xG düzeltilince yeniden bakılmalı.

**Aldatma** — Fake'in inandırıcılığı 60, Gövde fake'i 60, Presçinin fake'i 60 (A5), Fake bedeli 50, Fake süresi 30.
- xG 0,41 / 0,44 (−0,13), şut 1,1.
- Etkisiz: fake maç başına birkaç kez yapılıyor; inandırıcılığın artması sonuca yansımıyor.

### ALAN

**Alan gücü** — Alanın menzili 100, Alanın kuvveti 70, Bekçi alanı 40 (A7).
- xG 0,51 / 0,07 (+0,34 \*), rakip şut 0,5, pas isabeti %94,3 (+2,4 \*).
- ALAN grubunun ilk anlamlı etkisi: menzil büyüyünce alan daha sık devreye giriyor (alan kullanımı ölçümünde asıl engel menzildi). Rakibin şutu ve xG'si düşüyor: savunmada rakip pasını ve şutunu yavaşlatıyor.

**Alan kontrolü** — Alanın biçimi 100, Alanın enerji bedeli 70, Alanın kuvveti 30 (A7).
- xG 0,07 / 0,38 (−0,40 \*), şut 0,6 / 1,8 (−1,3 \*), şut şarjı 0,21 (−0,06 \*).
- Ters: hücumda neredeyse hiç şut yok. Alanın biçimi değişince (koni) oyuncunun kendi alanı kendi pasını ya da şarjını mı etkiliyor, yoksa ucuzlayan alan daha çok açılıp barı mı eritiyor (bar değişmedi: 0,78)? Bilinmiyor.
- Öneri: tek tek izlenmeli; ters çalışmanın en net örneklerinden biri.

**Yükleme** — Şarj aktarım hızı 80, Şarjın bardan bedeli 70, Koşarken aktarılan pay 60, Alışta şarjı korumak 40 (A7).
- Şut şarjı 0,34 (+0,08), ama şut 0,6 / 1,0, xG 0,19 / 0,28 (−0,19).
- Şarj artıyor (doğru), şut azalıyor: hızlı şarj eden oyuncu topu şarj etmek için tutuyor, şut anını kaçırıyor olabilir. Ya da daha şarjlı şut bekleyen kafa daha seçici.

### Zihinsel

**Tepki** — Algı gecikmesi 60, Hazırlık 50, Uzanmanın başlaması 50, Alanı açma zamanı 40 (A7), Prese başlamak 40 (A6), Karar süresi 30…
- xG 0,79 / 0,21 (+0,49), top payı %63 (+12 \*), maç başına pas 25,9 (en çok), isabet %92,8.
- İkinci en etkili özellik: daha hızlı algılayan ve karar veren takım topu daha uzun ve daha güvenli tutuyor. Önceki ölçümde de öndeydi.

**Okuma** — 25 mekanik (en geniş): Niyet sezmek 100, Rakibin enerjisini tahmin 100 (A5–A6), Fake'e kanmak 70, Alanın yönü 60, Açılan boşluk 60…
- xG 0,24 / 0,36 (−0,21), pas isabeti %85,2 (−6,6), top kazanma 3,3 / 3,6.
- Savunmada top kazanma biraz artıyor ama hücumda pas isabeti düşüyor. En geniş özellik, net etkisi eksi.
- Olası sebep: daha iyi okuyan oyuncu daha "cesur" seçiyor ama kafanın değerleri yanlış ölçeklenmişse cesaret kaybettiriyor (bkz. Karar).

**Vizyon** — Görülen arkadaşlar ve pas menzili 100, Algı hatası 60, Tek dokunuşu düşünmek 60, Alıcının ön hareketi 50 (A5)…
- xG 0,61 / 0,33 (+0,19), pas isabeti %89,3.
- Hafif artı. Eski ölçümdeki "Görüş ters çalışıyor" sorunu görünmüyor; düzelmiş olabilir.

**Yaratıcılık** — Düşünülen seçenek türleri 80, Kenardan pası düşünmek 60 (A5), Boşluk bulma 15.
- xG 0,46 / 0,56 (−0,19), şut 0,9 / 1,3 (−0,5 \*), şut şarjı 0,34.
- Daha çok seçenek türü (kenardan pas gibi) düşünülüyor, şut azalıyor. Yaratıcı seçenekler şuttan pay alıyor.

**Karar** — Seçenekleri tartmanın isabeti 80, Topu elde tutma 100 (A8), İleri bakış 60, Karar süresi 50, Alanın yönü 40 (A7)…
- xG 0,36 / 0,54 (−0,28), şut 1,1 / 1,4.
- Beklenen artı, çıkan eksi. **Önemli olasılık:** Karar 10'da seçimlere biraz rastgelelik (seçim hatası, A5) giriyor; Karar 16'da oyuncu kafanın değerine daha sadık. Kafanın değeri yanlışsa (örneğin uzak şutun fazla değerli sayılması) iyi karar veren oyuncu bu yanlışı daha tutarlı uygular. O zaman Karar, Okuma ve Bitiricilik'in eksileri aynı kökten gelir: kafanın değer ölçüsü.
- Öneri: bu kuşku doğruysa önce kafanın değer ölçüsü (xG ve şut değeri) düzeltilmeli; Karar ondan sonra artıya dönmeli.

**Soğukkanlılık** — Karar süresi 20, Tartı isabeti 20, Fiske 20, Şut isabeti 10, Karşı karşıya (A8).
- xG +0,07, gol 0,38 / 0,13 (+0,23).
- Baskıyı yumuşatıyor; baskı altında vuruş az olduğundan etkisi küçük. Mekaniklerde toplam ağırlığı en düşük özellik (0,9).

**Çalışkanlık** — Enerji harcama isteği 100, Geri koşuda ısrar 70, Markaj sadakati 30, Prese başlamak 30 (A6).
- xG 0,32 / 0,54 (−0,31), gol 0 / 0,38 (en kötü gol farkı), pas isabeti %85,7, bar 0,78.
- Ters: daha çok koşan takım daha kötü. Bar düşmemiş (0,78), yani yorulmuyor; ama geri koşu ve pres ısrarı hücumdan oyuncu eksiltiyor ya da düzeni bozuyor olabilir.

### Yer alma

**Hücumda yer alma** — Boşluk bulma 60, Markajdan kurtulma 50, Koşu zamanlaması 50 (A6).
- xG 0,27 / 0,27 (−0,09), top payı %44 (−7 \*), şut şarjı 0,20 (−0,06 \*).
- Oyuncular daha çok boşluğa koşuyor, top daha az tutuluyor, şutlar daha az şarjlı (koşu bitince hemen vuruluyor). Getirisi yok.

**Savunmada yer alma** — Bölge ve kapatma 65, Presin yaklaşımı 50, Adam devri 40, Presçinin fake'i 40.
- Top payı %63 (+12 \*), maç başına pas 26,8 (en çok), isabet %94,4; ama xG 0,40 / 0,44 (−0,13).
- Topu çok daha fazla tutuyor (iyi yerleşen savunma topu çabuk geri alıyor) ama bu bir pozisyona dönmüyor: kısır topa sahip olma.

### Savunma

**Kesme** — Pas arası kesme kapasitesi 60, Sızma 50, Dürtme 50.
- Top kazanma 2,4 / 2,9 (beklenenin tersi), xG −0,02.
- Etkisiz: kesme kapasitesi büyüyünce top kazanma artmıyor. Kesme fırsatı yeteneğe değil, kafanın araya girme seçimine bağlı.

**Markaj** — Markajın sadakati 70, Adamına yetişmek 50, Adam devri 15 (A6).
- Maçlar tabanla **bit bit aynı**. Varsayılan taktikte adam markajı yok (`adam` 0); Markaj yalnız adam adama oynayan takımda çalışıyor.
- Öneri: kadro ve taktik ekranında bu bağ görünmeli; ya da Markaj bölge savunmasında da bir işe bağlanmalı (bölgedeki rakibi takip).

### Bekçi

Bekçi aşaması (A8) bağlanmadı. Refleks, Yerleşim ve Alan hakimiyeti yine de etkili, çünkü motor eski anahtarlarını (`refleks`, `yerlesim`, `hakimiyet`) eski yoldan okuyor; Alan hakimiyeti ayrıca Bekçi alanına (A7) giriyor.

**Refleks** — rakip xG 0,20 (taban 0,38), A xG +0,17 farkla. Yönü doğru (Bekçi daha çok kurtarıyor), anlamlı değil.

**Yerleşim** — xG 0,57 / 0,57 (−0,09). Etkisiz.

**Alan hakimiyeti** — gol 0 / 0,25 (−0,27 \*), top payı %42 (−9), pas isabeti %88,1. Tutarsız; 8 maçta gürültü olabilir. Bekçi alanının büyümesi kendi takımının Bekçi'ye pasını ya da çıkışını etkiliyor olabilir.

**Birebir** — Maçlar tabanla **bit bit aynı**: Kalesinden çıkış ve Karşı karşıya mekanikleri A8'de; bağlanmadı.

## Öneriler (motor oturumu için)
- Sürme, Alan kontrolü, Çalışkanlık ve Okuma'nın neden ters çalıştığı tek tek izlenmeli (kafanın bu özelliklerle hangi kararı değiştirdiği).
- Karar'ın eksisi doğrulanmalı: iyi karar veren oyuncu kafanın yanlış değerini daha tutarlı uyguluyorsa, önce değer ölçüsü (xG, şut değeri) düzeltilmeli.
- Pas'ın değeri koridora bağlanmalı (bkz. Pas).
- Teknik ve Çeviklik gibi çok dağınık özelliklerin payı birkaç mekanikte yoğunlaştırılmalı.
- Hız'ın payı: temel koşu hızı ve sprint katı %100 Hız. Bu iki mekaniğe başka bileşen (Patlayıcılık, Denge) katmak baskınlığı azaltır.
- Bekçi aşaması bağlanınca Bekçi özellikleri ayrıca ölçülmeli.
- Bu ölçüm 4'e düşen özelliklerle ve daha uzun maçlarla tekrarlanmalı (Dayanıklılık ve Toparlanma ancak uzun maçta görünür).
