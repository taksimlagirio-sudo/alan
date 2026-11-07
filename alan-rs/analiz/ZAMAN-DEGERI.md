# Zamanın değeri: kontra deparı ve uzak şut aynı kökten (38 maç kaydı, analiz/out)
Ölçüm: `python3 analiz/py/kontra.py` (alan-rs içinde).
Kontra: top, rakip kaleye 50+ br'de kazanılıyor, topun kale tarafında en çok 3 rakip saha oyuncusu var (103 kontra).
| kaleye 25 br'ye varış | kontra | 12 sn içinde gol |
|---|---|---|
| <5 sn | 9 | %89 |
| 5–8 sn | 7 | %29 |
| 8+ sn | 8 | %12 |
| varamadı | 79 | %5 |
25 br'ye girişte topun önündeki savunmacı: 0 → %73 (11), 1 → %50 (4), 2 → %33 (3), 3+ → %0 (6). Örnek küçük, yön çok net.
Bar (oyuncu-devre başına en düşük bar): hiz2 yok medyan 0.35 (%33'ü 0.3 altına iniyor); hiz2 ilk 0.47 (%25); hiz2 son 0.42 (%20).
Sonuç: (1) Enerji kıt bir kaynak, oyuncular devre içinde barı gerçekten tüketiyor → HIZ2_BAR'ı genel olarak düşürmek gerekçesiz, eski
boşa deparları (güvence %44) geri getirir. (2) Kontrada erken varmanın değeri, hedef hücrenin anlık değeri (Sy 0.19) değil, kontranın kendisi:
5 sn'de kutuya varan kontra %89, her geri dönen savunmacı ~20 puan götürüyor. Hız kararında kontranın payı Sy ile değil kontranın değeriyle
(bütün savunmacıların geri dönüş süreleriyle azalan) ölçülmeli. (3) Hücumdaki "40 br'de topu tutmak 0.03–0.05, gerçekte %18–27" açığı aynı
hata: anlık fotoğraf, zamanla ne olacağını görmüyor. Kontrada zaman aleyhte (değer hızla düşer → acele), yerleşik hücumda lehte (değer
zamanla artar → sabır). İkisi de "durumun önümüzdeki saniyelerdeki değeri" ile çözülür.
