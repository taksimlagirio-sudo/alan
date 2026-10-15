# ALAN · zip71 sonucu ve "kafa = maç"

Claude Code'dan Design'a. Bu notu kullanıcıyla konuşarak yazdım; kullanıcının isteğiyle denemeyi ben yaptım. Motor değişikliği **siteye konmadı**; kendi kopyamda (`z72k`) ölçüldü. Kod farkı ekte: `kafa-mac.diff`.

## 1 · zip71 (hayaldeki Bekçi teması = `contact`)
Kullanıcının isteğiyle 14 maçta kestim; taban zip70.

| Gönderme: akıl / gerçek | zip70 | zip71 |
|---|---|---|
| Yakın (<15) | %46 / %38 | %40 / %33 |
| Orta (15–25) | %26 / %10 | %26 / %11 |
| Uzak (25+) | %11 / %3 | **%22 / %3** |

- Orta mesafedeki iyimserlik değişmedi; uzakta iki katına çıktı.
- Hız aynı makinede aynı (%0,4).
- Diğer satırlar gürültü sınırında. Gönderme / dk 10,9'dan 9,1'e düştü; Bekçi pas kovalaması dk'da 0,9'dan 1,5'e çıktı.

**Gönderme tablosu yeniden üretildi** (`tools/tablo-uret.js`, mevcut ızgara, 1120 hücre). Önce eski bayrakla üretildi: mevcut tabloyla birebir aynı, yani üreteç doğru. Yeni kuralla 715 hücre biraz değişti, ama mesafe ortalamaları neredeyse aynı (orta mesafe, ortadan %40,3 / %40,8). Dosya: `lab/tablo/yeni.json`.

**Önemli düzeltme:** Maçta akılın gönderme tahmini (`o.det.Pk`) tablodan gelmiyor. O an gerçek dünyayla `imagine` ile hesaplanıyor (`decide.js` ~357). Tablo yalnız konum değerlerinde (`shotQ`, ön süzme) kullanılıyor. Yani "tablodaki Bekçi kendi formülüyle yerleşiyor" açıklaması, kalibrasyondaki farkı açıklayamaz. Fark, kafadaki oynatmanın kendisinde.

## 2 · Kök: iki ayrı dünya
- Kafadaki uçuş (`simOne`) maçın kurallarını çağırmak yerine kendi içinde yeniden yazıyor. Örnekler:
  - Bekçi'nin hızı ve ataleti (`.17 + hiz × .006`, `× .18`);
  - kim kovalar (`chasePlan`; maçta `shape.flightRead` + Bekçi alan kuralı);
  - tepki süreleri birçok yerde ayrı ayrı yazılmış.
- Maçta kural değişince kopya geride kalıyor. Bekçi zip49'dan beri yedi sekiz kez değişti; her seferinde akılın kafası biraz daha ayrıştı.
- Tek tek kopya düzeltmek (zip71 gibi) işe yaramıyor: düzeltilen parça başka bir kopyayla etkileşiyor.

## 3 · Deneme: kafa = maç (gönderme)
**Gözlem:** Top havadayken maçta pahalı karar yok. Herkes yalnız yer alıyor, koşuyor; tek dokunuş ve önceden karar `_lite`'ta zaten kapalı. Yani uçuş maçın kendisiyle ucuza oynatılabilir: hafif bir tik 0,26 ms, normal maç tiki ~30 ms.

**`ucusMac`** (`decide.js`, `imagine` içinde `simOne` yerine):
1. Maçın kopyası alınır (`kopya`; olay listesi hariç). İç içe dünyalarda (bekleme kopyası, fake dünyası) kopya yerine `createMatch` + oyuncuların kopyası ve gerçek maçın taktiği kullanılır. Gerçek maça `match.js` adım başında `D._mac = m` ile erişilir.
2. Rakipler akılın gördüğü yere konur: algı hatası aynen korunur.
3. Çekirdek atılır; maç kendi adımıyla (`M.step`, `_lite`) tik tik ilerler:
   - yer alma, kovalama, Bekçi'nin yeri ve alan kuralı,
   - `contact`, gövdeler.
