# ALAN · Blok ölçümü 1: Dikine'ye karşı derin blok ve kontra (20 Ekim 2026)

> Motor: main d72cef5. Araçlar: `tools/blok-olc.js`, `tools/blok-ozet.js`. Veriler: `lab/bk1/`.
> Soru (kullanıcının): Dikine'ye karşı ciddi blok savunması, önde savunmaya gelmeyen bir oyuncu bırakarak, kontra kovalayan takımdan iyi mi?

## Düzen

- Herkes 10. Rakip hep Dikine (`T_dikine`). 4 tarz × 20 maç = 80 maç. Her maç 9000 tik (sitedeki 5 dk). Taraflar dengeli.
- **Blok, 1 önde** (`B_onde`):
  - Düşük blok, pres 0, şiddet 0,3. Karşı pres yok.
  - Top kazanılınca hızlı ve riskli oynar (tempo 0,9, risk 0,8, geçişte her zaman ileri).
  - Savunma dizilişi 3-2-1, hücum 1-2-3.
  - **`onde: 1`**: motorun kendi ayarı. Savunma dizilişinin en öndeki oyuncusu "önde bekle" işini alır: rakibin son adamının 3 br önünde durur, prese ve geri koşuya katılmaz.
- **Blok, hepsi geride** (`B_hepsi`): aynısı, `onde` yok. Herkes savunur.
- **Kale önü** ve **Kontra**: hazır tarzlar (`tools/taktik-ayar.js`). Kontra düşük blokla oynar ama önde adam bırakmaz.

Önde bekleme gerçekten oluyor:
- Önde bekleyen, top rakipteyken ortalama **f 67**'de duruyor. Öbür tarzlarda en öndeki oyuncu f 46–51'de.
- Top kazanılınca ilk pasların **%44'ü** ona gidiyor. Öbür tarzlarda %20–29.

## Sonuç

| A (rakip Dikine) | G-B-M | gol A / D | şut A / D | xG A / D | xG farkı | geçiş şutu A / D | top A | bar A / D |
|---|---|---|---|---|---|---|---|---|
| Blok, 1 önde | 5-5-10 | 0,85 / 1,15 | 4,3 / 6,7 | 1,63 / 2,72 | **−1,09 ±0,37 \*** | 0,3 / 1,0 | %47 | 0,69 / 0,62 |
| Blok, hepsi geride | 5-6-9 | 0,60 / 0,95 | 3,8 / 5,3 | 1,40 / 2,25 | −0,85 ±0,41 \* | 0,3 / 0,7 | %46 | 0,65 / 0,65 |
| Kale önü | 8-5-7 | 1,00 / 1,05 | 4,0 / 4,7 | 1,24 / 1,95 | −0,71 ±0,33 \* | 0,1 / 0,6 | %60 | 0,77 / 0,73 |
| Kontra | 3-10-7 | 0,50 / 0,95 | 3,9 / 5,7 | 1,44 / 2,18 | −0,74 ±0,37 | 0,5 / 0,8 | %46 | 0,66 / 0,65 |

Gol farkları (A − Dikine): Blok 1 önde −0,30, Blok hepsi geride −0,35, Kale önü −0,05, Kontra −0,45. Hiçbiri standart hatanın iki katını geçmiyor. 20 maçta gol sayısı küçük; xG daha güvenilir.

`*`: fark standart hatanın iki katından büyük. Geçiş şutu: topu kazandıktan sonraki 6 sn içinde.

## Okuma

1. **Dikine'ye karşı hiçbir tarz kazanmıyor.** Dördü de xG'de eksi. Dikine'nin panzehiri hâlâ yok; önceki ölçümlerle aynı.
2. **Önde adam bırakmak bloğu zayıflatıyor, kontrayı güçlendirmiyor.**
   - Hepsi geride olan bloğa göre:
     - Dikine'nin şutu 5,3'ten 6,7'ye, xG'si 2,25'ten 2,72'ye çıkıyor (savunmada bir kişi eksik).
     - Bizim şutumuz yalnız 3,8'den 4,3'e, xG'miz 1,40'tan 1,63'e çıkıyor.
   - Net: xG farkı −0,85'ten −1,09'a kötüleşiyor. Gol farkı ise hemen hemen aynı (−0,35 → −0,30), yani fark kesin değil.
3. **Kontra şuta dönüşmüyor.** Bütün tarzlarda maç başına geçiş şutu 0,1–0,5. Önde adam bırakan takımda bile 0,3. Dikine'nin geçiş şutu bizimkinden fazla.
4. **Bekleme bedeli yok:** önde bekleyenin takımında bar 0,69, rakibinde 0,62. Ayakta bekleyen forvet dinç kalıyor ama bunun maçta bir karşılığı yok.

### Önde bekleyene giden pas ne oluyor (8 maç, 32 pas, ayrıntı izi)

| sonuç | pas |
|---|---|
| Önde bekleyen aldı, **10 sn içinde şut yok** | 12 |
| Pası başkası aldı | 7 |
| Pas kesildi | 6 |
| Aldı, sonra top kaybedildi | 3 |
| **Şut** | 2 |
| Pas sönmüş ya da boşta kalmış | 2 |

- Pas kendi bölgemizden (x 23 / 120) atılıyor, forvet topu ancak **orta sahada** (x 55) alıyor. Varış ~52 tik (1,7 sn).
  - Yani forvet son adamın omzunda beklese de top ona çizginin önünde, geriye gelerek ulaşıyor.
  - Arkaya, son adamın gerisine top gitmiyor.
