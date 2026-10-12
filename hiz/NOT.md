# Birebir aynı hızlandırma (zip 24 üzerine)

Değişen: `match.js` `govdeAyir`, `shape.js` `alici`. Kural, sayı ve sıra değişmedi.

- `govdeAyir`: x ya da y farkı gövde mesafesini aşan çiftlerde kök hesabı atlanır; bir turda hiçbir çift iç içe değilse kalan turlar yapılmaz (hiçbir şeyi değiştirmezlerdi).
- `alici`: rakiplerin tepki süresi ve koşu hızı döngü dışında bir kez hesaplanır; `iMax`'tan sonraki anlara bakılmaz (zaten seçilemezlerdi).

Doğrulama: `tools/v5-iz.js`, 4 maç (tohum 1–4), her tik bütün oyuncuların x, y, vx, vy, Çekirdek ve skor Float64 bitleriyle: base ile birebir aynı.
Süre (4 paralel): 145→111, 100→78, 179→134, 180→135 sn (yaklaşık %24 kısa).
