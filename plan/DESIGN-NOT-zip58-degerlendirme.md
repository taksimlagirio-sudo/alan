# ALAN · zip51–58 değerlendirmesi: ne işe yaradı, ne yanlış gitti, sıradaki üç öneri

Claude Code'dan Design'a. Kaynak: 16 maç zip58 (120 sn, devre açık) ile aynı 16 senaryo zip50 (90 sn). Ayrıntılı ölçümler: DESIGN-NOT-zip51-58.md, DESIGN-NOT-zip58-uc-sikayet.md. Kullanıcı önerilerin üçünü de iyi buldu ve Design'a verilmesini istedi.

---

## 1 · Kontrol tablosu (yeni)

Bundan sonra her zip aynı 16 maçta oynatılacak ve bugüne kadar bulunan bütün sorunlar aynı ölçülerle bir önceki sürümle yan yana konacak (`tools/kontrol-kos.sh`, `tools/KONTROL-TABLOSU.md`). Sayımlar dakika başına; ↑ / ↓ iyi yön; %15'ten çok kötüleşen ⚠, iyileşen ✓.

| Ölçü | zip50 (90 sn) | zip58 (120 sn) |
|---|---|---|
| Sayı / maç | 2,19 | 3,81 |
| Sayı / dk | 1,46 | 1,91 |
| Gönderme / dk | 9,0 | 10,3 |
| Gönderme başına sayı % | 15,7 | 15,8 |
| Pas / dk | 57,2 | 58,1 |
| Pas tutma % ↑ | 91,0 | 90,9 |
| Kesme / dk | 17,6 | 19,6 |
| Hesap ms/tik ↓ | 29,0 | 33,6 ⚠ |
| Saha dışında Çekirdek (tik) ↓ | 24 | 0 ✓ |
| Bekçi pas kovalaması / dk ↓ | 1,00 | 1,56 ⚠ |
| Bekçi kovaladığını kazanıyor % ↑ | 75 | 64 |
| Sayılarda Bekçi yerinde % ↑ | 71 | 60 ⚠ |
| Yakın direkten sayı / dk ↓ | 0,63 | 0,78 ⚠ |
| Dar açı (60°+) gönderme başına sayı % ↓ | 16,7 | 11,8 ✓ |
| Dar açıda Bekçi–yakın direk ↓ | 2,85 | 3,79 ⚠ |
| Uzak geniş (25+, 60–75°) sayı % (n) ↓ | 0 (17) | 0 (24) |
| Yakın geniş (<15, 60–75°) sayı % (n) ↓ | 67 (6) | 75 (8) |
| Kontra: önü boşken 12+ tik topla duran / dk ↓ | 0,17 | 0,13 ✓ |
| Pastan sonra geri koşan pasçı % ↓ | 21,0 | 18,8 |
| Baskı altında "tut" seçimi % ↓ | 21,0 | 19,8 |
| "Tut"tan sonra 1 sn'de kayıp % ↓ | 11,9 | 31,6 ⚠ |
| Boş hücumcu (rakip yarısı, 8 br) zamanın % ↓ | 50,9 | 47,0 |
| Boş adam + çift adam anı / dk ↓ | 12,9 | 10,2 ✓ |
| bunlarda iki kişi "arkada" / dk ↓ | 3,75 | 1,50 ✓ |
| Taşıyıcıya baskı yok (rakip yarısı) % ↓ | 18,1 | 13,6 ✓ |
| Her an duran hücumcu ↓ | 0,37 | 0,21 ✓ |
| Uzun tutuşta 1 sn+ duran hücumcu ↓ | 0,84 | 0,23 ✓ |

---

## 2 · İşe yarayanlar
- **Çekirdek saha içinde (zip52):** saha dışında Çekirdek 24 tik → 0. Arka çizginin dibinden gönderilip giren sayı yok.
- **Durma:** her an duran hücumcu 0,37 → 0,21; uzun tutuşta 1 sn+ duran 0,84 → 0,23. Büyük iyileşme (büyük ihtimalle zip54 bekleme ve zip56 plan silme birlikte).
- **Savunma (zip55):** iki kişinin aynı adamın arkasında durduğu an yarıdan az (3,75 → 1,50 / dk); rakip yarısında baskısız taşıyıcı %18 → %14.
- **Ver-koş (zip56):** pastan sonra geri koşan pasçı %21 → %19. Küçük.
- **Uzaktan geniş açı:** sorun yok (0 / 24).
- **Devre arası ve 120 sn:** sorunsuz; devre sonrası santranın belirgin etkisi yok.

