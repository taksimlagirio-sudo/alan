# ALAN · Düzeltme 1 (donuk dünya): koşma düzeldi, ama gönderme iyimserliği ortaya çıktı

Claude Code'dan Design'a. zip70, 16 maç (taban zip69); hız aynı makinede, aynı anda (senaryo 0–1). Not: değişiklik günlüğüne zip70 kaydı eklenmemiş (en üstte hâlâ zip69).

## 1 · Kontrol tablosu
| Satır | zip69 | zip70 | Ölçüt |
|---|---|---|---|
| Koşarak alıp koşmaya devam eden % | 53,5 | **58,6** | ≥ 60 (yakın) |
| Koşarak alıp ilk 20 tikte duran % | 29,5 | **23,5** ✓ | |
| Baskı altında "tut" seçimi % | 23,4 | 17,8 ✓ | |
| Pas tutma % | 91,4 | 91,7 | kötüleşmesin ✓ |
| Hesap (aynı makine) | 57,2 | 59,0 ms/tik (+%3) | ± %10 ✓ |
| **Sayı / maç** | 3,63 | **2,69** (−%26) | |
| **Gönderme başına sayı %** | 16,1 | **11,2** | |
| Gönderme / dk | 11,1 | 10,9 | |
| Yakın direkten sayı / dk | 0,69 | 0,47 ✓ | |
Diğer satırlarda (boş adam, baskısız taşıyıcı, duran hücumcu) küçük kötüleşmeler; önceki sürümler arasında da bu kadar oynuyorlar.

İlk karar dağılımı (koşarak alınca) az değişti: sür %35 → %39, fake %45 → %42, tut %21 → %18.

## 2 · Ölçüm 1 tekrarı (başa baş kararlar, hızlı taşıyıcı)
| | zip69 sür / fake-tut | zip70 sür / fake-tut |
|---|---|---|
| Başa baş karar sayısı | 997 | 992 |
| 3 sn'de gönderme | %45 / %32 | %46 / %38 |
| İlerleme (br) | 16,0 / 13,5 | 15,6 / 11,4 |
| 5 sn'de sayı | %13,7 / %10,2 | %8,5 / %9,0 |
| 2 sn sonra top bizde | %73 / %75 | %75 / %72 |

- Akıl hâlâ aynı sıklıkta "sür"ü fake / tut ile başa baş görüyor (992 karar; bunların %47'sinde fark 0,01'den küçük).
- Gerçekleşen fark gönderme ve ilerlemede **sürüyor**; sayıda kaybolmuş, ama zip70'te sayının kendisi düştüğü için (aşağıda) bu, akılın düzeldiği anlamına gelmiyor.
- **Okuma:** Düzeltme 1 doğru yönde (koşma iyileşti), ama "sür" ile fake / tut arasındaki değer farkını yeterince açmadı. Sürme ve beklemenin değeri büyük ölçüde zaten kopyadan geliyor (`holdWorld`); fake'in değeri kendi iki dünyasından (`fakeSecenek`, elle kaydırılmış konumlar). Yani `predictWorld` bu karşılaştırmanın tek kanalı değil. **Sonraki ölçüm:** hızlı taşıyıcının başa baş kararlarında sür / fake / tut değerinin bileşenleri (kopyadan gelen değer, `runOn`, yerleşme cezası, fake'in iki dünyası, `macZ`). Bunun için karar kaydına bileşenler yazılmalı (Design); hangisinin farkı sildiği o zaman görülür.

## 3 · Sayı neden düştü: gönderme kalibrasyonu
Aklın girme tahmini (Pk) → gerçek, mesafeye göre:
| Mesafe | zip69 akıl / gerçek | zip70 akıl / gerçek |
|---|---|---|
| 0–15 | %45,6 / %36,1 | %46,2 / %38,2 |
| **15–25** | %27,2 / %14,8 | %25,8 / **%10,1** |
| **25+** | %11,5 / %7,0 | %10,5 / **%3,1** |
| Hepsi | %24,6 / %16,3 | %22,7 / %12,3 |

- **Gönderme iyimserliği zaten vardı:** zip69'da orta mesafede akıl %27 diyor, gerçek %15 (1,8 kat).
- **zip70 bunu ortaya çıkardı:** Diğer hamlelerin değeri gerçeğe yaklaşınca (savunmanın geri dönüşü görünüyor), iyimser kalan gönderme daha sık kazanıyor. Yakın gönderme azaldı (0–15 düz: 27 → 19), orta mesafede gerçek oran %10'a düştü. Tahminden büyük sapan seçenekler seçiliyor ("en iyimser seçenek kazanır" etkisi).
- **Muhtemel kök (K2, iki ayrı dünya):** zip62 günlüğünde yazılı: kafadaki gönderme denemesi (`imagine`) Bekçi temasını kendi yoluyla tartıyor, `contact`'ın sekme yönünü (çelme), her yerde uzanmayı görmüyor. zip59–62 Bekçi değişiklikleri maçtaki girme oranını düşürdü, akıldaki oran neredeyse aynı kaldı.

## 4 · Öneri
1. **Düzeltme 1 kalsın** (koşma ve "tut" düzeldi, hız aynı). Sayıdaki düşüş Düzeltme 1'in hatası değil, gönderme tahmininin iyimserliğinin görünür olması.
2. **Önce gönderme kalibrasyonu (Düzeltme 2'den önce):** Akıldaki gönderme denemesinde Bekçi teması maçtaki `contact` ile aynı olsun (uzanma, çelme yönü, hazırlık). Gönderme tablosu ondan sonra yeniden üretilsin. Ölçüt: mesafe dilimlerinde akıl / gerçek ±%3.
3. Karar kaydına sür / fake / tut değer bileşenleri; Claude Code bileşen ölçümünü yapar.
4. Ondan sonra Düzeltme 2 (pres bedeli).
