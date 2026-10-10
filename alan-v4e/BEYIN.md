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


## v4b eklemeleri (Design, 2026-10-10)
Ayarlar `AlanBeyin.A` içinde, hepsi kapatılabilir.
1. **Koordinasyon (`kosu`, `KN`, `KT`):** Top sahibinin kararında beyin, hamleyle birlikte en umutlu 2 topsuz koşuyu da aynı oynatmada tartar (en iyi 2 hamle × 2 koşu + 3 tek hamle = 7 deneme). Koşu adayları ucuz bir ön elemeyle (koşu noktasının değeri × pas hattı), sonucu oynatma belirler. Seçilen koşu `m.kosu[takım]` olarak en çok 90 tik sürer; koşucu Çekirdeği alınca, varınca ya da top kaybedilince biter. Yerleşimden sonra uygulanır (`konum`).
2. **Tablosuz uç değer (`uc: 'kaba'`):** Ufuk sonunda tablo yerine: 0,6 × e^(−Kuyu'ya uzaklık / 30) × (1 − 0,12 × önünde kalan rakip saha oyuncusu). Varsayılan hâlâ `'tablo'`; amaç tablonun ne kadar iş gördüğünü ölçmek.
3. **Hız (`icKonum: 6`, `icKarar: 15`, yeniden hesap yok):** İleri oynatmada topsuz yerleşim 6 tikte bir hesaplanır, arada son talimat sürer; top sahibi en erken 15 tikte bir yeniden düşünür. Gerçek kararda, yakalamada hesaplanmış seçenek yeniden hesaplanmadan kullanılır. Profil: zamanın %78'i ileri oynatmanın içindeki top sahibi kararlarındaydı.

### Ölçüm (tarayıcı, tek iş parçacığı; az örnek, yön)
| | tik/sn | 90 sn maç (tek çekirdek) | Pas tutma (45 sn × 2 maç) |
|---|---|---|---|
| v4 | 136 | ~40 sn | %62 |
| v4b hız | 185 | ~29 sn | %61 |
| v4b hız + koşu | 125 | ~43 sn (7 deneme, işçilerde paralel) | %66 |
- Aynı tohum aynı maçı veriyor (hash eşit).
- Koşu: 45 sn'de ~50 koşu; koşucuya atılan pasların tutması düşük (12 pasta %25). Koşu seçiliyor ama pas genelde koşucuya gitmiyor ya da tutmuyor; bakılacak.
- **Özellik baskınlığı (beyin, 16'ya karşı 6, 4'er maç × 45 sn):** Okuma 11–3, Hız 12–0, Aktarım 9–2. Okuma tek başına baskın değil; üçü de büyük fark yaratıyor. 30 maçlık hücreler Node araçlarıyla yapılmalı.


## Claude Code için test listesi
1. Özellik hücreleri, beyin açık (kosu: true), 30'ar maç: Okuma, Hız, Aktarım, Tutuş, Kesme, Sürme 16'ya karşı 6. Okuma'nın göreli ağırlığı; her biri ne kadar fark yaratmalı (tasarım kararı kullanıcının).
2. Uç değer: uc 'tablo' / 'kaba', aynı tohumlar; pas tutma, kendi yarıda, sayı/maç.
3. Koordinasyon: kosu açık / kapalı; koşucuya atılan pasların tutması (şu an 12 pasta %25, az örnek). Beklenen sebep: pas adayları koşucunun pas anındaki yerine göre üretiliyor, koşunun varacağı noktaya göre değil.
4. Hız: tools/v4-hiz.js ile v4 ve v4b, tek çekirdek ve işçilerle.
5. Belirlenimcilik: aynı tohum aynı hash; paralel ile tek iş parçacığı aynı.
Sayfa: ALAN v4b Maç.html (alan-v4b/ klasörüne bağlı; web sunucusundan açılmalı).