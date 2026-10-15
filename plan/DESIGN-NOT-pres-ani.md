# ALAN · Pres: ilk presçi her zaman basıyor, arkasında boşluk kalıyor

Claude Code'dan Design'a. Kullanıcı gözlemi: "Savunma oyuncuları baskı yönünü değerlendirmiyor; ileri doğru basıp arkada boşluk yaratıyorlar. Topa sahip olana her zaman basmak mantıklı değil, doğru anı kollamak gerekir."

Kaynak: yeni maç yok; zip69 kayıtları (16 maç) ve `shape.js` okuması. Araç: `tools/pres-iz.js`.

## 1 · Kodda
- `shape.js` (satır 7): "**ilk presçi her zaman basar**; ek presçiler yalnız seçili tetiklerden biri varken." Tetikler yalnız ikinci ve sonraki presçiler için.
- Pres hattı (`hatK`, blok): taşıyıcı hattı geçince basılıyor; hattın gerisinde de "fırsat iyiyse ya da topu uzun tutuyorsa" (hiç basmamak yok).
- Presçi uzaktaysa (> 4 birim) önce taşıyıcının kale tarafındaki noktaya (`gside`, 2 birim) koşuyor; yaklaşınca doğrudan Çekirdeğe ya da yandan. Yaklaşma açısı ileri pas yolunu (gölgesini) hesaba katmıyor; bıraktığı alanın bedeli `prV`'de yalnız tuttuğu adam (`tutQ`, zip55) olarak var.

## 2 · Kayıtta: presin sonucu (4017 pres başlangıcı)
Sonuç: presçi basmaya başladıktan sonra (4 sn içinde) ilk olan.
| Sonuç | Pay | Basmaya başlarken taşıyıcıya | Arkadaşlarının en geri hattından önde |
|---|---|---|---|
| Top kazanıldı | %27 | 9,0 br | 13,9 br |
| **Taşıyıcı pası presçinin arkasına verdi** | **%44** | 10,9 br | **25,4 br** |
| Önüne / yana pas | %29 | 10,2 br | 18,5 br |

**Arkasına verilen pasların %49'u 3 sn içinde göndermeyle bitiyor** (870 gönderme).

Bölgeye göre (taşıyıcının yeri; 0 kendi kalesi, 1 rakip kalesi):
| Bölge | Pres | Kazandı | Arkasına pas | → 3 sn'de gönderme (tüm preslerin) |
|---|---|---|---|---|
| Taşıyıcının kendi üçte biri | 1030 | %12 | %57 | %7 |
| **Orta üçte bir** | 1533 | %20 | %49 | **%31** |
| Son üçte bir | 1454 | %45 | %29 | %23 |

Basmaya başladığı uzaklığa göre:
| Uzaklık | Pres | Kazandı | Arkasına pas | → gönderme |
|---|---|---|---|---|
| 0–4 | 940 | %29 | %39 | %17 |
| 4–8 | 824 | %33 | %39 | %19 |
| 8–14 | 1287 | %28 | %45 | %20 |
| **14+** | 966 | **%19** | **%51** | **%30** |

**Yorum:** Kullanıcı haklı. Geçilen presçi, kazanan presçiden ortalama **11 birim daha önde** (hattından 25 birim). Orta sahada ve uzaktan başlayan pres en kötüsü: her 3 presten biri arkasına pas ve gönderme ile bitiyor. Son üçte birde pres işe yarıyor (%45 kazanma).

## 3 · Öneri
1. **İlk presçi de karar versin (tek kural):** Basmanın değeri = kazanma ihtimali × kazanç − geçilme ihtimali × arkada bıraktığı boşluğun tehlikesi. Geçilme ihtimali: taşıyıcının presçi varmadan önce oynayabileceği (ileri) paslar; boşluğun tehlikesi: presçinin hattından ne kadar önde olduğu ve arkasındaki rakipler. Değer negatifse basmaz, **oyalar**: kale tarafında, ileri pas yolunu kapatan açıda, belli uzaklıkta durur ve hattıyla bağını korur.
2. **Doğru an (tetik):** Oyalayan presçi, tetik oluşunca basar: taşıyıcı arkası dönük / kötü dokunuş / yan çizgiye doğru / yavaş pas alıyor / ileri pas yolu kapandı. Bu tetikler bugün yalnız ek presçiler için var; ilk presçiye de uygulansın.
3. **Baskı yönü:** Yaklaşma noktası (`gside`) taşıyıcının kale tarafı değil, **en tehlikeli ileri pasın gölgesi** olsun (taşıyıcı ile o alıcının arasına doğru, kale tarafında). Taşıyıcıyı yana / geriye zorlar; "Kenara sıkıştır" sistemindeki gibi ama her sistemde ileri yolu kapatarak.
4. Taktik ayarı (pres şiddeti, blok) bu kararın eşiğini değiştirir: yüksek şiddet daha çok basar (risk kabul), düşük şiddet daha çok oyalar. Menajerin seçimi anlamlı olur.

## 4 · Kabul (kontrol tablosuna eklenecek satırlar)
- Pres sonrası arkasına pas payı ↓ (bugün %44), bundan 3 sn'de gönderme ↓ (%49 → ?).
- Orta üçte birde pres → arkasına pas → gönderme ↓ (bugün %31).
- Pres sonrası top kazanma payı ≥ bugün (%27); toplam top kazanma kötüleşmesin.
- Yenen sayı ↓ ya da aynı.
