# ALAN · kontrol raporu

Yeni: `/tmp/claude-0/-home-user/3c53acc8-0565-5ab0-ac4b-2ecdd7a18db1/scratchpad/site32` · Önceki: `/tmp/claude-0/-home-user/3c53acc8-0565-5ab0-ac4b-2ecdd7a18db1/scratchpad/site31` · 2026-10-13T14:08:33Z – 2026-10-13T15:18:45Z

| | Kontrol | Sonuç |
|---|---|---|
| ✓ | Oyun açılıyor (kadro → taktik → canlı) | maç 69 sn'de hazır |
| ✓ | Laboratuvar açılıyor ve çalışıyor | hata yok |
| ✓ | Donma yok (4 maç, Liman–Demir Blok) | en uzun hareketsizlik 1 tik (0.0 sn) |
| ✓ | Hız (aynı kuyrukta, maç başına hesap) | 303 → 314 sn (+4%) · en yavaş adım 5491 ms · 0,5 sn üstü adım 183/maç |
| ✓ | Hızlandırmalar yerinde | core.js 4 · decide.js 8 · match.js 4 · shape.js 3 |
| ✓ | YZ bayrağı: skora göre taktik yalnız rakipte | motorda koşul var · canlı maç rakibe veriyor |
| ✓ | Blok etkisi: presçi (düşük → yüksek) | 0.15 → 0.88 kişi |
| ✓ | Şiddet etkisi: ikili (0 → 1) | 5.0 → 16.5 / maç |
| ✓ | Tempo etkisi: top yerinde bekliyor (0,2 → 0,8) | %48 → %33 |
| ✓ | Genişlik etkisi: takım genişliği (dar → geniş) | 21.4 → 30.7 birim |

## Oyun nasıl değişti (16 kurulum, 1'er tohum, önceki → yeni)

| | Önceki | Yeni | Fark |
|---|---|---|---|
| Sayı / maç | 3.88 | 3.31 | -0.56 |
| Gönderme / maç | 14.4 | 15.4 | +0.9 |
| Pas / maç | 95 | 98 | +3 |
| Pas tutma % | 89.3 | 88.8 | -0.5 |
| Top kapma / maç | 22.6 | 22.8 | +0.1 |
| İkili / maç | 16.2 | 14.4 | -1.8 |
| Top el değiştirme / maç | 26.5 | 26.1 | -0.4 |
| Taşıyıcıya basan (3 birim içinde) | 0.32 | 0.30 | -0.01 |
| Son bölgede boş hücumcu | 1.04 | 1.03 | -0.01 |
| Hesap süresi (sn) | 303 | 314 | +11 |

## Taktik değişkenleri (yeni sürüm, Liman–Saat Kulesi, 2 tohum; Liman'ın değerleri)

| Değişken | Presçi | İkili | Top bekliyor | Genişlik | Pas | Sayı (Liman–rakip) |
|---|---|---|---|---|---|---|
| varsayilan | 0.40 | 7.0 | %23 | 25.5 | 38 | 2.5–0.5 |
| blokDusuk | 0.15 | 4.5 | %30 | 26.1 | 41 | 2.0–0.5 |
| blokYuksek | 0.88 | 5.5 | %31 | 26.1 | 48 | 2.0–1.5 |
| siddet0 | 0.28 | 5.0 | %38 | 26.5 | 58 | 2.0–1.0 |
| siddet1 | 0.48 | 16.5 | %18 | 23.7 | 44 | 1.5–2.5 |
| tempo02 | 0.39 | 5.0 | %48 | 26.6 | 35 | 2.0–3.0 |
| tempo08 | 0.42 | 4.5 | %33 | 26.7 | 42 | 1.5–1.5 |
| genDar | 0.66 | 8.0 | %41 | 21.4 | 44 | 1.5–2.0 |
| genGenis | 0.47 | 7.0 | %37 | 30.7 | 40 | 2.0–1.0 |

## Donma ayrıntısı

- tohum 11 · skor 1,2 · süre 374 sn · en uzun hareketsiz 0 tik (tik -1) · en yavaş adım 5081 ms (tik 674) · 0,5 sn üstü adım 205
- tohum 12 · skor 2,1 · süre 320 sn · en uzun hareketsiz 0 tik (tik -1) · en yavaş adım 5089 ms (tik 4285) · 0,5 sn üstü adım 196
- tohum 13 · skor 1,0 · süre 308 sn · en uzun hareketsiz 0 tik (tik -1) · en yavaş adım 5491 ms (tik 148) · 0,5 sn üstü adım 164
- tohum 14 · skor 0,1 · süre 288 sn · en uzun hareketsiz 1 tik (tik 3667) · en yavaş adım 5047 ms (tik 4004) · 0,5 sn üstü adım 165

## Tarayıcı

```
--- oyun
hazır olana kadar 69 sn · ALAN CANLI Liman L 0 – 0 00:00 F Fırtına Maçı kaydet Ses açık Rakip seç Yeni maç Taktik Ece şarj %37 ❚❚ 0,25× 0,5× 1× 1,
--- lab
0 / 168 maç
hatalar: console: Failed to load resource: the server responded with a status of 404 (File not found)
```

Not: Taktik satırları 2'şer maçtan; büyük etkiler güvenilir, küçük farklar gürültü olabilir. Eşikler: donma < 2 sn; hız ≤ %15 yavaşlama; taktik etkisi blok ve şiddet ×1,3, tempo ×0,85, genişlik +3 birim. ⚠ işareti tabloda %25'ten büyük değişimi gösterir.
