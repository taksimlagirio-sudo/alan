# ALAN · zip59 kontrol tablosu: Öneri A (Bekçi'nin bölgesi)

16 maç, 120 sn, 120×60, herkes 20 (Alan gücü ve Soğukkanlılık 10), ayarsız takımlar; taban zip58 (aynı 16 senaryo). %15'ten çok kötüleşen ⚠, iyileşen ✓.

## Beklenen değişimler
| Satır | Beklenen | Çıkan | |
|---|---|---|---|
| Bekçi pas kovalaması / dk | ≤ 1,0 | 1,56 → **0,38** | ✓ |
| Kovaladığını kazanma | ≥ %75 | %64 → %67 | tutmadı (kovalama 4'te 1'e indiği için örnek az: ~12) |
| Sayılarda Bekçi yerinde | ≥ %75 | %60 → **%75** | ✓ |
| Yakın direkten sayı / dk | ≤ 0,5 | 0,78 → 0,66 | kısmen (8 maçta 0,44 idi) |
| Dar açıda Bekçi–yakın direk | ≤ 2,5 | 3,79 → 3,61 | tutmadı |
| Gönderme başına sayı | ↓ (~%12) | %15,8 → **%17,1** | ters |
| Sayı / dk | ↓ | 1,91 → 1,94 | değişmedi |
| Yakın geniş ≤ yakın düz | | %50 (8) ≤ %52 (23) | ✓ |
| Hesap ms/tik | değişmesin | 33,6 → 32,4 | ✓ |

**Özet:** Bekçi artık kalesini bırakmıyor (kovalama dörtte bire indi, sayılarda yerinde %75). Ama sayılar azalmadı: gönderme başına sayı arttı (%15,8 → %17,1). Yerleşmiş Bekçi'nin hızlı tepkisi gönderme tablosunda %12 zorlaşma gösteriyordu; maçta görünmüyor. Bekçi–yakın direk uzaklığı da değişmedi (3,6): açıortay hedefi dar açıda yakın direğe 2,5'ten yakın değil.

**Soru Design'a:** Yerleşmiş Bekçi (hız < 0,05, hedefine < 1) maçta ne sıklıkla gönderme anında "yerleşmiş"? Kısa sınamada zamanın %44'ü; ama gönderme anları, Bekçi'nin top hareketiyle yeniden yerleştiği anlar olabilir (top yeni geldi, Bekçi hâlâ kayıyor). Kayıtta Bekçi'nin gönderme anındaki hızı ölçülebilir; Claude Code bakabilir.

## Beklenmeyen değişimler (Bekçi kuralı bunlara dokunmamalı)
| Satır | zip58 | zip59 |
|---|---|---|
| Boş adam + çift adam anı / dk | 10,2 | 12,1 ⚠ |
| bunlarda iki kişi "arkada" / dk | 1,50 | 2,50 ⚠ |
| Taşıyıcıya baskı yok (rakip yarısı) % | 13,6 | 17,8 ⚠ |
| Geçiş "git" tik / dk | 47,5 | 39,0 ⚠ |
| Her an duran hücumcu | 0,21 | 0,25 ⚠ |
| Uzak geniş (25+, 60–75°) | 0/24 | 2/20 ⚠ |

Olası bağ: Bekçi artık pası kesmeye çıkmıyor; o topa artık bir saha oyuncusu koşuyor (kovala), adamını bırakıyor. Savunma satırlarındaki kötüleşme buradan gelebilir. Öneri B (zip60) aynı satırları hedefliyor; zip60 tablosunda bakılacak. Geçiş ve durma satırları gürültü olabilir (16 maçta 8'erlik yarılar arasında da oynuyor).

## Açı × mesafe (zip59)
| Açı | Yakın (0–15) | Orta (15–25) | Uzak (25+) |
|---|---|---|---|
| 0–30° | 12/23 (%52) | 12/32 (%38) | 5/69 (%7) |
| 30–60° | 10/14 (%71) | 4/28 (%14) | 1/53 (%2) |
| 60–75° | 4/8 (%50) | 3/35 (%9) | 2/20 (%10) |
| 75°+ | 1/4 (%25) | 2/27 (%7) | 0/15 (%0) |

## Tam tablo
| Ölçü | zip58 (16 maç, 120 sn) | zip59 (16 maç, 120 sn) |
|---|---|---|
| Sayı / maç  | 3.81 | 3.88 |
| Sayı / dk  | 1.91 | 1.94 |
| Gönderme / dk  | 10.3 | 10.3 |
| Gönderme başına sayı %  | 15.8 | 17.1 |
| Pas / dk  | 58.1 | 57.9 |
| Pas tutma % ↑ | 90.9 | 91.9 |
| Kesme / dk  | 19.6 | 18.1 |
| Hesap ms/tik ↓ | 33.6 | 32.4 |
| Saha dışında Çekirdek (tik) ↓ | 0.00 | 0.00 |
| Bekçi pas kovalaması / dk ↓ | 1.56 | 0.38 ✓ |
| Bekçi kovaladığını kazanıyor % ↑ | 64.0 | 66.7 |
| Sayılarda Bekçi yerinde % ↑ | 59.6 | 75.0 ✓ |
| Yakın direkten sayı / dk ↓ | 0.78 | 0.66 ✓ |
| Dar açı (60°+) gönderme başına sayı % ↓ | 11.8 | 11.0 |
| Dar açıda Bekçi–yakın direk ↓ | 3.79 | 3.61 |
| Uzak geniş (25+, 60–75°) sayı % (n) ↓ | 0.00 (24) | 10.0 (20) ⚠ |
| Yakın geniş (<15, 60–75°) sayı % (n) ↓ | 75.0 (8) | 50.0 (8) ✓ |
| Kontra: önü boşken 12+ tik topla duran / dk ↓ | 0.13 | 0.03 ✓ |
| Pastan sonra geri koşan pasçı % ↓ | 18.8 | 18.6 |
| Baskı altında "tut" seçimi % ↓ | 19.8 | 15.5 ✓ |
| "Tut"tan sonra 1 sn'de kayıp % ↓ | 31.6 | 35.0 |
| Boş hücumcu (rakip yarısı, 8 br) zamanın % ↓ | 47.0 | 49.2 |
| Boş adam + çift adam anı / dk (6 tikte bir örnek) ↓ | 10.2 | 12.1 ⚠ |
|   bunlarda iki kişi "arkada" / dk ↓ | 1.50 | 2.50 ⚠ |
| Taşıyıcıya baskı yok (rakip yarısı) % ↓ | 13.6 | 17.8 ⚠ |
| Topsuz rakipte 4 br içinde 2 savunmacı (bütün saha) / dk ↓ | 26.0 | 28.4 |
| Top kazanınca 3 sn içinde gönderme % ↑ | 15.8 | 17.1 |
| Top kazanınca 5 sn içinde sayı / dk  | 0.97 | 1.06 |
| Geçiş "git" tik / dk ↑ | 47.5 | 39.0 ⚠ |
| Baskı altında "tut": akıl kayıp % (n)  | – | – |
| Baskı altında "tut": gerçek kayıp % (aynı anlar)  | – | – |
| Her an duran hücumcu ↓ | 0.21 | 0.25 ⚠ |
| Uzun tutuşta 1 sn+ duran hücumcu ↓ | 0.23 | 0.14 ✓ |