- Topu alan forvet tek başına kaleye gitmiyor. Arkadan gelenleri bekliyor, kontra hızını yitiriyor.

## Neden (yorum, doğrulanmadı)

- **Son adamın arkası yok.** Önde bekleyen rakibin son adamının 3 br önünde duruyor; son adam da forvetin kale tarafında. Uzun top forvetin önüne değil, ayağına ya da geriye gidiyor. Bu, arka hat planındaki konuyla aynı: arkada boşluk olmadan kontranın bitişi yok (`plan/TASARIM-arka-hat.md`).
- **Tek forvet yalnız kalınca durur.** Kafa, 1'e 2'de sürmeyi ve şutu değerli görmüyor; destek bekliyor. Gerçek futbolda kontra forveti ya sürer ya faul alır ya da top tutar ve takım çıkar. Burada top tutuyor ama takım gelene kadar savunma da yerleşiyor.
- **Savunmada bir kişi eksik olmanın bedeli tam ödeniyor**, kazancı alınamıyor. Bu yüzden tarz bugün bir seçenek değil.

## Öneriler (motor oturumu için, karar onların)

- Önde bekleyene, topu kazandığımız an **son adamın arkasına koşu** değeri (arka hat planındaki "derinde bekle" cevabı). Pas forvetin önüne, boşluğa atılabilmeli.
- Kontrada tek forvetin **sürüp şut** seçeneği: 1'e 1 ve 1'e 2'de kafa şut ihtimalini daha cesur tartmalı, ya da forvet topu tutup faul ya da destek almalı (top tutma değeri).
- Dikine'nin panzehiri için: derin bloğun **Dikine'nin uzak şutlarına** karşı bir avantajı olmalı. Önceki ölçümde derin bloğun kaybı uzak şutlardan geliyordu.

## Ek · Hızlı forvet (Hız 16)

> Veriler: `lab/bk2/`. Aynı motor (d72cef5), aynı 20 tohum, aynı rakip. Tek fark: A'nın en öndeki oyuncusunun Hız'ı 16 (öbürleri 10). `ONDE_OZ=hiz:16`.
> Kontrol kolu: aynı hızlı oyuncu, ama önde beklemiyor, blokta savunuyor (Blok, hepsi geride).

| A (rakip Dikine) | G-B-M | gol A / D | şut A / D | xG A / D | xG farkı | geçiş şutu A / D | ilk pas öndekine |
|---|---|---|---|---|---|---|---|
| Blok, 1 önde, **hızlı forvet** | 8-5-7 | 1,45 / 1,15 | 5,7 / 7,8 | 2,34 / 3,51 | −1,17 ±0,35 \* | 0,8 / 0,8 | %57 |
| Blok, hepsi geride, **hızlı forvet** | 7-8-5 | 0,80 / 0,75 | 5,3 / 5,5 | 2,18 / 2,06 | +0,11 ±0,34 | 0,5 / 0,9 | %38 |

Gol farkları (A − Dikine): önde bekleyen +0,30 ±0,35, blokta +0,05 ±0,28.

Aynı tohumlarda Hız 10'a göre değişim (eşli karşılaştırma):

| | gol farkı | xG farkı | atılan gol | yenen gol |
|---|---|---|---|---|
| Önde bekleyen hızlı | +0,60 ±0,50 | −0,08 ±0,36 | **+0,60 ±0,37** | 0,00 ±0,36 |
| Blokta hızlı | +0,40 ±0,41 | **+0,96 ±0,54** | +0,20 ±0,29 | −0,20 ±0,28 |

Önde bekleyene giden pas (8 maç, 29 pas): **şut 6** (Hız 10'da 32 pasta 2), 10 sn şutsuz 11, kesildi 7, kayıp 4, başkası aldı 1. Topu daha ileride alıyor: x 65 (Hız 10'da 55).

### Okuma

- **Hızlı forvet önde beklemeyi işe yarar hale getiriyor:** atılan gol 0,85'ten 1,45'e çıkıyor, ona giden pasın şuta dönme payı %6'dan %21'e çıkıyor. Yenen gol değişmiyor. Bu kurulumla A, Dikine'ye karşı ilk kez gol farkında artıda (+0,30), ama 20 maçta kesin değil.
- **xG başka şey söylüyor:** önde bekleyen düzende Dikine'nin xG'si yüksek kalıyor (3,51), ama bu golle karşılanmıyor (1,15). Hızlı forvetin şutları gerçekleşen golde xG'nin üstünde. 20 maçta ikisi de gürültülü.
- **Aynı hızlı oyuncu blokta da çok değerli:** hepsi geride düzende xG farkı −0,85'ten +0,11'e çıkıyor. Hızlı oyuncu savunmada boşluğu kapatıyor, top kazanınca da çıkış veriyor.
- Yani Hız'ın değeri iki rolde de büyük. Hangisinin daha iyi olduğu bu ölçümde ayrışmıyor: önde bekleyen daha çok gol atıyor (1,45'e 0,80) ama daha çok pozisyon veriyor; blokta oynayan maçı dengeliyor.
- Özellik ölçümleriyle aynı sonuç: **Hız hâlâ en etkili özellik.** Tek bir oyuncunun Hız'ı 10'dan 16'ya çıkınca Dikine'ye karşı kayıp ortadan kalkıyor.

## Açık

- 20 maçta gol farkları anlamlı değil. Kesin söz için 40+ maç gerekir.