4. Biter: sayı olunca, Çekirdek birine geçince ya da süre dolunca. Süre dolarsa `simOne`'ın son değerlendirmesi kullanılır.
5. **Güvenlik:** Top yolu boşsa (Çekirdek Kuyu'da başlıyor gibi) ya da kopyada hata olursa eski `simOne` çalışır ve sayılır. 8 maçta hiç olmadı.

**Bayrak:** `ALAN_KAFA_ESKI` eski taklidi geri getirir. `ALAN_KAFA_PAS` aynı yolu paslara da açar (aşağıda).

**Sonuç** (8 maç, aynı senaryolar):

| Gönderme: akıl / gerçek | zip71 | kafa = maç |
|---|---|---|
| Yakın (<15) | %40 / %33 | **%46 / %43** |
| Orta (15–25) | %24 / %12 | **%18 / %10** |
| Uzak (25+) | %24 / %4 | **%8 / %1** |
| Toplam | %28 / %15 | **%22 / %16** |

| | zip71 | kafa = maç |
|---|---|---|
| Sayı / maç | 3,0 | 4,25 |
| Gönderme / dk | 9,9 | 13,8 |
| Bekçi pas kovalaması / dk | 1,56 | 0,81 |
| Yakın direkten sayı / dk | 0,75 | 0,94 |
| "Tut"tan sonra 1 sn'de kayıp % | 26 | 37 (izlenmeli) |
| Hız ms/tik | 64 | 82 (+%25–30) |

**Okuma:**
- Fark büyük ölçüde kapandı. Kalan 7–8 puan artık Bekçi ya da savunma taklidinden gelemez. Adaylar: algı hatası (bilerek var), kararla vuruş arasındaki birkaç tik, isabet hatası.
- Kullanıcının kararı: **bu kadar fark kalsın.** Görüş'ü düşük oyuncunun yanılması istenen bir şey.
- Akıl hangi göndermenin girdiğini bildiği için daha çok ve daha yakından gönderiyor; sayı arttı. Sayı düzeyi kullanıcının kararı.
- Hız: iki koşu aynı anda değildi ama benzer yük altındaydı. Bedel +%25–30.

## 4 · Paslar (`ALAN_KAFA_PAS`)
4 maç, senaryo 0–3.

| | zip71 | kafa = maç (gönderme) | + paslar |
|---|---|---|---|
| "Kesin" pas (P ≥ 0,99): akıl / gerçek | %99,4 / %94,9 | %99,4 / %94,0 | **%100 / %93,6** |
| P < 0,99 sınıfındaki pas sayısı | 92 | 81 | **2** |
| Pas tutma % | 92,1 | 89,8 | 92,7 |
| Hız ms/tik | 60 | 75 | **185** |

**Çalışmadı, iki sebeple:**
1. **Zar gürültüsü (v4'ün ilk dersi).**
   - Kafadaki maç her temasta zar atıyor: tuttu ya da tutmadı.
   - Pas denemesi 2 kez yapılıyor (`D.trials`); tahmin yalnız %0, %50 ya da %100 olabiliyor.
   - Akıl en iyi görüneni seçtiği için hep "iki denemede de tuttu" dediği pası seçiyor: 488 pasın 486'sı %100. Seçilenin şanslı çıkanı olması.
   - Eski `simOne` zar atmıyor, temasta beklenen değeri topluyor (`ctrlP`, `thruP`, sekme payı); bu yüzden 2 denemede bile düzgün oran veriyor.
   - Göndermede (6 deneme) aynı etki küçük ölçüde olabilir; kalan 7–8 puanlık iyimserliğin bir kısmı buradan gelebilir.
2. **Hız:** 3 kat. Pas kararları göndermeden çok daha sık.

(Kontrol: kafadaki pas denemelerinde top kime geçti, 150 tik: alıcı 647, rakip 963, başka arkadaş 111, pasçının kendisi 273. Kafa rakibi görüyor; sorun dünya değil, zar.)

**Paslar eski yolda kalmalı** (`ALAN_KAFA_PAS` kapalı, varsayılan).

**Yapısal çözüm önerisi (tek dünya + gürültüsüz):**
- `core.contact` ikiye ayrılsın:
  - (a) olasılıkları hesaplayan parça: tutma, delip geçme, sekme yönü ve payı;
  - (b) zarı atıp uygulayan parça.
- Maç ikisini kullanır. Kafadaki maç yalnız (a)'yı kullanır: dalları oranlarıyla toplar (temasta dallanır ya da beklenen değerle sürer), zar atmaz.
- Böylece kural tek yerde kalır, gürültü olmaz, az deneme yeter. Hız da buradan kurtulur.
- Ardından paslar yeniden denenir. Ölçüt:
  - P sınıflarında akıl ile gerçek arasındaki fark ±%3;
  - hız zip71'in ≤ +%30'u.

## 5 · Öneri: tek ilke
**Akıl kafasında ayrı bir dünya yazmaz, maçın kendisini çağırır.** Üç durum:
1. **Top serbestken** (pas, gönderme, aşırtma, kenardan, seken): maçın kopyası, `_lite`. Bu not bunun ilk adımı.
2. **Top birindeyken:** maçın kopyası, içindeki kararlar dondurulmuş (`holdWorld` zaten böyle). Hamle sonrası dünya (`predictWorld`) da ileride buraya taşınabilir.
3. **Topsuz kararlar** (pres, markaj, Bekçi çıkışı): maçta kafada oynatma yok; v4 tuzağı. Kural kalır, ama kullandığı parçalar (varış süresi, tepki, yer alma) maçın aynı fonksiyonlarından gelir. Kopyalı ölçüm çevrimdışı öğretmen olarak kalır.

**Hız:** Profil, kafadaki maçın zamanının çoğunun kopyalamada değil `shape.position`'da (her tikte herkesin yeri, ~%20) ve `step`'te geçtiğini gösteriyor. Hızlandırma orada yapılırsa hem maç hem kafa hızlanır. Yeni bir taklit yazmak çözüm değil.

**Güvenlik ağı:** İkiz ölçümü (`tools/gonder-ikiz.js`) her pakette kısa bir test olarak çalışabilir. Aynı anı kafada ve maçta oynatır; fark artarsa ilk pakette yakalanır.

**Temizlik:** Bu yol kalıcılaşırsa `simOne`'ın gönderme kolu, `ALAN_BEK_HAYAL_ESKI` ve gönderme tablosunun ayrı Bekçi kurulumu silinebilir.
