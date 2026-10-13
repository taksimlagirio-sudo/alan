# ALAN · kontrol raporu

Yeni: `/tmp/claude-0/-home-user/3c53acc8-0565-5ab0-ac4b-2ecdd7a18db1/scratchpad/site33` · Önceki: `/tmp/claude-0/-home-user/3c53acc8-0565-5ab0-ac4b-2ecdd7a18db1/scratchpad/site32` · 2026-10-13T16:50:47Z – 2026-10-13T18:09:55Z

| | Kontrol | Sonuç |
|---|---|---|
| ✓ | Oyun açılıyor (kadro → taktik → canlı) | maç 90 sn'de hazır |
| ⚠ | Laboratuvar açılıyor ve çalışıyor | console: Error: <line> attribute x1: Expected length, "{{ c.x1 }}". · console: Error: <line> attribute y1: Expected length, "{{ c.y1 }}". |
| ✓ | Donma yok (4 maç, Liman–Demir Blok) | en uzun hareketsizlik 0 tik (0.0 sn) |
| ✓ | Hız (aynı kuyrukta, maç başına hesap) | 261 → 258 sn (-1%) · en yavaş adım 5891 ms · 0,5 sn üstü adım 189/maç |
| ✓ | Hızlandırmalar yerinde | core.js 4 · decide.js 8 · match.js 4 · shape.js 3 |
| ✓ | YZ bayrağı: skora göre taktik yalnız rakipte | motorda koşul var · canlı maç rakibe veriyor |
| ✓ | Blok etkisi: presçi (düşük → yüksek) | 0.29 → 0.84 kişi |
| ✓ | Şiddet etkisi: ikili (0 → 1) | 2.5 → 13.5 / maç |
| ⚠ | Tempo etkisi: top yerinde bekliyor (0,2 → 0,8) | %16 → %31 |
| ✓ | Genişlik etkisi: takım genişliği (dar → geniş) | 24.3 → 32.3 birim |

## Oyun nasıl değişti (16 kurulum, 1'er tohum, önceki → yeni)

| | Önceki | Yeni | Fark |
|---|---|---|---|
| Sayı / maç | 3.31 | 2.88 | -0.44 |
| Gönderme / maç | 15.4 | 14.1 | -1.3 |
| Pas / maç | 98 | 96 | -2 |
| Pas tutma % | 88.8 | 89.1 | +0.3 |
| Top kapma / maç | 22.8 | 22.9 | +0.1 |
| İkili / maç | 14.4 | 15.8 | +1.4 |
| Top el değiştirme / maç | 26.1 | 25.8 | -0.3 |
| Taşıyıcıya basan (3 birim içinde) | 0.30 | 0.31 | +0.00 |
| Son bölgede boş hücumcu | 1.03 | 1.06 | +0.03 |
| Hesap süresi (sn) | 261 | 258 | -2 |

## Taktik değişkenleri (yeni sürüm, Liman–Saat Kulesi, 2 tohum; Liman'ın değerleri)

| Değişken | Presçi | İkili | Top bekliyor | Genişlik | Pas | Sayı (Liman–rakip) |
|---|---|---|---|---|---|---|
| varsayilan | 0.52 | 4.5 | %36 | 28.0 | 46 | 2.0–0.5 |
| blokDusuk | 0.29 | 5.0 | %25 | 29.6 | 45 | 1.5–1.0 |
| blokYuksek | 0.84 | 8.5 | %27 | 27.4 | 45 | 3.0–3.0 |
| siddet0 | 0.31 | 2.5 | %21 | 28.2 | 51 | 2.0–1.5 |
| siddet1 | 0.58 | 13.5 | %31 | 29.2 | 48 | 3.0–1.5 |
| tempo02 | 0.35 | 7.0 | %16 | 27.9 | 37 | 1.0–1.0 |
| tempo08 | 0.57 | 4.5 | %31 | 28.6 | 49 | 1.0–2.5 |
| genDar | 0.60 | 9.5 | %33 | 24.3 | 48 | 1.0–2.0 |
| genGenis | 0.54 | 6.5 | %39 | 32.3 | 52 | 1.0–2.5 |

## Donma ayrıntısı

- tohum 11 · skor 1,1 · süre 307 sn · en uzun hareketsiz 0 tik (tik -1) · en yavaş adım 4043 ms (tik 3053) · 0,5 sn üstü adım 191
- tohum 12 · skor 2,1 · süre 299 sn · en uzun hareketsiz 0 tik (tik -1) · en yavaş adım 5464 ms (tik 4890) · 0,5 sn üstü adım 192
- tohum 13 · skor 3,3 · süre 289 sn · en uzun hareketsiz 0 tik (tik -1) · en yavaş adım 4505 ms (tik 792) · 0,5 sn üstü adım 178
- tohum 14 · skor 1,3 · süre 326 sn · en uzun hareketsiz 0 tik (tik -1) · en yavaş adım 5891 ms (tik 559) · 0,5 sn üstü adım 194

## Tarayıcı

```
--- oyun
hazır olana kadar 90 sn · ALAN CANLI Liman L 0 – 0 00:00 F Fırtına Maçı kaydet Ses açık Rakip seç Yeni maç Taktik ŞU AN İlerleme Fırtına · orta sa
--- lab
0 / 168 maç
hatalar: console: Failed to load resource: the server responded with a status of 404 (File not found)
console: Error: <line> attribute x1: Expected length, "{{ c.x1 }}".
console: Error: <line> attribute y1: Expected length, "{{ c.y1 }}".
console: Error: <line> attribute x2: Expected length, "{{ c.x2 }}".
console: Error: <line> attribute y2: Expected length, "{{ c.y2 }}".
console: Error: <line> attribute x1: Expected length, "{{ c.x1 }}".
```

Not: Taktik satırları 2'şer maçtan; büyük etkiler güvenilir, küçük farklar gürültü olabilir. Eşikler: donma < 2 sn; hız ≤ %15 yavaşlama; taktik etkisi blok ve şiddet ×1,3, tempo ×0,85, genişlik +3 birim. ⚠ işareti tabloda %25'ten büyük değişimi gösterir.
