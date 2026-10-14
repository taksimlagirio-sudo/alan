# ALAN · Direk öncesi ölçüm (zip48, 120×60, direksiz)

Kaynak: 16 maç, herkes 20, ayarsız takımlar (zip46 + 120×60 kayıtları; zip48 motoru bunlarla birebir aynı).

## 1 · Gözlenen sayı (senaryo 0, Cansu, şarj %66, 1–0)
Kayıttan iz:
- Gönderme noktası (109, 1,5), (118, 4) değil. Ağzın ortasına göre açı 69°.
- Çekirdek arka çizgiye paralel kaymıyor, çapraz iniyor. Geliş açısı arka çizgiye dikle ~67°.
- Ağza kenarından giriyor: ağız ortasından 4,85 uzakta (ağız yarı genişliği 5). 0,15 birim payla sayı.
Yani "dar açı" değil, "kenarı sıyıran" bir sayı. Direk tam bunu yakalar.

## 2 · Sayıların gönderme açısına göre dağılımı
| Açı | Gönderme | Sayı | Sayı oranı | Sayıların payı |
|---|---|---|---|---|
| 0–30° | 52 | 11 | %21 | %22 |
| 30–60° | 86 | 25 | %29 | %50 |
| 60–75° | 33 | 10 | %30 | %20 |
| 75°+ | 26 | 4 | %15 | **%8** |
| Toplam | 197 | 50 | %25 | |

75°+ sayıların %8'i. 60–75° diliminin oranı (%30) en yüksekle aynı, dar açının asıl cömert olduğu yer burası.

### 2b · Açı ve mesafe birlikte (mesafe: ağız ortasına)
| Açı | Yakın (0–15) | Orta (15–25) | Uzak (25+) |
|---|---|---|---|
| 0–30° | 5/7 (%71) | 3/14 (%21) | 3/31 (%10) |
| 30–60° | 10/14 (%71) | 5/16 (%31) | 10/56 (%18) |
| 60–75° | 2/3 (%67) | 3/14 (%21) | **5/16 (%31)** |
| 75°+ | 1/5 (%20) | 2/12 (%17) | 1/9 (%11) |

- Karşı karşıya (yakın, düz) %71: zor değil, sorun yok.
- Uzaktan düz %10: makul.
- **Sorun bir hücrede: uzak ve geniş (25+, 60–75°) %31, uzaktan düzün üç katı.** Gözlenen Cansu sayısı da burada (30 birim, 69°). Hücrede 16 gönderme var, oran gürültülü; ama yön mantığa ters.
- Bekçi: Bu hücrede giren sayılarda Çekirdek Bekçi'nin ort. 2,1 birim yakınından geçiyor (uzaktan düz girenlerde 3,5). Bekçi yerinde ama durduramıyor. Adaylar: (1) ağız kenarını sıyırma, direk çözer; (2) Bekçi'nin yakın direkte uzanışı / tepkisi yetmiyor.
- **Ölçüt (kullanıcı):** Uzaktan geniş açı, uzaktan düzden kolay olmamalı; karşı karşıyadan çok daha zor olmalı.

## 3 · Ağza nereden giriyor (51 sayı)
- Geliş açısı (arka çizgiye dikle): 0–30° 18 · 30–60° 19 · 60–75° 11 · 75°+ 3.
- **Ağız kenarına 1 birimden yakın giren: 17 / 51 (üçte biri).** Geliş açıları 15°–81° arası dağınık; yalnız dar açıdan gelenler değil.

**Uyarı:** Direk yarıçapı sayı sayısını çok etkiler. Sayıların üçte biri ağzın son 1 biriminden giriyor; 1 birimlik direk dik gelenleri de keser ve sayıyı kabaca üçte bir düşürebilir. Sadece sıyırarak gelenleri kesmek için direk küçük olmalı (Çekirdeğin yarıçapı kadar ya da altı). Önce 2 yarıçap (ör. 0,3 ve 0,6) kısa denemede karşılaştırılsın.

## 4 · Ayrıca
`MOUTH = 5` mutlak: 100×50'de ağız sahanın %20'si, 120×60'ta %17'si. Saha büyüyünce ağız göreceli olarak küçüldü (sayı düşüşünün bir parçası olabilir). Bilinçli mi?

## 5 · "Herkes 20" tam değil
Kayıtlarda Alan gücü ve Soğukkanlılık 10, diğer bütün özellikler 20. Bu iki özellik `HEPSI_20` ayarına girmemiş görünüyor (sonradan eklendikleri için). Önceki ölçümler bozulmuyor (iki takımda ve her ölçümde aynı), ama 20 olmaları isteniyorsa ayar güncellenmeli.

## 6 · Direkten sonra Claude Code
Aynı 16 maç: 2 ve 2b tabloları, toplam sayı ve gönderme, direkten sekme sayısı ve direkten sekip giren sayı, 75°+ gönderme seçimi, hız. Uzak-geniş hücre hâlâ yüksekse: oradaki sayılarda Bekçi'nin neden yetişemediği (gerekirse daha çok maç).
