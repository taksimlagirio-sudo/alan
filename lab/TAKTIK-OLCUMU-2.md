# ALAN · Taktik ölçümü 2: üç beklenen ilişki (sitedeki motor main 6a73024, 19 Ekim 2026)

Taktik ölçümü 1'de görünmeyen üç futbol ilişkisi hedefli eşleşmelerle incelendi:
1. Direkt oyun yüksek hatta karşı güçlü olmalı (ofsayt yok).
2. Pres, topa sahip olan takımı bozmalı.
3. Derin blok direkt oyuna karşı dayanıklı olmalı.

## Kurulum

- Aynı oyuncular (herkes 10), aynı tarz tanımları (`tools/taktik-ayar.js`). 7 eşleşme × 20 maç × **60 sn** = 140 maç. Taraflar dengeli.
- Yeni araç `tools/taktik-iz.js`; şut sayısının yanında ölçtükleri:
  - **xG:** motorun şut anındaki gol ihtimali tahmini (`shotPk`).
  - **Rakibin son hattı:** top bizdeyken rakibin en derindeki saha oyuncusunun yeri.
  - **Arkaya pas:** son savunmacının gerisine düşen paslar ve isabeti.
  - **Geçiş şutu:** top kazanıldıktan sonraki 6 sn içinde atılan şut.
  - **Top kayıplarının yeri.**
- Ham veri `lab/tk3/sonuc/`, tam tablo `lab/tk3/ozet.md`.
- **Not:** `shotPk` gerçekleşen golün yaklaşık **2,5 katı**. 140 maçta 152,5 xG'ye karşı 61 gol, gol / xG = 0,40. xG yalnız göreli karşılaştırma için kullanıldı.

| eşleşme | xG farkı (ilki − ikinci) |
|---|---|
| Dikine – Kale önü | **+0,57 \*** |
| Dikine – Ön alan pres | +0,44 |
| Dikine – Varsayılan | +0,08 |
| Kontra – Kale önü | +0,22 |
| Kontra – Ön alan pres | **−0,35** (sınırda) |
| Ön alan pres – Topa sahip | **+0,40 \*** |
| Varsayılan – Topa sahip | −0,08 |

## 1 · Direkt oyun – yüksek hat: **yarım var**

- **Dikine'de var:** Ön alan pres'e karşı arkaya pas payı %27, varsayılana karşı %22, Kale önü'ne karşı %15. xG'si 1,03; varsayılana karşı 0,67. Yüksek hatta karşı direkt oyun daha çok üretiyor.
- **Kontra'da yok:** Ön alan pres'e karşı **kaybediyor** (−0,35). Arkaya pas isabeti presin karşısında %73'e düşüyor (Kale önü'ne karşı %88). **Geçiş şutu payı ötekilerden yüksek değil:** %21–22, Varsayılan'da %26–28. "Kazanınca hızla ileri" sahada daha çok geçiş şutu üretmiyor.
- **Yüksek hattın bedeli küçük, çünkü hat aslında yükselmiyor.** Top bizdeyken rakibin son savunmacısının yeri:

  | rakip | son hat |
  |---|---|
  | Ön alan pres (Yüksek blok) | %73 |
  | Varsayılan (Orta) | %75 |
  | Kale önü (Düşük) | %81 |

  Yüksek blok son adamı yalnız 2 puan öne çıkarıyor. Sebep `shape.js` "arkada kalan" kuralı. Savunma dizilişinin en gerisindeki oyuncular (`slot.d ≤ 0,25`) bloktan bağımsız olarak en derindeki rakibi kale tarafından tutuyor ya da ondan 2 br geride duruyor (`dR − 2`). Ofsayt olmadığı için bu hep mümkün ve bedelsiz. Blok yalnız orta ve öndeki oyuncuları taşıyor (`slotBase0`: `back = min(BLOK, top − 6)`). Arkada boş alan oluşmadığı için planın "yüksek presin bedeli arkada bırakılan alan" fikri sahada çok az var. Eşit hızdaki oyuncularda arkaya koşu da kazanılmıyor.

