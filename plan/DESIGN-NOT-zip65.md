# ALAN · zip65 (sürmenin riski kopyada): benim ölçüm hatam, ama yan etkisi iyi; bedeli hız

Claude Code'dan Design'a.

## 1 · Önce düzeltme: "sür %32 kayıp" yanlıştı
zip63 notunda "beklemeden sonra süren üç oyuncudan biri topu kaybediyor (%32)" yazdım. Ölçüm penceremde hata vardı: gerçek kaybı "bekleme + 60 tik içinde top rakibe geçti mi, arkadaş temiz alırsa dur" diye sayıyordum. Sürmede arkadaşa geçiş olmadığı için pencere sürmeden **sonraki** hamleyi (pas, gönderme) de içine alıyordu; o hamlelerin kaybı sürmeye yazılıyordu.

Düzeltilmiş ölçüm (sürmede kayıp = taşıyıcı topu bırakmadan rakibe geçmesi):
| | Beklenen | Gerçek |
|---|---|---|
| zip63, tut → sür (50) | %5,7 | **%2,0** |
| zip65, tut → sür (41) | %1,3 | %4,9 |
| zip65, doğrudan "sür" seçilen 1975 karar (45 tik içinde, top elden çıkmadan) | %1,5 | %1,5 |

**Sürmenin riski zaten doğru tahmin ediliyordu.** zip65'in değişikliği benim yanlış ölçümüme dayanıyor. Özür dilerim; bundan sonra pencereyi her hamle türü için ayrı tanımlıyorum (`tools/bekle-kalib.js` düzeltildi).

Düzeltilmiş kalibrasyonda kalan gerçek iyimserlik **önüne pas** (beklenen %6–8, gerçek %18–19) ve genel olarak "kesin" (P ≥ 0,99) paslar (zip64 notu: %99 → %94).

## 2 · Yine de zip65 oyunu değiştirdi (bayrakla ayrıldı)
Aynı 8 maç (senaryo 0–7), yalnız `ALAN_SUR_KOPYA_ESKI` farklı:
| | zip65 | Sürme kopyası kapalı |
|---|---|---|
| Hesap ms/tik | **41,3** | 34,8 (sürme kopyası **+%19**) |
| Koşarak alıp koşmaya devam eden % | **58,9** | 53,0 |
| Koşarak alıp ilk 20 tikte duran % | **25,8** | 33,2 |
| Pas tutma % | 93,1 | 91,0 |
| Sayı / maç | 2,75 | 3,00 |
| "Tut"tan sonra gerçek kayıp (aynı anlar) % | 18,2 | 8,7 |

Kopyada sürmek, sürmenin gerçek değerini (ilerleme, rakibin yerleşmesi) akla daha iyi gösteriyor: topu koşarak alan daha çok koşmaya devam ediyor (kullanıcının şikâyeti), pas tutma arttı. Bedeli hız (+%19) ve "tut"tan sonraki kayıpta artış (8 maç, gürültü olabilir).

## 3 · Hız birikti
| Sürüm | Hesap ms/tik |
|---|---|
| zip63 | 25,1 |
| zip64 (buluşma pası) | 35,5 (+%41) |
| zip65 (sürme kopyası) | 42,0 (+%18) |
İkisi birlikte **+%67**. Buluşma pası ölçülebilir bir şey değiştirmedi (zip64 notu); sürme kopyası değiştirdi.

## 4 · Öneri
1. **Buluşma pasını kapat** (ya da yalnız koşan ve önü açık arkadaşa üret), hız kazancı ~%30. İşaret kayda geçince ayrıca ölçülür.
2. **Sürme kopyasını tut, ama ucuzlat:** yalnız taşıyıcının önü açıkken ve "sür" ilk iki seçenekteyken; kopya süresi 30 → 20 tik. Kabul: koşmaya devam eden ≥ %58, hesap ≤ zip63 + %10.
3. Kalibrasyonda sıradaki gerçek iş: önüne pasın ve "kesin" pasların iyimserliği.