---

## 3 · Yanlış gidenler (açıkça)

**3.1 · Yedi tur ölçülmeden üst üste kondu.** zip51'den zip58'e her tur bir öncekinin üstüne kuruldu; arada hiçbiri maçta ölçülmedi. Sonuç: zip50'nin düzelttiği Bekçi kovalaması geri geldi (1,0 → 1,56 / dk; kazanma %75 → %64) ve hangi turun getirdiği bilinmiyor. Bundan sonra: **her tur tek değişiklik, kontrol tablosu geçmeden bir sonraki tur yok.**

**3.2 · zip58'in teşhisi sabit bir denemeden yapıldı, maçta tutmadı.** "Bekçi 1,5 kenar payı yüzünden köşeyi kapatamıyor" teşhisi, Bekçi'nin sabit tutulduğu 300 vuruşluk denemeden çıktı (%8 → %1). Maçta:
- Yakın direkten sayı 0,63 → 0,78 / dk (arttı).
- Aynı 8 maç `ALAN_BEKCI_ESKI` ile: yakın direkten sayı 13 → 7 (eski kenarla daha az), Bekçi–yakın direk 4,0 / 3,8.
- Sebep: açıortay hedefi dar açılı göndermelerin çoğunda zaten x 1,5'ten içeride değil; kırpma nadiren devreye giriyor. Köşeden giren sayılarda Bekçi gönderme anında **hedefinden 8–18 birim uzakta** (57°+ olan 9 sayının 7'sinde); çoğunda az önce bir pası kovalamış. Yani kullanıcının gördüğü sayılar Bekçi'nin duruşundan değil, yerinde olmamasından.
- Ders: tek anın yeniden kurulumu ya da sabit deneme teşhis için iyi, ama düzeltme maçta ölçülmeden "çözüldü" denmemeli.

**3.3 · zip57'nin kendi sınaması tutmadı, yine de gönderildi.** Günlükte "test aracındaki karar değerleri maçtaki kayda uymadı … bu anın kanıtı değil" yazıyor. Ölçüm: baskı altında "tut" seçimi aynı (%21 → %20), ama tuttuktan sonra kayıp **%12 → %32**. Kopya beklemenin riskini gösteriyor, karar buna göre değişmiyor. Bu ya kopyanın iyimser olması ya da kayıp değerinin (`−lossAt`) seçimi değiştirmeye yetmemesi demek; hangisi olduğu bilinmeden kural doğru sayılamaz.

**3.4 · "Kontrada bekleme" tek bir andan teşhis edildi.** Kural (zip54) mantıklı (akıl kafada ayrı taklit yazmasın), ama ölçümde kontrada topla duran zaten seyrek: 0,17 → 0,13 / dk. Bedeli hesap süresinde: **+%16** (29,0 → 33,6 ms/tik, kopya oynatması). Seyrek bir durum için pahalı; maliyet ölçülmeli ve gerekirse kopya yalnız gerektiğinde oynatılmalı.

**3.5 · Tahmin edilen etkiler doğrulanmadı.** zip51 "sayı ~2,1 → ~2,8 / maç" bekliyordu; çıkan 3,81. Dakika başına sayı da arttı (1,46 → 1,91): artış yalnız süreden değil, diğer turlardan. Bu artışın kaynağı bilinmiyor.

**3.6 · zip49'daki iddia.** "Direk 0 = zip48 ile birebir" denmişti; gönderme tablosu sürüm adı yüzünden tutmuyordu (zip49b'de düzeltildi). İddia sınanmadan yazılmamalı.

**3.7 · Kural yerine istisna birikiyor.** Bekçi için: kovala ama takımda ilk sen varacaksan → ama rakipten önce varacaksan → kaleye geliyorsa pay → kenar payı 1 ama eski bayrakta 1,5… Her gözlem yeni bir istisna ve yeni bir bayrak (şu an en az 8 `ALAN_*_ESKI` bayrağı). İstisnalar birbirini bozuyor (3.1) ve kullanıcının "anlaşılır, gizli katsayı yok" ilkesine aykırı. Aşağıdaki üç öneri bu yüzden kuralı **sadeleştiriyor**, yeni istisna eklemiyor.

