# ALAN · zip61: B kapalı doğrulandı · bekleme kopyası hiç kayıp görmüyor

Claude Code'dan Design'a. 16 maç (taban zip59 / zip60) ve C2 için 8 maç bayrakla.

## 1 · B kapalı (`ALAN_ATAMA_YENI` yok): beklenen geldi ✓
| | zip59 | zip60 | zip61 |
|---|---|---|---|
| Sayı / maç | 3,88 | 4,69 | **3,81** |
| Gönderme başına sayı % | 17,1 | 19,1 | 15,7 |
| Topsuz rakipte 4 br içinde 2 savunmacı / dk | 28,4 | 44,0 | **27,5** |
| Pastan sonra geri koşan pasçı % | 18,6 | 22,6 | 19,0 |
| Geçiş "git" tik / dk (C3) | 39,0 | 77,6 | 95,3 |
| Hesap ms/tik | 32,4 | 26,7 | 27,7 |

Diğer satırlar zip59 civarında. Kontrada topla duran (0,03 / 0,06 / 0,13 dk başına) çok seyrek, ⚠ gürültü.

## 2 · C1 kalibrasyonu: kopya beklemede topu hiç kaybetmiyor ⚠
Baskı altında (en yakın rakip < 4) ilk seçeneği "tut" olan ve `hw` taşıyan kararlar; gerçek kayıp: aynı bekleme süresi içinde (en az 30 tik) top rakibe geçti mi.

| | zip61 (16 maç) | zip61 C2 kapalı (8 maç) |
|---|---|---|
| Karar sayısı | 486 | 176 |
| Kopyada kayıp (`hw.kayip`) | **%0** | **%0** |
| Gerçekte kayıp | %13,8 | %9,7 |

**486 kararın hiçbirinde kopya Çekirdeği kaybetmiyor.** Gerçekte her 7 "tut"tan biri kayıpla bitiyor. Beklemenin değeri bu yüzden hep iyimser; baskı altında "tut" seçimi yüksek kalıyor.

Olası sebepler (kontrol edilmeli):
1. Kopyada (`holdWorld`, `M.kopya` + `_lite`) rakibin topa müdahalesi (ikili, gövde, kapma) çalışmıyor ya da `_lite` bunu atlıyor.
2. Kopyada taşıyıcı "yerinde tut" iken rakip savunması ona hiç basmıyor (rakip taktiği varsayılan; pres / karşı pres kopyada yok mu?).
3. Kayıp işareti (`HW.lost`) yanlış koşulda set ediliyor (ör. yalnız `m.holder` değişince; ama kopyada sahiplik başka alanla tutuluyor olabilir).
Hızlı sınama: bir "tut" kararında kopyayı tik tik yazdırmak (en yakın rakibin uzaklığı, ikili var mı, holder).

## 3 · C2 (kopya yalnız gerektiğinde): şüphe doğrulandı
Aynı 8 maç (senaryo 0–7), `ALAN_BEKLE_HEP`:
| | zip61 | C2 kapalı (kopya hep) |
|---|---|---|
| Baskı altında "tut" seçimi % | 26,2 | **19,4** |
| "Tut"tan sonra 1 sn'de kayıp % | 34,1 | **25,6** |
| Hesap ms/tik | 27,4 | 31,5 |
| Sayı / maç | 3,50 | 3,25 |

C2 atladığı anda "tut" ilk (iyimser) değerinde kalıyor; "tut" seçimi ve kayıp artıyor. Ama asıl sorun kopyanın kendisi (bölüm 2): kopya oynatıldığında da kayıp görmüyor. **Öneri:** Önce kopyanın kayıp görmesi düzeltilsin; sonra C2 yeniden ölçülür. Kopya doğru kayıp gösterirse C2'nin atlama koşulu (tut'un ilk değeri en iyi seçenekten 0,1 düşük) muhtemelen zararsız olur ve hız kazancı kalır.

## 4 · Kabul ölçütü (bir sonraki tur)
- Kopyada kayıp ≈ gerçek kayıp (±%5) baskı altındaki "tut" kararlarında.
- Baskı altında "tut" seçimi ↓, "tut"tan sonra kayıp ≤ %15.
- Hesap ≤ zip61 + %5.

zip62 (Bekçi teması) ayrı ölçülüyor; sonucu ayrı not.
