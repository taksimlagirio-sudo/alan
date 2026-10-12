# ALAN v5 · Claude Code görevi: aklın pas hesabını maçla aynı kurala getirmek

Klasör: `alan-v5/` (motor) · sayfalar: `ALAN v5 Taktik.dc.html`, `ALAN v5 Canlı.dc.html`, `ALAN v5 İkili Sahne.dc.html` (web sunucusundan açılır).

## Sorun
Aklın pas değerlendirmesi (`decide.js` → `imagine` → `simOne`) gerçek maçla iki yerde uyuşmuyor:

1. **Rakibin uzanması.** Maçta (`core.js` `oppReach`) rakibin menzili Çekirdeğin yeni yolu (vuruş, duvar, sekme) başladıktan sonra tepkiyle büyür. Hayalde (`firstTouch`) rakip baştan tam menzille uzanıyor. Sonuç: duvardan dönen ya da yakından geçen paslar hayalde olduğundan sık kesiliyor.
   - Denendi: `firstTouch`'a `oppReach` eklemek → pas tutma %85 → %77, sayı 90 sn'de 1,8 → 3,9 (kısa ölçüm). Yani hayal bu sefer iyimser oldu. Bunun sebebi ölçülmeli: belki başka bir yer (kovalama planı, `chasePlan` tepkisi) zaten bunu telafi ediyordu.
2. **Kendine pas (kenardan çalım · kendine sektirdi).** `opts` içinde `kendine: true`, `q: h`, `lead: T`. Hayalde taşıyıcının kopyası `recvPoint` ile "en erken alma noktası"nı arıyor; Çekirdek kendi ayağından başladığı için hemen orası çıkıyor ve kopya koşmuyor.
   - Denendi: kopya `lead`'e koşsun → akıl %15 yerine %59 tahmin etti, gerçek %8 (seçilen buluşma noktası değişti). Geri alındı.

Ölçüm (rakip 7 birim önde, hizada, 25 deneme): zorla kendine pas gerçekte %64 tutuyor, akıl %15 sanıyor. 9 birimde gerçek %20, akıl %14 (doğru).

## Görev
1. **Kalibrasyon ölçümü:** Normal maçlarda (en az 30 maç, karışık taktik) atılan her pas için aklın tahmini (`o.det.P`) ile gerçek sonucu (pas sahibinin takımında kaldı mı) kaydet. Pas türüne (pas, önüne, kenardan, kendine), mesafeye ve en yakın rakibin uzaklığına göre dilimle. Hangi dilimde ne kadar iyimser/kötümser?
2. **Uzanma kuralını hayale taşı** (`firstTouch` → `oppReach` ile aynı kural). Kalibrasyonu tekrarla. Hayal iyimserleşirse sebebini bul (kovalama planındaki tepki, `chasePlan` `rc`, drift) ve iki tarafı birlikte maçla aynı yap. Kural yazma, katsayıyla gizleme yok.
3. **Kendine pas:** kopyanın buluşma noktasına koşması, Çekirdeğin çekirdek konumundan başlaması, buluşma noktası seçimi. Gerçek ile tahmin uyuşana kadar.
4. Sonuç: maçta pas tutma, sayı, kendine pas sıklığı ve başarısı (İkili Sahne'deki kenar senaryoları: `kenarDuz`, `kenarUst`, `kose`, `ornD`, `ornK`, `ornB`).

## Son eklenenler (bilgi)
- İkili mücadele fiziksel hamle: hazırlık (Çeviklik), uzanma ya da omuz, vuruş anı geometrisi; adanma (`adandi`, `adT`)
- Yanılma tek mekanik: fake'e aldanan ve geçilen aynı (`yanil`), Okuma'ya bağlı fark etme süresi
- Yön değişimine tepki: maçta rakipler yeni yolu tepki süresi dolunca görür (`match.js` step); hayalde de kovalama planı ilk dönüşten önceki yola göre (`simOne` `bi`, `onceki`)
- Çekirdek katı: dönerken rakip gövdesine çarpar (fiske seker), rakip Çekirdeğin içine giremez
- Kenardan çalım: rakip duvar hattını kapatmıyorsa denenmez; iki adımlı dolaşma; duvar yolları gövdeden geçmemeli; kendine pas seçeneği
- Ekran saati: 30 tik = 1 maç saniyesi (motor değişmedi)