**Açık kalanlar:** "Herkes 20" ayarında Alan gücü ve Soğukkanlılık hâlâ 10 (HEPSI_20 bu ikisini kapsamıyor).

---

## 4 · Öneri A · Bekçi: Kuyu alanı Bekçi'nin bölgesi

**Sorun (ölçümle):** Bekçi dakikada 1,56 kez pas kovalıyor, %36'sını kaybediyor; sayıların %40'ında yerinde değil; köşeden giren sayılarda hedefinden 8–18 birim uzakta. Ortadan yakından (9–17 birim) giren sayılarda ise yerinde, ama top 4–5 tikte 1,4–2,6 birim yanından geçiyor; tepki süresi (`bekReact` 8 − Okuma payı) bu yoldan uzun, yalnız gövdesi (0,9) kapatıyor.

**Kural (tek cümle):** Kuyu alanı (yarıçap 9) Bekçi'nin bölgesidir; alanın içindeki her topa gider, dışındakine yalnız serbest topa ve rakipten belirgin önce varacaksa.
1. **Alan içi:** Bugünkü gibi; alanına giren top, pas ya da sekme onun.
2. **Alan dışı:** Uçan bir pası araya girip kesmek için alanını bırakmaz. Yalnız yerde serbest kalan (kimsenin olmayan) topa ve zip50'nin payıyla (Yerleşim 20'de 2 tik, 10'da 8 tik) rakipten önce varacaksa çıkar. `kaleye` payı kaldırılır (zaten alan içi kuralı kapsıyor).
3. **Dönüş:** Alanının dışındaysa ya da hedefinden 3 birimden uzaktaysa önce **ağızla top arasındaki en kısa noktaya** (topu görerek kaleye) koşar, sonra açıortaya yerleşir. Bugün doğrudan açıortay hedefine gidiyor; dönerken yakın direk açık kalıyor.
4. **Yerleşmiş Bekçi:** Hızı düşük (ör. < 0,05) ve hedefine 1 birimden yakınsa tepki süresi kısalır (ör. `bekReact` yerine `bekReact × 0,6`); koşarken bugünkü gibi. Yer tutmanın değeri olur, Yerleşim ve Okuma anlam kazanır. Katsayı oyuncuya ekranda gösterilebilir ("yerleşmiş: tepki hızlı").

**Kabul ölçütü (kontrol tablosu):** Bekçi pas kovalaması ≤ 1,0 / dk ve kazanma ≥ %75; sayılarda Bekçi yerinde ≥ %75; yakın direkten sayı ≤ 0,5 / dk; Bekçi–yakın direk ≤ 2,5; yakın geniş (<15, 60–75°) sayı oranı yakın düz (<15, 0–30°) oranından yüksek değil.

**Ayrıca:** zip51–57'den hangisinin kovalamayı geri getirdiğini Claude Code bayraklarla 8'er maçta ayırabilir (`ALAN_SAV_ESKI`, `ALAN_PLAN_ESKI`, `ALAN_BASKI_ESKI`, `ALAN_BEKLE_TAKLIT`). Öneri A uygulanacaksa buna gerek kalmayabilir; Design karar versin.

---

## 5 · Öneri B · Savunma: görevler tek elden, "her rakibe bir kişi, fazlası boşluk"

**Sorun (ölçümle):** Rakip yarısında zamanın %47'sinde 8 birim içinde savunmacısı olmayan bir hücumcu var; dakikada 10,2 örnek anda biri boşken başka bir hücumcunun 4 birim içinde iki savunmacı var. Bu anlarda savunmacıların görevleri: arkada + kapatma 51 · bölge + kapatma 49 · arkada + arkada 47 · arkada + bölge 35 · bölge + bölge 23 · kapatma + kapatma 23. Boş adama en yakın savunmacının görevi: bölge 146 · kapatma 99 · arkada 49. Yani aynı rakip iki ayrı görevin hedefi olabiliyor: her savunmacı görevini ayrı seçiyor, sonra ortak atama (`fazla`) yalnız bazı çakışmaları düzeltiyor.

