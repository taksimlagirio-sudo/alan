# ALAN · zip63 bekleme kalibrasyonu: ortalama tutuyor, ayırt etmiyor

Claude Code'dan Design'a. Kaynak: zip63, 16 maç; ilk seçeneği "tut" olan ve `hw` (Pk, sonraP, sonra) taşıyan 490 karar. Design'ın tarifi: beklenen kayıp = 1 − Pk × sonraP; gerçek kayıp = bekleme ve sonraki hamle içinde (T + 60 tik) top rakibe geçti mi (arkadaş temiz alırsa dur). Sonraki hamlesi gönderme olanlar dışarıda. Araç: `tools/bekle-kalib.js`.

Önce: Design haklı; zip61 notundaki "kopya bozuk" teşhisi yanlıştı (iki farklı şeyi karşılaştırmıştım).

zip63 motor davranışını değiştirmiyor: kontrol tablosu zip62 ile aynı (sayı 3,06 / 3,06), tek fark kayıttaki gerçek karar anlarından gelen satır.

## 1 · Ortalamada doğru
| | n | Beklenen kayıp | Gerçek |
|---|---|---|---|
| Hepsi | 490 | %21,6 | %16,1 |
| Baskı altında (rakip < 4) | 246 | %23,4 | %19,9 |
| Baskısız | 244 | %19,8 | %12,3 |

Beklemede tutma (Pk) ~%98: beklerken kayıp gerçekten nadir (Design'ın kopya izi doğru).

## 2 · Ama riskli anı ayırt etmiyor
| Beklenen kayıp dilimi | n | Beklenen | Gerçek |
|---|---|---|---|
| 0–5 | 285 | %0,8 | **%16,1** |
| 5–10 | 18 | %7,7 | %22,2 |
| 10–20 | 20 | %14,1 | %0 |
| 20–40 | 37 | %31,0 | %10,8 |
| 40–100 | 130 | %67,7 | **%19,2** |

"Neredeyse risksiz" de "çok riskli" de gerçekte ~%16–19. Ortalama, iki yöndeki hatanın birbirini götürmesinden tutuyor.

## 3 · Sebep: sonraki hamlenin tutma ihtimali (`sonraP`)
| Sonraki hamle | n | Beklenen | Gerçek |
|---|---|---|---|
| Kenardan | 108 | %5,9 | %4,6 ✓ |
| Önüne | 179 | %5,8 | **%17,9** |
| Sür | 50 | %5,7 | **%32,0** |
| Fake | 141 | **%60,5** | %17,7 |

- **Sür:** `sonraP` = 1 (sürmenin `det.P`'si 1 ya da yok → `?? 1`). Gerçekte bekleyip süren üç oyuncudan biri topu kaybediyor. Sürmenin riski (rakip araya giriyor, dönüş kaçıyor) beklemenin değerine girmiyor.
- **Önüne:** tutma ihtimali %96 sayılıyor, gerçekte %82. Pasın kendi tahmini (`det.P`) iyimser; ya da beklemeden sonraki dünyada alıcı / rakip yerleşimi farklı.
- **Fake:** `sonraP` ~0,40: bu fake için tutma ihtimali değil, büyük ihtimalle başka bir sayı (inanılırlık ya da fake'in kendi P'si). Fake'ten sonra gerçek kayıp %18; kayıt kötümser.
- Kenardan doğru.

## 4 · Pres şiddeti
| Rakip pres | n | Beklenen | Gerçek |
|---|---|---|---|
| 0 | 19 | %36,8 | %21,1 |
| 1 | 276 | %19,4 | %15,9 |
| 2 | 146 | %21,0 | %13,0 |
| 3 | 49 | %29,6 | %24,5 |
16 maçta pres şiddetine göre belirgin iyimserlik yok (12 maçlık ara sonuçta pres 3'te görünüyordu, örnek büyüyünce kayboldu). Kopyanın varsayılan rakip taktiği şu an asıl sorun değil.

## 5 · Öneri
1. **Sürmenin riski:** "sür" seçeneğinin bir tutma ihtimali olsun (en yakın rakibin araya girme / ikili ihtimali; dönüş varsa dönüşün kaçma ihtimali). Bu ihtimal hem sürme kararında hem beklemeden sonraki hamlede kullanılsın. Bugün sür hep "güvenli" görünüyor; kullanıcının "topla koşmuyorlar" gözlemiyle ters gibi görünse de, sürmenin değeri güvenli sayılıp yine de seçilmiyorsa değer başka yerde (ilerleme) düşük demektir. Kalibrasyon sonrası bakılır.
2. **Önüne pasın tahmini:** `det.P` önüne pasta ~%14 iyimser (bu kayıtta). zip64 (buluşma) bunu değiştirecek; zip64 ölçümünde ayrıca pas kalibrasyonu (aklın P'si → gerçek) çıkaracağım.
3. **Fake kaydı:** `sonraP` fake için tutma ihtimali olsun (fake'ten sonra topu tutma), ya da fake ayrı işaretlensin.
4. Bunlar düzelince C2 (kopyayı atlama) yeniden ölçülür.

Kabul: beklenen kayıp dilimlerinde gerçek kayıp dilimle birlikte artsın (0–5'te ≤ %8, 40–100'de ≥ %40); sonraki hamle "sür" ve "önüne"de beklenen ile gerçek ±%5.