## Tam tablo (zip64 → zip65, 16 maç)
| Ölçü | zip64 (16 maç, 120 sn) | zip65 (16 maç, 120 sn) |
|---|---|---|
| Sayı / maç  | 2.50 | 3.00 |
| Sayı / dk  | 1.25 | 1.50 |
| Gönderme / dk  | 11.5 | 10.8 |
| Gönderme başına sayı %  | 10.6 | 13.3 |
| Pas / dk  | 56.5 | 58.3 |
| Pas tutma % ↑ | 91.0 | 92.3 |
| Kesme / dk  | 20.3 | 17.8 |
| Hesap ms/tik ↓ | 35.5 | 42.0 ⚠ |
| Saha dışında Çekirdek (tik) ↓ | 0.00 | 0.00 |
| Bekçi pas kovalaması / dk ↓ | 0.72 | 0.63 |
| Bekçi kovaladığını kazanıyor % ↑ | 95.7 | 85.0 |
| Sayılarda Bekçi yerinde % ↑ | 69.2 | 58.7 ⚠ |
| Yakın direkten sayı / dk ↓ | 0.53 | 0.44 ✓ |
| Sayılarda Bekçi topa değmiş % ↑ | 23.1 | 34.8 ✓ |
| Bekçi teması (gönderme): tuttu %  | 12.5 | 12.2 |
| Bekçi teması: Kuyu'ya seken % (n) ↓ | 5.65 (19) | 7.90 (26) ⚠ |
| Dar açı (60°+) gönderme başına sayı % ↓ | 8.33 | 8.25 |
| Dar açıda Bekçi–yakın direk ↓ | 3.27 | 3.06 |
| Uzak geniş (25+, 60–75°) sayı % (n) ↓ | 0.00 (25) | 0.00 (18) |
| Yakın geniş (<15, 60–75°) sayı % (n) ↓ | 14.3 (7) | 25.0 (8) ⚠ |
| Kontra: önü boşken 12+ tik topla duran / dk ↓ | 0.09 | 0.06 ✓ |
| Pastan sonra geri koşan pasçı % ↓ | 20.2 | 18.3 |
| Baskı altında "tut" seçimi % ↓ | 18.5 | 17.7 |
| "Tut"tan sonra 1 sn'de kayıp % ↓ | 22.6 | 29.8 ⚠ |
| Boş hücumcu (rakip yarısı, 8 br) zamanın % ↓ | 48.2 | 45.5 |
| Boş adam + çift adam anı / dk (6 tikte bir örnek) ↓ | 13.4 | 11.1 ✓ |
|   bunlarda iki kişi "arkada" / dk ↓ | 2.97 | 2.34 ✓ |
| Taşıyıcıya baskı yok (rakip yarısı) % ↓ | 11.5 | 12.3 |
| Topsuz rakipte 4 br içinde 2 savunmacı (bütün saha) / dk ↓ | 31.3 | 27.5 |
| Top kazanınca 3 sn içinde gönderme % ↑ | 16.9 | 14.1 ⚠ |
| Top kazanınca 5 sn içinde sayı / dk  | 0.63 | 0.81 |
| Geçiş "git" tik / dk ↑ | 112 | 120 |
| Baskı altında "tut": akıl kayıp % (n)  | 0.00 (393) | 0.61 (329) |
| Baskı altında "tut": gerçek kayıp % (aynı anlar)  | 8.14 | 13.4 |
| Önüne pas: hedefin ±3 içinde alınan % ↑ | 18.8 | 17.7 |
| Önüne pas: alış–hedef uzaklık ↓ | 8.27 | 8.39 |
| Koşarak alıp koşmaya devam eden % ↑ | 53.3 | 58.6 |
| Koşarak alıp ilk 20 tikte duran % ↓ | 32.1 | 25.6 ✓ |
| Koşarak alınca ilk karara kadar (tik) ↓ | 3.76 | 3.72 |
| Taşıyıcı topla koşuyor % ↑ | 60.1 | 60.3 |
| Her an duran hücumcu ↓ | 0.20 | 0.23 ⚠ |
| Uzun tutuşta 1 sn+ duran hücumcu ↓ | 0.00 | 0.03 ⚠ |
