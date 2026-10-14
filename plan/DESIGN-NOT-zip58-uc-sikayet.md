# ALAN · zip51–58: kullanıcının üç şikâyeti işe yaradı mı (16 maç)

Kaynak: 16 maç, 120 sn, devre açık, 120×60, herkes 20 (Alan gücü ve Soğukkanlılık 10), ayarsız takımlar. Karşılaştırma: aynı 16 senaryo zip50 (90 sn). Sayımlar dakika başına. İlk parti notu (DESIGN-NOT-zip51-58.md) bunu tamamlıyor.

## Özet
| Şikâyet | Durum |
|---|---|
| Bekçi köşede yakın direkten çok yiyor | **Düzelmedi.** Sebep kenar payı değil: Bekçi o anlarda yerinde değil (yeniden kovalıyor). |
| Kontrada topu alan bekliyor, pasçı geri koşuyor | Bekleme zaten seyrekti, aynı. Geri koşan pasçı biraz azaldı. Baskı altında tutup kaybetme arttı. |
| Savunmada yanlış adama koşma, boş adam | Biraz azaldı. "İki savunmacı aynı adamda, yakında boş adam" hâlâ sık. |

## 1 · Bekçi ve yakın direk ⚠
| | zip50 | zip58 |
|---|---|---|
| Bekçi ile yakın direk arasından giren sayı / dk | 0,63 | 0,78 |
| Dar açıdan (60°+) sayı · bunların yakın direkten | 8 · 6 | 12 · 7 |
| 60°+ göndermelerde Bekçi'nin yakın direğe uzaklığı | 2,8 | 3,8 |
| Bekçi pas kovalaması / dk | 1,0 | **2,1** |
| Kovaladığı topu kendi takımı alıyor | %75 | %64 |
| Sayılarda Bekçi yerinde | %71 | %60 |

**Kenar payı (zip58) sorunu çözmüyor.** Aynı 8 maç `ALAN_BEKCI_ESKI` ile: yakın direk sayısı 13 → 7 (eski kenarla daha az), yakın direk uzaklığı 4,0 / 3,8. Kırpma maçta nadiren devreye giriyor.

**Asıl sebep (an izi, zip58'de yakın direkten giren 25 sayı):**
- **Dar açıdan girenlerin hemen hepsinde Bekçi yerinde değil:** 57°+ olanlarda hedefine 11,7 · 11,2 · 11,9 · 12,0 · 13,8 · 8,2 · 18,5 birim (yalnız 2'si ~4). Çoğunda az önce bir pası kovalamış. Yani köşe sayıları zip49b'deki aynı mekanizma: Bekçi çıkıyor, yetişemiyor, dönerken yakın direkten yiyor.
- **zip50'den sonra Bekçi kovalaması iki katına çıktı** (dk başına 1,0 → 2,1) ve kazanma düştü (%75 → %64). zip50'nin "rakipten önce varmıyorsa çıkma" kuralı bir yerde gevşemiş ya da zip54–57'den biri Bekçi'yi yeniden çıkarıyor. Bayraklarla ayırmak gerekir (`ALAN_SAV_ESKI`, `ALAN_PLAN_ESKI`, `ALAN_BASKI_ESKI`, `ALAN_BEKLE_TAKLIT`); Claude Code bunu 8'er maçla yapabilir.
- **Ortadan, yakından (9–17 birim) girenlerde Bekçi yerinde** (hedefine < 1), top 1,4–2,6 birim yanından 4–5 tikte geçiyor. Bekçi'nin tepki süresi (`bekReact` 8 − Okuma payı) bu yoldan uzun, sadece gövdesi (0,9) kapatıyor. Futbolda da yakından şut zor, ama bu sayılar "kalecinin yanından" diye görünüyor. Ayrı bir karar: yakın şutta Bekçi'nin uzanışı.

Açı × mesafe (zip58): yakından (0–15) her açıda %61–75; 15–25'te %10–25; 25+'ta %0–7. Uzaktan geniş açı sorunu yok.

## 2 · Kontrada bekleme ve ver-koş
- **Top kazanınca (santra hariç, önünde 8 birim boş) topla 12+ tik duran:** zip50'de 16 maçta 4, zip58'de 4. İkisinde de seyrek; durduğunda ilk seçenek "tut" (değerler birbirine çok yakın: 0,075 / 0,074 / 0,073 gibi). Kullanıcının gördüğü an bu kadar seyrekse ölçümde yakalanmıyor; tohum ve tik verilirse o an ayrıca izlenir.
  - Not: Top kazanınca ilk karar bazen 30 tik gecikiyor; bunların hepsi santra (top ortada bekliyor), sorun değil.
- **Pastan sonra 2 birimden çok geri koşan pasçı:** %21 → %19 (ilk partide %17). Pasçının 1 sn'de ileri gidişi 5,4 → 5,5. Küçük düzelme.
- **Baskı altında (rakip < 4) "tut":** seçim %21 → %20, ama tuttuktan sonra 1 sn içinde kayıp **%12 → %32**. Kopya bekleme riski gösteriyor, karar seçimi değişmiyor. `−lossAt` ağırlığı ya da kopyanın tahmini gerçekle karşılaştırılmalı (Claude Code: "tut" anlarında aklın beklediği değer ve gerçekte olan).

## 3 · Savunmada boş adam ve çift adam
| | zip50 | zip58 |
|---|---|---|
| Rakip yarısında 8 birimde savunmacısız hücumcu (zamanın) | %51 | %47 |
| Aynı anda başka bir hücumcuda 2 savunmacı (anlar / dk) | 12,9 | 10,2 |
| Bunlarda "arkada + arkada" (iki kişi aynı adamın arkasında) / dk | 2,8 | 1,5 |
| Taşıyıcının 12 birim içinde rakip yok (rakip yarısında) | %9,3 | %7,2 |

Çift adam anlarında savunmacıların görevleri (zip58): arkada + kapatma 51 · bölge + kapatma 49 · arkada + arkada 47 · arkada + bölge 35 · bölge + bölge 23 · kapatma + kapatma 23. Boş adama en yakın savunmacının görevi: bölge 146 · kapatma 99 · arkada 49.
- Görev çakışması hâlâ sık: aynı rakip iki ayrı görevin (arkada / kapatma / bölge) hedefi olabiliyor. zip55'teki ortak atama (`fazla`) "arkada" ve "bölge"yi çakışmada yeterince yeniden dağıtmıyor gibi.
- Öneri: Bir rakip zaten birinin "kapatma" ya da "arkada" hedefiyse, ikinci savunmacı (bölge / arkada) ona değil, bölgesindeki en tehlikeli boş rakibe yönelsin.

## 4 · Genel (16 maç)
| | zip50 | zip58 |
|---|---|---|
| Sayı / maç | 2,19 | 3,81 |
| Sayı / dk | 1,46 | 1,91 |
| Gönderme / dk | 9,0 | 10,3 |
| Pas / dk | 46,6 | 49,6 |
| Saha dışında Çekirdek | 24 tik | 0 ✓ |
| Devreden sonraki ilk 10 sn'de sayı / maç | – | 0,25 (sıradan 10 sn 0,32) |
| Hesap (4 maç birlikte, ms/tik) | 29,0 | 33,6 (+%16) |

## 5 · Sıradaki öneri
1. Bekçi kovalamasının neden ikiye katlandığı: bayraklarla 8'er maç (Claude Code).
2. Görev çakışması (çift adam) kuralı (Design).
3. Baskı altında "tut": aklın tahmini → gerçek (Claude Code).
