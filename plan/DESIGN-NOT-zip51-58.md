# ALAN · zip51–58 ölçümü (ilk parti)

Kaynak: 8 maç (senaryo 0–7), 120 sn, devre arası açık, 120×60, herkes 20 (Alan gücü ve Soğukkanlılık 10), ayarsız takımlar. Karşılaştırma: aynı 8 senaryo zip50 (90 sn, devresiz). Maç süreleri farklı; sayımlar dakika başına. 8 maç: yön gösterir.

## 1 · Genel (zip51)
| | zip50 | zip58 |
|---|---|---|
| Sayı / maç | 2,50 | **3,88** |
| Sayı / dk | 1,67 | 1,94 |
| Gönderme / dk | 9,3 | 10,9 |
| Pas / dk | 46,9 | 48,7 |
| Kesme / dk | 19,0 | 18,4 |
| Hesap (4 maç birlikte, ms/tik) | 29,0 | 33,7 (+%16) |

- Beklenen ~2,8 sayı/maç; çıkan 3,9. Dakika başına sayı da arttı (1,67 → 1,94): yalnız süre değil, başka turlar da sayıyı artırmış.
- Hesap +%16: büyük ihtimalle zip54'teki bekleme oynatması (maçın kopyası). Kabul edilebilir; kontrol paketinde ayrıca bakılabilir.
- Devreden sonraki ilk 10 sn: 2 sayı (maç başına 0,25; sıradan 10 sn'de 0,32). Santranın belirgin etkisi yok.
- Eski kaydın birebir oynaması (len 5400, devresiz) ölçülmedi: araya giren kural değişiklikleri maçı zaten değiştiriyor; mac-isci yolu Canlı'da sınanmalı.

## 2 · Çekirdek saha içinde (zip52) ✓
- Saha dışında serbest Çekirdek: zip50'de 24 tik → **0**.
- Arka çizgiye 1 birimden yakın gönderilip giren sayı: 0.

## 3 · Bekleme (zip54, zip57)
| | zip50 | zip58 |
|---|---|---|
| Top kazanınca ilk 3 sn'de taşıyıcının tutuşu | 19,8 tik | 18,5 tik |
| İlk 3 sn'de gönderme | %13 | %15 |
| Baskı altında (rakip < 4) "tut" seçimi | %21 | %19 |
| "Tut"tan sonra 1 sn içinde kayıp | %12 | **%24** |

Baskı altında "tut" seçimi neredeyse aynı, ama tutan topu iki kat daha sık kaybediyor. Kopya beklemenin riskini gösteriyor olmalı; buna rağmen seçim sıklığı düşmemiş. İki olasılık: (1) kopyadaki kayıp değeri (`−lossAt`) seçimi değiştirecek kadar ağır değil; (2) kopya ile gerçek arasında fark var (aklın tahmini → gerçek). Claude Code ikinci partide "tut" anlarında kopyanın tahminini gerçekle karşılaştırabilir (karar kaydında beklemenin değeri gerekirse).

## 4 · Presçi ve ver-koş (zip55, zip56) ✓
- Rakip yarısında taşıyıcının 12 birim içinde rakip yok (Bekçi hariç): tutuşun %9,3'ü → **%6,4**.
- Pas veren 1 sn'de ileri: 5,4 → 5,9 birim; 2 birimden çok geri koşan pasçı %21 → **%17**.

## 5 · Bekçi (zip58) ⚠
| | zip50 | zip58 | zip58 + `ALAN_BEKCI_ESKI` |
|---|---|---|---|
| Pas kovalaması / maç | 1,9 (90 sn) | 3,6 | 3,1 |
| Kovaladığı topu kendi takımı alıyor | %73 | %55 | %84 |
| Sayılarda Bekçi yerinde | %79 | %50 | %62 |
| 60°+ göndermelerde Bekçi'nin yakın direğe uzaklığı | 2,8 | **4,0** | 3,8 |

- Kenar payını 1,5'ten 1'e indirmek yakın direk uzaklığını değiştirmiyor (4,0 / 3,8). Açıortay hedefi çoğu dar açılı göndermede zaten x 1,5'ten içeride değil; kırpma nadiren devreye giriyor. Sabit Bekçi denemesindeki (%8 → %1) kazanç maçta görünmüyor.
- Yakın direk uzaklığı zip50'ye göre arttı (2,8 → 4,0): bu kenar payından değil, araya giren turlardan (zip54–57) geliyor. Dar açılı göndermede Bekçi daha sık yerinden uzakta yakalanıyor.
- Kovalama dakika başına zip50'de 1,27, zip58'de 1,8 (eski kenarla 1,55): biraz arttı.
- Kovalama kazanma ve "yerinde" farkı (%55 ↔ %84, %50 ↔ %62) 8 maçta ~30 kovalama ve 26 sayıya dayanıyor: gürültü sınırında.

**Öneri:** Dar açıda Bekçi'nin gönderme anında nerede olduğu ve neden açıortayında olmadığı: Claude Code ikinci partide (senaryo 8–15) aynı izi çıkarır (yerinde değilse görevi ve hedefine uzaklığı).

## 6 · Sıradaki
İkinci parti: senaryo 8–15 (yaklaşık 25 dk). Sonra 16 maçlık tam tablo, açı × mesafe ve Bekçi izi.
