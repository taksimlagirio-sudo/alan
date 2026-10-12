# ALAN v5 · Akıl farkındalık haritası

Fizikteki ve maç adımındaki her mekanik maçta kendiliğinden çalışır. Ama oyuncunun kararı (`decide.js`) bir mekaniği ancak biz onun tahminine elle eklediysek görür. Bu dosya hangisinin hangisi olduğunu tutar. **Yeni bir mekanik eklenince buraya bir satır eklenir.**

Durumlar:
- **Görüyor:** karar bu mekaniği tartıyor.
- **Kısmen:** tartıyor ama eski ya da kaba bir varsayımla.
- **Görmüyor:** maçta çalışıyor, kararda yok.

| Mekanik | Nerede | Akıl | Not |
|---|---|---|---|
| Şarj (hareketle harcanan enerji) | core, match | Görüyor | `chargeRate`: tutma, sürme ve beklemenin şarj kazancı |
| Zaman ve skor | match | Görüyor | `macK` riski, `macZ` geçen zamanın değeri; etkisi küçük |
| Kenardan pas | decide, match | Görüyor | Sadece Okuma 8+ ve doğrudan hat zayıfsa aday |
| Kenardan çalım | decide, match | Görüyor | Aday olarak üretiliyor |
| Tek dokunuş | decide, match | Görüyor | Okuma 6+ eşiği var |
| Gövdeyle koruma | core, decide | Görüyor | İkili tahmininde (`shieldK`) |
| Dolanma (presçi) | match | Görüyor | Beklemenin değerinde; en yakın iki rakip ikinci yarıda önüne dolanmış sayılır |
| Bekçi'nin uzanması, tepkisi ve yeri | core, shape (`bekci`) | Görüyor | Pas ve gönderme denemesindeki Bekçi maçın kuralıyla (açıortay) yerleşir; gönderme tablosu bununla yeniden üretildi (sürüm `v5-2`) |
| Gönderme isabeti (baskı, koşu, güç) | decide (`gonEk`) | Görüyor | Maç ve akıl aynı fonksiyonu kullanıyor; gönderme adayının denemesi bu dağılmayla yapılıyor. "Buradan sonra gönderirim" değeri (gönderme tablosu) de bugünkü isabetle ölçüldü |
| Kontrol hatası (yanlış kontrol, kaçırma) | core, decide | Görüyor | Pas denemesinde alıcı söndüremediği hızla Çekirdeği geliş yönünde kaçırır, oyun oradan sürer (eski sabit %40/%60 kaldırıldı) |
| Katı gövde (oyuncular iç içe girmez) | match (`govdeAyir`), decide (`dolanma`, `simOne`) | Görüyor | Tutma ve sürme riskinde dolanma payı. Pas denemesinde oyuncular maçtaki fonksiyonla (`AlanMatch.govdeAyir`) ayrılır |
| Çekirdeğin gövdeden sekmesi | match, decide | Görüyor | Pas denemesinde alıcı olmayan arkadaşın gövdesinden Çekirdek seker (maçtaki kural) |
| Perde | match (`perde`) | Görüyor | Presçi gelirken yakındaki arkadaş yoluna girer (Okuma'yla artan ihtimal); presçi onun etrafından dolanır. Akıl bunu tutma ve sürme riskindeki dolanma payıyla görür |
| Topla hız (Sürme'ye bağlı) | match, decide (`topK`) | Görüyor | Sürme ve tutma riskinde taşıyıcının hızı Sürme'ye bağlı, sürerken koşu |
| Çeviklik (dönüşte yavaşlama) | match (`ivme`), decide (`donT`, `yuru`) | Görüyor | Tutma ve sürme riskinde yön değiştirme süresi. Pas denemesinde herkes maçın ivme kuralıyla (`AlanMatch.ivme`) koşar |
| Çekirdeği döndürme ve fiske | core, match | Kısmen | Gövdeyle koruma tahmininde var; sürme yönü seçiminde dönüş süresi yok |
| Presçinin fake'i | shape, decide (`fakeOkunmaz`) | Görüyor | İkili tahmininde: taşıyıcı fake'i Okuma farkıyla okur; okuyamazsa gövdeyle koruma yanlış yanda sayılır |
| Bekçi'nin alanı (siyah daire) | match | Görüyor | Saha oyuncusu oraya giremiyor; adaylar oraya üretilmiyor |
| Kafada oynatma | decide | Kapalı | `look: false` (11 Ekim'e kadar yanlışlıkla açıktı). Elle kurulan bir kopyada oynar, yukarıdakilerin hiçbirini görmez; Okuma 20'de asıl hesabın yerine tamamen geçiyordu. Boşta kalan Çekirdeğin değeri 0 sayılıyordu, düzeltildi |

## Tablolar
- **Gönderme tablosu** (`shot-table.js`): fizik ölçümü, maçlardan öğrenilmiyor. **Bekçi kuralı, gönderme isabeti, Kuyu ya da Çekirdek fiziği değişince** `decide.js`'deki `SHT_SURUM` değiştirilir ve tablo `shotCell` ile yeniden üretilir (5 iş parçacığında ~1 sn). Sürüm uyuşmazsa motor hücreleri ilk istendiğinde kendisi ölçer.

## Kalan eksikler
1. ~~Pas denemesindeki kovalama: gövdeleri ve Çevikliği yok sayıyor.~~ Kapandı (Claude Code, pas kalibrasyonu: `CLAUDE-CODE-SONUC.md`).
