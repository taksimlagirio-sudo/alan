# ALAN · Özellik ölçümü 1 (main bbd5ead, 18 Ekim 2026)

Soru: hangi özellik maçı ne kadar etkiliyor, yükseldikçe hep iyileşiyor mu, sahada izi var mı.

## Kurulum

- İki takım birebir aynı: Alan sistemi, pres 1, Orta blok, diziliş 2-2-2 / 2-1-3, bütün özellikler 10.
- A takımının saha oyuncularında **tek özellik 4 ya da 16** (Bekçi özellikleri yalnız Bekçi'de). A, maçların yarısında takım 0, yarısında takım 1.
- 18 özellik × 2 değer × 40 maç + 40 taban maçı = **1480 maç**, her biri 120 sn (7200 tik).
- Motor maç işçisiyle aynı sırada yüklenir (`sabit-mat.js` önce); node ve bun aynı maçı verir.
- Araçlar: `tools/oz-olc.js` (maç), `tools/oz-ozet2.js` (özet), `tools/oz-kopya2.js` + `oz-kopya2-ozet.js` (kopya testi). Ham veri: `lab/oz1/sonuc/`, tam tablo: `lab/oz1/ozet.md`.
- **Gürültü:** iki aynı takımın 40 maçında gol farkı −0,40 ±0,22 (beklenen 0). 40 maçla ~0,45 golden küçük farklar tek başına güvenilmez; puan başına etki (16 ile 4'ün farkı / 12) iki kolu birleştirdiği için daha sağlam.

## Sonuç: puan başına etki

`*` = farkın standart hatanın iki katından büyük olması.

| Özellik | Gol farkı / puan | Gol farkı: 4 / 16 | Şut / puan | Sahadaki iz |
|---|---|---|---|---|
| **Hız** | **+0,258 \*** | −1,68 \* / +1,43 \* | +0,88 \* | Koşu hızı ±%11; 4'te 40 maçın 35'i yenilgi, 16'da 28'i galibiyet |
| **Çeviklik** | **+0,100 \*** | −0,93 \* / +0,28 | +0,29 \* | Düşükken çok zararlı, yüksekken az yararlı |
| **Alan gücü** | **+0,079 \*** | −0,40 / +0,55 \* | +0,53 \* | Rakibin pas isabeti %79 (16), top sahipliği ±3 puan |
| **Sürme** | **+0,079 \*** | −0,68 \* / +0,28 | +0,02 | Top sahipliği −4 / +7 puan \* |
| **Refleks (B)** | **+0,077 \*** | −0,40 \* / +0,53 \* | | Tek oyuncuyla |
| **Tepki** | **+0,075 \*** | −0,42 / +0,47 \* | +0,25 \* | Kapma ve şut |
| Kesme | +0,052 | −0,23 / +0,40 | | Kapma (16) \* |
| Gönderme | +0,042 | −0,28 / +0,23 | | Şutta iz yok |
| Kontrol | +0,035 | −0,23 / +0,20 | | |
| Güç | +0,031 | −0,25 / +0,13 | | |
| Soğukkanlılık | +0,029 | 0,00 / +0,35 | | |
| Alan hakimiyeti (B) | +0,021 | −0,23 / +0,03 | | |
| Uzanma | +0,010 | +0,03 / +0,15 | | |
| Yerleşim (B) | +0,008 | −0,15 / −0,05 | | |
| Yükleme | 0,000 | −0,07 / −0,07 | | |
| Yer alma | 0,000 | −0,10 / −0,10 | | |
| **Pas** | **−0,040** | +0,23 / −0,25 | | Pas isabetinde iz yok |
| **Görüş** | **−0,040** | **+0,47 \*** / 0,00 | −0,15 \* | Düşük Görüş top sahipliğini artırıyor (+3,6 \*) |

## Bulgular

1. **Hız baskın.** Bir puan Hız, sıradaki özelliklerin ~2,5 puanı, zayıfların 6–25 puanı kadar. Planın "kimse tek özellikle domine etmesin" şartı bugünkü motorda tutmuyor; ileride maaş ve değer özellik ölçümünden türeyecekse herkes Hız'a para verir.
2. **18 özelliğin 6'sı açıkça etkili** (Hız, Çeviklik, Alan gücü, Sürme, Refleks, Tepki). Kesme sınırda. **Yükleme, Yer alma, Uzanma, Yerleşim'in ölçülebilir etkisi yok** (puan başına |etki| < 0,01). Planın "sahada görünür olmalı" ilkesine göre bunlar ya büyütülmeli ya da birleştirilmeli.
3. **Görüş ters çalışıyor:** Görüş 4 olan takım +0,47 gol farkıyla kazanıyor (21-9-10). Kodda iki aday:
   - `shape.js:128` boş topa koşma payı `(20 − Görüş) × 0,6 + 1` tik: düşük Görüş'lü oyuncu "ilk ben varırım" diye yanılıp **ikinci kişi olarak da topa koşuyor**. Bu yanılgının bedeli yok, aksine boş topları kazandırıyor (düşük Görüş'te top sahipliği +3,6 \*). Tasarımın "okuma hatası" saydığı davranış motor için iyi bir davranış.
   - Kopya testinde Görüş 16 olan takım daha çok pas deniyor (259'a 229) ve isabeti düşüyor (%66'ya %72). `decide.js:477` (`reuseLook`) Görüş'le eski "kafadaki oynatma" sonucuna daha çok güveniyor; bayat bir sonuç cesur pası seçtiriyor olabilir. Doğrulanmadı.
4. **Pas etkisiz:** Kopya testinde (aynı an, aynı zar) Pas 4 / 10 / 16'da pas isabeti %70 / %72 / %72, top kaybı aynı. Ters çalışmıyor, **etkisi neredeyse yok**: Pas yalnız açı ve hız hatasını belirliyor (`decide.js:95`), katsayı küçük (Pas 10'da ±2°). Maç ölçümündeki eksi işaret gürültü sınırında.
5. **Asimetri:** Çeviklik, Hız, Sürme düşükken yüksekken olduğundan daha çok etkili. Belli bir değerin altında iş bozuluyor; tavana doğru getiri azalıyor.

