# ALAN v5 · Claude Code sonucu: aklın pas hesabı maçla aynı kurala getirildi

Görev: `CLAUDE-CODE-GOREV.md`. Değişen dosyalar: `decide.js`, `match.js`, `shape.js` (bu klasördekiler). `core.js` değişmedi.
Ölçüm araçları ve ham veriler: GitHub `taksimlagirio-sudo/alan`, `arsiv` dalı, `tools/v5-kalib*.js`, `tools/v5-kendine.js`, `lab/kalib`, `lab/sonuc`, `lab/sahne`.

## 1 · Kalibrasyon (başlangıç, zip 19 olduğu gibi)
36 maç, 2068 pas, karışık taktik. Akıl iyimserdi: tahmin %92, gerçek %80 (+12). Pas türünde: `pas` +2, `önüne` +14, `kenardan` +14. Uzun paslarda (+18) ve pası atanın yakınında rakip yokken (+19) en büyük.
İyimserliği ikiye ayırdım: pasın anı kopyalanıp (a) aynı hamle aklın kendi hesabıyla 30 denemeyle, (b) gerçek maç kurallarıyla 12–20 kez farklı zarla oynatıldı.
- Seçme yanılgısı (2 denemeyle tartılıp en iyisinin seçilmesi): +6
- Aklın hesabının hatası: +9 (kenardan +15, rakip uzaktayken +18; baskı altında +1)
Yani hayal, pasın atıldığı yerde değil, uzakta (alıcı tarafı, uzaktan gelen rakip) yanılıyordu.

