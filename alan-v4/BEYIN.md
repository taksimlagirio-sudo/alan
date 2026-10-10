# ALAN v4 · Tek beyin

## Fikir
Oyun tek bir beynin dünyasında oynanır. Oyuncular kafasızdır, her komutu beyin verir.
Dünyanın yasaları beynin içindedir ve beyin onları bilir: hareket, Çekirdek, alanlar, şarj, temas, ikili mücadelenin zarı, sayı.
Beyin karar verirken kendi dünyasını ileri oynatır. Yasaları değiştiremez, sonucu seçemez; serbest olduğu tek yer komutlardır.
Maç "Maça başla" deyince önceden oynanır (yükleme ekranı), sonra izlenir. Taktik değişince maç o andan yeniden oynanır.

## Dosyalar
- `core.js`, `match.js` (yasalar): v3g ile aynı, tek fark kararların `AlanBeyin` üzerinden istenmesi.
- `shape.js`: topsuz yerleşim, pres, kovalama kuralları (beynin refleks katmanı). Tek ek: `m.kovM` (beynin kovalama ayarı).
- `beyin.js`: beyin.
  - Refleks katmanı: v3g'nin oyuncu kuralları. İleri bakış kapalıyken maç v3g ile tik tik aynıdır (iz karşılaştırmasıyla doğrulandı).
  - İleri bakış katmanı (`AlanBeyin.A`): karar anlarında aday komutları kendi dünyasında ileri oynatarak tartar.
    - Top sahibi: en umutlu 3 aday (refleks kararın sıraladığı), her biri bir deneme.
    - Savunma: 45 tikte bir pres yoğunluğu (taktiğin ±1'i); pas atıldığında kaç kişinin kovalayacağı.
    - İleri oynatmada maçın gerçek zarı kullanılmaz (her deneme kendi zarıyla); içeride herkes refleks katmanıyla oynar.
    - Ufuk: en az 20 tik, sonra top birinin eline geçene kadar (en çok 60). Sonda: sayı ±1, top bizde +V, rakipte −V (durum değeri tablosu), boşta 0.
    - Okuma: dünya, komutun verildiği oyuncunun gözüyle görülür (rakip konumu (20 − Okuma) × 0,05 birim hatalı); seçim, sonuçlar arasında Okuma'ya bağlı sıcaklıkla (softmax).
  - `planla(m)`: bu tik tartılacak karar varsa denemeleri döndürür; `rollout(m, iş)`: bir deneme; `oyna` / `oynaAsync`: maçı beyinle oynatır.
  - `paketle` / `ac`: dünyanın durumunu işçilere taşır.
- `beyin-isci.js`: denemeleri paralel çalıştıran işçi (tarayıcıda Web Worker; Node'da `tools/v4-paralel.js`). Paralel sonuç tek iş parçacığındakiyle tik tik aynıdır.
- `../ALAN v4 Maç.html`: maç sayfası (yükleme ekranı, izleme, taktik değişince o andan yeniden hesap).

## Ölçümler (90 sn'lik maç, Node, 4 çekirdek)
| | Süre (tek çekirdek) | Süre (4 çekirdek) | Pas tutma | Sayı/maç | Kendi yarıda |
|---|---|---|---|---|---|
| Refleks (v3g) | 16 sn | – | %47 | 4,3 | %57 |
| Beyin · kaliteli | 80 sn | ~45 sn | %60 | 5,5 | %44 |
| Beyin · hızlı (`icHafif`) | 44 sn | ~26 sn | %50 | 4,5 | %52 |
Karışık taktikler, rastgele özellikler, 6'şar maç. Okuma 16'lık takım Okuma 6'lık takıma (diğer her şey eşit) 6 maçta 24–9.

## Araçlar
- `tools/v4-olc.js`: aynı maçı refleks / beyin ile oynatıp ölçer.
- `tools/v4-hiz.js`: tek iş parçacığı ve işçilerle aynı maç; süre ve eşdeğerlik.
- `tools/trace.js` + `tools/compare.js`: v3g ile tik tik karşılaştırma.
