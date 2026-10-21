# ALAN · Genişlik ölçümü 1: hücum genişliği Geniş / Normal / Dar (21 Ekim 2026)

> Motor: main 33c66f8. Araçlar: `tools/blok-olc.js` (yayılım ve kanattan şut ölçüleri eklendi), `tools/gen-ozet.js`. Veriler: `lab/gn1/`.
> Soru (kullanıcının): hücum genişliğinin geniş ya da dar olması ne kadar etkiliyor? Aynı takımlar, tek fark genişlik.

## Düzen

- İki takım aynı: herkes 10, taktik varsayılan (`tools/taktik-ayar.js` V), diziliş 2-2-2 / 2-1-3. Tek fark `genislik`: Dar (28), Normal (38), Geniş (46). Motorda bu yalnız hücum şeklinin enini belirler (`shape.js` `slotBase0`, `GEN`); savunma şekli aynı.
- 3 eşleşme × 20 maç = 60 maç, her maç 9000 tik (5 dk). Taraflar tohumla dengeli.

## Sonuç

| A – B | maç | G-B-M (A) | gol A / B | gol farkı | şut A / B | xG A / B | xG farkı | top A | pas A / B | kapma A / B | yayılım A / B (top bizdeyken) | rakip savunmanın yayılımı (A hücumda / B hücumda) | kanattan şut A / B |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Geniş – Dar | 20 | 6-9-5 | 0.60 / 0.60 | +0.00 ±0.23 | 4.2 / 4.5 | 1.63 / 1.88 | −0.25 ±0.28 | 48% | 71 / 76 | 9.8 / 9.7 | 30.6 / 25.6 | 22.3 / 20.6 | 59% / 63% |
| Geniş – Normal | 20 | 5-8-7 | 0.45 / 0.50 | −0.05 ±0.20 | 4.8 / 4.0 | 1.91 / 1.70 | +0.21 ±0.32 | 52% | 75 / 69 | 9.4 / 9.3 | 32.4 / 29.5 | 24.0 / 22.6 | 71% / 77% |
| Dar – Normal | 20 | 8-9-3 | 0.90 / 0.70 | +0.20 ±0.22 | 4.7 / 4.2 | 2.03 / 1.60 | +0.43 ±0.29 | 51% | 74 / 70 | 10.3 / 9.9 | 26.1 / 29.1 | 20.8 / 21.5 | 65% / 64% |

`*`: fark standart hatanın iki katından büyük. Yayılım: saha oyuncularının y yönündeki en uç iki oyuncusu arası (saha eni 60). Kanattan şut: Kuyu hizasından 12 br'den fazla yanda atılan şut.

## Okuma

1. **Genişlik sahada değişiyor:** top bizdeyken yayılım Dar 26, Normal 29–30, Geniş 31–32 br (saha eni 60). Ayar işliyor.
2. **Sonuca etkisi küçük, hiçbir fark anlamlı değil.** Gol farkları −0,05 ile +0,20 arası, xG farkları −0,25 ile +0,43 arası; hepsi standart hatanın iki katının altında. 20 maçta görülebilecek en küçük etki ~0,5 gol; genişliğin etkisi bundan küçük.
3. **Zayıf eğilim futbolun tersine: dar biraz daha iyi.** Dar, Normal'e karşı +0,20 gol, +0,43 xG (yaklaşık 1,5 standart hata); Geniş, Dar'a karşı xG'de −0,25. Futbolda genişlik savunmayı açar; burada açmıyor.
4. **Savunma genişliğe az uyuyor:** Geniş hücuma karşı rakip savunmanın yayılımı 22,3, Dar hücuma karşı 20,6 br. Hücum 5 br genişleyince savunma ancak 1,7 br genişliyor. Savunma kompakt kalıyor; geniş duran oyuncular boş ama Kuyu'dan uzak. Genişlik boşluk açmıyor, yalnız oyuncuları topa uzaklaştırıyor.
5. **Kanattan şut payı değişmiyor:** bütün eşleşmelerde şutların %59–77'si Kuyu hizasından 12 br'den fazla yanda. Genişlik şutun yerini değiştirmiyor.

## Neden (yorum, doğrulanmadı)

- ~~Kanattaki boş oyuncuya pas gidince blok kayıyor, kayma bedelsiz.~~ **Yanlış çıktı, bkz. aşağıdaki kayma ölçümü:** savunma yavaş kayıyor, taraf değiştiren pas alıcıyı boş bırakıyor. Genişliğin getirisizliği kaymadan değil, boş kalan kanat oyuncusunun bundan sonra ne yaptığından gelmeli (ölçülmedi).
- 7v7 sahada (60 br en) dar oynamak pas mesafelerini kısaltıyor; pas isabeti ve şarj kaybı (tek enerji) kısa pası ödüllendiriyor olabilir.

## Tasarım için

- Genişliğin anlamlı olması için savunmanın kayması bir bedel taşımalı: kayma gecikmesi (Tepki, Yer alma) ya da geniş duran oyuncuya karşı ayrı bir adam. Bkz. `plan/TASARIM-tarz-dongusu.md` fikir 1.
- Kanattan Kuyu önüne pasın (orta) değeri ayrıca ölçülmeli: genişliğin futboldaki asıl getirisi bu.

## Ek · Kayma ölçümü (`lab/ky1/`, `tools/kayma-olc.js`, `tools/kayma-ozet.js`)

Soru (kullanıcının): savunma genişliğe kaymaları çok senkronize olduğu için mi uymuyor? 16 maç (Geniş ve Dar, Normal'e karşı), 1931 alınan pas.

| topun yanal yer değişimi | pas | uçuş (sn) | savunmanın kayması: uçuşun ortasında | topu alırken | aynı yöne kayan savunmacı | kayma farkı (sd, br) | alıcıya en yakın savunmacı (br) |
|---|---|---|---|---|---|---|---|
| < 5 br | 598 | 1,07 | %4 | %11 | %40 | 2,5 | 7,7 |
| 5–10 br | 471 | 1,07 | %6 | %12 | %46 | 2,5 | 9,4 |
| 10–20 br | 508 | 1,27 | %4 | %10 | %53 | 3,0 | 10,0 |
| ≥ 20 br (taraf değiştirme) | 354 | 1,74 | %2 | %17 | %79 | 3,6 | 11,4 |

- **Hayır, kayma hızlı ya da kusursuz değil, tersine yavaş.** Taraf değiştiren pasta top 20+ br yana gidiyor, savunma alıcı topu alana kadar bunun yalnız %17'si kadar kayıyor (şeklin kuralı %40 ister). Uçuşun ortasında neredeyse hiç kaymamış (%2).
- Kodda şeklin dayanağı top havadayken varış noktası (`shape.js:110`, `E`); yani kural pas atılır atılmaz kaymayı ister. Ama savunmacıların çoğu o anda şekil yerinde değil, bir işte (adam, bölge, kapatma); şekil kayması onlara pek yansımıyor.
- **Senkron:** taraf değiştirmede savunmacıların %79'u aynı yöne kayıyor, aralarındaki fark küçük (sd 3,6 br). Birlikte ama az ve geç kayıyorlar.
- **Taraf değiştirme boşluk açıyor:** alıcıya en yakın savunmacı 11,4 br (kısa paslarda 7,7).
- Sonuç: genişlik boşluk açıyor; getirisinin olmaması, kanatta boş alınan topun sonrasından geliyor olmalı (Kuyu'dan uzak; orta ya da içeri sürme değeri). Sıradaki ölçüm: taraf değiştiren pastan sonraki 6 sn'de şut ve xG.