## 2 · Bulunan uyuşmazlıklar ve düzeltmeler (hepsi maçtaki kuralın aynısı; yeni kural ya da katsayı yok)
1. **Rakip tepkisi:** Hayalde rakipler pasa `14 − Okuma×0,5 + (20 − pas atanın Okuma'sı)×0,4` tikte tepki veriyordu (Okuma 10: 9 + 4). Maçta 9. Fazladan gecikme (`laneLag`) sıfırlandı. Tek başına bu, aklın hatasını +9'dan +3'e indirdi.
2. **Uzanma** (`firstTouch`): maçtaki `oppReach` (yeni yol başlayınca tepkiyle büyür, gövde yarıçapı 1,15'in altına inmez). Tek başına eklenince hayal iyimserleşiyordu (+8): eski tam menzil, aşağıdaki eksikleri örtüyordu. Aşağıdakilerle birlikte doğru.
3. **Alıcının karşılama noktası:** Maç `shape.flightRead`'de "yetişebildiği yerlerin en rahatı"nı seçiyor; hayal `recvPoint` ile en erken noktayı seçiyordu. Kural `AlanShape.alici` olarak dışarı alındı; maç ve hayal aynı fonksiyonu kullanıyor (maç sonucu birebir aynı kaldı, doğrulandı).
4. **Alıcı yeniden bakar:** Maçta alıcı 3 tikte bir yeniden seçer, seçtiği an geçtiyse Çekirdeğin peşine düşer. Hayal bir kez seçip sabit kalıyordu. Kendine pasta ilk seçilen nokta Çekirdeğin çıktığı yer olduğundan kopya hiç koşmuyordu: "akıl %15, gerçek %64"ün asıl sebebi.
5. **Çekirdeğin çıktığı yer:** Maçta gövdenin kenarı (`corePos`), hayalde oyuncunun merkeziydi. Hayal de kenardan başlatıyor.
6. **Hareket kuralı:** Hayalde oyuncular sabit hızla (ya da o anki hızla) gidiyordu. Artık maçın kuralı: hedefe doğru, maçın ivme fonksiyonuyla (`AlanMatch.ivme`, Çeviklik dahil). Pası görmeyen rakip maçtaki gibi eski hedefine koşmayı sürdürür (iz kaydında presçi taşıyıcıya koşarken duvardan seken Çekirdek arkasından geçiyordu; hayalde ise yolun üstünde bekleyip kesiyordu).
7. **Kovalama yenilenir:** Maçta kovalayanlar 3 tikte bir yolun son hâline göre hedef seçer; hayal ilk anda ve ilk dönüşte seçiyordu. Artık 3 tikte bir; yol dönünce (duvar, gövde) yeni yolu tepki süresi dolunca görürler. Tepkisini vermiş oyuncu yeniden beklemez.
8. **Katı gövdeler:** Maçtaki ayırma kodu `AlanMatch.govdeAyir` olarak dışarı alındı (maç birebir aynı, doğrulandı); hayal her tik aynı fonksiyonu çağırıyor. Kenarda rakibin yanından geçen taşıyıcının yavaşlaması buradan geliyor; "rakip duvar tarafında" sahnesindeki iyimserliği (%80'e karşı gerçek %20) bu kapattı.

Bulunan iki hata (kuralı değiştirmeyen):
- **Önbellek bayatlığı** (`decide.space`, `decide.front`): önbellek dizi nesnesine bağlı ve tik kontrolü yok. Karar `m.ps` ile çağrıldığı için bir noktanın ilk hesaplanan boşluk değeri maç boyunca kalıyordu (oyuncular yer değiştirse de). Tik kontrolü eklendi (v4'te vardı). Belirlenimciliği de bozuyordu: kopyadan oynatılan maç yeniden oynatılanla aynı çıkmıyordu.
- **Çift hareket** (`simOne`): kovalamaya sonradan katılan oyuncu hem "kendi hedefine" listesinde hem kovalama listesinde yürütülüyordu; iki hedef arasında kalıp duruyordu. Bekçi de aynı.

## 3 · Sonuç
Aklın hesabının hatası (kopyalarla, 12 maç): +9 → **−4** (her dilim ±10 içinde; baskı altında hafif kötümser −10).
Maçta tahmin ile gerçek (16 maç, bütün paslar): +12 → **+1**.

| | zip 19 | yeni |
|---|---|---|
| sayı / maç (90 sn) | 1,69 | **2,63** |
| 0–0 | 6 / 16 | 1 / 16 |
| pas tutma | %78 | **%90** |
| pas / maç | 54 | 41 |
| top el değiştirme / maç | 17 | 10 |
| kendine pas / 16 maç | 1 | 10 (gerçek %80, akıl %93) |

Pas sayısı azaldı: akıl riskli pası artık doğru görüyor.

## 4 · İkili Sahne kenar senaryoları (zorla kendine pas, 25 deneme)
| sahne | akıl önce | akıl şimdi | gerçek | şimdi kendisi seçiyor mu |
|---|---|---|---|---|
| kenarIc (rakip içeriden) | %1 | %99 | %100 | 25/25, hepsi başarılı |
| rakip 7 birim önde, hizada | %8 | %60 | %80 | 9/25, %78 |
| kenarUst (rakip duvar tarafında) | %14 | %24 | %36 | 1/25 |
| ornK | %13 | %25 | %40 | 2/25 |
| rakip 9 birim önde | %11 | %15 | %24 | 0/25 |
| kenarOn | %34 (3 örnek) | %81 | %67 | 1/25 |

`kenarDuz`, `kose`, `ornD`, `ornB`: kendine pas adayı üretilmiyor (aday kuralı: koşu çizgisi ya da Çekirdeğin yolu bir gövdeye yakın). Bu sahnelerin örnek hamlesi "kenardan çalım" (ayrı hamle türü).

## 5 · Açık kalan
- Kendine pasta hafif kötümserlik (10–20 puan) kalıyor; maçta 10 örnekte hafif iyimser. Az örnek.
- Seçme yanılgısı +5 (iki denemeyle en iyiyi seçmek). Kural değil, deneme sayısı meselesi; ele alınmadı.
- Gönderme tablosu (`shot-table.js`) eski hesapla üretildi; Çekirdeğin çıkış noktası değiştiği için `SHT_SURUM` yükseltilip yeniden üretilmesi gerekebilir (ölçülmedi).
- Pas denemesinde yalnız rakipler, alıcı ve Bekçi hareket ediyor; öteki arkadaşlar yerinde kalıyor (maçta yerleşime göre kayarlar).