**Kural:**
1. Her karar tikinde rakipler tehlikesine göre sıralanır (taşıyıcı ilk; sonra `threat` × hat açıklığı).
2. Sırayla her rakibe, ona en çabuk varabilecek **boştaki** savunmacı atanır (maliyet: varış süresi). Taşıyıcıya atanan presçidir; ikinci presçi ancak bütün tehlikeli rakiplere birer kişi atandıktan sonra.
3. Bir rakibe yalnız bir kişi. Atanmayan savunmacılar boşluk tutar (bölgesindeki en tehlikeli boşluk) ya da arkada güvence olur.
4. Taktik ayarları maliyete girer: adam adama → atama kalıcı (önceki eşleşme daha iyisi büyük farkla çıkmadıkça değişmez); bölge → savunmacı yalnız bölgesindeki rakiplere atanabilir; pres şiddeti → ikinci presçinin eşiği.
5. Salınımı önlemek için atama değişimine eşik (önceki eşleşmeyi koru, yeni eşleşme belirgin daha iyiyse değiş).

Bu, bugünkü ayrı görev seçimini ve ardından gelen düzeltmeyi (`fazla`) tek adımla değiştirir. Menajere tek cümleyle anlatılır ve tahtada çizgilerle gösterilebilir (kim kimi tutuyor).

**Kabul ölçütü:** boş adam + çift adam anı ≤ 5 / dk; boş hücumcu zamanın ≤ %40; iki kişi "arkada" ≈ 0; taşıyıcıya baskı yok ≤ %14; top kazanma ve yenen sayı kötüleşmesin; salınım (hedef değişimi / dk) artmasın.

---

## 6 · Öneri C · Kontra ve bekleme

**Sorun (ölçümle):**
- Baskı altında "tut" seçimi %20, tuttuktan sonra 1 sn'de kayıp %32 (zip50'de %12). Kopya beklemenin riskini göstermeli; seçim değişmiyor.
- Durduğu anlarda seçenek değerleri birbirine çok yakın ("tut" 0,075 · "tut 0,5 sn bekle" 0,074 · pas 0,073): akıl ayırt edemiyor, yazı-tura.
- Geçiş "git" kararı varsayılanla neredeyse hiç tetiklenmiyor (zip42 ölçümü: "önde sayıca üstünsek git" kuralı yüzünden).

**Öneri:**
1. **Önce ölç, sonra düzelt:** Claude Code baskı altındaki "tut" anlarında aklın kopyada gördüğü kayıp olasılığını gerçekte olanla karşılaştırır (kalibrasyon). Bunun için karar kaydında "tut" seçeneğinin kopyadaki sonucu (kayıp mı, değer ne) gerekli; Design karar mesajına ekleyebilir. Kopya iyimserse sebebi kopyada (ör. rakibin karşı presi kopyada var mı, `m.kp` kopyalanıyor mu); kalibre ise `−lossAt` ağırlığı.
2. **Geçişte zamanın bedeli:** Top kazanıldıktan sonraki ilk 2–3 sn'de her bekleyiş, rakibin yerleşmesi kadar değer kaybettirir. Kopya bunu kendisi göstermeli (rakip geri koşar); göstermiyorsa 1. maddedeki fark bunu ortaya çıkarır. Ayrı bir katsayı eklenmesin; önce kopyanın doğruluğu.
3. **Varsayılan geçiş "eşitsek de git":** Bugünkü varsayılan ("önde sayıca üstünsek git") kontrayı neredeyse kapatıyor. Bu bir taktik varsayılanı, kullanıcının kararı; önerimiz değiştirmek.
4. **Hesap maliyeti:** zip54'ün kopya oynatması +%16. Bekleme seçeneği değerlerin birbirine çok yakın olduğu (ya da baskı olan) anlarda oynatılsın; açıkça kötü olduğu anlarda (önü bomboş kontra) oynatılmasın.

**Kabul ölçütü:** "Tut"tan sonra kayıp ≤ %15; baskı altında "tut" seçimi kayıpla birlikte düşsün; kontrada önü boşken duran ≤ 0,1 / dk; geçiş "git" süresi artsın; hesap ≤ zip50 + %5.

---

## 7 · Çalışma düzeni önerisi
1. Her tur **tek** değişiklik ve bayrağı. Günlüğe "Claude Code ölçecek" yerine artık "kontrol tablosunda beklenen değişim: şu satır şu yöne" yazılsın.
2. Claude Code her zip'te kontrol tablosunu çıkarır (16 maç, ~50 dk). ⚠ olan satır varsa bir sonraki tura geçilmeden bakılır.
3. Sabit deneme ve tek anın yeniden kurulumu teşhis için; "çözüldü" ancak tabloda.
4. Sıra: **A (Bekçi) → B (savunma) → C (kontra)**. A'nın tablosu geçtikten sonra B.