## Kopya testi (Pas, Görüş, Hız; 43 an × 4 zar, 4 sn ufuk)

| özellik | değer | pas isabeti A | şut A / rakip (100 kolda) | top kaybı (100 kolda) |
|---|---|---|---|---|
| Hız | 4 / 10 / 16 | 73 / 72 / 72 % | 10,5 / 13,4 / 18,0 — rakip 16,3 / 21,5 / 15,7 | 33 / 33 / 21 |
| Pas | 4 / 10 / 16 | 70 / 72 / 72 % | 12,2 / 13,4 / 11,6 | 32 / 33 / 31 |
| Görüş | 4 / 10 / 16 | 72 / 73 / 66 % | 14,5 / 13,4 / 13,4 | 32 / 33 / 33 |

Hız kontrol olarak beklenen etkiyi gösteriyor (araç doğru ölçüyor). Örnek küçük; yön için.

## Sınırlar

- Tek taktik, tek diziliş, herkes 10 tabanı. Bir özelliğin değeri taktiğe göre değişebilir (planın çeşitlilik şartı); ölçülmedi.
- Özellik takımın bütün saha oyuncularında birden değişti; tek oyuncunun etkisi daha küçük ve konuma göre değişir.
- Etkileşimler (Hız + Sürme gibi) ölçülmedi.

## Önerilen sonraki adımlar

1. **Görüş:** boş topa ikinci koşucunun bedelini ya da doğru okuyanın ödülünü bul (koşmamak neden kaybettiriyor?). Sonra Görüş'ü yeniden ölç.
2. **Hız:** baskınlığın nereden geldiğine bak (koşu hızı mı, topla hız mı, `shape.js:177` tehlike çarpanı mı); her birinin payını bayrakla ayrı ayrı kapatarak ölç.
3. **Etkisizler** (Yükleme, Yer alma, Uzanma, Yerleşim, Pas): her biri için katsayıyı büyütüp etkisinin ölçülebilir olduğu en küçük değeri bul, ya da özelliği başka biriyle birleştir.
4. Değişikliklerden sonra aynı ölçüm (araçlar hazır; 1480 maç ~6 saat, 4 çekirdek).