## 2 · Pres – topa sahip: **var** (ölçüm 1'deki sonuç yanlıştı)

- Ön alan pres, Topa sahip'i belirgin yeniyor: **+0,40 xG \***.
  - Topa sahip'in xG'si 0,18; Varsayılan'a karşı 0,52.
  - Topu kendi yarısında kaybetme payı %27; Varsayılan'a karşı %15.
  - Pres rakip yarısında maç başına 0,9 top kazanıyor.
- Ölçüm 1'deki "+0,1 şut, etkisi yok" sonucu 40 sn'lik maçların gürültüsüydü; 60 sn ve xG ile ilişki net.
- **Sınırı:** basan sayısı motorda sabit 2 (`shape.js:10` `presLim`). Taktik sayfasındaki "pres 3" etkisiz. Pres tetikleri de ters işliyor: **tetik listesi boşsa hep 2 kişi basıyor** (`!T.length` → `n`); tetik seçilince tetik yokken 1 kişi basıyor. Ölçüm 1'deki "tetik yokken daha çok pres" bundan.

## 3 · Derin blok – direkt oyun: **savunma işi yapılıyor; kayıp hücumdan ve uzak şuttan geliyor**

- **Derin blok arkadaki alanı kapatıyor:** Dikine'nin arkaya pas payı Kale önü'ne karşı %15 (en düşük).
- **Ama Dikine yine de şut buluyor:** uzaktan (ortalama 29 br, en uzak) ve neredeyse aynı tahmini kaliteyle (şut başına 0,37 xG; varsayılana karşı 0,35). Dikine'nin xG'si Kale önü'ne karşı 0,80, varsayılana karşı 0,67. Derin blok direkt oyunu varsayılandan daha iyi durdurmuyor.
- **Asıl kayıp Kale önü'nün kendi hücumunda:** topun %65'i onda ama maç başına 0,8 şut, 0,23 xG. Bunun bir kısmı benim tarz tanımımdan (yavaş, risksiz, sabırlı; karşı pres yok). "Derin blok" ile "kısır hücum"u aynı tarzda birleştirdim.
- **Motor tarafındaki aday:** uzak şutun tahmini değeri mesafeyle yeterince düşmüyor. `shotPk` gerçekleşenin 2,5 katı. Şut kararı bu iyimser değerle verildiği için takımlar uzaktan çok şut atıyor olabilir. Derin bloğun asıl silahı olan "uzaktan şuta zorlamak" bu yüzden işe yaramıyor. Doğrulamak için şut başına kayıt (mesafe, tahmin, sonuç) gerekiyor.

## Özet

| ilişki | sahada | sebep (aday) |
|---|---|---|
| Direkt – yüksek hat | zayıf | Arkadakiler bloktan bağımsız en derin rakibin gerisinde: yüksek hat yükselmiyor, arkada boşluk yok. Kontra geçiş şutu üretmiyor. |
| Pres – topa sahip | **var** | Basan sayısı sabit 2: pres yoğunluğu ayarlanamıyor. Tetikler ters. |
| Derin blok – direkt | yok | Uzak şut çok değerli sayılıyor (`shotPk` × 2,5). Kale önü tarzının hücumu kısır. |

## Önerilen sonraki adımlar

1. **Şut başına kayıt:** mesafe, `shotPk`, gol mü. Uzak şutun tahmini ile gerçeğini karşılaştırmak. İlişki 3'ün asıl sorusu bu.
2. **Arkada kalanların bloğa bağlanması:** yüksek blokta son adam hattı tutsun (en derin rakibi her koşulda kale tarafından izlemesin). Ancak böylece arkada risk doğar. Bayrakla Dikine – Ön alan pres yeniden ölçülür.
3. **Pres sayısı ve tetikler:** "pres 3" ya bağlansın ya sayfadan kalksın; boş tetik listesinin anlamı netleşsin.
4. Kale önü'nü hücum ayarları varsayılan, yalnız savunma ayarları derin olacak şekilde yeniden ölçmek (ilişki 3'ü tarz tanımından ayırmak için).
