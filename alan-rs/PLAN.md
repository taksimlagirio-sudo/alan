# ALAN motoru: Rust'a taşıma

Hedef: telefon ve bilgisayar uygulamasında yerel çalışan maç motoru. JS motoru (`alan-v5/`) taşıma bitene kadar sitede çalışmaya devam eder; Rust motoru `alan-rs/motor` kütüphanesidir, uygulamalar onu bağlar.

## Kararlar (3 Kasım 2026)

- **Yalnız bugünkü davranış.** `_ESKI` deney dalları taşınmaz. Kaynak, `sina/sadele.js`'in ürettiği sade JS'tir: her `self.ALAN_*_ESKI` okuması varsayılan değeriyle değiştirilir, ölü dallar terser'le silinir. Sade JS orijinaliyle bit bit aynı maçı oynar (909623, 713732: 300 tik, iz aynı).
  - Üretmek: `TERSER=<terser yolu> node sina/sadele.js ../alan-v5 kaynak sina/bayrak-varsayilan.json core yetenek match shape decide sabit-mat value-table`
  - `kaynak/` git'e girmez; JS motoru değişince yeniden üretilir.
- **Maç tıpatıp aynı olmak zorunda değil, ama oyun aynı kalitede olmalı** ("salaklaşmasın"). Doğrulama iki katman:
  1. Fizik ve matematik: işlev işlev JS ile bit bit aynı (sınama vektörleri `sina/`).
  2. Kafa ve yerleşim: çok maçlık istatistik (pas isabeti, rakibe pas, şut, gol, sahiplik, kayıp, bar) JS ile aynı aralıkta.
- **Belirlenimli matematik.** `std`'nin `exp`/`sin`… işlevleri yok; `mat.rs` (sabit-mat.js'in karşılığı). ARM ve x86'da aynı maç. JS'in `Math.max/min/round` anlamı `jmax/jmin/jround`.
- **Yapı** (`alan-v5/TASARIM-yapi.md`): dünya tek veri yapısında (oyuncu satırları, nesne kopyası yok), gizli durum yok (bağlam açıkça), önbellekler sıradan bağımsız, kafa ile maç aynı işlevi çağırır.
- **Paralel kafa**: kararın bağımsız tartıları (dalga) iş parçacıklarına dağılır; sonuç sıradan bağımsız.

## Sıra

| # | Parça | JS kaynağı | Durum |
|---|---|---|---|
| 1 | Belirlenimli matematik | sabit-mat.js | ✅ 20.013 vektörde 14 işlev bit bit aynı |
| 2 | Sabitler, saha, Q/P parametreleri | core.js | |
| 3 | Yetenekler (ALAN_Y, OZ) | yetenek.js | |
| 4 | Top fiziği (stepBall, predict, solveLaunch, rollTo) | core.js | |
| 5 | Temas, dönüş (Turn), Bekçi, Kuyu, stepAll | core.js | |
| 6 | Enerji (EN2/EN3, bar, dolum, sprint) | core.js, match.js | |
| 7 | Maç döngüsü (step, ikili, olaylar) — kafasız (sabit kararlarla) | match.js | |
| 8 | Yerleşim (shape) | shape.js | |
| 9 | Kafa (decide) | decide.js | |
| 10 | Kayıt yükleme, kare çıktısı, CLI, istatistik karşılaştırma | mac-isci.js | |
| 11 | Paralel dalga, hız ölçümü | paralel.js | |
| 12 | Uygulama bağı (C ABI / uniffi) | — | |

## Araçlar

- `cargo test --release` : sınamalar.
- `sina/mat-vektor.js` : matematik vektörleri (JS'ten).
- `kos` : komut satırı (kayıt → maç → istatistik).
