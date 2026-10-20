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

## Açık

- Ölçüm aynı oyuncularla (herkes 10). Hızlı bir forvetle (Hız 16) önde bekleme farklı sonuç verebilir; ayrıca denenebilir.
- 20 maçta gol farkları anlamlı değil. Kesin söz için 40+ maç gerekir.
