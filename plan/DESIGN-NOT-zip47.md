# ALAN · zip47 kontrolü

## Birebir aynılık ✓
- **120×60 (yeni varsayılan):** zip47, zip46 + `ALAN_SAHA_BOY = [120, 60]` ile 8 / 8 maçta her tikte aynı.
- **100×50:** zip47 + `ALAN_SAHA_BOY = [100, 50]`, zip46 ile 8 / 8 maçta aynı (kayıttaki saha alanı dışında; eski kayıtlarda o alan yoktu).

## Ekranlar
- Canlı 120×60'ı doğru çiziyor; maç sonuna kadar oynandı. Konsolda yeni hata yok (`{{ taktikSrc }}` 404'ü zip46'da da var, zararsız: şablon dolmadan iframe bir kez istek atıyor; istenirse src ilk anda boş verilebilir).
- Taktik açılıyor, tahta 100×50 şeması.

## Küçük düzeltme
Taktik tahtasının altındaki "hat aralığı 42 br · genişlik 32 br" yazısı 100×50 biriminde. 120×60'ta gerçek değerler ~50 ve ~38 br. Ya yazıda × KX / × KY ile gerçek birim gösterilsin, ya da birim oran (% saha) olsun. Aynısı ayar satırlarındaki "42 br", "32 br", "ort. 18 br" için de geçerli (18 br destek mesafesi mutlak mı oranlı mı, sınıfına göre).
