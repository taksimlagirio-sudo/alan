# ALAN v5 · Motor değişiklik günlüğü

> **15 Ekim 2026'dan itibaren motoru Claude Code yönetir** (bkz. `CALISMA-DUZENI.md`). Aşağıdaki girdiler zip71'den sonrası; ayrıntı ve ölçümler `DESIGN-NOT-zip78-sitede.md`'de.

## 19 Ekim 2026 · Bekçi yalnız savunmacının yetişemeyeceği topa (main)
- Menajer (218269 0:23): "Aras'ın burada çıkmasına ne gerek var?" Deniz topa aynı anda varıyordu.
- `ALAN_BEKCI_CIK_ESKI` altında: Bekçi ancak takımında açık ara (4 tik) ilk o varacaksa çıkar.

## 19 Ekim 2026 · Bekçi'nin çıkış adımı (main)
- Menajer: önüne atılan topa çıkan Bekçi'nin yetişebilmesi.
- `ALAN_BEKCI_ADIM_ESKI`: Bekçi topa Refleks'le tepki verir (saha oyuncusu gecikmesi yerine); ilk adımı (fren, yön değiştirme) iki kat sert; Bekçi çıkınca yakındaki savunmacı topa gitmeyi bırakmaz.
- 218269 t696: Aras 4 tik erken çıkıyor, 2 br daha yakın; Yıldırım yine bir adım önce. Tek maçta birebir. Ölçüm yapılmadı (menajer isteği).

## 19 Ekim 2026 · Bekçi aktif (main)
- Menajer: "Bekçi'ye pas bir seçenek olmalı, ileri açılmayı düşünebilmeli, rakibin önüne atılan topa yetişebilecekse açılmalı."
- `ALAN_BEKCI_CIK_ESKI`: rakibin önüne atılan pasa (buluşma kaleye 22 br içinde) rakiple aynı anda varabiliyorsa çıkar.
- `ALAN_BEKCI_PAS_ESKI`: geri pas seçeneği (kendi yarı sahasından, en çok 40 br).
- `ALAN_BEKCI_ILERI_ESKI`: top uzaktayken savunma hattının 5 br gerisine kadar açılır (süpürücü).
- `ALAN_BEKCI_DONUS_ESKI`: çok açılmış Bekçi dönüşte orada takılı kalmaz.
- 6 maç: Bekçi'nin kaleye uzaklığı 8,7 → 14–21 br; geri pas 0 → 0–11/maç; çıkış 0 → 6 (Bekçi kendisi hiç almadı). Tek maçta birebir.

## 19 Ekim 2026 · İç omuz (main)
- Menajer (218269 0:31): "Alp Ece ile kenar arasına daha yakın; marke ederken dahi Ece'ye göre kaleye hiza olarak daha yakın durmalı." Sonra: "Kenar kaçışını önemsememek mantıklı mı, başka pozisyonları bozmaz mı?" Kural yalnız dışa kaçışa daraltıldı.
- `ALAN_IC_OMUZ_ESKI`: arkadaki adamın koşusunu kaleye doğru ve içeri doğru okur; dışa kaçışı okumaz (kaleye 25 br içinde yarısını).
- 6 an: dış tarafta durma belirgin azaldı, kale tarafı mesafesi aynı, eşleşme değişimi toplamda aynı. Tek maçta birebir.

## 19 Ekim 2026 · Şut menzilinde şut çizgisi (main)
- Menajer (Sahne Masası, 218269 0:06 ve 0:32): "Alp hem Kaan'ın hem Ece'nin golünde kaleyle açılarını hiç doğru şekilde kapatmaya çalışmıyor; illa başarması gerekmez ama doğrusunu denemesi gerekir."
- `ALAN_KALE_ACI_ESKI`:
  - Kaleye 35 br içinde ilk presçi şut çizgisinin 1,5 br içine girer.
  - İlk presçi kale tarafındaki noktaya en çabuk varacak olandır.
  - Rakip topa önce varacaksa kovalayan ve buluşmaya giden rakibin alacağı yerin kale tarafına koşar.
- Sahne: Ece'nin golünde Alp çizgide; Kaan'ınkinde çizgiye giriyor, yetişemiyor. Tek maçta birebir.
- Sahne Masası: şu anki kurallarla 7 an daha (iki kurulumun ilk yarıları).

## 19 Ekim 2026 · Arkadaki blok çizgisinden öne çıkmaz (main)
- Menajer: "Cansu ve Deniz'in rakibi karşılamak adına rakip yarı sahaya geçmesi ne kadar mantıklı? Arkada alan bırakıyorlar."
- `ALAN_ARKA_BLOK_ESKI`: 'arkada · X' hedefi dizilişteki yerinden (blok yüksekliği) öne çıkmaz; rakip çizgiyi geçene kadar kale ile arasındaki çizgide, kendi hizasında bekler.
- Sahne: Liman alçak bloğa dönüyor, öndekiler basıyor; top 1:49'da geri kazanıldı. Tek maçta birebir.

## 19 Ekim 2026 · Boş topun beklenen sahibi (main)
- Menajer: "Sadece Kaan'a vermek saçma, takım genelinde olmalı; diğer takım da topun kendisinde olacağının, Alp'in topu nerede alacağının farkında olmalı."
- `ALAN_BEKLENEN_SAHIP_ESKI`: kalesine 25 br yakın boş topta şekil değişimini durduran kural, kazanan belirgin önceyse (12+ tik) uygulanmaz; iki takım şimdiden kazanacak takıma göre dizilir.
- Sahne: tik 3200'den Liman bütünüyle savunmada (Kaan–Volkan, Tuna–Yıldırım, Cansu–Kartal, Deniz–Efe, Ece Alp'e basıyor); Fırtına çıkış için açılıyor.
- Tek maçta birebir; genel ölçüm yapılmadı.

## 19 Ekim 2026 · Rakip boş topu alırken bas ve pas yollarını kes (main)
- Menajer (Sahne Masası, 577951 01:46): Ece topa sırtını dönüyor; "Kaan da Alp'in pas açılarını kapatabilir, orta sahadaki arkadaşlarının önünde durabilir."
- `ALAN_BOS_BAS_ESKI` (5689d0a): rakip kendi kalesine 35 br içinde boş topu bizden önce alacaksa takımın topa en erken varanı yine gider, alırken basar.
- `ALAN_BOS_KES_ESKI`: rakip boş topu 6+ tik önce alacaksa topu kovalamayan hücumcular onun alacağı noktadan arkadaşlarına giden pas yollarına (%60, alıcı tarafı) yerleşir; en yakın yol, 20 br sınırı.
- Sahne: Ece Alp'in 2 br arkasında; Kaan 'yol kes · Volkan'; Alp topu Baran'a çıkardı.
- İkisi de tek maçta birebir; genel maç ölçümü yapılmadı.

## 19 Ekim 2026 · Kaçan pas gövdeden geçmez; geçişte topa çık; sapan şutta sekme yok (main)
- 218269 02:14 (menajer: "top resmen içinden geçiyor"): arkadaşın kaçırdığı top oyuncunun öbür tarafına konuyordu, karşıdan gelen pas gövdenin içinden geçti. `ALAN_KACIR_ICINDEN_ESKI`: top değdiği tarafta kalır, gövde çarpışması dolaştırır (a55a626).
- 577951 (menajer: "gol sorun değil, mesele çok müsait olması"): geçişte top sahibine kimse çıkmıyordu.
  - Yerleşirken (YR) buluşma işi atlanıyordu.
  - Pres "bir sonraki hamlesinden önce varamam" diye çıkmıyordu.
  - `ALAN_TOPA_CIK_ESKI`: en çabuk varan kale taraflı savunmacı çıkar, oyalar; gs.oy ortak oyalama hâli.
  - 4 maç: boş alış %38 → %30, şut 19 → 26 (gürültülü).
- `ALAN_SEKME_SAPAN_ESKI`: sapan şutta kale önü sekme koşusu yok (Ece).
- Arka çizgi düzeltmesinin müsaitlik ölçümü (eski → yeni): boş alış %49 → %38, boş şut 9/21 → 2/19.
- Sahne Masası (yapıt): maç dosyasındaki an motorda yeniden oynatılır, kurallar yan yana.

## 19 Ekim 2026 · Arkadaki savunmacı kale ağzına çökmez (main)
- Menajer (577951 01:44 ekranları): "Burada savunma dağıtımında problem var."
- Motorda yeniden üretildi:
  - Liman kontrada; Fırtına'da Kartal ('arkada · Kaan') kale ağzında, Alp ve Baran ('arkada · alan') kaleci alanının kenarında.
  - Efe, Yıldırım, Volkan 'geri koşu · kale' ile x≈104'te.
  - Kaan ceza sahası önünde en yakın savunmacıdan 6,5 br uzakta aldı, vurdu (2-2).
- Neden: tek tartıda (UA) arkadaki hedefi = rakibin 18 tik sonraki yeri + 2,5 br + açık topta koşu hızı × 30 tik, yani koşan forvetin 14–16 br gerisi.
- `ALAN_ARKA_YAKIN_ESKI`: yakin(): arkadaki hedef adamından en çok 4 + kaleye uzaklığının %15'i geride.
  - Tek tartıda ve süpürücü dalında.
  - Adamı olmayan arkadakiler için en derin rakibin okunan yeri de aynı payla.
- Sahne (3100'den yeni kural): Kartal Kaan'ın 4–5 br kale tarafında, pas gelince buluşmaya çıkıyor; Alp ve Baran ceza sahası önünde. Kaan 29 br'den vurup yine attı (uzak şut ayrı konu).
- 4 maç × 2 dk:
  - kale ağzında savunmacı 0,44 → 0,20
  - en derin hücumcuya en yakın savunmacı 4,0 → 4,0
  - taşıyıcıya 3,0 → 3,6
  - gol 4 → 6, şut 21 → 19 (gürültülü)
- Bayrak eski → 045e0e5 birebir.

## 19 Ekim 2026 · Koşu yarışı ve gecikmeli hız okuma varsayılan kapalı (main)
- 1747335'in maç ölçümü (3 maç, hepsi açık) kötü çıktı: savunma arkasına varış 41 → 22, arkada durma %4,7 → %3,1, güvence işi iki katı.
- Eleme: koşu yarışı açıkken koşucu kazanamayacağını görüp koşmuyor; gecikmeli hız okuma iki karşılaştırmada da varışı azalttı.
- İkisi artık açılırsa çalışır: `ALAN_KOSU_YARIS_YENI`, `ALAN_OKU_GECIK_YENI`. Kurtulmada markajcının gecikmesi, hız anlık okununca algı gecikmesinin yarısı.
- Son hâl (hatlar arası + kurtulma + kafa), 4 maç × 2 dk, eski → yeni:
  - hatlar arası %6,7 → %8,0
  - savunma arkasında %5,1 → %4,5
  - varış 55 → 39
  - koşu yarışı kazanılan %14 → %19
  - güvence %10 → %14,5
  - gol 4 → 4, şut 22 → 21
  - Yeni işler: hatlar arası %0,8–1,6; kör taraf %0,3 (bir maçta); gel-dön hiç seçilmedi.
- Açık: hatlar arası arkaya koşuyla yarışıyor (forvet arkaya koşmak yerine kısa geliyor); güvence artışının nedeni bilinmiyor.

## 19 Ekim 2026 · Topsuz hareket: hatlar arası, markajdan kurtulma, koşu yarışı (main, 1747335)
- Menajer: "Oyuncular markajdan kaçmaya, savunma arkasına veya hatlar arasına girmeye çalışıyor mu?" → "Üçünü de yap, kafa da bilsin."
- Ölçüm (önce):
  - Rakip yarısında topsuz hücumcular zamanın %78–83'ünde orta hattın önünde, %5–10'unda hatlar arasında, %3–6'sında savunma arkasında.
  - Arkaya koşu planlarının yalnız %3–7'si savunma arkasına varıyor (çizgiye ~16–19 br uzaktan başlıyor).
  - Koşu sürerken taşıyıcıda koşucuya pas seçeneği hemen her zaman var, ama değeri seçilenin %0–6'sı: koşucu yarışı %79 kaybediyor (5–8 tik geride).
  - Neden: koşu tartısında savunmacının tepkisi pası okuma gecikmesiydi (9–15 tik); maçta savunmacı koşuyu algı gecikmesiyle (~6 tik) izliyor.
- `ALAN_KOSU_YARIS_ESKI`: runV ve teamV'de savunmacının tepkisi algı gecikmesi (10 − Tepki×0,4).
- `ALAN_OKU_GECIK_ESKI`: savunmacı rakibin hızını da yeri gibi gecikmeli görür (seen'e gvx/gvy, OKU ileri bunu kullanır). Önce hız anlıktı: hiçbir dönüş markajcıyı şaşırtamıyordu.
- `ALAN_HAT_ARASI_ESKI`: savunma hattı (en derin iki) ile orta hat (sonraki iki) arasında 6 br'den fazla derinlik varsa, aranın %40 ve %70'inde topun gördüğü açık şeritler (pas yolu ≥ %40, 4 br'de savunmacı yok); en iyi üçü aday, bağ gevşer. İlk sürüm (savunmacıların yan aralıkları) hep kapalı yol veriyordu.
- `ALAN_KURTUL_ESKI`: 6 br içinde markajcısı olana:
  - gel-dön: bir yana çaprazlama topa gelir, öbür yandan markajcının arkasına döner (p._kv ara nokta)
  - kör taraf: markajcının toptan uzak omzunun arkası
  - teamV'de markajcı algı gecikmesi ve dönüş kaybıyla cevap verir
  - Ön tartımda markajcı ara noktanın kale tarafında sayılır
  - Kısa denemede gel-dön ön elemeyi hiç geçemedi, çünkü pas yolu çoğu zaman başka savunmacılarla da kapalıydı. Formül işe yaradığı anda seçer.
- `ALAN_KURTUL_KAFA_ESKI`:
  - predictWorld'de arkadaş ara noktadan sonra hedefine döner; markajcı dönüşü algı gecikmesiyle fark eder
  - Buluşma pası gel-dön yolunda aranır
- Bayraklar eski → d8bd047 birebir (üç maç, 2400 tik). Maç etkisi ölçümü (skor, şut, hatlar arası payı, varış) sürüyor.

## 19 Ekim 2026 · Kafa adam devrini bilir (main)
- Menajer: "Kafa da adam devrini bilsin."
- `ALAN_ADAM_DEVIR_KAFA_ESKI`: predictWorld'e devir haritası: topa basanlar (resp: respond'un seçtikleri; yoksa en yakın iki), adamı açık kalanlar; markRef'i topun yeni sahibi olup topa basmayan markajcı en yakın açık adamın kale tarafına (1,6 br). sonDunya ve sürme sonrası dünyada respond kümesi geçirilir.
- Pas kalibrasyonu 4 maçta değişmedi (kafa / gerçek birkaç puan içinde). Bayrak eski → 452ddb9 birebir.

## 19 Ekim 2026 · Adam devri ve yakın markaj (main)
- Menajer (255860 01:23, ekran): "Burada Rüzgâr arkada nasıl bu kadar boş?" Liman'ın bütün rolleri adam adama (tl.sav.adam 1). Maç 2c18ed4 ile; bende 01:23 birebir çıkmadı (iPad), mekanizma aynı: t2490'da Deniz'in adamı Rüzgâr, Deniz pres yapıyor.
- Bulgu 1: tek tartı (UA) tutulan = bütün adamcıların markRef'i, pres yapanınki de; adamı topla olan markajcı havuz dışında. `ALAN_ADAM_DEVIR_ESKI`: meşgul (pres, karşıla, önde) markajcının adamı tutulmuş sayılmaz; markRef === h olan markajcı havuza girer.
- Bulgu 2: kapatHat markajcıyı toptan adama pas yolunun %55–86'sına koyabiliyordu (t2548 Deniz Rüzgâr'dan 20 br). `ALAN_MARKAJ_YAKIN_ESKI`: aday noktalar adamdan en çok 4 + 0,15 × kaleye uzaklık.
- Ölçüm aracı /tmp/kk/bos-derin.js. 4 maç: adam adama takımda boş derin 735 → 492 tik (bölüm 57 → 56), bölge takımında 1350 → 1059.
- İki bayrak eski → 2c18ed4 birebir (üç maç, 2400 tik).

## 19 Ekim 2026 · Top oyuncuların içinden geçmez; kafa arkadaşa çarpmayı bilir (main, 2c18ed4)
- Menajer: "Toplar hâlâ oyuncuların içerisinden geçebiliyor, bu normal mi?" Sonra: "Kafa da arkadaşa çarpmayı bilsin."
- Bulgu (/tmp/kk/govde-gec.js): iki bilinçli kural. Delip geçme: sert top rakibin tam ortasından %55'e kadar geçiyordu; kafa geçmez sayıyordu. Arkadaş: alıcısı belli pasta alıcı dışındaki arkadaş saydamdı (arkGecir; önce gövdesi pası durdurup rakibe sektiriyordu).
- `ALAN_GOVDE_DELME_ESKI`: core contact'ta thru yalnız yolun gövdeye en yakın uzaklığı ≥ 1 ise; sonra _gec (6 tik gövde yok sayma) kurulmaz.
- `ALAN_YOL_VER_ESKI`: shape'te pas/şut yolunun 2 br yakınındaki ya da top yanına varınca yola girecek arkadaş (alıcı, vuran, Bekçi dışında), yolun 2,2 br dışına çıkar (yol ver). arkGecir kapanır, arkadaş gövdesi seker.
- `ALAN_ARKADAS_CARP_ESKI`: ucusFormul'de yoldaki arkadaşın top varınca yeri (yan hızı + 0,02 br/tik² kaçış; Bekçi çekilmez) yoldaysa gövde (gvNow yolu, sekme yarışı); arkadaşa çarpan top d1'e (kayıp) yazılmaz. 722701 t1737 Efe (yolun 1,08 br yanından koşarak yola girdi) ve t920 Kaan (2,4 br'den 0,29 hızla) örnekleri.
- 4 maç: gövdeden geçiş maç başına 2–5 → 0–1 (kalan: aynı tik başka temas, vuranın dibi). Arkadaşa çarpan pas 6 → 1. Şut 33 → 44, gol 11 → 10. Pas kalibrasyonu kafa %94–95 / gerçek %90–98.
- Üç bayrak eski → e6ebe8a birebir (üç maç, 2400 tik).

## 19 Ekim 2026 · Kafa = maç: Bekçi kayışı, dönüş, pres şiddeti, sürmede Çekirdek (main, e6ebe8a)
- Menajer: 265001 dedektör listesi (a4b94b8 ile; 00:01 ve 00:06 birebir yeniden üretildi, sonrası iPad hesap farkıyla ayrışıyor). Sürerken %100 sanılıp kaptırılan üç top, %81 ve %98 sanılan iki şut, %86–87 sanılıp kesilen iki pas, yer/zaman sapmaları.
- 00:06 Kaan şutu: kafada %81, maçta 0/16 (Tufan tuttu). Bekçi neredeyse duruyor (bekHar 0,14); kafa yine tam koşan modeli uyguluyordu (kayış savurma + donma + dönüş kadar gecikiyordu). `ALAN_KAYIS_HARMAN_ESKI`: koşan model cezayla kademeli (0,5'te tam). Şimdi %2,5.
- Sürmede kayıp (kendi oynatmamda 01:06 Kaan, aynı tür): topu geri koşarken alıp ileri sürmek, dönerken rakibe kaymak. `ALAN_KEEP_DONUS_ESKI`: keepP'de dönüş süresi (donS) boyunca rakip vO + taşıyıcının ona kayışı / 2 hızıyla yaklaşır.
- `ALAN_KEEP_SIDDET_ESKI`: keepP ikili eşiğinde rakibin pres şiddeti (maçtaki gibi).
- `ALAN_KEEP_YON_ESKI`: keepP'de ikili, rakip yetiştiğinde Çekirdeğin olacağı açıyla. Maçın kuralı: gidiş yönü, yakın rakipten en çok 0,7 rad çevrik, dönme hızı Turn.omega. İlk deneme Çekirdeği anında dönmüş sayıyordu, 4 maçta 27 emin kayıp çıktı (433811 t395 Gökay: Çekirdek dönerken Kaan'ın önünden geçti); dönme hızıyla düzeldi.
- Araçlar: /tmp/kk/sur-kal.js (sürme kalibrasyonu), sut-dene/sut-tik savurma > 3 tik düzeltmesi.
- Sürme (4 maç): kayıp 6 → 6 (yönsüz 10), kafa %96–99 / gerçek %99. Şut (4 maç): kafa 20,6 / gerçek 12 → 17,3 / 11; hâlâ iyimser, ayrıca bakılacak.
- Dört bayrak eski → a4b94b8 birebir (üç maç, 2400 tik).

## 19 Ekim 2026 · Top kaybında önce alana yerleş; kafa presçinin oyaladığını bilir (main, a4b94b8)
- Menajer: "Kafayı da hizala, presçinin dalmadığını bilsin. Ayrıca hâlâ top rakibe geçtiğinde temel hedefleri markaj oluyor gibi oyuncuların, oysa direkt alan savunmasına geçmeleri lazım, adam adama savunma seçilmiş olsa bile; mantıken önce geriye yerleşmek gerekir … arkayı boş bırakamaz."
- `ALAN_OYALA_KAFA_ESKI`: keepP'de commitThr'ye maçtaki oyalama eki (oyalaEk: D._mac.sh.gs; son adam +0,25, arkasında arkadaşı varken +0,1; top 26 br'den uzak).
- `ALAN_YERLES_ESKI`: top el değiştirince (sh.gain) savunma "yerleşiyor" (sh.yr). Yerleşirken adam adama askıda (adamT): mark haritası, süpürücü markajı, tek tartı havuzu, geri koşu dışlaması. Tek tartıda kapatma işi (topun gerisindeki rakibe pas yolu) verilmez. Geçiş savunmasının geçersiz kılması gevşek payla (topun 3 br önüne kadar; top 64 br'den yakınken). Yerleşme biter: presçi ve önde bekleyen dışında herkes (biri eksik olabilir) topun en çok 3 br önünde ve dizilişteki yerine 8 br'den yakın, ya da 6 sn.
- Ölçüm aracı: /tmp/kk/kayip-dal.js (her top kaybında iki dal, 8 sn). 63 kayıpta: rakibin en yakın ort. 42,6 / 45,3 / 46,8 → 46,5 / 47,5 / 49,2; şut 10 → 6, gol 5 → 2; 2 sn sonra kale tarafı 2,9–3,2 → 3,2–3,7; geri alınan 27 → 28. Kafa bayrağının dalları: fark küçük.
- Üç maç (5400 tik): şut 21 → 24, gol 8 → 10; kafadaki toplam 15,3 (gürültü: 24 şut).
- İki bayrak eski → 3f237b8 birebir (üç maç, 2400 tik).

## 19 Ekim 2026 · Oyalama: geçişte presçi topa dalmaz, açıyı kapatıp geri geri çekilir (main, 3f237b8)
- Menajer: "Mesele presçinin üzerine gitmemesi zaten, arkadaşları gelene kadar açısını kapatıp ilerleyişini engellemesi?"
- Bulgu: presçi kale tarafına geçince 4 br'ye girip doğrudan topa yürüyordu (ya da çalım gösterip dalıyordu). Son adamken %50 kazanma tahminiyle ikiliye giriyordu (commitThr thrLast).
- `ALAN_OYALA_ESKI`: geçiş savunması dengesizken ve top kaleye 26 br'den uzakken presçi (k = 0), top sahibinin 10 tik sonraki yerinin en tehlikeli seçeneği yönünde 3,2 br önünde durur (pres · oyala). İkili eşiği son adam +0,25, arkada arkadaşı varken +0,1 (DU.oyala, oyalaC, oyalaR, oyalaD). Dengelenince ya da 26 br içinde normal pres.
- Sahne 722701 t1800 (geçiş savunması açık): eskide Deniz 3. sn'de geçiliyor; yenide geri geri çekiliyor, Kaan 2. sn'de geliyor, rakip topu geri çeviriyor.
- Üç maç (5400 tik, kaba): şut 22 → 21, gol 11 → 8; geçişte 2 sn sonra kale tarafı 3,5–4,4. Geçiş savunması öncesinden (6b0b502) beri: şut 35 → 21, gol 17 → 8.
- Kafa, top sahibinin hamlelerinde presçinin artık dalmadığını bilmiyor (duel/keepP eski davranışla); ayrıca bakılmadı.
- Bayrak eski → 3fa2e07 birebir (üç maç, 2400 tik).

## 19 Ekim 2026 · Geçiş savunması: dengesizken adam bırakılır, topla kale arasına geri koşulur (main, 3fa2e07)
- Menajer: "Şu an en büyük problem geçiş savunmasında gibi … adam markajını değil de alanı ve geriyi savunmanın daha önemli olduğunun pek farkında değil gibiler. Olabildiğince oyalamak asıl hedef geçişte, ve hızlıca geriye dönmek … rakibin geçişe çıkan 2 oyuncusu varsa, orta sahaları bekliyorsa bizim orta sahalar da bekliyor."
- Bulgu (geçiş dedektörü, /tmp/kk/gecis-bul.js): 722701 t2294'te Kartal topla kaleye 50 br'den 22'ye indi; kale tarafında kimse yokken altı markajcı topun gerisindeki adamlarını tuttu. Bölge takımında orta sahalar "bölge" işinde topun 30 br önünde kaldı (t1803). Geri koşu kuralı yalnız işi olmayana ve top kaybından sonraki 4 sn'ye uygulanıyordu; tek tartı topun gerisindeki rakiplere kapatma işi veriyordu, markajcılar havuz dışındaydı.
- `ALAN_GECIS_SAV_ESKI`: top kalemize 64 br'den yakın ve kale tarafındaki savunmacı sayısı < önündeki hücumcu sayısı + 1 (top sahibi dahil) ise dengesiz; ≥ +2 olunca dengeli (arada önceki hâl). Dengesizken hedefi topun önünde kalan (markaj, bölge, kapatma, adamının kale tarafı) topla kale arasına geri koşar (geri koşu · kale: topun %35 gerisi, kendi bölgesinin eni). Hedefi kale tarafında olup kendisi önde kalan (presçi, karşılayan dahil) koşarak döner (· geri). Presçi, karşılayan, buluşma, kovala, önde bekleyen kendi işinde. Ekranda açıklamaları.
- Sahneler: t2290 eskide gol, yenide 4,7 sn'de top savunmada. t1800 yenide 4,5 sn'de altı savunmacı kale tarafında.
- Üç maç (5400 tik, kaba): geçişte 2 sn sonra kale tarafı ortalaması 2,2–3,2 → 3,7–4,3; şut 35 → 22, gol 17 → 11.
- Oyalama: presçi zaten kale tarafına geçiyor ("pres · kale tarafına"); ama koşan top sahibinin önüne kimse geçemiyorsa oyalayan da yok (t2290'da 5 sn boyunca kale tarafında kimse yoktu). Ayrıca bakılmadı.
- Bayrak eski → 6b0b502 birebir (üç maç, 2400 tik).

## 18 Ekim 2026 · Kayıt noktası: `iyi-nokta` (6b0b502)
- Menajer: "Şu an oyun hiç fena olmayan bir noktada, lütfen burayı kaydedelim."
- `iyi-nokta` dalı = main 6b0b502 (hareketli Bekçi, köşe şutu, yolun yaşı; risk talimatı pas/sür/tut; sekme düzeltmeleri; kafa ↔ gerçek dedektörü). Geri dönmek için: `git checkout iyi-nokta` ya da o daldan site. Yerelde etiket `v5-iyi-nokta` (etiket gönderimi ortamda kapalı; `v5-kafa-oyunu` gibi uzakta dal olarak).

## 18 Ekim 2026 · Hareketli Bekçi yöne göre geç tepki verir; uzak köşe şut adayı; kafada yolun yaşı (main, 6b0b502)
- Menajer (722701 02:57, Kaan): "Mesele asla hiçbir şutun girmemesi … sert bir şutun tam köşeye girebilmesi lazım … kaleci hareket halindeyken gücünün azalması gerekiyor … açıyı daraltsa bile anlık tepki süresinde yavaşlama olmalı."
- Bulgular. Uzak köşe (34,1) hiç tartılmıyordu: sutHedef tek boşlukta erken dönüyordu, kaba eleme de aynı köşeye iki şutla dört yeri dolduruyordu. Yakın köşeye sert şut, gövdenin kenarından çıkıp Tufan'ın gövdesine gidiyordu. Kafada yolun yaşı maçtan bir tik öndeydi.
- `ALAN_BEKCI_HAREKET_ESKI`: core bekHar = koşu hızı oranı × (1 − cos)/2. Cos, Bekçi'nin hızı ile topun yoluna doğru olan yön arasında. Tepki + bekHarT (4) × ceza, dalış × (1 − bekHarD (0,25) × ceza). Maçta top çıktıktan sonra bu kadar tik eski hedefine gider. Formülde aynı süre koşusunu sürdürür, yana kayış dönüşten (gec) sonra başlar, uzaklaşan topa menzil varacağı yerden. sutKapali ve açı boşlukları şut yönüne göre. İlk deneme (8 tik, %50, yönsüz): 3 maçta 34 şut 22 gol; yönlü ve 4 / %25: 35 şut 17 gol.
- `ALAN_SUT_NISAN_ESKI`: şut bırakma noktasından hedefe nişanlanır. `ALAN_SUT_KOSE_ESKI`: iki köşe her zaman aday, yakın hedefler birleşir, her hedefin en iyi şutu kafada. `ALAN_YAS_ESLE_ESKI`: formülde yeni topun ilk adımında yolun yaşı 0.
- Sahne 722701 t5323 Kaan: uzak köşeye sert şut kafada 0,83, maçta 12/12; yakın köşe kafada 0,06, maçta 0/12; plase kafada 0, maçta 0/12.
- Üç maç (722701, 433811, 909623; 5400 tik, kaba): şut 27 → 35, gol 5 → 17 (%19 → %49), kafadaki toplam 6,2 → 21,1 (kafa önce de sonra da %20 iyimser). Kaçan iki emin şut bıçak sırtı: Bekçi'nin donmadan sonraki koşusu kafada kaba.
- Bayraklar eski → f5394d5 birebir (üç maç, 2400 tik).

## 18 Ekim 2026 · Risk talimatı sürme ve beklemeyi de caydırır (main)
- Menajer, geri pas üzerine görüş istedi; görüş: sıkışınca geri çevirmek (sabır 1) kendi başına saçma değil, saçma olan yolu olmayan pas ve sekme değeriydi; ama düzeltmeden sonra Ece %49 tutan sürmeyi (0,220) %96'lık Tuna pasına (0,212) yeğliyordu. Menajer: "Yap, sürme ve beklemeye de uygula."
- `ALAN_RISK_SUR_TUT_ESKI`: risk caydırma (P0 = 0,85 − 0,6 × risk) sür ve tut seçeneklerinde de, det.P (keepP, sürmenin/beklemenin sonuna kadar) ile.
- Sahne: 662562 t1188 Ece pas → Tuna %96 (0,212). t622, t722, 909623 t110 değişmedi.
- 662562 ilk yarı (tek maç, kaba): Liman ort. tutma 0,40 → 0,50 sn, sür %41 → %27, bekle %14 → %21, pas %39 → %38, tutan pas 30/32 → 33/34, şut 1 → 2; Demir Blok sür %64 → %45, pas %20 → %34, gönder %1 → %7, ort. tutma 0,90 → 0,56 sn.
- Bayrak eski → da23eb4 birebir (909623, 433811; 1500 tik).

## 18 Ekim 2026 · Rakibe çarpan pas başarısız sayılır (main)
- Menajer: "Şu birine çarpar da bize seker sistemleri aşırı aptalca, ayrıca o pozisyondan oraya dönmek de aşırı değersiz olmalı."
- `ALAN_SEKME_KAYIP_ESKI`: ucusFormul sekme payından rakibin dokunup sektirdiği kısmı ayırır (Psek, temas yeri endSek). Pas, önüne, kenardan (arkadaşa, kendine) değerinde Psek'in değeri yok; temas yerinde lossAt bedeli düşülür. Kimsenin dokunmadığı, sahipsiz kalıp yarışla kazanılan top (boşluğa ara pas) sahipsiz top değeriyle (ff694e3) kalır. Risk caydırma ve kart zaten doğrudan alışla.
- Sahne: 662562 t1188 önüne → Deniz −0,689 (doğrudan ~%0: risk −0,70), önüne → Mert −0,755; Ece sürüyor (0,220). t622, t722, 909623 t110 değişmedi. Bayrak eski → ff694e3 birebir (909623, 433811; 1500 tik). Uzun ölçüm yok.

## 18 Ekim 2026 · Sekmeyle dönen top alıcının değeriyle tartılmaz (main, ff694e3)
- Menajer (662562 00:39, aynı maç, 52cbbc6): "Ece neden Deniz'e atar, o pas nasıl geçer gözükür?"
- t1188 Ece (101,22) önüne → Deniz (85,33) seçildi: 0,787, %88. Yol 2 br yanındaki Bulut'un (96,27) üstünden. Formül: Bulut i=4'te topa değer (f 0,996), kontrol ihtimali %5,6 (cap 1,41, göreli hız 1,70; maçta da %5 — Bulut o %5'i tuttu), %88'lik sekme payı bize, yeri (99,6, 20,8) Ece'nin dibi. Önüne pas alış yerinde (ALIS_YERI: endM) tartıldığı için Deniz o noktada topu almış gibi değer aldı (zincir, kale önü) → 0,79. 'tut' aynı yerde 0,20.
- `ALAN_SEKME_DEGER_ESKI`: ucusFormul P'yi ayırır: Ps (rakipten sekip ya da sahipsiz kalıp bize dönen pay; savurmada kaptırılan topun bize kalan payı da) ve yeri endS; doğrudan alış P − Ps ve yeri endD. Pas, önüne, kenardan (arkadaşa, kendine): doğrudan pay recvVal (alıcı, endD), sekme payı V(sonDunya(endS, tf), endS, şarj 0,3). Risk caydırma doğrudan payla. Karar kartında doğrudan alma ihtimali. Kafa ↔ gerçek kaydı toplam P (bizde kalma) ile.
- Sahne: Ece sürüyor (0,220; Tuna'ya %96 pas 0,212). 662562 t622 / t722 ve 909623 t110 değişmedi. Bayrak eski → 7fd680d birebir (909623, 433811; 1500 tik). Uzun ölçüm yok.

## 18 Ekim 2026 · Risk talimatı düşük ihtimalli pası caydırır; sonrasında pas yolunda Bekçi (main, 7fd680d)
- Menajer: "Risk talimatı düşük olasılıklı pasları caydırsın, yap." Ölçüm başlarken durdurdu ("dur dur"), 662562'yi gönderdi (52cbbc6 ile oynanmış, akış birebir): "Arkadaşına verse bomboş kaleye atacak, ona vermiyor, sonra saçma sapan bir pas atıyor … bu sefer diğeri yukarı saçma sapan bir şut çekiyor."
- 662562 00:20–00:24 (Demir Blok hücumu, Aras kalesinin kenarında): t601 Rüzgâr (22,17) sür 0,98 > pas → Şimşek 0,80 (%99); t612 pas aldatması; t622 sür 0,951 > pas → Şimşek 0,928; t634–645 Şimşek'e pas %0 (yol Aras'ın üstünde), Rüzgâr duvardan Gökay'a; t722 Şimşek (9,40) kenardan → Ayaz %40 (0,35) > önüne → Rüzgâr %95 (0,24) ("yukarı şut" denen bu).
- Sürmenin şişkinliği: değeri sonraDeger'in "sonra önüne → Şimşek"inden; sonraDeger pas yollarını laneOk ile tartıyor, laneOk Bekçi'yi saymıyor. Gerçek pas hesabı (uçuş formülü) sayıyor: aynı pas şimdi %7–11. → `ALAN_SONRA_BEKCI_YOL_ESKI`: sonraDeger'de pas, önüne, kenardan (iki bacak) ve kenardan kendine pas yolunda, kale önünden (Kuyu + 6 br) geçen pası Bekçi keser: R = bekBody + min(bekDive, (t − tepki) × dalış hızı) + (t − tepki − 2) × koşu × 0,6, yolun Bekçi'ye uzaklığına sgm(/0,3). t622: sür 0,921 < pas → Şimşek 0,928.
- `ALAN_RISK_CAYDIR_ESKI`: P0 = 0,85 − 0,6 × risk (takım; oyuncunun risk talimatı ya da eğilimi ±0,3); pas, önüne, kenardan, çalımda P0'ın altındaki her puan değerden düşülür (riskCay 1; 0,6 ile 662562 t722 dönmedi). Önce risk yalnız kaybın bedelini ölçekliyordu (1,4 − 0,8 × risk), rakip yarıda kayıp ucuz olduğundan %29'luk pas seçilebiliyordu. Karar kartında "risk talimatı caydırıyor (−x)". Sahneler: 909623 t110 Kaan kenardan → Cansu %29 (0,217) → pas → Ece %99; 662562 t722 Şimşek → önüne → Rüzgâr %95.
- Uzun ölçüm yapılmadı (menajer durdurdu). Bayraklar eski → 52cbbc6 birebir (909623, 433811; 1500 tik).

## 18 Ekim 2026 · Sekmeli pasta kafa ile maç; şut denememe (main, 52cbbc6)
- Menajer (909623 01:15–01:25, ekran görüntüleri): "Hem şut çekmeyi doğru düzgün denemiyorlar hem de aptal saptal kenardan pas veriyorlar." Cansu kenardan → Kaan (%79, Gökay kesti), Tuna kenardan → Ece (%45 seçildi, önüne → Cansu %88 varken).
- Sahne birebir üretilemedi: iPad'deki maç 00:08'e kadar c49aa41 ile aynı, 01:09'dan sonra hiçbir sürümle (c49aa41, 42757c5, 4273257, 26cf90d, fc88857) tutmuyor. Büyük ihtimalle Safari (JavaScriptCore) ile node (V8) matematik işlevlerinde son basamakta ayrışıyor, fark birikiyor. Aynı türden sahnelerle genel incelendi.
- Şut: Liman her sürümde 86 sn'de şutsuz. Kaleye 34 br içinde 13 karar, 7'sinde gönder adayı; Pk 0,00–0,13. Maç kopyalanıp şut zorlandı (909623 t1440 Kaan (98,45), t1745 Kaan (98,40), t1857 Tuna (104,46); güçlü ve plase, 12'şer deneme): 72 denemede 0 gol (plase hep Bekçi'de, güçlü savunmacıda). Kafa doğru: bu mesafeden alçak bloğa şut girmiyor. Liman'ın taktiği sut 0,25 (menzil 34 × 0,875). Değiştirilmedi; dengenin değişmesi menajerin kararı.
- Duvardan pas: dört ilk yarı (909623, 433811, 619497, 7452), fc88857: 66 pas, kafa %70, gerçek %53. 31 kaybın 16'sı rakip dokunup sektirdikten sonra (sekeni yine rakip aldı), 7'si temiz kesme, 5'i alıcı kontrol edemeyip sektirdi, 2'si gövdeden sekme; 25'i duvardan dönüşte.
- 619 t1830 Kaan(85,42) → Ece(83,25) alttaki duvardan (düz yol 17 br, duvardan 51 br): Taş dönüşte kesti; formül teması gördü (f 0,89) ama sekmeyi Taş'ın karar anındaki yerinden (90,40) başlattı, sekeni pasçı Kaan 1 tik sonra alıyor sandı (%100). Maçta Taş 5 br koşup (85,41)'de kesti, top Kaan'dan uzağa yuvarlandı. → `ALAN_SEKME_TEMAS_ESKI`: sekme, dokunanın uzandığı yerden (temas noktasından menzil kadar, geldiği yanda); dokunan oradan kovalar. Sahne: %85 → %40 (değer 0,71 → 0,09), Kaan sürüyor.
- 909623 t2496 Tuna(85,46) → Cansu kenardan, Taş 2 br üstünde: formül %97 (sıyırıp yoluna devam). Maçta Taş savurma sırasında (t2497) yeniden uzandı, top çıkınca adanmış (noTouch), top gövdesinden geri sekti, Tuna'nın dibinde öldü, Tuna 8 tik dokunamadı, Taş aldı. → `ALAN_ADANMIS_GOVDE_ESKI`: kararda zaten adanmış rakip (adandi > tik + Tb) dokunamaz, gövdesine değen top core govde kuralıyla seker (E 0,35, yönlendirme yok), o adanma bitince kovalar. `ALAN_SEKME_BELIRSIZ_ESKI`: top savunmacının gövdesinin 1,5 br içinden geçiyorsa sekme, sıyırma ile önden çarpıp geri dönmenin ortalaması. Sahne: kenardan → Cansu artık seçilmiyor (tut).
- `ALAN_SEKME_BEKLE_ESKI` (shape.js flightRead): topa dokunup sektirenin bekleme süresi (cds; hızlı topta 12 tik) varış tahmininde sayılır; önce yalnız atanın 8 tiki sayılıyordu, sektiren dibinde olduğu için bizimkiler "rakip önce varır" deyip sekene koşmuyordu. Formül (sekYaris) bunu zaten sayıyordu. 619 t1849 sahnesini değiştirmedi (top Kaan'dan uzağa yuvarlanıyor, Taş bekleme süresine rağmen önde: est 16 / bizim en iyi 32).
- Dört ilk yarı, 52cbbc6: 77 pas, kafa %66, gerçek %53 (fark 17 → 13 puan); kafada %80+ sanılıp giden top (kafa ↔ gerçek 'sürpriz') 13 → 8. Düz pas kafa %90–92 / gerçek %86–100, önüne %75–84 / %64–92. Kalan 36 kaybın 20'si temiz kesme, çoğu kafada %20–45 (riski bilinen, değeri yüksek görülen pas); 14 dokunup sekme, 2 gövdeden sekme. Liman'ın risk talimatı 0,25 iken %29'luk pas seçilebiliyor: sıradaki konu.
- Dört bayrak eski → fc88857 birebir (909623, 433811; 1500 tik). Hesap: 2700 tik 82–90 sn (5 süreç, 4 çekirdek).

## 18 Ekim 2026 · Savurma yolu kapalı kenardan pas (main, fc88857)
- Kafa ↔ gerçek'in ilk taramasında bulundu. Kenardan pasın savurma yolu iki yönde de rakip gövdesiyle kapalıyken kafa pası mümkün sanıyordu: kenardan üreteci savurPlan'ı hiç sormuyordu (pas ve göndermede `if (!sv) o.v = null` vardı). Maç vuruşu başlatamıyor (savurBasla: decT = 1), oyuncu her tik aynı pası seçip 5–7 tik donuyordu (909623 00:57 Kaan → Ece, 619 01:12 Şimşek t2159–2165).
- `ALAN_KENAR_YOL_ESKI`: arkadaşa ve kendine kenardan pasta savurma planı yoksa aday değil (kancaK içinde, tek dokunuşta da).
- Üç ilk yarıda (909623, 433811, 619497 kurulumları, 2700 tik) "savurmanın yolu kapalı" 14 + 13 + 20 = 47'den 0'a indi (commit mesajında yanlışlıkla 57 yazıyor). Bayrak eski → 26cf90d birebir (909623, 433811; 1500 tik).
- Taramanın gösterdiği, düzeltilmeyen: kenardan pas kafada iyimser (kafa %61–76 → gerçek %36–53, üç maçta da); 619'da Mert'in sürmesi kafada %100 tutacakken Ayaz aldı (01:29).

## 18 Ekim 2026 · Kafa ↔ gerçek: sapma dedektörü (main, 26cf90d)
- Menajer: "Kafada olanla gerçekleşenin birbirinden farklı olduğu durumları gösteren bir şey yapamaz mıyız? Detection mekanizması olsun diye."
- match.js: top sahibinin her hamlesinde (karar ve tek dokunuş) kafadaki beklenti kaydedilir: hız (o.launch), savurma süresi (o.savT), tutma ihtimali (det.P), gol ihtimali (det.Pk), alış yeri (det.endM / hedef), alış süresi (o.tf; decide.js'te pas, önüne, kenardan için yazıldı). Vuruş çıkınca (launch) çıkan hız; sonuç: topu alan (take), gol, santra, sonraki karar. kafaSapma: a3 kural (hız %20 ve 0,3'ten çok; vuruş çıkmadı / çıkmadan alındı), a2 sürpriz (pasta %80+, sürme-beklemede %85+ bizde sanılıp giden top; vuruş 8 tikten çok gecikti), a1 ayrıntı (alış yeri 8 br / zamanı 1 sn sapması, %30 altı sanılıp alınan pas, gönderme gol ihtimali). Eşikler M.KS.
- Kayıt yalnız M.kafaAc(m) ile açılan maçta, sayılamaz alanda (m._kafa): kopya almıyor, kafadaki kopyalar kaydetmiyor. Kayıt açıkken 909623 ve 433811 4273257 ile birebir (1500 tik).
- mac-isci.js: kayıtlar kare mesajında (kafa); taktik değişince yeniden oynatılan kısmın kayıtları gönderilmez, sayfa snapT'den sonra biteni siler.
- canli.html: KAFA ↔ GERÇEK paneli (AKIŞ'ın altında): süzgeç (Önemli · Kural · Sürpriz · Hepsi), satır: saat, tür etiketi, oyuncu · hamle, sapma ve kafadaki değer · ihtimal; dokununca karar anına gider, oyuncuyu seçer (karar kartı açılır). Zaman çizgisinde a ≥ 2 sapmalar ince çizgi. Kalibrasyon: tür başına kafadaki ortalama ihtimal → gerçekte top bizde kalma oranı (pas, önüne, kenardan, kendine, tek dokunuş; göndermede beklenen gol → gol).
- Doğrulama: ALAN_KANCA_KENAR_ESKI ile 909623'te eski hız hatasını yakalıyor ("hız kafada 2,95, çıkan 1,28", Kaan, t141). İlk tarama: hız sapması yok; 47 "savurmanın yolu kapalı" (→ fc88857); kenardan pas kalibrasyonu iyimser.

## 18 Ekim 2026 · Tek dokunuş kenardan pasın hızı (main, 4273257)
- Menajer (909623, 00:04–00:08): "Hala ciddi problemler var." Deniz'in tek dokunuş kenardan pası Kaan'a gitmeden Bulut'un önünde duruyor, Bulut kesiyor.
- Sebep: maç (c49aa41) tek dokunuşta her vuruşun hızını gelen topla sınırlıyor (savurBasla, kanca: u×(1−0,35·min(1, A/π))+0,4). Kafadaki formül bu sınırı pasa, önüne pasa ve göndermeye uyguluyordu (`ALAN_KANCA_HIZ_ESKI`), kenardan pasa uygulamıyordu. t201 Deniz: kenardan→Kaan kafada 2,83 hız, %82, değer 0,422 (kontrol edip atsa aynı pas 0,095–0,098, %46–56); maçta 0,82 hızla çıktı (gelen top 0,65), duvara varamadan yavaşladı, t241 Bulut (74,43) aldı.
- `ALAN_KANCA_KENAR_ESKI`: tek dokunuşta (ön hareket yoksa) kenardan pasın, arkadaşa ve kendine, hızı aynı kanca sınırıyla; değer o hızla tartılır.
- Sahne (190'da çevirme): tek dokunuşun en iyisi pas→Mert 0,082 (seçilmedi); Deniz kontrol ediyor, t207 kontrollü kenardan pas Kaan'a, 00:09'da top Tuna'da, top bizde.
- Bayrak eski → 42757c5 birebir (909623, 433811). Uzun ölçüm yapılmadı.

## 18 Ekim 2026 · Amaçlı bekleme: rakibi üstüne çekmek, koşuyu görmek (main)
- Menajer: "Mesele oyuncu üzerine çekmek veya koşu görmek için beklemiyor oluşları zaten."
- Sebep: beklemenin sonrası kaba sonraDeger tahminiyle ve yarım ağırlıkla (İlke C-c: kaba tahmin sistematik iyimserdi). Bekledikten sonra koşucuya ya da boşalan arkadaşa açılacak pas ne kadar iyi olursa olsun bekleme şimdiki pasa yeniliyordu.
- `ALAN_BEKLE_GERCEK_ESKI`: düz bekleme ve 30 tiklik beklemede sonrası iç içe decide (D._noHold, D._noFake) ile W2'de (sonDunya 'drive': rakip tepki, arkadaşlar hedefine), tam ağırlık; 60 tiklik bekleme kaba tahminle. Amaç etiketi: b2'nin alıcısının en yakın savunmacısı ≥2 br uzaklaşmış ve bir rakip taşıyıcıya ≥3 br yaklaşmışsa 'çek', alıcı ≥4 br (≥2 br ileri) ilerlemişse 'koşu'; o.why ve det.lookTo; maç akışında olay.
- Hesap süresi (433811, 1500 tik, tek süreç): c49aa41 19,3 sn; tüm beklemelerde gerçek hesap 37,2; yalnız düz 21,5 (amaçlı bekleme 6–10/yarı, bekle %11); düz + 30 tik 28,9 (seçilen).
- Üç ilk yarı: 12–19 amaçlı bekleme/yarı/maç ("Tuna rakibi üstüne çekiyor · Cansu açılacak", "Mert bekliyor · Kaan koşusunu bitirince verecek"); Liman ort. tutma 0,47 sn · bekle %26 · sür %24 · pas %45. 7452 sahnesi: Deniz Tuna'nın önüne pas 0,099 > bekleme 0,075 (sonra pas 0,082).
- Bayrak eski → c49aa41 birebir (7452, 619497; 1500 tik). Ekran: beklemenin amacı karar listesinde.

## 18 Ekim 2026 · Topu tutmuyor, sürmüyor (main, c49aa41)
- Menajer: "Şimdi de topu hiç tutmaz ve sürmez oldular, her saniye pas veriyorlar."
- Üç ilk yarı (7452, 433811, 619497), Liman: 805dab3 ort. tutma 0,65 sn · bekle %25 · sür %34 · pas %31; afc9e8e 0,37 · 4 · 31 · 58. İlk kararda pas max(sür, tut)'u %36 → %68 geçiyor (medyan fark −0,018 → +0,058). Gruplar (üç maç): Tuna düzeltmeleri eski → pas %41 (tek tek hiçbiri yetmiyor: şişkin 'sonra' tahminleri toplu kalktı); amaç grubu eski → bekle %12 (sabır ödülü); kombinasyon koşuları etkisiz.
- 433811 t55 Deniz: ham bekleme 0,066, bekleme bloğundan sonra −0,030; bekledikten sonra (pas) 0,09 ama üstünlük bedeli: ust 2 (topun 15 br'sinde hücumcu − topun önündeki 30 br'de savunmacı; alçak blok 30 br ötede, sayılmıyor) → .06×0,5 + .15×2×0,5 = 0,18. Kural eskiden de vardı; şişkin sonra tahminleri örtüyordu.
- `ALAN_UST_BLOK_ESKI`: üstünlükte topla kale arasındaki bütün savunmacılar. `ALAN_SONRA_SAVUNMA_DUZ_ESKI`: sonra tahmininde tepki yalnız kenardan pasta (düz pasta da uygulamak için bayrak). `ALAN_SUR_KARAR_ESKI`: sürme kararı (match.js h._surK; hedef 2,5 br, süre mesafe/0,25+6 tik, rakip 3,5 br ya da başlangıçtan 2 br yakın, D.surKarar 0,025; gönder serbest). `ALAN_AMAC_BEKLE_ESKI`: beklemenin açması (o.hw varsa) ve sürmenin açması predictWorld'de (arkadaşlar koşusunu sürdürmüş). `ALAN_AMAC_GERI_ESKI`: geride/yanda boştaki arkadaş 0,4 ağırlıkla seçenek.
- Sonra (üç ilk yarı): Liman ort. tutma 0,46 sn · bekle %3 · sür %37 · pas %56; ilk kararda pas önde %46–60 (medyan +0,008). Bekleme düşük kalıyor (sabrın bekleme ödülü bilerek kaldırıldı). 7452 sahnesi: Deniz'in en iyisi Tuna'nın önüne pas 0,099 (Mert 0,064, sür 0,059), sonra Ece; Tuna kanattan sürüyor.
- Bayraklar eski → afc9e8e birebir (7452, 433811; 1500 tik). Ekran: sürdürülen sürme başta, 'sürmeye devam'.

## 18 Ekim 2026 · Kombinasyon koşuları: üçüncü adam, ver-kaç, bindirme, "Bindirir" talimatı (main, afc9e8e)
- Menajer: "Üçüncü adam koşusu da lazım, bindirme sistemi vs." Ölçmeden.
- Durum: pas havadayken topsuz oyuncular yerini zaten alıcının buluşma noktasına göre tartıyordu (shape E = fr.first'in buluşma noktası) ve zincir (predictWorld) koşan arkadaşı hedefine doğru ilerletiyordu; eksik olan koşu adaylarıydı (savunmanın arkasına koşu yalnız belli rollerde, bağ uzağa koşuyu eziyordu) ve bindirme hiç yoktu.
- `ALAN_UCUNCU_ESKI`: pas B'ye havadayken B'nin buluşma noktasından kaleye doğru (±30°, ±60°) 10/16/22 br noktalar aday (oyuncunun şimdiki yerinden ileri); amaç payı S.ucK (0,05) × B'den yol açıklığı, varış B'nin alışından en çok 20 tik sonra değilse 0. Pası veren için 'ver-kaç'. Bağ vacL ile gevşer.
- `ALAN_BINDIRME_ESKI`: çıkış noktası taşıyıcı ya da pas ona giderken buluşma noktası. Dış (çizgi tarafı, çizgiden 2 br içeride en çok 7 br) ve iç (7 br içeri), 10/16 br ileride. Pay S.bindK (0,08) varılacak yerde 8 br'de rakip yokken × (yanında 10 br'de ≤1 savunmacı 1, 2 savunmacı 0,5). Kendiliğinden: kanattaki (orta çizgiden 8 br+) taşıyıcının 3–25 br gerisinde, 22 br yanında, kendi kalesinden 35 br ileride. Talimatlı (tl.huc.bindir = numara): koşul yok (−5…40 br, 25 br ileride), pay ×1,5, üç savunmacıda bile 0,25, bağ ×0,5.
- Taktik ekranı: 'Bindirir' satırı (kendiliğinden → 1…6 numaraya), motor TLV.huc.bindir = null.
- 7452 ilk yarı: kanatçılar topu yalnız 41 tik tutuyordu (taşıyıcıya bağlı bindirme 2 kez tetiklendi, ikisinde seçildi) → tetik pası havadayken de. Sonra: talimatsız 135, talimatlı (Mert → 6, Ece → 4) 158 oyuncu-tik bindirme; üçüncü adam ~350, ver-kaç ~130 oyuncu-tik; bu görevdeyken pas alan 2–4.
- Bayraklar eski → 5f99f69 birebir (7452, 619497; 1500 tik). Taktik ve maç sayfası yerel tarayıcıda hatasız; 'Bindirir' satırı görünüyor. Uzun ölçüm yok.

## 18 Ekim 2026 · Amaç katmanı: her hamlenin bir amacı, sabır beklemek değil (main, 5f99f69)
- Menajer: "Sabırlı oynamak beklemek demek değil; riskli dikine pası azaltmak, rakibi pasla açmak, oyunu genişletmek, pas imkânı yaratmak. Oyuncular bir amaçla mı yapıyor?" Ölçmeden, hepsini ekle; maça bakıp karar verecek.
- Durum (iki ilk yarı, 011e480): en iyi iki seçenek arasındaki fark kendi yarı sahada kararların %72–74'ünde, ortada %53–63'ünde taktik eğiliminden (±0,02) küçük; %25–32'si 0,005'ten küçük. Konum değeri ilk üçte ikide düz (0,06–0,08). Sabır = bekleme +0,02 (+ riskli ileriye eksi, baskıda geri pasa artı); tempo = kısa yan/geri pasa sabit artı.
- `ALAN_AMAC_ESKI` (decide.js amac, tercih'ten sonra): İlerle = D.amIleri (0,02) × kaleye yaklaşma / 30 br + D.amGec (0,008) × geride kalan rakip (uzaklık halkası: kaleye uzaklığı topun eski ve yeni yeri arasında kalanlar); eksi ise × (1 − 0,6 × sabır). Aç = D.amAc (0,006) × (alıcının önündeki seçenek − şimdiki) × (1 + sabır); seçenek = ilerideki (3 br'ye kadar geride) her arkadaş için yol açıklığı × boşluk (en yakın rakip 2 br'de 0, 8 br'de 1). İkisi × tutma ihtimali, tercihEk ile. o.amac kayda geçer.
- `ALAN_SABIR_BEKLE_ESKI`: sabırla bekleme ödülü yok. `ALAN_TEMPO_YAN_ESKI`: tempoyla kısa yan/geri pas ödülü yok (kontra temposu duruyor).
- `ALAN_GENIS_AMAC_ESKI` (shape.js roleY): kanat oyuncusunun içeri gelme bedeli min(eski, 1 − bloğun dışında kalma / 8 br) × taktik genişliği (Geniş 1,3, Dar 0,7); görev 'genişlik' (blok dışı > 2 br). `ALAN_CEK_AMAC_ESKI`: aday noktada tutan savunmacı (işi onu tutmak ve 12 br içinde, ya da 5 br dibinde) %80 izler; taşıyıcıdan arkadaşa pas yolu farkı × arkadaşın V'si × S.cekK (1); 0,012'den büyükse görev 'açma · açılan · çekilen'. Görev, plan korunurken de kalır (p._amacJob).
- Ekran: SON KARAR ve İnceleme'de her seçeneğin altında amaç (ileri · n rakip geride kalır / geri çevirir / pas seçeneği açar a → b / bekleyip X · rakibi üstüne çeker). Oyuncu kartında 'genişlik' ve 'açma' açıklaması. Yerel tarayıcıda sayfa hatasız, satırlar görünüyor.
- İlk dakika (7452): kanat oyuncuları zamanlarının ~%35'inde 'genişlik', 'açma' iki takımda 131 oyuncu-tik. Bayraklar eski → 011e480 birebir (7452, 619497; 1500 tik). Hesap süresi aynı. İki ilk yarı: çökme yok; 7452'de ileri pas 18 → 30 (tek yarı, ölçüm değil). Uzun ölçüm yok (menajerin isteği).

## 18 Ekim 2026 · Kanatta boş Tuna: pas vermiyor, saçma duvar pası, markajcı süpürücü (main, 011e480)
- Menajer (7452, 498cf41 ile oynandı, 02:37–02:41): Tuna sağ kanatta bomboş, açı var, Deniz inatla vermiyor; sonra üst duvardan Tuna'ya sektiriyor, Rüzgâr kesiyor. "Tuna'nın neden boş olduğu da defans açısından problem."
- Vermemesi: (1) Ön eleme arkadaş başına tek seçenek tutuyordu; kaba ölçü hızı görmüyor, Tuna için kaleye yakın 'önüne' yumuşak türü kalıyor (%42–67), ayağına pas ve sert türler eleniyordu. Elemeden geçince sert önüne pas %99 tutuyor. (2) Beklemenin değeri 0,143: "bekledikten sonra kenardan pas" 0,198 %100. sonraDeger'in kaba kenardan tahmini menzilsiz (88 br) ve alıcıyı savunmacılar yerinde dururken tartıyordu (yalnız Bekçi kayıyor); 70 br'lik Deniz → alt duvar → Kaan 0,137 %100.
- Duvar pası: formül %84 diyordu; değer Tuna'nın durduğu yerde (97,56) tartılıyordu ama top Tuna'ya hiç varmıyor, payın çoğu Gökay'dan seken topu Ece'nin alması (sekYaris). Maçta Gökay dokunup yavaşlatınca Liman savunmaya döndü (Ece 'markaj · Ayaz'), top Rüzgâr'a yuvarlandı.
- `ALAN_ADAY_TUR_ESKI`: her arkadaşa her türün (yumuşak/sert ayrı) en iyisi kafada denenir. `ALAN_KENAR_ALIS_ESKI`: kenardan pas alış yerinde tartılır. `ALAN_SONRA_KENAR_ESKI`: sonra tahmininde kenardan pas menzili. `ALAN_SONRA_SAVUNMA_ESKI`: sonra tahmininde alıcı respond dünyasında (uçuş süresi, kenardan için iki bacak).
- Aynı sahne (4795'ten sonra yeni kurallar): duvar pası 0,007'ye düşüyor, bekleme 0,143 → 0,066, Tuna'nın önüne sert pas 0,063 (beklemeyle başa baş; Sabır 1 beklemeye +0,02 veriyor). Deniz Ece'ye veriyor, top Tuna'ya ulaşıyor, 02:45 Tuna'nın şutu (Kaya tuttu); kayıp yok.
- Tuna boş: Ayaz 'Markajcı', adamı Tuna (maç başından), ama 3-2-1'in arka üçünde süpürücü. Oyuncu döngüsünde süpürücü dalı markaj dalından önce 'continue' ediyor: Ayaz arkada alan tutuyor; tek tartı (UA) da Tuna'yı 'markajcısı var' diye iş listesine almıyor. Liman topa sahipken Ayaz Tuna'ya 8 br'den yakın: %2. `ALAN_MARKAJ_SUPURUCU_ESKI`: arka hattaki markajcı (taşıyıcıyı karşılamıyorsa) adamını takip eder → %41.
- Bayraklar eski → 805dab3 birebir (7452 ve 433811, 1500 tik). Hesap süresi 14,0 → 14,4 sn / 1500 tik. Üç ilk yarı yeni motorla: çökme yok, aşan pas 0. Uzun ölçüm yok.
- Açık kalan: kendi yarı sahamızdan hücum üçte birine konum değeri neredeyse düz (V(60,30) 0,078; Tuna'nın alış yeri (80,55) savunma kaydıktan sonra 0,083). Kanattaki boş adama pas bu yüzden beklemeyle başa baş.

## 18 Ekim 2026 · Hedefini aşan önüne pas atılmaz (main)
- Menajer: "mesele önüne atmasının değerli olması değil, önüne pasın bu kadar aptalca atılması." Haklı: zaman bedeli (4058e98) yalnız değeri düşürüyordu; asıl hata pasın hızı.
- 433811 00:13: Cansu'nun Deniz'e ara pasının iki türü vardı. Yumuşak (0,83): top hedefe 66 tikte, Deniz 59 tikte; yolda buluşuyorlar. Sert (1,23, varış 0,8): top hedefe 39 tikte, Deniz 59 tikte; top 20 tik önce geçip 38 br ötede, kendi köşemizde alınıyor. Sert tür alıcının varışına bakılmadan üretiliyor, formül de köşedeki alışı geçerli pas sayıyordu.
- `ALAN_ASAN_PAS_ESKI`: alıcı topu nişan alınan yerin (top yolunda) `D.asanPay` = 3 br'den fazla ötesinde alacaksa o seçenek yok. Aynı sahnede sert tür düşüyor, Cansu topu tutuyor.
- Yayındaki motorla (498cf41) 433811 ve 619497 ilk yarılarında seçilen 94 önüne pasın 9'u hedefini aştı: Ece → Deniz 36 br, Cansu → Deniz 38 br (ikisi de kendi köşemize), diğerleri 6–10 br. Yeni kuralla 0. Bayrak eski → 4058e98 birebir (iki maç, 1500 tik). Uzun ölçüm yok.

## 18 Ekim 2026 · Yavaş şuta "kesin gol" sanma, hedefini aşıp köşeye giden pas (main, 4058e98)
- Menajer (433811; maç f539df2 ile oynandı, 498cf41'den 4 dk önce açılmış): 02:20 Rüzgâr kale önünde bomboşken topu tek dokunuşla 0,77 hızla Aras'ın kucağına gönderiyor; 00:13 Cansu (49,6) "önüne → Deniz" pasını veriyor, top Deniz'i geçip kendi köşemize gidiyor.
- Rüzgâr: iki hata üst üste. (1) Tek dokunuşun hız sınırı (gelen top 0,41 → en çok ~0,78) formülde yoktu, 3,15 hızla tartılıyordu (498cf41'de düzeldi). (2) O düzeltmeyle de formül 0,78 hızlı şuta %92 gol diyordu: şut formülünde Bekçi yana kayarken yürür sayılıyordu (hız ×1), maçta ise kovalarken koşuyor (×1,25; bekKay de böyle). Maçta aynı sahnede şut y 26'dan direk dibine 34,5'e kadar her hedefe zorlandı: hepsini Aras tuttu.
- `ALAN_BEKCI_KAYIS_ESKI`: ucusFormul ve sutKapali'de Bekçi'nin kayışı koşu hızıyla. Aynı sahne (4185'ten sonra yeni kurallar): Rüzgâr tek dokunuşta vurmuyor, topu alıp sürüyor, 02:22'de sert vuruyor (3,14) ve gol.
- Cansu: formül pasın (0,9, 58,3)'te 124 tikte alınacağını biliyordu; kendi yarı sahada konum değerleri düz (köşe 0,061, orta 0,064) ve kesilme payı sıfırdı, beklemek 0,5 sn için ~0,01–0,02 bedel öderken 2,5 sn fazla süren pas bedelsizdi: 0,077 ile en iyi.
- `ALAN_ASIM_ZAMAN_ESKI`: önüne pas nişan alınan yerden sonra alınacaksa aradaki süre beklemenin bedeliyle (tempo × bekTempo / sn, tutma ihtimaliyle) düşülür. Aynı sahnede köşe pası listeden düşüyor, Cansu Mert'e veriyor.
- Bayraklar eski → önceki sürüm birebir (433811 ve 619497, 1500 tik). Uzun ölçüm yok.

## 18 Ekim 2026 · Tek dokunuşta kendi etrafında dönme (main)
- Menajer (909623, 00:11): Tuna kendi etrafında dönüyor. Tek dokunuş güçlü şutun savurması: top gövdenin etrafında 593–646° (1,6 tur) döndü, sonra 0,62 hızla bırakıldı. Tek dokunuşun hızı gelen topla sınırlı (gelen 0,35 → en çok ~0,62; kanca kuralı, savurBasla), ama savurma tam hızlı vuruşa (3,26) göre planlanıyor, sınırdan sonra yeniden planlanmıyordu. Karar formülü de bu sınırı bilmiyordu: şutu tam hızla tartıp (girme %13,7) seçiyordu.
- `ALAN_KANCA_HIZ_ESKI`: sınır uygulanınca savurma o hıza göre yeniden planlanır (gerekmiyorsa hemen itişle çıkar); formül tek dokunuş pasını, göndermesini ve alıcının tek dokunuş şutunu bu sınırlı hızla tartar.
- Aynı sahnede (önceki adımlar birebir, 342. tikten sonra bayrak açık): Tuna tek dokunuşla Kaan'ın önüne pas veriyor, savurma 45°. Bayraklar eski → önceki sürüm birebir (1500 tik). Uzun ölçüm yok.

## 18 Ekim 2026 · Sonsuz stoper paslaşması, saçma şutlar (main, kod taramasından sonra)
- Menajerin iki maçı. 619497 (bütün takımlar 20): Deniz ile Mert ilk 37 sn'de ~40 kez birbirine pas verdi. Kendi yarımızda bütün seçeneklerin değeri 0,067–0,072 arası; taktiğin sabrı (1) her ileri hamleye −0,02, her geri pasa +0,02 veriyordu, önü 35 br boş, rakipsizken bile. `ALAN_SABIR_RISK_ESKI`: sabır ileri hamleyi riskiyle (tutma ihtimali %80'de tam, %100'de hiç), geri pası taşıyıcıdaki baskıyla cezalandırır/ödüllendirir. Aynı sahnede paslaşma kalktı, Liman 18. sn'de şut buldu.
- 909623 (rakipler 10): Tuna köşeden (109, 54), en yakın rakip 17 br uzaktayken tek dokunuşla vurdu: girme %13,5, değer 0,62. Formül Bekçi çelince topun %53 bizde kalacağını ve bunun yerini Bekçi'nin dokunduğu nokta (Kuyu'nun içi, değeri 1,0) sayıyordu. `ALAN_SEKME_YERI_ESKI`: seken top, onu alacak oyuncunun aldığı yerde. `ALAN_BEK_YON_AGIRLIK_ESKI`: Bekçi'nin çelme yönleri eşit değil, Bekçi'nin kendi tercihiyle (rakipten uzak; core contact kuralı) yumuşak tartılır. `ALAN_SEKME_BEKCI_ESKI`: göndermeden sonra topu aldığımız yerin değeri Bekçi o noktaya göre yerleşmişken (önce şutçuya göre durduğu yerde: girme ihtimali 0 olan şut, seken topun 'açık açıdan şut' değeriyle seçiliyordu).
- `ALAN_FISKE_TEPKI_ESKI`: fiske eşiği Tepki/10 ile çarpılmaz (Tepki 20 olan oyuncu rakip yetişemeyecekken de fiske atıyordu).
- Uzun ölçüm yapılmadı (menajerin isteği). Bayraklar eski → yayındaki sürüm birebir (1500 tik). İki sahnede ilk dakikadaki seçilen şutlar: eskide %13'lük şutlar 0,6–0,8 değerle, yenide değerleri ihtimaliyle orantılı.
- Kod taramasının ikinci turu (40 düzeltme) yayında değil: `tur2-bekliyor` dalında. 24 maçlık ölçümde toplam top kaybı aynı, pas kaybı 241 → 260, formül 0,8–0,95 bandında iyimser (%89 bekleyip %82); iki değişiklik (karşılama yerinde kontrol, savurmada koşu yok) belirgin kötüydü.

## 18 Ekim 2026 · Kod taraması: 17 mantık hatası (main 67cbff1) — maçlar değişti
- Menajer: koddan maç oynatmadan mantıksız şeyleri tara; sonra "hepsini düzelt". Beş parçalı okuma, iddialar kodda tek tek doğrulandı. Elenen ikisi: Bekçi'nin her yerde dalış menzili (menajer kararı, DESIGN-NOT-bekci-temas.md) ve eski kafa oyunundaki (simOne) alıcı farkı (ölü kod).
- Zaman birimi (`ALAN_SANIYE_ESKI`): motor 30 tik = 1 sn, birkaç yer 60 sayıyordu. Karşı pres '2 sn' 4 sn sürüyordu, 'son 30 sn' koşulu 60 sn kala başlıyordu, kontra en çok 8 sn sürüyordu; YZ uyumu ve olay saati de.
- Karar (decide.js): şutta baskı cezası yalnız gonEk, pasın payı üstüne çarpılmaz (`ALAN_SUT_BASKI_ESKI`) · savurmada kaptırma ayağa/önüne pasta iki kez düşülüyordu, sonraki hamlede hiç; şimdi her vuruşta bir kez (`ALAN_SAVUR_KAYIP_ESKI`) · fake'e aldanan formülde fake boyunca (9–18 tik) yanlış yöne koşuyordu, maçta 3–8 tik yanılıp dönüyor; formül maçın yanılma süresi ve ivme kuralıyla (`ALAN_FAKE_YANIL_ESKI`) · ağız pası Kuyu'nun içine (kale ortasına 3 br'ye kadar) iniyordu, sürme ve boşluk hedefleri de; uçuş formülü ve sekme yarışı Kuyu'nun içindeki topa saha oyuncusunu yetiştiriyordu (`ALAN_KUYU_HEDEF_ESKI`) · tahmin dünyasında topa dört savunmacı çıkabiliyordu (`ALAN_SON_DUNYA_DORT_ESKI`) · boşluk hesabında alıcının eski yerdeki kopyası koşucu sayılıyordu (`ALAN_BOSLUK_HAYALET_ESKI`) · tek dokunuşta Bekçi iki kez kayıyordu (`ALAN_TEK_BEKCI_ESKI`). Son üçü bu haftaki kendi değişikliklerimden.
- Savunma (shape.js): pres, topu önce durarak tutmuş oyuncuyu yeniden alınca yüzlerce tiktir tutuyor sayıyordu; savururken karar sayacını 99 okuyordu (`ALAN_PRES_SAYAC_ESKI`) · 'ikisini kapat' dünyasında savunmacının kendisi ve arkadaşları yoktu, her nokta eşit çıkıp hep durduğu yer seçiliyordu (`ALAN_IKISI_KAPAT_ESKI`).
- Fizik (core.js, match.js): gövde süpürmesi temasın sonucunu (delip geçme, ayaktan kaçma) geri alıp topu gövdenin önüne yapıştırıyordu (`ALAN_GOVDE_SONUC_ESKI`) · kontrol edemediği gövdeden top o topun ömrü boyunca hiç sekmiyordu (`ALAN_GOVDE_CD_ESKI`) · kaleci çelmesi eşitlikte hep −80° (y = 0 kenarı) (`ALAN_CELME_YON_ESKI`) · temas yolda ilk değene değil yola en yakına gidiyordu (`ALAN_TEMAS_SIRA_ESKI`) · Refleks 16'nın üstünde yerleşmiş Bekçi yerleşmemişten geç uzanıyordu; formül de payı sıfırda kesiyordu (`ALAN_BEKCI_REFLEKS_ESKI`).
- Maç döngüsü: koşul etkinken yapılan taktik değişikliği bir sonraki tikte siliniyordu (mac-isci.js, `ALAN_KOSUL_ESKI`; işçide denendi: risk 0,85 eskide 0,5'e dönüyor, yenide kalıyor) · sürme bayrağı top elde kalırken hiç sıfırlanmıyordu (`ALAN_SUR_BAYRAK_ESKI`) · santrayı yapan ilk 30 tik eski sürme hedefine koşuyordu (`ALAN_SANTRA_ESKI`) · sersem/adanmış oyuncu duran topu alabiliyordu (`ALAN_DURAN_TOP_ESKI`).
- 24 maç, 4500 tik (önce 682efe3 · şimdi): pas 2162 · 2315; başarı %89,4 · %89,6; pas kaybı 229 · 241; ikili 63 · 40; dürtme 49 · 23; fiske 50 · 42; toplam top kaybı 391 · 346; şut 256 · 232; gol 21 · 29; Bekçi tuttu 91 · 78. Bütün yeni bayraklar eski → 682efe3 birebir (üç maç, 1500 ve 4500 tik). Tam maç (devre arası dahil) hatasız; hız aynı.

## 18 Ekim 2026 · Kadro denemesi: Liman 20, rakipler 10
- Menajerin isteği. kadro.js'teki geçici deneme: Liman'ın bütün oyuncularının bütün özellikleri 20 (Alan gücü ve Soğukkanlılık 10, önceki gibi); üç rakibin (Demir Blok, Fırtına, Saat Kulesi) bütün oyuncularının bütün özellikleri 10 (Bekçi özellikleri dahil; taraf aynı). Anahtarlar `HEPSI_20`, `RAKIP_10`; asıl değerler silinmedi. Maç sayfası kadroyu Taktik ekranının kaydettiği kopyadan okur: Taktik'ten bir kez 'Maça çık' gerekir. 20'ye 10 maç (1800 tik) ve maç işçisi hatasız.

## 18 Ekim 2026 · Uzun pas: alıcı payla buluşur; zincir hızlandı (main 682efe3) — maçlar değişti
- Menajer: önceki sürümde toplam top kaybı nasıl arttı? Pas pas etiketli ölçüm (scratch `/tmp/kk-pas-etiket.js`: her pasın karar etiketleri ve sonucu; aynı 12 maç): zincirle, boşlukla ve yeni sert pasla seçilen 42 pas 0 kayıp. Artış sıradan paslardan: maç başına 7,8 → 11,3 (12 maçın 9'unda), 35 br'den uzun paslarda 18 → 41 (formül 28 bekliyordu). Tek tek kapatma: yeni diziliş kapalı 104, zincir kapalı 111, ön elemede boşluk kapalı 138 (hepsi açık 135, önce 94). Kök: savunma artık alıcıya çökmediği için pas daha çekici; bu, uçuş formülünün uzun yavaş paslardaki eski iyimserliğini açığa çıkardı. Örnek (Kartal → Volkan, P %97): top Volkan'ın 1,8 br yanından geçti, uzaklaşan topa menzil kısa olduğu için dokunamadı, 60 tik kovaladı, yolun sonunda Mert kesti. Formül bir tik geç kalan alıcıya %30, kıl payı yetişene %50–80 veriyordu; maçtaki alıcı da tam zamanında varmayı hedefliyordu.
- `ALAN_ALICI_PAY_ESKI`: pasın alıcısı buluşma noktasına `rcvBuf` (3 tik) önce varıp yolun üstünde bekler (top ona gelir, tam menzille alır); payla nokta yoksa eski kural. Uçuş formülünde arkadaş da aynı payla alır, uzaklaşan topa kısalan menzil ona uygulanmaz.
- Hız: boşlukta yerleşmiş Bekçi'ye karşı şut değeri nokta başına tikte bir kez (değerler aynı); sonraki hamle formülünde boşluk payı kaldırıldı (zincir payı duruyor). Eski ve yeni 900 tikte 8,4–10,5 sn bandında (ölçüm gürültülü; fark ~%5–15).
- 12 maç etiketli (önce · önceki sürüm · şimdi): pas kaybı 94 · 135 · 101; 30–50 br 27 · 39 · 28 (beklenen 32). Kalan: 50 br'den uzun paslar 40 pasta 12 kayıp (beklenen 8).
- 24 maç (Y12 · önceki · şimdi): pas başarısı %89,1 · %87,4 · %89,4; pas kaybı 238 · 281 · 229; toplam top kaybı 396 · 426 · 391; şut 257 · 238 · 256; gol 23 · 23 · 21. Bütün yeni bayraklar eski → Y12 birebir. Kısa çökme denemesi ve maç işçisi hatasız.

## 18 Ekim 2026 · Pasın değeri genel: zincir, savunmanın kayması, boşluğa pas (main e7e6b04) — maçlar değişti
- Menajer: pasın değeri yalnız 'oradan şut ihtimali' olamaz; aşağıya verilen pas da Tuna'ya gidebilir (iki-üç adımlı oyun); kanada pas savunmayı genişletir, hatlar arasında boşluk açar. Bu genel düşünülmeli.
- Bulgu: tahmin dünyasında savunma topa doğru tek noktaya çöküyordu (topa en yakın iki savunmacı dışındakiler topla kale arasındaki %45 noktasına koşuyordu); aralar hiç açılmıyordu. Konum değeri şut açısı olmayan her yerde yalnız kaleye uzaklık (0,06 + 0,28 × tehdit); zincirin her halkası ~0,07. space (boşluk) top nerede olursa olsun 0,8–1 veriyordu (Bekçi boşluk noktasına göre yerleşmemiş, pas yolu yok).
- `ALAN_TAHMIN_DIZILIS_ESKI`: topa çıkan ikisi dışındaki savunmacılar topun yeni yerine göre maçın kendi diziliş kuralıyla (slotBase: çizgi topun derinliğiyle, blok 22 br derin, top yana gidince 0,4 kayar; adam adama: adamının kale tarafı) yerleşir, süresi yettiği kadar.
- `ALAN_ZINCIR_ESKI`: her arkadaş için 'top ona varınca' dünyası (sert pas süresiyle); arkadaşın zincir değeri = en iyisi(kendi değeri, 0,9 × o dünyadan en iyi pas: yol açıklığı × alıcının değeri, alıcıya en çabuk varan iki savunmacı çıkmış, gerisi kaymış). Pas seçeneğinde alıcının değerine 'alıp arkadaşa verme' payı eklenir (karar + alıcının pası + arkadaşın pası = üç pas); sonraki hamle formülünde de. Boşluğa pas (`bosluk`): kale önündeki noktalara bir arkadaşın savunmadan önce varması × pas yolu × oradan şut (Bekçi yerleşmiş).
- `ALAN_VEND_BOSLUK_ESKI`: ön elemede space yerine bosluk. `ALAN_PAS_MENZIL_ESKI`: pas menzili sabit 50 br (ayağa 55) değil topun fiziği (80 br'ye kadar aday; varamayan launchFor'da elenir). `ALAN_PAS_SERT_ESKI`: 12 br'den uzun pasta sert pas da aday (yumuşak 50 br'lik pas 79 tikte varıyor).
- 209182 kurulan durum: Tuna'ya ayağa sert pas artık aday (P %99, 0,12; önce hiç üretilmiyordu), zincir Tuna'yı en değerli halka görüyor (0,16).
- 24 maç (önce · sonra): gol 23 · 23, şut 257 · 238; pas başarısı %89,1 · %87,4; pas kaybı 238 · 281; toplam top kaybı 396 · 426 (ilk 12 maçta 215 · 204, ikinci 12'de 181 · 222). Aynı 12 maçı yalnız iki bayrak (menzil, sert pas) eski oynatınca toplam 265, gol 16: 12 maçlık sette fark ±40 kayıp, ±7 gol oynuyor; değişiklik gürültünün içinde. Bütün yeni bayraklar eski → önceki sürüm birebir. Hız: 900 tikte 7,7 → 11,6 sn (tik başına ~13 ms); maç işçisi 15 sn'de 1440 tik. Kısa çökme denemesi hatasız.

## 18 Ekim 2026 · Kanattaki boş arkadaş, rahatken top kaybı (main fc053cc) — maçlar değişti
- Menajerin maç dosyası (209182, 01:12–01:15): orta sahada topu tutan Deniz, sağda bomboş duran Tuna'yı hiç düşünmüyor; baskı yokken topu Taş'a kaptırıyor. Maç node'da birebir yeniden üretilemedi (hiçbir sürüm 01:05'teki olayları vermedi; büyük ihtimalle iPad tarayıcısının matematik fonksiyonları V8'den son basamakta farklı, maç bir süre sonra ayrışıyor). Durum ekran görüntülerinden ölçülen konumlarla kuruldu (scratch `/tmp/kk-kur-durum.js`).
- `ALAN_SONRA_BEKCI_ESKI` (asıl sorun): sonraki hamle formülü (sonraDeger) arkadaşa pası, Bekçi topun şimdiki yerine göre dururken ölçüyordu; kanattaki Tuna ortada duran Bekçi'ye karşı açık şut açısı görüyor, 'sür, sonra Tuna'ya pas' 0,41 çıkıyordu. Pasın kendisi (Bekçi alış yerine göre kaymış dünyada) 0,08. Deniz her kararda 'biraz sürüp sonra veririm' deyip veremiyor, sürmeye devam edip topu kaptırıyordu. Artık sonraki hamlede de Bekçi, topun alıcıya varışı ve şut hazırlığı boyunca alıcıya göre kayar (kaymanın kuralı tek yerde: `bekKay`).
- `ALAN_BEKCI_SUT_ANI_ESKI`: tahmin dünyasında Bekçi yalnız uçuş süresi kadar, tepkiden sonra kayıyordu; alış noktası 1–2 br değişince kanattan şut değeri 0,09'dan 0,62'ye sıçrıyordu (aynı hedefe P %55'lik sert pas, P %93'lük pastan değerli). Bekçi topu sürekli izler (yerleşmede tepki yok) ve şut hazırlığı boyunca da kayar (`sutHazir` 5 tik; maçta top alındıktan sonra şut en erken 4–6 tikte çıkıyor, ölçüldü).
- `ALAN_FAKE_SURE_ESKI`: kimse aldanmayan fake 'şimdiki en iyi seçenek' kadar değerliydi, sonraki ince ayarlar (tercih payları) ona uygulanmıyordu; %0 inanılırlıklı fake en iyi seçenek çıkıyordu. Fake artık beklemenin bedelini öder (tempo × süre, `tut` ile aynı) ve bütün paylardan sonra tartılır.
- Kalan (dokunulmadı, menajere soruldu): kanattaki boş oyuncunun değeri düşük (~0,09): konum değeri oradan şut ya da önündeki boşluk; kanattan içeri oyun sayılmıyor. Alıcının sonraki hamlesini (sonraDeger) değere katmak bu durumda Tuna'yı 0,09'dan 0,095'e çıkardı (Kaan markajlı, Mert geride). Deniz–Tuna o an ~51 br, doğrudan pasın sabit sınırı 50 br (`passMax`).
- 24 maç (3 kurulum × 8 tohum, 4500 tik; önce · sonra): pas başarısı %86,8 · %89,1; pas kaybı 263 · 238; ikili kaybı 114 · 56; dürtme 70 · 53; fiske çarpması 65 · 49; toplam top kaybı 512 · 396; şut 199 · 257, gol 21 · 23, Bekçi'nin tuttuğu şut 57 · 92; fake 18 · 9. Üç bayrak birlikte açılınca önceki sürüm birebir geri geliyor (3085, 4500 tik). Kısa çökme denemesi ve maç işçisi hatasız.

## 18 Ekim 2026 · Bekçi'nin çelme yönünü pasçı ve şutçu bilemez (main 0b54e86) — maçlar değişti
- Menajer: seken topu düşünerek şut çekmek saçma; şutçu Bekçi'nin topu nereye çeleceğini bilemez. Formülün sekme yarışı (sekYaris) Bekçi'nin maçta seçeceği yönü (14 br ileride hücumcuya en uzak nokta) biliyormuş gibi sayıyordu. Kenardan şutta Bekçi topu şutçunun tarafına çeliyor, formül sekmeyi %99,5 bizim görüyor, gol ihtimali 0 olan şut 0,80 değerle seçiliyordu (Tuna, 618499 t2252; P %96'lık pas 0,40).
- `ALAN_BEK_YON_ESKI`: Bekçi'nin sekmesinin payı, çelebileceği yönlerin (kalesinden dışarı −60°, −30°, 0°, 30°, 60°) ortalaması. Aynı anda şut 0,22'ye iniyor, Tuna Kaan'a pası seçiyor. Saha oyuncusunun sekmesi değişmedi (gövdeden yansıma ve hücum yönüne çevirme).
- Bulgu (dokunulmadı): maçta Bekçi çelme yönünü yalnız 14 br ilerideki noktaya bakarak seçiyor, topun sonra yuvarlanacağı yolu görmüyor; aynı şut 200 kez oynatılınca seken top %73 hücumcuda kalıyor.
- 24 maç (3 kurulum × 8 tohum, 4500 tik; önce · sonra): şut 236 · 199, gol 24 · 21, Bekçi'nin tuttuğu şut 90 · 57; pas başarısı %87,2 · %86,8; pas kaybı 257 · 263; toplam top kaybı 437 · 442 (ikinci 12 maçta %87,1 · %87,9, ilk 12'de %87,3 · %85,7: fark gürültü içinde). %80–95 bandı iki sürümde de beklenen %89, gerçek ~%80 (duran bulgu). Kısa çökme denemesi ve maç işçisi hatasız.

## 18 Ekim 2026 · 618499'un ilk 40 saniyesi (main 5370bb5) — maçlar değişti
- Menajerin maç dosyası (618499, iki takım da 20'lik): ilk 40 saniyede peş peşe saçma kayıplar. Maç tik tik izlendi (scratch `/tmp/kk-top-yakin.js`, `kk-poz.js`, `kk-ufd2.js`: formülün aday başına yakalama payları, `kk-rv.js`: alıcı değerinin parçaları, `kk-fk.js`: fake'in iki dünyası, `kk-mc.js`: aynı andan 200 farklı zarla oynatma). Beş kök:
- `ALAN_BEKCI_FISKE_ESKI`: Bekçi kendi Kuyu alanında fiske yapıyordu; %2'lik kaçan fiske kendi kalesine yuvarlandı (0-1). Bekçi artık fiske yapmaz.
- `ALAN_ONCEDEN_TAZE_ESKI`: alıcı kararını top gelmeden 5–9 tik önce, tahmini dünyada veriyordu; top ayağa gelince o karar (durum çok değişmediyse) uygulanıyordu. Şimşek'in önceden kararı 'Rüzgâr'ın önüne', o an verilse 'sür' çıkıyordu; pas Rüzgâr'ın 7 br önüne gitti, kesildi. Karar artık o anki dünyayla aynı tikte yeniden verilir (gecikme yok); önceden karar yalnız tek dokunuşa hazırlık için.
- `ALAN_SEK_ATAN_ESKI`: formülün sekme yarışında pası atan, dibine dönen seken topu hemen alıyor sayılıyordu; maçta atan topa 8 tik dokunamaz. Ece'nin Tuna'ya pası 1,7 br'deki Bulut'un gövdesinden geçiyordu: formül Bulut'un %100 değeceğini görüyor ama sekmeyi Ece'nin alacağını sanıyordu (P %97). Düzeltmeyle aynı pas %45; Ece sürmeyi seçiyor.
- `ALAN_BEKCI_YER_ESKI` (en büyüğü): seçeneklerin tahmin dünyasında (predictWorld, sonDunya) Bekçi olduğu yerde kalıyordu. Top orta sahadayken kale önünden 9 br çıkmış Bekçi'ye göre köşeden şut değeri %78, kale önünden %11 çıkıyordu; köşeye atılan önüne pasın alıcısı bu yüzden çok değerli görünüyordu (başlama vuruşunda Ece aynı pası beş şiddette denedi: %99'luklar 0,05, köşeye kaçan %68'lik 0,49 değer aldı; top Tuna'yı geçti, Rüzgâr kesti). Bekçi artık topun yeni yerine göre maçtaki yerleşim noktasına (AlanShape.bekci) kayar, kayma hızıyla, süresi yettiği kadar. Yerleşmiş Bekçi'ye karşı şut değeri her yerde ~0,1.
- `ALAN_FAKE_IKILI_ESKI`: fake süresince taşıyıcı yerinde durur; kimse aldanmazsa dünya 'fake yapmamak kadar' sayılıyordu (bedeli yok). Dibinde iki presçi varken Mert 9 tik durdu, Ayaz ikiliyle aldı. Artık aldanmayan dünyada fake boyunca topu tutma ihtimali keepP(T), bir kısmı aldanırsa yarısı; kaybın bedeli taşıyıcının yerinde.
- Bakılıp dokunulmayan: kalecinin çeldiği şutun sekmesini formül %83 bizim sayıyor; aynı andan 200 kez oynatınca %73 bizim, %20 Bekçi tuttu, %5,5 gol — formül doğru, şut 'seken topa pas' olarak gerçekten değerli. Alıcının topu tutma ihtimalinde 8 tik ufku (6 br'deki rakip hiç sayılmıyor, 5 br'deki tam) duruyor.
- 618499 yeniden: ilk 80 saniyede el değiştirme 12 → 4 (sonraki sürümde ilk 40 saniyedeki kayıplar: baskı altında P %36 sürme, P %72 pas).
- 12 maç (Y9 · şimdi): pas başarısı %85,4 · %87,3; pas kaybı 149 · 129; 'kesin' (≥%95) bant gerçek %95 · %96; %80–95 %86 · %88; önüne beklenen/gerçek %89/%88 · %88/%90; kenardan %77/%77 · %81/%84; ayağa %94/%92 · %94/%92; ikili + dürtme 88 · 90; fiske çarpması 25 · 35; toplam top kaybı 262 · 254; şut 94 · 129, gol 8 · 15, Bekçi'nin tuttuğu şut 30 · 52. Beş bayrak birlikte açılınca Y9 birebir geri geliyor. Kısa çökme denemesi (788458, 439121; 600 tik) ve maç işçisi (618499) hatasız.

## 18 Ekim 2026 · "Kesin" sanılan paslar (main ac51228) — maçlar değişti
- Menajerin isteği. Araç: scratch `/tmp/kk-kesin.js` (karar anına dönüp seçilen seçeneğin formül kaydı: kim ne kadar yakalar, en iyi payı; maçta alıcının işi ve hedefi tik tik; kesenin yeri). Dört kök bulundu:
- `ALAN_CIKIS_ESKI`: maçta her vuruş savurmadan geçer, formül yalnız göndermede sayıyordu. Kenardan/kendine pasta gecikme ve savurmada kaptırma yoktu; top bütün paslarda oyuncunun şimdiki yanından çıkıyordu (bırakma açısı değil: dibindeki presçi 10 tik uzak sanılıyordu). Artık savurma planı her vuruşta, top bırakma açısından (taşıyıcı savururken yarı hızla yürür) çıkar; pasın planı karardan (`D._savCa`).
- `ALAN_MENZIL_PLAN_ESKI`: maçın alıcı ve kovalama planı sabit 1,4 br menzille; temas kuralında arkadaşın menzili Uzanma'yla (20'de 2,0), rakibinki topun yaşıyla büyür. Formül gerçek menzili saydığından alıcıyı topa 0,1–1 tik önce yetişir görüyor, maçtaki alıcı 'yetişemem' deyip 15–40 br ileri koşuyordu. Planda gerçek menzil.
- `ALAN_ATAN_BEKLE_ESKI`: kendine pasta 'ilk yetişilen nokta' atanın ayağının dibi çıkıyordu; atan topa ilk 8 tik dokunamaz, 8 tik bekliyordu. Atanın planından o süre çıkarıldı (formül de öyle sayıyor).
- `ALAN_TEK_VARIS_ESKI`: formülde varış belirsizliği tik başına bağımsız zar gibi toplanıyordu; her tik 0,6 tik geç kalan alıcı, top yavaşlayınca birkaç tikte %97 alıyordu (yüksek bantlar iyimser, düşükler karamsar). Artık oyuncu başına tek belirsizlik: o ana kadar almış olma ihtimali sgm(en iyi pay / τ), tik başına yakalama bunun artışı. Şut karşılaştırması değişmedi.
- Denenip bırakılan: varış belirsizliğini süreyle büyütmek (τ + κ·t, κ 0,04/0,08): pas %84 → %81, 'kesin' bant değişmedi.
- 12 maç (önceki · şimdi): pas başarısı %83,1 · %85,4; pas kaybı 171 · 149; 'kesin' (≥%95) %91 · %95; %80–95 bandı beklenen/gerçek %89/%80 · %90/%86; önüne %88/%82 · %89/%88; kenardan %89/%82 · %77/%77; ayağa %92/%94 · %94/%92; toplam top kaybı (pas + fiske + ikili) 302 · 263 (ilk sürümde 388). Kısa çökme denemesi ve maç işçisi hatasız.

## 18 Ekim 2026 · Alıcı ve pas kuralları: kısa yolda atalet, ilk noktada buluşma, tek alıcı, arkadaş pası bırakır (main 84ce4fe) — maçlar değişti
- Menajer: denenip yüklenmeyen dört düzeltme neden yapılmıyor. Dördü birlikte pası kötüleştiriyordu (%83 → %79) ve arkadaşın gövdesine çarpan kayıpları artırıyordu (10 → 23); örnekler: top arkadaşın gövdesinden sekiyor (temas kuralı ona dokundurmuyor ama gövdesi durduruyor), sekme `defl` olunca arkadaş hazırlıksız dokunuyor (%2–3 tutma), top rakibe. Kök: arkadaş topu bırakıyor ama yolundan çekilmiyordu.
- `ALAN_ARKADAS_GOVDE_ESKI`: alıcısı belli, sekmemiş pası alıcı dışındaki arkadaşın gövdesi durdurmaz (govde ve govdeIt; temas kuralıyla aynı koşul). Şut etkilenmez. Formüle arkadaş gövdesi eklemek (denenen) gereksiz kaldı.
- Geri gelenler: `ALAN_VARIS_KISA_ESKI` (gec yürünecek yola göre: maçın ivme kuralından n tikte alınan en uzun yol saklanır, s birim yolun süresi ondan; menzil içinde kayıp yok), `ALAN_YAKIN_TEMAS_ESKI` (formülde menzildeki rakip hemen dokunur, tepki süresince koşusunu sürdürür), `ALAN_ALICI_ERKEN_ESKI` (alıcı ilk yetiştiği noktada; ilerideki boş nokta ancak savunma payı ≥ rcvMarginT), `ALAN_ALICI_TEK_ESKI` (alıcısı belli sekmemiş pasta başka arkadaş 'pası o alır' diye öne geçmez).
- 12 maç (3 kurulum × 4 tohum, 4500 tik; ilk sürüm · fiske+kendine · hepsi): pas başarısı %83,6 · %82,9 · %83,1; 'kesin' (≥%95) %91 · %92 · %91; pas kaybı 183 · 179 · 171; arkadaş gövdesinden kayıp 10 · 9 · 5 (kalanı alıcının tutamayıp kendi gövdesinden sektirmesi); fiske 96 · 19 · 29; ikili kaybı 109 · 93 · 102; toplam 388 · 291 · 302. Kenardan %75 · %80 · %82. Kısa çökme denemesi (788458, 439121; 600 tik) ve maç işçisi hatasız.
- Duran bulgu: önüne pasta beklenen %88, gerçek %82; kenardan %89 / %82.

## 18 Ekim 2026 · Ucuz top kayıpları: fiske yolu, kendine pasta koşu (main 880e817) — maçlar değişti
- Menajerin gözlemi: hepsi 20'lik oyuncular baskıdan, savunmanın iyiliğinden olmayan basit pas ve top kontrol hataları yapıyor. Sayım (scratch `/tmp/kk-kayip.js`: pas kayıplarını türüne ayırır, karar anındaki beklenen P ile gerçek sonucu bantlar; `kk-kesik.js` karar anına dönüp formülün iç sayılarını verir; `kk-onune.js` önüne pasta hedef / alıcının buluşma noktası / kesilme yeri). 20'likte pas açı ve hız hatası zaten 0; alıcının kontrol hatası yok denecek kadar az.
- **Fiske** (`ALAN_FISKE_YOL_ESKI`): en büyük ucuz kayıp. Taşıyıcı Çekirdeği presçiden uzak yana hızla çevirirken (fiske) yön yalnız dönüşün orta noktasına göre seçiliyordu; Çekirdek presçinin gövdesine çarpıp boşa çıkıyordu (3 dk'da ~8). Dönüş yolu taranır: dibindeki (3 br) presçi gövde etrafında topun peşinden tam hızla dolanır (maçın dolanma kuralı), uzaktaki hızıyla sürer, gövdeler iç içe girmez (1,7). Yol kapalıysa öbür yönden, ikisi de kapalıysa fiske yok (Çekirdek gövdede kalır, ikili işler); fiske başladığı yönde sürer.
- **Kendine pas** (`ALAN_KENDINE_KOS_ESKI`): top atanın 3 br'sindeyken savunma onu top sahibi saymayı sürdürüyor (`ALAN_KENDINE_PRES`), ama atanın kendisi de top sahibi sayılıp eski hedefinde 4–6 tik bekliyordu; dibindeki rakip topu alıyordu. Artık hemen alıcı kuralıyla buluşma noktasına koşar.
- 12 maç (3 kurulum × 4 tohum, 4500 tik, hepsi 20'lik; önce · sonra): fiske çarpması 96 · 19; ikili kaybı 109 · 93; pas kaybı 183 · 179 (başarı %83,6 · %82,9, gürültü ±1,2; kenardan %75 · %80; 'kesin' sanılan ≥%95 paslar %91 · %92); toplam top kaybı 388 · 291.
- **Denenip yüklenmeyenler** (kod scratch `denenen/` ve `yama-*.py`; 12'şer maçta pası kötüleştirdi): kısa yolda atalet (gec yürünecek yola göre: menzil içindeki noktaya 12–21 tik dönüş kaybı ekleniyordu; düzeltilince savunma da topa daha iyi yetişiyor, pas %83 → %80); formülde menzildeki rakibin anında teması ve alıcı dışındaki arkadaşın gövdesi (eski simOne'da vardı); alıcının ilk yetiştiği noktada buluşması (yolun 10–30 br ilerisindeki boş noktaya koşuyordu); alıcısı belli pasta başka arkadaşın 'pası ben alırım' demesi (temas kuralı ona dokundurmuyor). Her biri tek tek doğru görünüp toplamda pas başarısını 1–3 puan düşürdü, arkadaş gövdesine çarpan pası 10'dan 15–23'e çıkardı. Bulgu olarak duruyor: önüne pasta formül alıcıyı topu ilk yetiştiği yerde alıyor sayıyor (beklenen %88–90, gerçek %81–84), kenardan pasta %86–89 / %70–80.

## 18 Ekim 2026 · Maç ekranı: oynatma hızı çubukla (main 7899281)
- Menajerin isteği: hız 0–4 arası istenen değere. canli.html'de altı düğme (0,25×, 0,5×, 1×, 1,5×, 2×, 4×) yerine kaydırma çubuğu (0–4×, 0,05 adım) ve yanında seçilen hız; 0× oynatmayı durdurur. 1× önceki gibi saniyede 30 tik (içeride hiz = × / 2). Tarayıcıda (390 px) denendi: 1×'le başlıyor, 2,75× / 0× / 1,5× seçiliyor, taşma yok, hata yok; yardımcısız maç işçisi tarayıcıda maçı hesaplıyor.

## 18 Ekim 2026 · Paralel yardımcılar kaldırıldı (main 97b005b) — formül sisteminde davranış aynı
- Menajerin isteği. Formül sisteminde yardımcı işçilere iş düşmüyordu (kafada gönderme ve aldatma dünyası görevleri yok). mac-isci.js yardımcı açmıyor, açılmalarını (en çok 1,5 sn) beklemiyor; kuyruk bozulunca paralelsiz baştan hesap kuralı da gitti. `paralel.js`, `yardimci-isci.js` silindi; kuyruk görevleri (yalnız eski sistemde, `ALAN_FORMUL_ESKI`) aynı işçide sırayla. `_headers` (yalnız SharedArrayBuffer için bütün siteye konan çapraz köken yalıtımı) silindi. Maç ekranındaki hesap yazısından 'N çekirdek' çıktı. Laboratuvarın kendi işçileri (`lab-isci.js`, maç başına) yerinde.
- Deneme: maç işçisi node'da işçi gibi (kur + yeni maç) 15 sn'de 1440 tik, hata yok; 909623 300 tik hatasız. Eski sistem artık tek çekirdekte (300 tik kp-336: 3 yardımcıyla 39 sn, tek çekirdekte 52 sn).

## 17 Ekim 2026 · Aşırtma koddan çıktı (main ea7ea34) — davranış aynı
- Menajerin isteği: oyunda aşırtma yok, adı da kalmasın. Seçenek hiçbir yerde üretilmiyordu; kalan kontroller (decide.js: kaba eleme, pas türü, tercih, risk kazancı, aile sayımı), maçtaki sayaç (`m.st.lob`) ve olay metni, paralel tür listesindeki adı (`TUR`) kaldırıldı. Maç birebir aynı (kp-336, 300 tik); eski sistemde (`ALAN_FORMUL_ESKI`) tek çekirdek ile 3 yardımcı aynı (1042 paralel iş).

## 17 Ekim 2026 · Formül sistemi: iç içe karar kalktı (main af13ad7) — maçlar değişti
- Menajerin onayıyla: aldatmanın iki dünyasında (hepsi aldandı / kimse aldanmadı), beklemeden ve sürmeden sonra tam bir ikinci karar verilmiyor (hesabın ~%85'iydi). Yerine `sonraDeger`: buradan sonra yapılabilecek en iyi hamle, kararın kendi ölçüleriyle — gönderme (shotQ; kaçarsa top rakibin kale önünde), her arkadaşa pas (laneOk × alıcının değeri, recvVal ile aynı), önüne pas (arkadaşın 8 br önü, hücum ve koşu yönü; arkadaş ve top savunmadan önce varır mı, space'teki yarış), beş yöne sürme (8 br, düz 14 br; kararın sürme dünyası sonDunya gibi: iki savunmacı cevap, gerisi yerleşir; varılan yerin değeri ya da önündeki boşluk — boşluk değeri Vend yalnız dizinin sonunda; Bekçi'yle ikili). Deney: `self.ALAN_SONRA_FORMUL_ESKI`.
- Kararın kendi hesabı değişmedi: Bekçi'yle ikili tek fonksiyona (`bekDuelP`), risk tek fonksiyona (`riskOf`) alındı, aynı işlemler.
- Kenardan pas da sonraki değerde (main 0007875; menajer: aşırtma oyunda yok, kenardan olmalı): kararın koşullarıyla (Okuma 8+, doğrudan hat bankIf'ten zayıfsa en tehlikeli bankN arkadaşa; duvardaki ayna noktası, iki bacağın laneOk'u; alıcı şarjı −0,05). Kendine kenardan: önünde/yanında 12 br'de rakip, duvara 9 br'den yakın, 7/10/13 br × 0/2,5/5 kayma, koşu çizgisi ve topun yolu açık, buluşma noktasına boşluk yarışı. 600 tikte (909623) arkadaşa 1513, kendine 125 aday tartıldı.
- İlk denemede sürme Vend'le (boşluk dahil) tartıldı: beklemeden sonrası 0,86'ya çıktı; kararın ölçüsüne dönüldü (0,29–0,6, eski iç içe kararda 0,47–0,6). Formülde çalım (kenardan çalım dahil) yok.
- Hız: 300 tik 3085 22 → 3 sn, 909623 18 → 4 sn (ilk sistemle 48 sn). Kısa çökme denemesi (3 maç, 300 tik) hatasız; tek çekirdek ile 3 yardımcı aynı (kp-336, 300 tik). Paralel yardımcılara artık iş düşmüyor.

## 17 Ekim 2026 · Formül sistemi: kafada oynatma kalktı (main 8deca33) — maçlar değişti
- Menajerin isteği: kafada dünya oynatma (uçuş, bekleme, sürme kopyaları) makul değil ve bir gün bırakılacak; tablo da istenmiyor. Eski hali koru, hepsini formüle al. Eski sistem `kafa-oyunu` dalında (a1befbd) ve `self.ALAN_FORMUL_ESKI = true` ile geri gelir.
- Pas/şut uçuşu (`imagine` → `ucusFormul`): topun yolu fizikle (predict, duvar/direk dönüşleri). Nişan hatası üçgen dağılım, üç noktalı ortalama. Her tik, her aday (rakipler + alıcı) için yol parçasına en yakın uzaklık (maçın temas kuralı, uzaklaşan topa awayK, menzil en az 1,15). Varış: tepki + dönüş (gecOf) + koşu; olasılık sigmoidle. Bekçi: dalış menzili (yolun yaşıyla) + uzun uçuşta kayma (koşu hızının %40–80'i), temas yumuşaklığı 0,12. Dokunuş: tutma (ctrlP), delip geçme (thruP; gövdeye, 1,0 br altına çarpan top geçmez), oyuncu kendi delinen payına bir daha dokunmaz. Savurma süresince kaptırma (keepP).
- Çelinen top (`ALAN_SEK_ESKI` → eski rbAt): maçın sekme kuralı (gövdeden yansır, hücum yönüne çevrilir; Bekçi rakipten uzağa çeler, en az 0,8 hız). Sekmenin yolu predict'le; yol boyunca topu ilk kesen takım alır (serbest top tepkisi, dokunanın bekleme süresi). Bekçi alanının dışına ancak rakipten belirgin önce varacaksa çıkar (shape.js kuralı, yolun sonundaki yarışta da). Sekme Kuyu'ya giderse kimse kesemezse gol.
- Konum değerinde şut (`ALAN_SUT_FORMUL_ESKI` → eski tablo): kapalı formül; savurma planından nişan hatası, isabet (üçgen dağılımın birikimlisi), Bekçi'nin yol noktasına topun varış süresinde dalış + kayma menzili, gövde ve delip geçme. Boşluk taramasındaki tablo üst sınırı da kalktı.
- Bekleme/sürme (`ALAN_BEKLE_FORMUL_ESKI`): maç kopyası (holdWorld) yerine seçeneğin bitiş dünyası (sonDunya); sonraki hamle aynı karar fonksiyonuyla bu dünyada. Sürme kopyası yok; tutma ihtimali keepP (kopya gözlemi olmadığı için (keepP + 1) / 2 güncellemesi de yok).
- Gerçek motorla karşılaştırma (scratch `kk-sut-gercek.js`: aynı şut maçta 60 kez, gol/biz/onlar · formül): 10 br 0/1,00/0 · 0/,98/,02; 20 br ,18/,82/0 · ,13/,85/,02; 25 br ,17/,82/,02 · ,13/,85/,02; dar açı (15 br, y18) 0/,10/,90 · 0/0/1,00; 20 br y42 ,15/,82/,03 · ,14/,80/,06. Konum değeri: 10–15 br 0 (maç 0), 20–25 br ,10 (maç ,17).
- Hız: 300 tik (3085) 48 → 22 sn. Kısa çökme denemesi (3085, 788458; 300 tik) hatasız; tek çekirdek ile 3 yardımcı aynı (kp-336, 150 tik). Oynanış sınaması menajerde.

## 17 Ekim 2026 · Kalan denetim bulguları (main) — maçlar değişti
- Menajerin isteği: dokunulmayan denetim bulgularının hepsi düzeltilsin (hız ve oynanış sınaması menajerde).
- Uçuş süresi (`ALAN_UCUS_ESKI`): pas değerlendirmesinde savunmaya ve Bekçi'ye verilen süre topun gerçek uçuşu (reachTime: maçın top kuralı, alanların direnci; kenardan pasta iki bacak, sekme kaybıyla; savurma süresi eklenir). Önce |v0| × 0,7: yumuşak pasta süre yarı yarıya kısa. Boşluk yarışı, buluşma, hat önü, boşluk değeri (space/front), pas yolu açıklığı (laneOk, destekte laneOpen): boş sahada fizikten tablo `ucusT(L, varış hızı, yol noktası)` (25 br: sert 26, normal 51, yumuşak 76 tik; önce hepsi 23).
- Seçenek dünyası (`ALAN_SON_DUNYA_ESKI`): her seçenek kendi süresinde (pas: uçuş; sürme: uzunluk / topla koşu hızı; bekleme: süresi) varış noktasına göre kurulan dünyada tartılır (en çabuk varacak iki savunmacı cevap verir, gerisi yerleşir, arkadaşlar yerlerine gider). Sürmenin tutma ihtimali ve şarjı da kendi süresiyle. Önce her seçenek topun şimdiki yerine göre T + 8 tiklik aynı dünyada, sonra en umutlu 6'ya yerleşme farkı bir daha ekleniyordu; yerleşme düzeltmesi artık yalnız çalıma.
- Alış/kayıp yeri (`ALAN_ALIS_ORT_ESKI`): kafadaki denemelerin bitişi ayrı ortalanır: alış yeri arkadaşın aldıklarından, kayıp yeri rakibin aldıklarından (önce hepsinin ortalaması).
- Kopya durumu (`ALAN_KOPYA_DURUM_ESKI`): kafadaki yeni kopyalar (bekleme, uçuş) maçın şekil/kazanma anı/geçiş durumu ve top sahibinin son seçenekleriyle başlar; önce ilk tikte top el değiştirmiş sayılıp pres kilitleri ve geri koşular siliniyor, savunmanın tehlike okuması kapalı kalıyordu. İleri bakış (kapalı) derin kopya ve mutlak tikle (`ALAN_BAKIS_KOPYA_ESKI`).
- Yalnız kısa çökme denemesi (300 tik, 2 maç, hata yok) ve tek çekirdek ile 3 yardımcı aynılığı. Bu denemede hesap öncekinden belirgin yavaş göründü (seçenek başına dünya); menajer hız sorunu olursa söyleyecek.

## 17 Ekim 2026 · Benzer hatalar: değer tutarlılığı, atalet, kopyada yabancı bağlar (main) — maçlar değişti
- Menajerin isteği: Efe → Volkan hatasının benzerleri başka yerlerde de olmalı. Üç okuma denetimi (değer fonksiyonlarının tutarlılığı; varış sürelerinde atalet ve sabit top hızı; kopyalarda gerçek maçtaki oyuncuyu gösteren bağlar) ve maçlarda belirti sayımı (scratch `kk-belirti.js`).
- Değer: `ALAN_CEVAP_TEK_ESKI` sürme/bekleme de pasla aynı rakip cevabıyla (eski cevap en yakın iki savunmacıyı şut yolunu kusursuz kesen yere koyuyordu: sürmenin dağınıklık payı silinip şutu 0,15'e iniyordu); `ALAN_RISK_SUR_ESKI` kontra temposu ve risk kazancı ileri sürmeye de; `ALAN_SUR_KAYIP_ESKI` sürme kopyası tutma ihtimalinin yerine geçer ((keepP + kalan) / (1 + deneme)), önce ikisinin küçüğü (kayıp iki kez); `ALAN_SONRA_KAYIP_ESKI` kopya topu kaybetmeden atlattıysa sonraki hamle (keepP + 1) / 2 ile; `ALAN_RUNON_DAGINIK_ESKI`, `ALAN_DESTEK_DAGINIK_ESKI` önündeki boşluk ve destek yerinin açtığı da dağınıklıkla.
- Atalet (maçın ivme kuralı, AlanShape.gec; çok çağrılan döngülerde oyuncu başına yön tablosu gecTab/gecOf, aynı değerler): `ALAN_ALICI_ATALET_ESKI` alıcının buluşma noktası (alıcı ve rakipler; maçta ve kafada aynı fonksiyon); `ALAN_KOVALA_ATALET_ESKI` topa yarış (kim kovalar/vazgeçer, şekil değişimi); `ALAN_PRES_ATALET_ESKI` presçi seçimi; `ALAN_BEDEL_ATALET_ESKI` tek tartıda iş bedeli; `ALAN_SIDDET_ATALET_ESKI` önüne pasın şiddeti; `ALAN_VARIS_RAKIP_ESKI` boşluk yarışında rakibin varışı da.
- Kopya bağları: `ALAN_BEKLE_ONBELLEK_ESKI` bekleme kopyasının önbelleği yalnız aynı tikte (maçın oyuncu dizisi bütün maç aynı nesne: taşıyıcı 5–13 tik sonra eski tikin bekleme dünyasını alıyordu; zar yalnız ıskalayınca çekildiği için anlık kopyadan yeniden hesap ayrışabiliyordu); `ALAN_BAG_KOPYA_ESKI` tutQ/tbPrev/_atQ bu maçın oyuncusuna (kopyada pres seçiminin 'adamını bırakma' bedeli işlemiyordu), cevapta markRef tahmin dünyasındaki yeriyle, kafadaki Bekçi temasında atan/alıcı algı kopyası. Bekleme kopyası hatası sayılır (davranış aynı; önce 'top tutuldu' okunup görünmüyordu).
- Denetimde bulunup yapılmayanlar: ileri bakış (`D.look`, kapalı) açılırsa kopyalar gerçek oyuncuların geçmişini bozar; değer ufukları (sürmenin her uzunluğu aynı T), yerleşme düzeltmesinin çift sayımı, alıcı değerinde im.end ortalamasına kesilen denemeler; top uçuş süresi (|v0|·0,7, yumuşak pasta 2 kat kısa); boşluk yarışında top 1,1.
- Ölçüm (6 maç, ilk 60 sn, eski · yeni; maçlar ayrışıyor, örnek küçük): kafa/bekleme hatası 0 · 0; koşana yanına/gerisine pas 25 · 23; alıcının koşusunu bozması 37 · 35; yapışık arkadaş 9 · 7; kontrada geri/yan seçim 7 · 4; şut 28 · 19 (gözlenecek). Hesap ilk sürümde ~%35 yavaştı (boşluk taramasında nokta başına gec), yön tablosuyla giderildi (yüklenen sürüm; yeniden ölçülmedi). Menajerin isteğiyle ayrıca sınama yapılmadı.

## 17 Ekim 2026 · Kontra: önü boş koşucuya önüne pas (main) — maçlar değişti
- Menajerin gözlemi (maç dosyası 439121, 02:12–02:13): Efe'nin de Volkan'ın da önü bomboş; pas 'önüne' diye Volkan'ın tam üstüne gidiyor, Volkan saçma bir yere koşuyor. Sahnede (tik 3987): Alp kesti → Yıldırım → Efe (93,42). Liman'ın 5 oyuncusu topun gerisinde, önde yalnız Deniz ve Bekçi. Volkan kaleye (65,53) koşuyor. Efe'nin seçimi: 'önüne (sert) [boşluk] → Volkan (95,55)', değer 0,068: Volkan'ın 6 br yanı, biraz gerisi; Volkan koşuyu bırakıp çizgiye indi. Öndeki seçenekler: 12 br sürme 0,058; Volkan'ın koşusunun önüne pas adayı hiç yoktu.
- Nedenler: (1) konum değeri kale uzağında düz (b0 + b1·e^(−d/22): 80 br'de 0,067, 100 br'de 0,063); önündeki rakip sayısı payı (dağınıklık) yalnız V'de vardı, boşluğa pas noktalarını sıralayan, kaba elemeyi yapan ve ileri bakışın sonunu tartan Vend'de yoktu: ileri nokta (76,49) ile yan nokta aynı. (2) Boşluk noktasına varış her yöne aynı: koşan alıcının yanı/gerisi koşusunun önü kadar yakın sayılıyordu. (3) Buluşma pası yalnız en hızlı topla (1,1) ±4 tik aynı anda varılan noktayı arıyordu; kısa mesafede top hep erken, Volkan'a aday çıkmadı.
- `ALAN_VEND_DAGINIK_ESKI`: Vend de dağınıklık payıyla (V ile aynı fonksiyon `dagin`). `ALAN_VARIS_ATALET_ESKI`: boşluk noktasına varış + max(yan hız / (0,021 × Çeviklik), geri hız / (0,018 × Çeviklik) × 1,5) (maçın ivme sınırları). `ALAN_BULUS_YAVAS_ESKI`: top en hızlısıyla en çok 4 tik geç kalıyorsa buluşulabilir (erken varacaksa yavaş atılır; şiddet türleri ayrıca denenir); koşu üstünde ilk buluşulabilen nokta ve 1 sn ilerisi aday.
- Aynı an (tik 3986'ya eski kodla, karar yeni kodla): Efe 'önüne (sert) [buluşma] → Volkan (79,7, 51,4)' 0,095; Volkan topu koşarken (74,53)'te aldı, Yıldırım'a buluşma pası, Yıldırım yine Volkan'ın önüne (39,53): 5 sn'de 55 br ilerleme. Hesap: 5 tohumda 300 tik 225 → 247 sn (+%10, tohuma göre −%25 … +%60; maç akışı değişiyor). Tek çekirdek ile 3 yardımcı aynı; 909623 900 tik hatasız. *Araçlar:* scratch `kk-karar-iz.js` (durumdan tik tik, kararların bütün seçenekleri; SADECE=KARAR), `kp-devam.js` (durumdan durum).

## 17 Ekim 2026 · Adam adamada maç 'hazırlanıyor'da kalıyordu (main) — düzeltme
- Menajerin maç dosyası 788458 (Liman'da bütün rollere adam adama talimatı, duruş tahtası): maç hiç başlamadı. Tarayıcıda (yerel sunucu, işçiye giden kurulum maç dosyasınınkiyle değiştirildi) hesap 00:02'de durdu, işçi hata vermedi. Neden zinciri: tik 69'da kafadaki uçuş kopyasında markajcı Ece'nin markRef'i gerçek maçtaki Yıldırım'ı gösteriyordu (ileri bakış kopyası oyuncuları yayarak kopyalıyor, bağlar yeniden kurulmuyor); algı listesinde bulunamayınca `ileri(undefined)` hata verdi. Kopya hatası kuyruğu bozuk işaretliyor; maç işçisi maçı son anlık kopyadan tek çekirdekle yeniden hesaplıyor, hata belirlenimli olduğu için yine çıkıyor: sonsuz döngü. (Node'daki ölçüm araçları yeniden başlatmadığı için görünmüyordu.) Markaj talimatı ilk kez kullanıldığı için şimdiye dek çıkmamıştı.
- `ALAN_MARKAJ_KOPYA_ESKI` (shape.js position başı): markRef bu maçta değilse aynı kimlikli (id, takım) oyuncuya bağlanır.
- mac-isci.js: kuyruk bozulunca baştan hesap yalnız paralel açıkken (paralel kapatılır, tek çekirdekle yeniden); tek çekirdekte sürer. Önce tek çekirdekli cihazda da herhangi bir kopya hatası maçı kilitliyordu.
- Sınama: 600 tik kafa hatası 0; tek çekirdek ile 3 yardımcı aynı (bozuk işareti yok); tarayıcıda hesap ilerliyor (90 sn'de 00:22, bu sunucuda). Bu kurulum hesapta ~2 kat yavaş (300 tik: 3085 24 sn, 788458 58 sn; adam adamasız 44 sn): 'hazırlanıyor' daha uzun sürer. *Araçlar:* scratch `tarayici-kurulum.js` (maç dosyasının kurulumuyla sitede maç), `par-kur.js` (kurulumdan paralel aynılık), `kk-sure-kur.js`.

## 17 Ekim 2026 · Takım arkadaşının etrafından dolaşma (main) — maçlar değişti
- Menajerin gözlemi (maç dosyası 3085, 01:58–02:00): topu almaya giden oyuncuyla koşu atan oyuncu aynı noktaya gidiyor, üst üste biniyorlar. Sahnede (tik 3541–3616): Baran pası kesip Kartal'a verdi; Kartal pası karşılamaya, Volkan kontra koşusuna aynı yönde (151°) 3 br yan yana çıktı; Volkan'ın çizgisi Kartal'ın içinden geçiyordu. Tik 3577–3605 arası merkezler arası tam 1,70 br (gövde çapı): iç içe değil ama itişerek yapışık koştular; sonra Kartal, Volkan'a 1,7 br'den pas verdi, yolları çapraz kesişti, yine yapıştılar. Ekranda toplar gövdeden büyük çizildiği için üst üste görünüyor. Neden: oyuncu hedefe dümdüz koşuyordu; gövdeler ancak iç içe girince ayrılıyordu (govdeAyir), kimse yol vermiyordu.
- `ALAN_ARKADAS_KACIN_ESKI` (match.js move): herkesin tik başındaki yeri ve gitmek istediği hız (sıradan bağımsız). Takım arkadaşıyla 12 tik içinde gövde payına (çap + 0,3) girilecekse: o an + gövde boyu kadar sonra arkadaşın yolun hangi yanında olacağına bakılır, ters yana kıvrılınır (karşıdan geçenin arkasından, yanındakinden açılarak); erken değecekse sert (en çok ~58°). Top sahibi ve pası alacak olan yol vermez; ona yol verilir, öbürleri yarı yarıya. Rakibe uygulanmaz (pres, omuz bilerek temas).
- Aynı sahnede şimdi: en yakın 2,5 br, temas yok; Kartal topu alıp sürdü (maç oradan ayrıştı). Hesap süresinde payı %1'in altında. Tek çekirdek ile 3 yardımcı aynı, 900 tik hatasız. *Araçlar:* scratch `kp-yaz2.js` (kurulumdan durum dosyası), `kk-kp-iz.js` (durumdan tik tik iki oyuncu).

## 17 Ekim 2026 · İç taraf: savunmacı rakibin gideceği koridora gitmez (main 9c78a45) — maçlar değişti
- Menajerin gözlemi (açık top sürümü, Tuna–Baran): savunmacı hücumcu oraya gitmeden oraya gidiyor; hücumcu başka yöne gider. Sahnede tik 15–70: Baran "arkada · Tuna" ile (94, 54)'e, Tuna orta sahada (51–70, 55) iken onun kenar koridoruna, 40 br önüne yürüdü, merkezi bıraktı; Tuna tik 75'te dönüp topa gelince geri döndü. İki neden: kale tarafı hedefi Tuna'nın 1,5 sn sonraki yerinin önüne konuyordu; öne çıkmama sınırı derinliği tutup yanı o uzak hedeften bırakıyordu.
- `ALAN_IC_TARAF_ESKI`: kale tarafı işinde hedef rakibin okunan yerinden (Görüş kadar) kale ortasına doğru, aradaki pay = işaret payı + kaleye doğru koşu hızı × açık top ufku. Öne çıkmama sınırına takılınca yan, rakibin okunan yeri ile kale ortası arasındaki çizgiden (savunmacının tuttuğu hizada). Rakip yaklaştıkça çizgi kenara döner, savunmacı onunla kayar; rakip dönerse merkez açık kalmaz.
- Aynı sahnede şimdi: Baran tik 15–40 y 40–43'te (merkez), Tuna dönüp alıcı olunca da merkezde. Menajerin isteğiyle davranış sınaması yapılmadı; tek çekirdek ile 3 yardımcı aynı, 900 tik hatasız. *Araç:* scratch `kk-ikili.js` (savunmacı–rakip tik tik).

## 17 Ekim 2026 · Açık top: savunma çizgisi öne fırlamaz (main 23bfca2) — maçlar değişti
- Menajerin gözlemi (909623, tik 11, başlama vuruşu): savunmada herkes ileri koşuyor. Sahnede Fırtına'nın 6 oyuncusunun hepsi topa doğru: Alp ve Baran (arkada alan) 28 br öne, Efe ve Kartal topun gerisinden koşan Tuna ile Kaan'ın pas yoluna, topun önüne 25 br; Liman'ın dört koşucusu aynı anda kaleye koşuyor. Neden: Yüksek blokta top ortadayken arka çizgi topun 6 br gerisine (orta çizgiye) kuruluyor, taşıyıcıya baskı olup olmaması hesaba girmiyordu; koşucular 4–18 tik ileriden okunuyordu (orta sahadan kaleye koşu ~4 sn). Ofsayt yok: yüksek çizgi daha da tehlikeli.
- `ALAN_ACIK_TOP_ESKI` (açık top / kapalı top): taşıyıcıya kale tarafından 3 br içinde kimse yoksa (7 br ve ötesi tam açık) ve topu arkamıza gönderebilecek yerdeyse (kaleye uzaklık 62 × KX'ten az; 46 × KX ve yakını tam), savunmacılar bulunduğu hizanın önüne çıkmaz (açıklık oranında); koşucular 1 sn daha ileriden okunur (arka çizgi koşucunun gideceği yerin gerisine çekilir); topun gerisindeki rakibe pas yolu işi açıklık oranında değersiz. Pas havadayken: ileri pas açık, geri/yan pas kapalı (çizgi çıkabilir).
- Top açık olsun olmasın: rakip topun gerisinde mi sayılırken 1 sn sonraki yerine bakılır (ileri koşan kanat pas yolu değil, kale tarafı işi); üstüne koşan rakibe (kaleye doğru hızı 0,1–0,25 br/tik ve üstü) doğru adım atılmaz: hizasında bekler, yana kayar, rakip yaklaşınca kale tarafı hedefi geriye düşer. Pres, buluşma, karşılama, markaj, geri koşu, önde bekle muaf.
- Tik 11'de şimdi: Alp ve Baran 7 br öne (top ortada, açıklık 0,75), Efe 8 br, Kartal hizasında "arkada · Kaan"; presçiler aynı. Menajerin isteğiyle davranış sınaması yapılmadı; tek çekirdek ile 3 yardımcı aynı, 900 tik hatasız. *Araç:* scratch `kk-hiz.js` (tik tik hız ve hedef).

## 17 Ekim 2026 · Savunmada tek tartı (main) — maçlar değişti
- Menajerin maç dosyası (909623, kendi kurulumu) ile tik 46 birebir kuruldu: pas havada kenardaki Tuna'ya; Alp "arkada · Tuna" 33 br çapraz, Kartal buluşmaya, Baran öne; Ece ve Kaan kaleye koşarken boş. Neden: arkadakilerin seçimi "önde tutulan" rakibi orta sahaların önceki tikteki işinden okuyordu (aynı tikte başka işe geçmişlerdi); ortak atama (en tehlikeli boş kalmaz dahil) top havadayken çalışmıyordu.
- `ALAN_SAVUNMA_TEK_ESKI`: kim kimi tutar tek tartıda, her tik: her rakip için iş (derin → okunan yerinin kale tarafı; değil → pas yolu; alıcı → buluşma), tehlikeye göre sırayla en çabuk varacak boştaki savunmacıya (arkadakiler öndeki işe +45 tik bedel, orta sahalar derine +6, önceki işinde ×0,6); en tehlikelinin %30'u altına adam yok. Eski arkadaki seçimi, bölge eşleşmesi, ortak atama, havada kapatma ve buluşma presi tek tartı açıkken devre dışı; pres, karşılama, önde bekle, markaj, arkada alan, bölge, Kuyu tarafı kuralları sürer.
- Menajerin isteğiyle davranış sınaması yapılmadı (maç maç kendisi bakacak); tek çekirdek ile 3 yardımcı aynı, kafada hata yok. Tik 46'da: Volkan "arkada · Kaan", Yıldırım "arkada · Ece", Efe buluşma, Alp ve Baran arkada alan.

## 17 Ekim 2026 · Savunma: koşuyu okuma, en tehlikeli rakip boş kalmaz (main) — maçlar değişti
- Menajerin gözlemi (İnceleme modu, başlama vuruşu): savunanların hepsinin hedefi topun çevresi, koşucuları kimse tutmuyor, en arkadakiler öne çıkıyor. Ölçüm: savunmacılar koşan rakibin şimdiki yerine gidiyordu; derin rakip seçimi topun gerisinden koşanları saymıyordu; en tehlikeli rakip için yalnız 'işi gereksiz' ve bölgesindeki savunmacı aranıyordu.
- `ALAN_KOSU_OKU_ESKI`: rakibin okunan yeri = yer + hız × (4 + Görüş − 6) tik (2–18). Arkadakilerin derinliği, derin rakip seçimi, arkada adam tutma, bölge/markaj hedefi ve pas yolu kapatma bununla. Okuması düşük takım (Fırtına notu: "Okuma düşük, boşluğa zayıf") yine zayıf kalır.
- `ALAN_TEHLIKE_BOS_ESKI`: en tehlikeli rakip atanmamışsa işi belirgin az tehlikeli (%80 altı) olan, en çabuk varacak savunmacı ona gider (derindeyse kale tarafına, değilse pas yoluna); bağlılıkla (rakip en tehlikelilerin %80'i içinde kaldıkça).
- Ölçüm (ilk 60 sn, varsayılan kurulum 909623; eski · yalnız okuma · ikisi): kale tarafında 8 br içinde kimse yok %8,7 · %9,1 · %2,5; atama yok %13,9 · %21,7 · %8,2; geçilme 6 · 2 · 2; adam değişimi 31 · 29 · 32. *Araçlar:* scratch `kk-savunma.js`, `kk-hat.js`, `kk-baslama.js`.
- Not: "Adam adama" sistemi yapay zekâ takımlarında markaj ataması (markRef) kurmuyor; adam adama da bölge gibi oynuyor. Ayrı bakılacak.

## 17 Ekim 2026 · Maç ekranı: tik tik izleme, İnceleme, Maç dosyası (main 62fcb86, maç değişmez)
- Oynat düğmesinin yanında «  ‹  ›  » (1 sn / 1 tik geri-ileri; durdurur).
- İnceleme: sahada bütün oyuncuların hedefleri (takım renginde kesikli çizgi); yan panelde o tikte her oyuncunun görevi, açıklaması, hedefine uzaklığı; top sahibinin son kararı ve tarttığı seçenekler. Satıra dokununca kart açılır. Top sahibi dışındakilerin "düşüncesi" bugün yalnız görev ve hedef (kayıtta karar ayrıntısı yok).
- Maç dosyası: `kayitMac` (kurulum, taktik değişiklikleri) + izlenen an; iPad'de paylaşma menüsü. Menajerin maçını birebir kurmak için (varsayılan kurulum menajerin maçını vermiyordu).

## 17 Ekim 2026 · Presçi kale tarafında kalır (main) — maçlar değişti
- Menajerin gözlemi (01:39, Baran–Ece): presçi topa basarken adamının önünü açıyor. Sahnede: kale tarafında başlayan presçi, arkadaki Çekirdeğe ulaşmak için taşıyıcının etrafında dolaşıyor (7° → 89°), yanda uzanıp geçiliyor. Pres planının "yan" hedefi ve dolanma noktası (her tik presçinin 90° önü) kale tarafı koşulunu kendisi bozuyordu.
- `ALAN_PRES_KALE_SINIR_ESKI`: ilk presçinin hedefi ve dolanma noktası, tehlike yönüne (İlke A; çoğu zaman kale) göre en az 1,2 br önde kalacak açıyla sınırlanır; Çekirdek arkadaysa kale tarafında bekler ("pres · kale tarafında bekliyor").
- Ölçüm: temiz sahnede kale tarafı payı −3,4 → en az +1,4 br, taşıyıcı geçemedi. Maçta (60 sn) olay az, fark görülemedi. Menajerin maçı kurulamadı: varsayılan kurulumla 909623 devre arası 3–1 (onunki 1–0); taktik/kadro ayarları farklı. *Araçlar:* scratch `kk-baran-sahne.js`, `kk-kale-tarafi.js`, `tarayici-kur.js` (tarayıcıdan maç kurulumunu yakalar), `kk-an-bul2.js` (o kurulumla oynatır).

## 17 Ekim 2026 · Bütün site çapraz köken yalıtımlı (main 266daef, maç değişmez)
- Maç ekranı kabuğun (index.html) içinde çerçeveyle açılıyor; yalıtım en üstteki sayfadan gelir. `_headers` yalnız `/canli*` için olduğundan kabuktan açılan maç tek çekirdekte kalıyordu (iPad göstergesi "1 çekirdek"; paralelin "hissedilmemesi" büyük ihtimalle bundandı). Şimdi tek kural `/*`: COOP same-origin, COEP require-corp, CORP same-origin.
- Chromium'da kabuk → Tohumla oyna: çerçeve yalıtımlı, gösterge "4 çekirdek", hata yok. *Araçlar:* scratch `sunucu-yalitim.js`, `tarayici-kabuk.js`.

## 17 Ekim 2026 · Ön hareket, geliş hızı, değer tavanı, Kuyu'suz boşluk (main) — maçlar değişti
- Ön hareket (`ALAN_ON_HAREKET_ESKI`): alıcı, önceden kararındaki en iyi vuruş en iyi seçeneğe tek dokunuş payı (0,02) kadar yakınsa ona hazırlanır. Yerleşme: Çekirdeğin değeceği yer, topun geldiği yana bakan yarıda bırakma noktasına en yakın yer (decide.js `onPlan`). Ön savurma: dönüş top gelmeden başlar, değme anında bırakma hızındadır; kanca hız sınırı yok. İyi okuyan alıcı önceden kararını erken verir (Görüş 20 → 5 tik).
- Geliş hızı (`ALAN_GELEN_HIZ_ESKI`): top tutulunca hızı sıfırlanıyor, alış onu sonra okuyordu. Tek dokunuşun hata çarpanı (otFactor) hiç işlemiyordu (eski motorda da), kanca gelen hızı kullanamıyordu. Şimdi çarpan ortanca ~1,25–1,35.
- Değer tavanı (`ALAN_TERCIH_TAVAN_ESKI`): taktik tercih payları (kanat, istek, pas/sürme, sabır, tempo, kontra tempo), zamanın değeri ve risk çarpanı olasılık biriminde; olumlu pay (1 − v) ile ölçeklenir. Önce ileri pas 1,02'ye çıkıp Pk 1,00 şutu geçiyordu (Tuna şut gösterip pas verdi).
- Kuyu'suz boşluk (`ALAN_RUNON_KUYU_ESKI`): alıcının önündeki boşluk tartısı (runOn) Kuyu içini saymaz; önce kale önüne her pas ~0,98.
- Ölçüm (maçın ilk 60 sn'si): ön hareketle 15 hazırlık, 11'i alışta geçerli; tek dokunuş 7 (5'i tuttu), ön hareketsiz 2 (ikisi kayıp). Maçlar ilk farktan sonra ayrışır, örnek küçük. İlk vuruş sahnesi (kanattan Tuna'ya) düzeltmelerden sonra çoğunlukla Bekçi'nin pası kesmesiyle bitti; ilk vuruşla gol görülmedi. Tek çekirdek ile 3 yardımcı aynı.
  *Araçlar:* scratch `kk-on.js` (tek dokunuş sayımı), `kk-ilk.js` (ilk vuruş sahnesi), `kk-on-an2.js` (alıştaki seçenekler), `/tmp/on-dbg-kur.sh` (ölçüm kopyası).

## 17 Ekim 2026 · Plase yalnız itişle (main, karma bırakmanın ardından) — maçlar değişti
- Plase yalnız itişle: hızı itişin sınırında (Güç 10'da 1,0, 20'de 1,3), açı hatası yalnız nişan hatası (`ALAN_PLASE_ITME_ESKI`). Önce plase dönüşle 2,2 hızında, itiş payı %0 vuruluyordu.
- Savurma planı, aynı sürede bitebilen planlardan daha çok iteni seçer (`ALAN_ITME_ONCELIK_ESKI`).
- Şut tablosu `ALAN_SHT_PLASE` (v5-21): Kuyu kenarında plase ~1, 15 br ~0,5. Tek çekirdek ile 3 yardımcı 240 tik aynı, kafada hata yok.
- Ölçüm: maçın ilk 60 sn'sinde tek dokunuş yalnız 1 kez (ön hareket önerisi menajere sunuldu).

## 17 Ekim 2026 · Karma bırakma (main 6e55ff6) — maçlar değişti
- Saf teğet savurmada dönüş çok fazlaydı (pas başına ortanca ~195°, şut ~2 tur; Çekirdek öndeyken ileri pas 270°). Bırakma hızı artık teğet (ω × 1,15) ile gövdeden itişin bileşkesi; itiş en çok Güç 10'da 1,0, Güç 20'de 1,3. Dönüş ivmesi 0,45–0,70. Geri çekme en çok 135° (`TASARIM-savurma.md` güncellemesi). Deney: `ALAN_SAVUR_ITME_ESKI`.
- İsabet: dönüşün payı zamanlamadan (ω × τ), itişin payı eski nişan hatasından.
- Maçın ilk 900 tiki, ortanca dönüş: kısa pas 10–20°, orta 13–44°, uzun 101–128° (geri çekmeyle toplam ~175°), şut 226–300°; Çekirdek öndeyken pas 10–16°.
- Şut tablosu yeniden ölçüldü (`ALAN_SHT_KARMA`, v5-20). Tek çekirdek ile 3 yardımcı 240 tik aynı, kafada hata yok. Aynı koşulda (makine yüklüyken) 900 tik sitedeki sürüme göre ~%20 yavaş, gönderme 3 → 5.

## 16 Ekim 2026 · Savurma, kanca, boşluğa nişan, Bekçi'yi gören değer (main 4ab0811) — maçlar değişti
- Savurma (`TASARIM-savurma.md`, `ALAN_SAVURMA_ESKI`): pas ve şutta Çekirdek gövdenin etrafında döner (α Güç'e bağlı), top bırakma anının teğetiyle çıkar; isabet bırakma zamanlamasından. Yolu rakip gövdesinden geçen savurma seçilmez, çarpan savurma bozulur (`ALAN_SAVUR_ENGEL_ESKI`). Tek dokunuşta kanca (`ALAN_KANCA_ESKI`). Şut hedefi: Bekçi'nin yetişemeyeceği en geniş iki boşluk (`ALAN_SUT_HEDEF_ESKI`). Dağınıklık değeri olasılık biriminde, 1'i geçmez (`ALAN_DAGINIK_TAVAN_ESKI`; önce karşı karşıya tutmak 1,59 çıkıyordu).
- Şut tablosu savurmayla, maçtaki gönderme yoluyla yeniden ölçüldü (`ALAN_SHT_SAVUR`, 1120 hücre, seçenek başına 24 deneme; `ALAN_SHT_SAVUR_ESKI`). Eski tablo basit taklitle ölçülmüştü, Kuyu önünü hep 1 sayıyordu; yenisi Kuyu kenarında ~0,4–0,7.
- Konum değeri Bekçi'nin gerçek yerine bakar: ağızda kalan en geniş boşluk tablodaki Bekçi'ye göre oranlanır, yalnız düşürür (`ALAN_V_BEKCI_ESKI`). Tutma ihtimali Bekçi'yi de sayar (maçta Bekçi ikiliye girer; `ALAN_KEEP_BEKCI_ESKI`). Karşı karşıya: Bekçi 2 br öndeyken 'tut' 0,99 → 0,37.
- Kafadaki gönderme kopyasında taşıyıcı vuruşa kilitli, karar vermez (`m._ucus`); savurma bozulursa deneme biter, top çarpan rakiple paylaşılmış sayılır. Önce kopyada kararlar iç içe açılıyordu (58 kat, karar başına dakikalar) ve ikiliyi okuyan taşıyıcı kopyada hata veriyordu (kuyruk bozuk → maç paralelsiz yeniden).
- Ölçüm: karşı karşıya sahnesi (Tuna–Tufan, değer düzeltmelerinden önce) eski motor 0/36, yeni 10 br karşıdan 4/4, 15 br 7/12. Değer düzeltmelerinden sonra sahne yeniden ölçülmedi. Sıfırdan maçın ilk 900 tiki node'da 96 → 73 sn, gönderme 1 → 3. Tek çekirdek ve 3 yardımcı 240 tik birebir aynı, kafada hata yok.
  *Araçlar:* scratch `kk-sahne.js` (karşı karşıya), `kk-iz.js` (tik izi), `kk-sure2.js` (tek karar, konum ortamdan), `kk-tablo2.js` (tablo üretimi).

## 16 Ekim 2026 · Ayrı tohum + tam paralel (main 96555dd) — maçlar bir kez değişti
- Kafadaki her deneme ve yalancı hareketin her dünyası ortak zardan tek tohum çeker, kalan zarını kendi tohumundan alır (`ALAN_ALT_TOHUM_ESKI`). Görevler bağımsız: yardımcı işçilerde ya da aynı çekirdekte birebir aynı sonuç.
- Hesabın ~%87'si paralele açık (909623 · tik 4770 kopyası). Ölçüm: Chromium, maçın ilk 600 tiki 54,7 → 36,8 sn (4 sanal çekirdekli, ~2 etkin çekirdek). Tavan tahmini: 4 çekirdekte ~2,9×, 8'de ~4×.
- Bir kerelik: eski tohumlar başka maç oynar. Belirlenimcilik korunur.

## 16 Ekim 2026 · Havadaki topta plan yenileme (main e33e2fb) ve paralel kafa (main 66de97a)
- `ALAN_UCAN_PLAN_ESKI`: top havadayken koşu planı, topu alacak oyuncu ya da alınacak nokta (8 br) değişince yeniden tartılır (909623 00:11: Kaan 90 tik eski planla ileri koştu).
- Paralel kafa (maç birebir aynı): kafadaki gönderme denemelerinin zarları maçtaki sırayla önceden çekilir (kuyruk, `ALAN_KUYRUK_ESKI`), oyunları yardımcı işçilerde (SharedArrayBuffer, `paralel.js`, `yardimci-isci.js`, `ALAN_PAR_ESKI`). `_headers` ile canli sayfası çapraz köken yalıtımlı; yalıtım yoksa tek çekirdek.
- `ALAN_GEC_ESKI`: dönme süresi önbelleği anahtarına hız (fren sınırından beri önbellek hesap sırasına bağlıydı).
- Ölçüm: kale önü yoğun pencerede node 40 → 25 sn (3 yardımcı, bu makine ~2 etkin çekirdek); Chromium'da maçın ilk 600 tiki 54 → 49 sn (gönderme denemesi az). Tam maç ölçülmedi. Profil: zamanın çoğu kafadaki denemeler; paslar ve yalancı hareket dünyaları henüz tek çekirdek.
- Denenip bırakılan: B (kurtarma değeri) bu anda tetiklenmedi (keepP 1,0); asıl sorun ev bağının değer biriminde olmaması. Kısa diziliş (ön çizgi topun en çok 60 br önünde) tek başına etkisiz, siteye alınmadı.

## 16 Ekim 2026 · Duvar çalımı ve gerçekçi momentum (main 5c5b3bc) — sahnelerde denendi, maçta ölçülmedi
- Kenardan çalım hiç aday üretmiyordu: tek sabit dönüş noktasında duvara giden yol rakibin gövdesinden geçiyordu. Birkaç dönüş noktası + geniş koşul (`ALAN_CALIM_GENIS_ESKI`), yumuşak "ölü top" vuruşu (`ALAN_CALIM_OLU_ESKI`).
- Kafadaki hesap düzeltmeleri: Çekirdek gövdenin kenarından çıkar (`ALAN_CALIM_CEK_ESKI`); gövde kontrolü gerçek top yolunda, rakibin o tikteki yerine göre, çarpma payı 1,35 (`ALAN_CALIM_YOL_ESKI`, uzanan rakip için `ALAN_CALIM_ADT_ESKI`); yolda kesmede rakibin momentumu (`ALAN_CALIM_DON_ESKI`, `ALAN_CALIM_HIZ_ESKI`); taşıyıcı topa top oradayken varmalı (`ALAN_CALIM_YETIS_ESKI`); rakip gövdesiyle geçiş noktasını keserse dolanma gecikmesi (`ALAN_CALIM_KAPI_ESKI`); dolanma açıklığı 3 → 2,2 br (`ALAN_CALIM_DAR_ESKI`).
- Momentum (`ALAN_FREN_ESKI`): fren en çok ≈ 7 m/sn², koşarken yana dönüş ≈ 8 m/sn² (1 br ≈ 0,42 m varsayımıyla); tam hızdan durma 0,2 → 0,67 sn. Hızlanma aynı. Herkesi etkiler.
- Alıcı deparı (`ALAN_ALICI_DEPAR_ESKI`): kendi pasının alıcısı Çekirdeği alana kadar depar atar (topu almaya hazırlanırken depar düşüyor, yuvarlanan topa yetişemiyordu). Herkesi etkiler.
- Duvarı arama (`ALAN_DUVAR_SUR_ESKI`): önde yakın rakip ve yakın duvar varken duvar tarafına sürme seçeneği; sür-sonra hesabında her zaman tartılır.
- Sahneler (909623 · tik 4770 kopyası, Tuna–Kartal): Kartal koşarak (5/3,5 br) ve uzanmışken (4/4 br) temiz çalım; top Kartal tarafındayken çalım seçilmiyor (doğru). Duvara sürme yakın rakipte kayıp görülüyor, sonrasında çalım değil pas planlanıyor.
  *Araçlar:* scratch `sahne-calim-dbg.js` (sahne kurucu; ADN=uzanma, KV=rakip hızı, ZORLA=çalımı zorla).

## 16 Ekim 2026 · Belirlenimcilik: her cihazda aynı maç (oyun mantığı değişmedi)
- `sabit-mat.js` (işçilerde ilk yüklenen): exp, log, pow, sin, cos, atan, atan2, acos, tanh, hypot yalnız + − × ÷ ve √ ile (fdlibm algoritmaları). Safari/iPad (JavaScriptCore) ile V8 (Chrome, node) bu fonksiyonlarda çağrıların %3–33'ünde son basamakta ayrışıyordu; motor farkı birkaç saniyede büyütüyor, aynı tohum iki cihazda iki ayrı maç oluyordu. Sınama: aynı maç node ve bun'da (JavaScriptCore) 600 tik bit bit aynı; V8'de alınan kopya JSC'de 200 tik aynı. Doğruluk 1–2 son basamak. Deney: `ALAN_MAT_ESKI`.
- `match.js`: `D._tick`/`D._mac` tikin başında (`ALAN_TIK_BASI_ESKI`). Alıcının önceden kararı önceki tikin alan önbelleğini (eski konumlar) okuyordu; maç kopyasından kurulan maç ~33 tik sonra ayrılıyordu. Düzeltmeyle 150 tik aynı.
- Sonuç: eski tohumlar artık başka maç oynatır (rakamlar son basamakta değişti). Anlar yeni motorla yeniden tarandı: 9 maç, 67 an.

## 16 Ekim 2026 · Anlar ekranı (motor davranışı değişmedi)
- Sitede `anlar.html`: farklı tohumlarla oynanmış maçlardan, konuşulan sorunların görüldüğü anlar (11 tür: top uzaktayken pres, önü boş sürüş · kimse yavaşlatmıyor, iki kişi topta · alıcı boş, baskısız topta yüksek hat, harcanan kontra, önü boşken tut, arkadaki orta sahada, boş topa boşuna koşu, koşucunun arkasına pas, eksi değerli seçim, pivot geride). Her türün yanında "maçta" sıklığı: motor değişince yeniden taranıp karşılaştırılır.
- Bir ana dokununca `canli.html?an=ID`: maçın o anki tam kopyası işçiye `tip: 'an'` ile gider, `grafOku` ile kurulur, **bugünkü motorla** oynar (motor değişmediyse asıl maçla birebir aynı; denendi). Taktik değişikliği an içinde de çalışır.
- İlk set: 3 maç (Liman – Fırtına / Demir Blok / Saat Kulesi, tohum 41, 52, 63; main 6ad507e motoru), 41 an. Kalan 6 maç taranıyor.
- Araçlar: `tools/anlar/` (BENIOKU.md).
  *Yer:* `anlar.html`, `anlar/`, `canli.html` anYukle/anBasla, `alan-v5/mac-isci.js` grafOku + 'an'.

## 16 Ekim 2026 · Kendine pas düzeltmesi; buluşma noktasına pres — ölçülmedi
- `ALAN_KENDINE_PRES_ESKI` daraltıldı: kendine pas ancak top atanın 3 br yakınındayken top sahibi sayılır. (00:05 Tuna topu 15 br önüne itti; Volkan topun olmadığı yerde Tuna'ya basmaya devam etti, topu geç fark etti. Önceki sürümün hatası.)
- `ALAN_BULUSMA_PRES_ESKI`: rakip pası (kendine pas dahil) havadayken ve kimse topa alıcıdan önce varamıyorken (kovalayan yok), buluşma noktasına en çabuk varacak savunmacı (arkada, önde ve markajcı olmayan) oraya koşar. Görev "pres · buluşma". Savunma adımlarının sonunda uygulanır.
- Not: ilk yazımda bu kural presçi listesine ekleniyordu ve top sahibi yokken hata verip hesabı durduruyordu; siteye gitmeden kontrol çalışmasında yakalandı.

## 16 Ekim 2026 · Kendine pasta pres; kale ağzı pası — ölçülmedi
- `ALAN_KENDINE_PRES_ESKI`: kendine pas (önüne / duvardan) havadayken savunma (shape) topu atanı top sahibi sayar; pres, arkadakiler ve kapatma bozulmaz. (00:48 Kaan, Volkan topu önüne itince presi bırakıp bölgesine döndü.)
- `ALAN_AGIZ_PAS_ESKI`: kaleye 40 br'den yakın her arkadaş için onunla kale ortası arasındaki 4, 8, 12 br noktaları "önüne" pas adayı (boşluk taramasının "alıcı herkesten önce varmalı" elemesi olmadan). (00:51 Yıldırım'ın Kartal'a pası köşeye gitti; kale ağzı hiç aday değildi.)
- Teşhis notu: kurulan sahnede gönderme 1,00 çıktı ama kopyada şut gerçekten gol oluyordu (kaleci şut çizgisinde değildi); gönderme hesabında hata kanıtı yok, dokunulmadı. "Önüne pas yalnız kaleye doğru" önerisi kullanıcı tarafından reddedildi (yanlış öneriydi).

## 16 Ekim 2026 · Karşılama önce orta sahaların; arkadakiler sıra kurar — ölçülmedi
- `ALAN_ORTA_KARSILA_ESKI`: önü boş taşıyıcıyı arkadakiler ancak taşıyıcı kaleye 40 br'den yakınsa ya da hiçbir arkada olmayan savunmacı taşıyıcının kale tarafına (3 br) onlardan önce yetişemiyorsa karşılar (karşılayan zaten tek kişi). (00:02 Tuna kanatta, orta çizgi ötesinde; Baran karşılamaya çıkıyordu.)
- `ALAN_ARKA_DIZI_ESKI`: adamı olmayan arkadakiler topa (buluşma noktasına) yakınlığa göre sıralanır: ilki top–kale ortası çizgisinde, sonuncusu kale ortasının önünde (aynı derinlik), diğerleri eşit aralıkla arada. Tek başınaysa ikisinin karışımı: top kaleye 30 br'den yakınsa %20, 70 br'den uzaksa %60 ortaya. (00:01 Alp ve Baran ikisi de kanattaki Tuna'nın buluşma noktasına kayıyordu; ortada Kaan ve Ece.)
- Süreç notu: kurallar tek karelere göre yazıldı, yan etkiler (santra) gözden kaçtı. Arka hat için kuralların bütünü bir sonraki adımda tek belgede gözden geçirilecek.

## 16 Ekim 2026 · Arkadakiler orta sahaya çıkmaz · `ALAN_ARKA_SINIR_ESKI` — ölçülmedi
- Taşıyıcıyı karşılama yalnız taşıyıcı kaleye yarım sahadan (GW/2) yakınken.
- Adamı olmayan arkadakinin kaleye uzaklığı dizilişteki yerininkini (base) geçemez. (00:00 santrada Alp "karşıla · Ece" 32 br, Baran "arkada · alan" 28 br orta noktaya koşuyordu.)

## 16 Ekim 2026 · Önü boş taşıyıcıyı arkadakiler karşılar · `ALAN_TASIYICI_KARSILA_ESKI` — ölçülmedi
- Taşıyıcının kale tarafında 6 br içinde arkadaki olmayan savunmacı yoksa taşıyıcı arkadakilerin tutacağı rakiplerin başına eklenir; hızlı eşleşme kimin kime gideceğini seçer (adam devri). Karşılayanın hedefi: taşıyıcının kale tarafı (%70) + ondan en tehlikeli yakın (30 br) rakibe pas yolunun %35 noktası (%30). Görev "karşıla · X". (00:52 Ece'nin önü boş, Efe arkadan; Alp Kaan'ı bırakmıyor, uzaktaki Baran Ece'ye koşuyordu.)
- Not: kontra koşucusunun sabit kalması (`ALAN_KOSUCU_SABIT_ESKI`) kullanıcıya katı geldi; şimdilik kalıyor, sorun görülürse "topun 12 br gerisi + önü açık" koşuluna dönülecek.

## 16 Ekim 2026 · Sürme ve kontra koşucuları — ölçülmedi
- `ALAN_SUR_SONRA_ESKI`: en umutlu iki "sür" maçın kopyasında (holdWorld, hedefe sürerek) oynatılır; varınca en iyi hamlenin değeri (beklemeyle aynı ölçü, tempo bedelinin yarısı) alınır, daha iyiyse sürmenin değeri olur. Önce sürme yalnız 6 br ötedeki yer değeriyle tartılıyordu (Efe: bekle 0,162 · sür ×3 0,069).
- `ALAN_UZUN_SUR_ESKI`: önü açıksa (yolun 5 br yakınında rakip saha oyuncusu yok) 12 ve 20 br sürme, düz ve ±23°.
- `ALAN_KOSUCU_SABIT_ESKI`: kontra koşucuları (m.gc.on) kontra boyunca korunur; top onları geçse de listeden çıkmaz.
- `ALAN_KONTRA_GUVENCE_ESKI`: kontrada güvence en çok 1; takım riski 0,7 üstündeyse 0. Önce rakibin yukarıda kalan her oyuncusuna bir güvence (00:05 Efe kontrada "güvence"ye 24 br geri koştu).
- `ALAN_ICERI_KOSU_ESKI`: kontra koşusu adayları iç şeritler (kale ortasından 8 br içi); kanat şeridi ancak içeride koşan bir arkadaş varsa (00:04 Efe çapraz taç çizgisine koşuyordu).
- Hız: her kararda iki ek kopya + iki ek karar; hesap yavaşlayabilir.

## 16 Ekim 2026 · Pas: koşunun önüne · `ALAN_ONUNE_KOSU_ESKI` — ölçülmedi
- Buluşma pası kaleye doğru koşan (ileri hız > 0,08) her arkadaşa da düşünülür (önce yalnız görevi "…koşu…" olana).
- "Önüne" (hat arama, pas çizgisine dik ±3 br) pası koşan alıcıda, topun varış süresince (L / 1,1 tik) alıcının gideceği yol kadar (en çok 20 br) ileri kaydırılır. (01:06 Efe → Yıldırım: top koşunun arkasına düştü, Deniz araya girdi; karar kartında 0,202 · %99.)
- Not: aynı anda Efe'nin kararında "tut 0,5 sn bekle" 0,162, üç "sür" 0,069 (aynı değer) — sürmenin değeri ayrıca incelenecek.

## 16 Ekim 2026 · Kapatma bölgede kalır · `ALAN_KAPATMA_BOLGE_ESKI` — ölçülmedi
- Arkadakiler (süpürücüler, sweep) ortak atamada kapatmaya aday değil; kapatma bağlılığında (top birindeyken ve havadayken) da kalmazlar. Diğerleri kapatmaya ancak kapatma noktası (pas yolu noktası) kendi bölgesinin içindeyse (evden S.zoneR + Yeralma × S.zoneOk) gider. Markaj talimatı ve pres ayrı. (00:01 Alp, en arkadan 32 br öne Cansu'nun pas yoluna çıkıyordu; Ece ve Kaan arkada boş.)
- Karar kartı: görev kartında top sahibinin son kararı (en iyi 4 seçenek, değer, ihtimal) (main `fe5839f`).

## 16 Ekim 2026 · Arkadakiler: buluşma noktası ve kale ağzı — ölçülmedi
- `ALAN_BULUSMA_ESKI`: pas havadayken (sekmemiş) arkadakiler alıcının topla buluşacağı noktaya (flightRead) göre yer alır: adamı alıcıysa o noktanın kale tarafına; adamı yoksa kale ortasından o noktaya giden çizgide. (01:30 Alp, Ece'nin topu alacağı yerin değil topun uçtuğu köşenin tarafına koşuyordu.)
- `ALAN_KALE_AGZI_ESKI`: savunmadaki derinlik (gD) x farkı değil kale ortasına uzaklık. Etkiler: arkadakilerin adam seçimi (derin rakip), adamı olmayan arkadakinin yeri, arkada boş bırakma, önde tutulan, tehlikedeki "arkasında boşluk".

## 16 Ekim 2026 · Pres: önce kale tarafı · `ALAN_KALE_TARAFI_ESKI` — ölçülmedi
- İlk presçi rakiple kale arasında değilse topa girmez: rakibin 10 tik sonraki yerinin 2 br kale tarafına koşar; düz yol rakibin 2,5 br yakınından geçiyorsa yanından (3,5 br) dolaşır (görev "pres · kale tarafına"). Rakip kaleye presçinin hızının %90'ından hızlı gidiyorsa yanında, iç tarafta koşar ("pres · yanında"). Topa girme planı (direkt/dolaş/fake) yalnız kale tarafındayken ya da top açıkta ve 1,6 br yakındayken. "Kenara sıkıştır" sistemi eskisi gibi. (01:06 Volkan, Tuna'yı arkadan kovalıyordu.)

## 16 Ekim 2026 · Kontra başlangıcı ve hızlı hücum (main `f6ef99c`) — ölçülmedi
- Kontra açılırken de rakip en öndeki oyuncumuzun gerisinde sayılır (önce topun gerisinde: kendi kalesi önünde top kazanan takım hiç kontraya çıkamıyordu, 00:15 Baran). Bayrak `ALAN_KONTRA_SURE_ESKI` (sürme ve başlangıç birlikte).
- Hızlı hücum (`ALAN_HIZLI_HUCUM_ESKI`): top bizdeyken, el değiştirmeden (ör. uzun pas presi aştı) rakip en öndeki oyuncumuzun gerisinde eşikten (4) az kişiyle yakalanırsa kontra gibi oynanır (aynı risk/tempo, koşucular, 4 sn). Topa sahip olma başına bir kez; son bölgede başlamaz. Evre kodu `huc.hizli` (EVKOD 8), Canlı'da "Hızlı hücum".

## 16 Ekim 2026 · Hücum: kontra ve riskli oyun (beş kural) — ölçülmedi
Kaynak: 01:14–01:16 Fırtına kontrası (Yıldırım keser, Volkan geri Alp'e, Alp→Baran→Alp; Kartal ve Volkan koşarken).
1. `ALAN_KONTRA_SURE_ESKI`: kontra (m.gc) rakibin dağınıklığını topa göre değil en öndeki oyuncumuza göre ölçer (rakibin kaç saha oyuncusu onunla kendi kalesi arasında, eşik 4); en çok 240 tik. Önce: tek geri pas kontrayı kalıcı bitiriyordu.
2. `ALAN_KONTRA_TEMPO_ESKI`: kontrada tempo ödülü kısa geri/yan pasa verilmez; öne (6+ br) pasa verilir (D.cevTempo × 0,5 × ilerleme/20 × P). Not: cevTempo küçük (0,025), etkisi küçük.
3. `ALAN_RISK_KAZANC_ESKI`: ileri pasın (6+ br, pozitif değerli) değeri × (1 + (risk − 0,5) × 0,8). Risk 0,8 → ×1,24; kontrada risk 1 → ×1,4.
4. `ALAN_DAGINIK_ESKI`: V (yer değeri) × rakip dağınıklığı: noktayla rakip kalesi arasında 0 saha oyuncusu ×1,6 · 1 ×1,3 · 2 ×1,1. Kayıp tarafı (Vo) da aynı ölçüyle.
5. `ALAN_KOSUCU_PAS_ESKI`: buluşma pası (zip66'da herkese kapatılmıştı) görevi "…koşu…" olan arkadaşa açık: koşucunun varacağı noktaya önüne pas.

## 16 Ekim 2026 · Savunma: tehlike ölçüsü · `ALAN_TEHLIKE_ESKI` — ölçülmedi
- Rakibin tehlikesi = gittiği yer (1 sn sonraki konumunun kaleye yakınlık/merkez tehlikesi) × topun ona ulaşması (topa uzaklık 15 br üstünde azalır, en az ×0,45; pas yolunun anlık açıklığına bakılmaz) × arkasında boşluk (kale tarafında 10 br yanında savunmacı yoksa ×1,3) × özellikleri (Gönderme ve Hız, ×0,7–1,4).
- Kullanıldığı yerler: arkadakilerin adam seçimi, bölgede tehlikeli rakip çekimi, ortak atama (kapatma).

## 16 Ekim 2026 · Savunma: tehlike konumdan · `ALAN_ANLIK_YOL_ESKI` — ölçülmedi
- Rakibin tehlikesi (`dng`) artık yalnız konum (kaleye yakınlık); anlık pas yolunun açıklığı çarpanı kalktı. Adam tutmak sürekli iş: yol kapalı diye rakip bırakılmaz. Etkiler: arkadakilerin adam seçimi, bölgede tehlikeli rakip çekimi, ortak atama (kapatma). (00:09 Deniz, Kartal–Volkan yolu kapalı diye Volkan'ı tutmuyordu.)

## 16 Ekim 2026 · Savunma: adamı olmayan arkadaki · `ALAN_ARKA_ALAN_ESKI` — ölçülmedi
- "arkada · alan" artık topun 3 br önüne değil, top ile kale ortası arasındaki çizgide, en derindeki rakibin 2 br gerisine gider (ikinci boştaki 4 br daha geride; kaleden en az 6 br). (00:09 Deniz, Volkan yanındayken Kartal'a 40 br koşuyordu.)

## 16 Ekim 2026 · Savunma: kapatma = pas yolu · `ALAN_PAS_YOLU_ESKI` — ölçülmedi
- Kapatmaya giden, topla rakip arasındaki çizgide rakibe yakın durur (yolun üçte biri, en az 3 br). Önceden rakibin kale tarafında duruyordu: markajdan farkı yoktu, arkadakiyle aynı yeri tutuyordu (00:05 Yıldırım ve Baran ikisi de Kaan'ın arkasında).
- Sayfa: yükleme ekranında hesaplanan maç saati; işçi hata verirse mesaj (main `cd326ff`).

## 16 Ekim 2026 · Savunma: hızlı eşleşme · `ALAN_HIZLI_ESLES_ESKI` — ölçülmedi
- Arkadaki iki (üç) oyuncu derindeki rakipleri sırayla değil, en geç varanın da en erken varacağı şekilde paylaşır; gerekirse adam devredilir. Eski eşleşme %15'ten fazla yavaş değilse sürer. (00:03: Baran Ece'ye 19 br koşuyordu; Alp Kaan'ı Baran'a bırakıp Ece'ye gitmeliydi.)

## 16 Ekim 2026 · Savunma: üç kural (main `8feae43`) — ölçülmedi, kullanıcı maçta deniyor
Kaynak: kullanıcının iPad maçı (tohum 909623), görev kartlarıyla görülen anlar.
- Boş top (`ALAN_BOS_TOP_ESKI`): rakip topa 6+ tik önce varacaksa topa koşulmaz; oyuncu işine (adamı, bölgesi) devam eder. Kendi kale önündeki kural ayrı. (00:41 Alp, Kaan topa çok daha yakınken topa koşuyordu.)
- Arkada boş bırakma (`ALAN_ARKA_BOS_ESKI`): kapatacağı rakipten daha derinde, kendisinden başka kimsenin arkasında olmadığı rakip varsa kapatmaya gidilmez, kapatmada kalınmaz. (00:42 Alp, Kaan'ı bırakıp Mert'i kapatmaya çıkıyordu.)
- Önde tutulan (`ALAN_ONDE_TUTULAN_ESKI`): arkadaki, bir arkadaşının kale tarafından tuttuğu rakibe çıkmaz; rakip o arkadaşı geçerse alır. (00:50 Mert, Ece'nin orta sahada tuttuğu Yıldırım'a çıkıyordu.)
- Sayfa: tohum kutusu (Maç sekmesinde maç yokken ve üst çubukta; main `c000087`).

## 16 Ekim 2026 · Çalışma: testleri kullanıcı yapar
Kullanıcının kararı: Claude Code istenen kuralları uygular ve siteye koyar; ölçüm ve maç testi yapmaz. Mantık hatalarını kullanıcı maçta görüp bildirir. Her kural kendi bayrağıyla, tek tek geri alınabilir.

## 15 Ekim 2026 · Canlı sayfa: görev göstergesi (main `b34694c`)
- Sahada bir oyuncuya dokununca kart açılır: ad, rol, motorun görev etiketi (ör. `bölge · Kaan`), düz Türkçe açıklaması, hedefine uzaklık. Sahada: seçilen oyuncunun halkası, hedefe kesik çizgi ve çarpı, bağlı olduğu rakibin kesik halkası. Aynı oyuncuya ya da boşa dokununca kapanır. Kart, oyuncunun karşı yarısında durur.
- İşçi (`mac-isci.js`): kare 42'den 84 sayıya çıktı; 42–83 oyuncu başına görev sırası, hedef x, hedef y. Görev yazıları `kare` mesajında `is` dizisiyle bir kez gelir. Oynanışa etkisi yok (yalnız okuma).
- Yükleme çubuğu artık başlama payıyla aynı ölçüde (%30) dolar (önce %20'de doluyordu).

## 15 Ekim 2026 · Boş top yolu: çökme önlendi (main `21ea5ba`)
- Boş top yolunun son noktası okunurken çökme: `shape.js` flightRead, `decide.js` chasePlan ve recvPoint korumalı. Oynanış aynı.

## 15 Ekim 2026 · Canlı sayfa: başlama payı %30 (main `ec4dc5c`)
- Maç, oynanacak devrenin kalanının %30'u hesaplanınca başlar (önce %20); takılınca da aynı kural. Sebep: tarayıcıda hesap izlemeden yavaş kalabiliyor (bu konteynerde zip78 ~11 tik/sn, izleme 30 tik/sn).

## 15 Ekim 2026 · zip78 · Kale önünde boş top · `self.ALAN_KALE_KOVALA_ESKI` (main `d0b1110`)
- Top kendi kalesine 25 birimden yakın ve boştayken, rakibin ilk varışından en çok 4 tik sonra varabilecek her saha savunmacısı topa koşar; aynı durumda takım topu almadan hücum şekline geçmez.
- 4 maç: seken topu savunma %14 → %32; gönderme / maç 32 → 22; gönderme başına sayı %15 → %23.

## 15 Ekim 2026 · zip76 · Kaleci yeteneğe bağlı · `self.ALAN_KALECI_ESKI` (main `0e05745`)
- Elleri Alan hakimiyeti'yle güçlenir (×(1 + (H − 10) × 0,05)); tutamadığını rakipten uzağa çeler (17 yön, kale önü cezalı); çelerken topu iter (en az 0,8 + (H − 10) × 0,04). Gönderme tablosu yeniden üretildi (sürüm aynı).

## 15 Ekim 2026 · zip75 · Bölgede ve kapatmada bağlılık · `self.ALAN_BOLGE_BAGLI_ESKI` (main `efaae71`)
- Bölge savunmacısı tuttuğu rakibi yarıçapın 1,5 katında kaldıkça tutar; arkadakinin rakibi topun 4 birim önüne geçene kadar listede kalır; kapatmaya giden rakip tehlikeli kaldıkça işinde kalır (pas havadayken de); top el değiştirince hafıza sıfırlanır.

## 15 Ekim 2026 · zip74 · Kafa = maç, şekil onayı, adama bağlılık (main `bfc0866`)
- `decide.js` `ucusMac`: gönderme denemesi maçın kopyasında maçın kendi adımıyla (`ALAN_KAFA_ESKI`; paslar için `ALAN_KAFA_PAS`, kapalı).
- `shape.js`: şekil "topu kim alacak" tahmini 10 tik aynı kalınca değişir (`ALAN_SEKIL_ONAY_ESKI`); arkadaki tuttuğu adamı tehlikeliler arasında kaldıkça bırakmaz (`ALAN_ADAM_BAGLI_ESKI`).

## 15 Ekim 2026 · zip69 · B: Top gelmeden karar · `self.ALAN_ONCEDEN_KARAR_ESKI`
- **Kural** (`match.js step` başı):
  - Pas havadayken (gönderme değil) alıcı, Çekirdeği karşılayacağı noktada olacakmış gibi aynı akılla (`D.decide`) bir sonraki hamlesini seçer.
    - Karşılama noktası `alici()`'nin seçtiği nokta: `_rcvX/_rcvY`.
    - Şarj `recvCh` ile.
  - Düşünme süresi alıştaki gecikmeyle aynı (`9 − Tepki × 0,4 + rastgele 0–3`). Karar o kadar tik önce verilir; alışta `decT = 1`, hamle bir sonraki tikte.
  - Alıştan sonra yeniden karar yalnız durum değiştiyse:
    - en yakın rakip 1,5 birimden çok yaklaştı,
    - ya da alıcı karşılama noktasından 4,5 birimden çok saptı (dokunma menzili + Çekirdeğin son tikteki yolu).
  - Tek dokunuş (karşılamada karar) değişmedi; o zaten önceden karar.
  - Kafadaki maçta (`_lite`) yok: hız için; akıl alıcının bir sonraki hamlesini zaten ayrıca tartmıyor.
  - Karar başına bir `decide` çağrısı alıştan uçuşa taşındı. Durum değişirse bir tane daha.
- **Sayaç:** `m.st.onceden[takım]`: önceden verilen kararın kullanıldığı alışlar.
- **Kısa sınama** (2 tohum × 700 tik):
  - Önceden karar 3 alışta kullanıldı.
  - Koşarak alıp 10 tik sonra koşan 2/17 → 2/15.
  - Hız aynı (10,1 / 10,0 ms/tik).
  - **Örnek etkiyi görmeye yetmez.**
  - İlk denemede eşik 3 birimdi, hiç kullanılmadı (alıcı karşılama noktasından ~3 birim uzakta alıyor); 4,5'e çekildi.
- **Kontrol tablosunda beklenen (zip68'e göre):**
  - Alıştan ilk karara (tik) **↓** (3,7 → ~1).
  - Koşarak alıp koşmaya devam eden **↑** (≥ %60); ilk 20 tikte duran **↓**.
  - Ayrıca `m.st.onceden` / alış sayısı: önceden kararın kullanılma payı. Düşükse sapma eşikleri incelenir.
  - Pas tutma ve top kaybı kötüleşmesin. Hesap aynı makinede zip68 ± %5.

## 15 Ekim 2026 · zip68 · A: Alıcı topa gelir · `self.ALAN_TOPA_GEL_ESKI`
- **Kaynak:**
  - Kullanıcı önerisi ve Claude Code ölçümü: ayağa pasta son 10 tikte alıcının %30'u toptan uzaklaşıyor.
  - "Kesin" pas kayıplarının %60'ı alıcıya varmadan araya girme.
- **Sebep:** `shape.js alici()` buluşma noktasını `freeAt − rcvRisk × e^(−pay / T)` ile seçiyor.
  - Pay yalnız buluşma noktasında ölçülüyordu (rakip oraya ne zaman varır).
  - Daha uzak, daha boş nokta daha değerli görünüyordu; alıcı oraya koşuyor, rakip pası **yolda** kesiyordu.
- **Kural (yeni katsayı yok):**
  - Buluşmanın payı, Çekirdeğin o noktaya kadarki yolundaki her noktada (rakip varışı − Çekirdek varışı) en küçüğü.
  - Yolda bir rakip Çekirdekten önce varıyorsa sonraki noktalar seçilemez.
  - Rakip yola yetişebiliyorsa alıcı topa gelir; yol boşsa koşusunu sürdürür.
  - Önüne pas da aynı fonksiyondan geçer, ama yolu boş seçildiği için koşu sürer.
- **Kısa sınama** (tohum 5, 600 tik; alıcı topa 1,5–12 birimdeyken hızı toptan uzağa):
  - Uzaklaşan alıcı %39 → %19.
  - Pas tutma 5/9 → 9/9 (küçük örnek).
  - Hız aynı (10,2 / 9,2 ms/tik).
- **B (top gelmeden karar) bu turda yok.** Ayrı tur, kendi bayrağıyla: `ALAN_ONCEDEN_KARAR_ESKI`.
- **Kontrol tablosunda beklenen (zip67'ye göre):**
  - Ayağa pasta son 10 tikte toptan uzaklaşan **↓** (%30).
  - "Kesin" pas kaybı **↓** (%6,6), özellikle "alıcıya varmadan kesti" payı.
  - Pas tutma **↑**.
  - Koşarak alıp koşmaya devam eden: değişmesin ya da ↑.
  - Hesap: aynı makinede zip67 ± %5.

## 15 Ekim 2026 · zip67 · Sürme kopyası zip65 kapsamına döndü
- **Kaynak:** Claude Code zip66 ölçümü. Hız zip63 ile aynı (aynı makinede 39,6 / 38,8). Ama koşmaya devam etme kazancı gitti: %58,6 → %50,4; ilk 20 tikte duran %25,6 → %33,5.
- **Değişiklik:** Sürme kopyası yine "sür" en iyi seçeneğe 0,1 yakınken ve 30 tik.
  - zip66'nın dar kapsamı (önü 6 birim açık + ilk iki seçenek, 20 tik) bayrakla duruyor: `self.ALAN_SUR_DAR = true`.
  - Buluşma pası kapalı kaldı.
- **Kontrol tablosunda beklenen (zip66'ya göre):**
  - Koşmaya devam eden ≥ %58; ilk 20 tikte duran ≤ %26 (zip65 düzeyi).
  - Hesap: aynı makinede zip63'ün ~%20 üstü (zip65 bayrak ölçümü +%19). Kabul edilen bedel.
  - "Tut"tan sonra kayıp izlenecek (zip65 29,8 · zip66 18,2).
- **Sıradaki:** "Kesin" pasların kaybının %60'ı araya girme.
  - Akıldaki kesme hesabı ile maçın `contact` kuralı aynı mı, karşılaştırılacak; kendine pas %16,8 kayıp.
  - Ayrı tur.

## 15 Ekim 2026 · zip66 · Buluşma pası kapalı · sürme kopyası ucuzladı
- **Kaynak:** Claude Code zip64 ve zip65 notları.
  - Buluşma pası ölçülebilir bir şey değiştirmedi, hesap +%41.
  - Sürme kopyası koşmaya devam etmeyi artırdı: %53 → %59; ilk 20 tikte duran %33 → %26. Hesap +%19.
  - Sürmenin riski zaten doğru tahmin ediliyormuş (Claude Code ölçüm penceresi düzeltmesi). Kopyanın değeri risk değil, sürmenin **ilerleme değerini** maçın kuralıyla göstermesi.
- **1 · Buluşma pası varsayılan kapalı.** Açmak için `self.ALAN_BULUS_YENI = true`; `ALAN_BULUS_ESKI` artık etkisiz.
  - `lastDec.opts` artık `bulus` işaretini taşıyor (açıldığında seçilme oranı ölçülebilsin).
- **2 · Sürme kopyası** (`ALAN_SUR_KOPYA_ESKI` değişmedi) yalnız şu durumda oynatılıyor:
  - taşıyıcının **önü açık** (6 birim içinde, rakip Kuyu yönünde rakip yok),
  - ve "sür" ilk iki seçenekte (önce: en iyiye 0,1 yakın).
  - Kopya süresi en çok 20 tik (önce 30).
- **Kısa sınama** (tohum 3, 500 tik): kopya kapalı 8,7–9,1, açık 7,8–8,0 ms/tik. Sandbox'ta fark görünmüyor; maç akışı da değiştiği için kesin değil.
- **Kontrol tablosunda beklenen (zip65'e göre):**
  - Hesap ms/tik **↓** (hedef ≤ zip63 + %10, yani ~27,6).
  - Koşarak alıp koşmaya devam eden ≥ %58; ilk 20 tikte duran ≤ %26.
  - Pas tutma ≥ %92.
  - "Tut"tan sonra kayıp izlenecek (zip65'te 22,6 → 29,8).
  - Bekçi satırları zip63'e dönsün (buluşma kapandığı için akış değişir).
- **Sıradaki (kalibrasyon):**
  - "Kesin" pasların (P ≥ 0,99) %6'lık iyimserliği; kendine pas %99 → %87.
  - Claude Code kayıp sebebini ayıracak: alıcının kontrolü, araya giren, sekme.

## 15 Ekim 2026 · zip65 · Sürmenin riski maçın kuralıyla · `self.ALAN_SUR_KOPYA_ESKI`
- **Kaynak:** Claude Code bekleme kalibrasyonu (zip63, 490 karar).
  - Ortalama tutuyor, ama dilimler ayırmıyor.
  - Sebep sonraki hamlenin tutma ihtimali: sür beklenen %6 / gerçek %32; önüne %6 / %18; fake kayıtta yanlış sayı.
- **Kural:** Karar başına en umutlu "sür" seçeneği maçın kopyasında oynatılır (`holdWorld`, yeni `to` parametresi).
  - Taşıyıcı hedefine sürer (`driveTo`); rakipler maçtaki gibi araya girer, ikiliye girer, uzanır.
  - Tutma ihtimali = kopyada topu kaybetmeme; `keepP`'den büyük olamaz.
  - Değer: `Ps × Vend − (1 − Ps) × lossAt` (Vend, eski değerden geri çıkarılır).
  - `det.P` artık bu ihtimal; beklemeden sonraki hamle "sür" olduğunda `hw.sonraP`'ye o yazılır.
  - Kopya yalnız "sür" en iyi seçeneğe `bekleAtla` (0,1) kadar yakınken oynatılır; süre en çok 30 tik, tek kopya.
  - `det.Psim`: kopyanın sonucu (0 / 1), kayıt için.
- **Fake kaydı:** `hw.sonraP` fake için `null`. Fake'in `det.P`'si aldatma ihtimali, tutma değil; kalibrasyonda ayrı tutulsun.
- **Önüne pas iyimserliği (%14):** Bu turda dokunulmadı; zip64 (buluşma) ölçümünde pas kalibrasyonu ayrı çıkacak.
- **Kısa sınama** (2 tohum × 700 tik):
  - Kopyalarda sürmede tutma ort. 0,86.
  - Top kapma 4 → 1–2 (küçük örnek).
  - **Hız belirsiz:** 8,3 → 12–16 ms/tik. Maç akışı da değiştiği için sandbox'ta ayrılamadı. **Kontrol paketinde hesap satırı mutlaka bakılmalı;** pahalıysa kopya süresi ya da atlama eşiği küçültülür.
- **Kontrol tablosunda beklenen:**
  - Kalibrasyon: sonraki hamle "sür"de beklenen ile gerçek ±%5. Beklenen kayıp dilimleri gerçekle birlikte artsın.
  - "Tut"tan sonra kayıp **↓**. Top kaybı / dk **↓** ya da eşit.
  - Taşıyıcı topla koşuyor **±** (sür daha doğru tartıldığı için iki yöne de gidebilir; izlenecek).
  - Hesap ms/tik: **⚠ izlenecek** (hedef ≤ zip64 + %10).

## 15 Ekim 2026 · zip64 · Buluşma pası · `self.ALAN_BULUS_ESKI`
- **Kaynak:** Claude Code "önüne pas ve topla koşma" notu. Önüne pasın yalnız ~%22'si hedefin ±3'ünde alınıyor; %44'ü önce, %34'ü öte.
- **Sebep:** Boşluğa pas adayları noktadan seçiliyor. Alıcı o noktaya bugünkü yerinden düz koşacakmış gibi tartılıyor; koşu yönü hesapta yok.
- **Kural:** `decide.js options`'a yeni aday ailesi (`bulus: 1`, tür "önüne").
  - Her arkadaş için koşu yolu kurulur: koşuyorsa (hız > 0,08) hızının yönünde, değilse hedefine doğru, maçtaki hızıyla (`(0,17 + Hız × 0,006) × 1,25`).
  - Yol 3 tiklik adımlarla taranır. Çekirdeğin oraya varışı boşluğa pasın ölçüsüyle: `L / 1,1`.
  - Alıcının varışıyla farkı en küçük nokta; fark 4 tikten azsa aday.
  - Aday, diğer seçenekler gibi kafadaki denemeyle tartılır.
- **Kısa sınama** (2 tohum × 800 tik): Buluşma adayı kararların %85'inde var, 2 kez seçildi. Alış noktasının hedefe uzaklığı 10,3 → 8,4. Hız değişmedi (9,7 / 8,9 ms/tik). Örnek çok küçük.
- **Bu turda yapılmadı:** "alışta koşu sürsün" ve "ilk karar erken".
  - `take()` ilk kararı zaten 2–4 tikte veriyor (`decT = max(2, 9 − Tepki × 0,4 + rnd × 3)`, Tepki 20). Ölçülen "ilk karar 16 tik" başka bir yerden geliyor olabilir (tek dokunuş, karşılama, karar kaydının yalnız değişince yazılması).
  - Claude Code ilk kararın gerçek tikini `m.lastDec` yerine `decide` çağrısından ölçmeli; sonra bakılır.
- **Kontrol tablosunda beklenen:**
  - Önüne pasın hedefin ±3 içinde alınma payı **↑**.
  - Alış noktası ile hedef arası **↓** (bugün 8,2).
  - Koşarak alıp koşmaya devam eden **↑** (dolaylı).
  - Pas tutma, top kaybı ve sayı kötüleşmesin; hesap değişmesin.

## 15 Ekim 2026 · zip63 · C1 kaydına beklemeden sonraki hamle (motor davranışı değişmez)
- **Claude Code bulgusu:** baskı altındaki 486 "tut" kararında kopyada kayıp %0, gerçekte 30 tik içinde kayıp %13,8.
- **İnceleme** (tohum 3, rakip 2,97 birimde, kopya tik tik):
  - Rakip "geri geri" görevinde 1,7 birimde bekliyor ve 40 tikte ikiliye girmiyor.
  - Gerçek maçta da ikili çok seyrek: aynı maçın 2500 tikinde 1 ikili; taşıyıcı 2,2 birimde baskıdayken 73 tik.
  - Yani yerinde tutarken kayıp kopyada da maçta da nadir. Gerçekteki %14'lük kayıp büyük ihtimalle beklemeden **sonraki** hamleden geliyor: pas kesilir, fiske ya da dönüş kaçar.
  - `hw.kayip` yalnız bekleme süresince kaybı sayıyor. Beklemeden sonraki hamlenin riski değere giriyor (`v2 = Pk × b2.v − …`, `b2.v` kendi tutma ihtimaliyle), ama kayda yazılmıyordu.
  - İki sayı aynı şeyi ölçmüyor.
- **Ek (kayıt):** `aday[].hw`'ye üç alan eklendi.
  - `sonra`: beklemeden sonra seçilecek hamle.
  - `sonraP`: o hamlenin tutma ihtimali (gönderme için girme).
  - `Pk`: beklemede tutma ihtimali.
  - Kalibrasyon: **beklenen kayıp ≈ 1 − Pk × sonraP** ile gerçek kayıp (bekleme + sonraki hamle) karşılaştırılsın.
- **Ayrıca ölçülmeli:** Kopyanın rakibi varsayılan taktikle oynuyor (`createMatch(seed, {})`, şiddet 0,5). Gerçek rakibin pres şiddeti ve karşı presi (`m.kp`) kopyada yok. Yüksek preste kopya iyimser olabilir. Ölçüm, rakip pres şiddetine göre ayrılırsa görünür.
- **C2:** Kopya düzeltilene kadar `ALAN_BEKLE_HEP` ile karşılaştırma geçerli. Hız kazancı kalsın mı, kalibrasyondan sonra karar.

## 15 Ekim 2026 · zip62 · Bekçi teması: iki değişiklik, iki bayrak
Kaynak: Claude Code "Bekçi teması" notu. 8 maç, 177 gönderme, 32 sayı.
- 19 sayıda Bekçi topa hiç değmedi; öne çıkmış hâlde elleri yoktu.
- 13 sayıda değdi. Kuyu'ya doğru seken 12 temasın 10'u sayı.

### 1 · Bekçi her yerde Bekçi uzanmasıyla · `self.ALAN_BEK_ALAN_ESKI`
- **Karar (kullanıcı):** Bekçi'nin uzanmasının yer sınırı yok.
- `core.js contact`: `ALAN_BEK_ALAN` (12) koşulu bayrak kapalıyken uygulanmaz; Bekçi her yerde `bekR` ile uzanır. Önce öne çıkış 13 birime kadar gidiyor, 12'nin dışında saha oyuncusu uzanmasına düşüyordu.
- Akıl `decide.js`'teki kendi yol kontrolünde Bekçi'ye zaten sınırsız `bekR` veriyordu. Bu değişiklikle maç da akılla aynı.
- **Kontrol tablosunda beklenen:**
  - Sayılarda Bekçi'nin değdiği pay **↑**.
  - Değmeden giren sayı **↓**.
  - Bekçi pas kovalaması ve kovaladığını kazanma değişmesin (zip59 kuralı Bekçi'yi alanında tutuyor).

### 2 · Bekçi'nin çelmesi kapasiteden bağımsız · `self.ALAN_BEK_CELME_ESKI`
- **Kural:** `contact` sekme yönü `st = steer × bekSteer × (0,5 + (Hakimiyet − 10) × 0,04)`, `min(1, capE / vr)` çarpanı Bekçi için yok.
  - Kapasite "tutar mı"yı, Alan hakimiyeti "nereye çeler"i belirler.
  - Saha oyuncularında değişiklik yok.
- **Kontrol tablosunda beklenen:**
  - Kuyu'ya doğru seken temas **↓** (bugün %7).
  - Gönderme başına sayı **↓**, sayı / dk **↓**.
  - Tutma oranı değişmesin.

### Gönderme tablosu
- `SHT_SURUM 'v5-18 · direk 0.3 · Bekçi çelme'`, yeniden üretildi. **Değerler birebir aynı.** Kafadaki gönderme denemesi (`imagine`) Bekçi temasını kendi yoluyla tartıyor, `contact`'ın sekme yönünü görmüyor.
- **Açık:** Akıl çelmeyi görmüyor (ayrı taklit). Ölçümde gönderme kararları sayı oranıyla uyumsuz çıkarsa `imagine`'in Bekçi teması `contact`'a bağlanmalı.

### Sabit deneme (yalnız yön; kanıt değil)
300 vuruşluk dört sahnede sayı sayısı değişmedi. Sahneler düz şut, temas az. Asıl ölçüm maçta.

## 15 Ekim 2026 · zip61 · zip60 ölçümünden sonra: B kapalı, C1 kaydı düzeldi
- **B (atama) varsayılan kapalı.** zip60 ölçümü:
  - Topsuz rakipte 4 br içinde iki savunmacı arttı: 28 → 44 / dk; B kapalıyken 29.
  - Sayı arttı: B kapalıyken 4,63 → 3,50.
  - Hedef satır (boş adam + çift adam) değişmedi.
  - Bayrak tersine döndü: açmak için `self.ALAN_ATAMA_YENI = true` (`ALAN_ATAMA_ESKI` artık etkisiz). Kod yerinde; "aynı rakibe 4 birim içinde ikinci kişi yok" koşuluyla yeniden tasarlanacak.
- **C1 düzeldi:** `match.js lastDec.opts` seçenekleri yeniden kurarken `hw`'yi atıyordu; artık taşıyor. Karar kaydında "tut" seçeneklerinin kopya sonucu (`aday[].hw`) dolu gelir.
- **C2, C3 değişmedi.** C2 şüphesi (baskı altında "tut" seçimi %14 → %22–26) `ALAN_BEKLE_HEP` ile 8 maçta ayrılacak.
- **A2 uygulanmadı** (bkz. sohbet notu):
  - Bekçi tepkisi Refleks'ten okunuyor (`ALAN_OKC_K.bekci = 'refleks'`). Refleks 20'de tepki zaten 0 tik; uzanma 3. tikte tavanda (2,9).
  - Asıl soru temas anı: kapasite (1,3–1,7) ile güçlü şut (~3), hazırlık (ilk 2 tik %40) ve sekme yönü. Ölçüm istendi.
- **Kontrol tablosunda beklenen (zip60'a göre):**
  - B kapandığı için sayı, gönderme başına sayı, topsuz rakipte çift savunmacı ve geri koşan pasçı zip60'ın "B kapalı" koluna dönsün.
  - Diğerleri değişmesin.

## 15 Ekim 2026 · zip60 · Paket: B + C1 + C2 + C3 (her biri kendi bayrağıyla)
Bayraklar birbirinden bağımsız. Hepsi `true` iken (C1 hariç) zip59 ile aynı davranış.

### B · Savunma: görevler tek elden · `self.ALAN_ATAMA_ESKI`
- **Yer:** `shape.js`, Bekçi hedef satırından hemen önce. Bütün savunma görevleri (pres, arkada, bölge, kapatma, ortak atama) seçildikten sonra çalışır, onların üstüne yazar.
- **Kural:**
  1. Taşıyıcı dışındaki rakipler `threat × (0,3 + 0,7 × hat açıklığı)` ile sıralanır.
  2. Adam adama oyuncu (`tl adam ≥ 0,5`) kendi adamında kalır, havuzdan çıkar.
  3. İki presçi varken, tehlikeli (en tehlikelinin `ortakMin` %30'u üstü) ve kimsenin almadığı rakip sayısı havuzdan fazlaysa ikinci presçi havuza döner.
  4. Sırayla her rakibe havuzdan **varış süresi en kısa** savunmacı atanır. Bölge oyuncusuna evinden uzaklık ×0,5 eklenir. Önceki eşleşme 1,3 kat pahalıya kadar korunur. Hedef `kapatHat` (adam ile Kuyu arası, pas hattına göre); görev "atama · Ad".
  5. Çok düşük tehlikeli rakip (en tehlikelinin %15'i altı) atanmaz.
  6. Atanmayan savunmacı başkasına atanmış rakibin peşindeyse evinde "boşluk tut"; değilse görevini (boşluk, arkada) sürdürür.
- **Kısa sınama** (2 tohum × 700 tik, rakip yarısı değil bütün saha, karar tiki başına):
  - 8 birimde savunmacısı olmayan hücumcu: 3,28 → 2,49 (5 kişiden).
  - Görev değişimi: 0,79 → 0,54.
  - 4 birimde iki savunmacı: 0,03 → 0,14 (arttı; büyük ihtimalle presçi ile atanan, taşıyıcının yanındaki rakipte).
  - Hız: 10,1 → 10,7 ms/tik.
- **Kontrol tablosunda beklenen:**
  - Boş hücumcu zamanın % **↓** (≤ %40).
  - Boş adam + çift adam anı / dk **↓** (≤ 5).
  - İki kişi "arkada" **↓** (≈ 0).
  - Taşıyıcıya baskı yok % ≤ %14.
  - Top kazanma ve yenen sayı kötüleşmesin.
  - Hesap ms/tik +%5'ten az artsın.
  - **İzlenecek:** 4 birimde iki savunmacı (kısa sınamada arttı).

### C1 · Bekleme kaydı (davranış değişmez, bayrak yok)
- Karar mesajında (`mac-isci` → `aday[].hw`) her "tut" seçeneği için kopyanın sonucu: `{ kayip, tik, T, v }`.
  - `kayip`: kopyada Çekirdek kaybedildi mi.
  - `tik`: kaçıncı tikte.
  - `T`: bekleme süresi.
  - `v`: seçeneğin son değeri.
- Kopya oynatılmadıysa (C2 atladıysa) `hw` yok.

### C2 · Kopya yalnız gerektiğinde · `self.ALAN_BEKLE_HEP`
- Rakip 4 birimden uzakken ve "tut"un ilk değeri en iyi tut-dışı seçenekten `bekleAtla` (0,1) kadar düşükken kopya oynatılmaz; "tut" ilk değerinde kalır. Baskı varken hep oynatılır.
- **Kontrol tablosunda beklenen:**
  - Hesap ms/tik **↓** (zip50 + %5'e doğru).
  - "Tut"tan sonra kayıp, baskı altında "tut" seçimi, kontrada duran: değişmesin.

### C3 · Varsayılan geçiş "eşitsek de git" · `self.ALAN_GECIS_ESKI`
- `GECIS_VARS`: `gecisH` ve `kazaninca` verilmemişse `'esit'` (önce `'ustun'`). Kontra `'esit'`, Yerleş `'hic'` aynı.
- **Kontrol tablosunda beklenen:**
  - Kontrada önü boşken duran / dk **↓** (≤ 0,1).
  - Top kazanınca ilk 3 sn'de gönderme **↑**.
  - Geçiş "git" süresi **↑**.
  - Yenen kontra sayısı izlenecek.

## 15 Ekim 2026 · zip59 · Öneri A: Kuyu alanı Bekçi'nin bölgesi (tek değişiklik)
- **Kaynak:** Claude Code'un zip51–58 değerlendirmesi, Öneri A. Çalışma düzeni: her tur tek değişiklik; "çözüldü" ancak kontrol tablosunda.
- **Kural (tek cümle):** Kuyu alanı Bekçi'nin bölgesidir. Alanın içindeki her topa gider; dışındakine yalnız serbest topa ve rakipten belirgin önce varacaksa.
  1. **Alan içi:** Alana giren top, pas ya da sekme Bekçi'nin. Takımdaşla "ilk kim varır" yarışı yok.
  2. **Alan dışı:** Uçan bir pası araya girip kesmek için alanını bırakmaz. Yalnız serbest topa (sekme, boşta, durmuş) ve zip50'nin payıyla (Yerleşim 20'de 2 tik, 10'da 8) rakipten önce varacaksa çıkar. `kaleye` payı kaldırıldı (alan içi kuralı kapsıyor).
  3. **Dönüş:** Hedefinden 3 birimden uzak ve Kuyu–top hattından 1,5'ten uzaksa önce o hattın kendisine en yakın noktasına koşar (görev "Bekçi · dönüş"), hatta girince açıortaya yerleşir.
  4. **Yerleşmiş Bekçi:** Hızı 0,05'ten düşük ve hedefine 1'den yakınsa uzanma tepkisi ×0,6 (`core.js bekR`). Ekranda gösterilebilir: "yerleşmiş: tepki hızlı".
- **Bayrak:** `self.ALAN_BEKCI_A_ESKI = true` → zip58.
- **Gönderme tablosu:** `SHT_SURUM 'v5-17 · direk 0.3 · Bekçi alanı'`. Tablodaki Bekçi yerinde durduğu için yerleşmiş sayılır. Açı dilimine göre ortalama girme oranı:
  - 0: 0,491 → 0,434
  - 2: 0,457 → 0,398
  - 4: 0,346 → 0,292
  - 6: 0,220 → 0,212
  - 7: 0,154 → 0,150
  - Ortadan ve yakından gönderme yaklaşık %12 zorlaşıyor; dar açı değişmiyor.
- **Kısa sınama (tohum 3, 1200 tik):** Bekçi zamanın %44'ünde yerleşmiş, %12'sinde dönüşte. Hız eski bayrakla aynı (sandbox 15,9 / 17,2 ms/tik).
- **Kontrol tablosunda beklenen değişim:**
  - Bekçi pas kovalaması / dk: **↓** (≤ 1,0). Kovaladığını kazanma: **↑** (≥ %75).
  - Sayılarda Bekçi yerinde: **↑** (≥ %75).
  - Yakın direkten sayı / dk: **↓** (≤ 0,5). Dar açıda Bekçi–yakın direk: **↓** (≤ 2,5).
  - Gönderme başına sayı: **↓** (tablo %12 düştü; yerleşmiş Bekçi).
  - Sayı / dk: **↓**.
  - Yakın geniş (<15, 60–75°) sayı oranı yakın düzden (<15, 0–30°) yüksek olmasın.
  - Hesap ms/tik: değişmesin.
  - Diğer satırlar değişmesin.
- **Açık (bu tur değil):** HEPSI_20 Alan gücü ve Soğukkanlılık'ı kapsamıyor.

## 14 Ekim 2026 · zip58 · Bekçi çizgide durabilir (yakın direk)
- **Gözlem (kullanıcı, tohum 658430, 03:57, Kartal 3–3):** Kartal (9,5; 2,5)'ten güçlü gönderdi. Çekirdek Aras'a 1 birim kala dokundu, sekti ve Aras ile yakın direk arasından girdi.
- **İz** (`test-bekci.js`): Aras'ın hedefi açıortayda (1,05; 26,27), ama `p.tx` 1,5'e kırpılmış. Aras (1,57; 26,3)'te; yakın direkle arasında Çekirdeğin geçeceği aralık kalıyor.
- **Sebep:** `shape.js` sonundaki saha payı (`p.tx = cl(p.tx, 1.5, GW − 1.5)`) Bekçi'ye de uygulanıyordu. Bekçi çizgiden 1,5'ten yakına gelemiyordu; dar açıda yakın direk açık kalıyordu.
- **Kural:** Bekçi'nin kenar payı 1 (Çekirdek ve gövde fiziğinin `0,8` sınırının içinde). Açıortay kuralı değişmedi.
  - Önce bir "yakın direğe kayma" kuralı da denendi; açıortay hedefi zaten doğru olduğu için geri alındı (ortadan gelen şutta Bekçi'yi bir yana çekiyordu).
- **Sınama:** Aynı vuruş, ±1,7° dağılım, 300 tekrar, Bekçi sabit.
  - Eski yer (1,57; 26,3): 24 sayı (%8).
  - Yeni hedef (1,05; 26,27): 3 sayı (%1).
  - 1,5 sınırı: 22 sayı.
- **Gönderme tablosu:** `SHT_SURUM 'v5-16 · direk 0.3 · Bekçi çizgide'`, yeniden üretildi. Kafadaki Bekçi zaten kırpılmıyordu; değişim 69°+ dilimlerde +0,005.
- **Deney:** `self.ALAN_BEKCI_ESKI = true`. `test-bekci.js { bayrak, bayrakSonra }`.
- **Not:** Kullanıcının tarayıcısındaki kayıt bu turda okunamadı (oturum). Maç anındaki yeniden oynatma yerine yukarıdaki sabit Bekçi denemesi yapıldı.
- **Claude Code ölçecek (16 maç):**
  - Açı × mesafe tablosu; 60°+ dilimlerinde sayı oranı.
  - Bekçi'nin yakın direğe uzaklığı (dar açılı göndermelerde).
  - Gol, hız.

## 14 Ekim 2026 · zip57 · Baskı altında da bekleme maçın kuralıyla
- **Gözlem (kullanıcı, kayıt tohum 658430, 03:52):** Fırtına kontrasında Volkan (34, 50) topla durdu.
- **Karar kaydı** (`mac-isci` karar mesajı):
  - Tik 6953–6980: düz "tut" 0,80–0,91; "önüne → Yıldırım" 0,15–0,83.
  - Tik 6997'de pasın değeri 0,14'e düştü.
- **Sebep:**
  - Beklemenin dünyasını maçın kuralıyla kuran blok (`holdWorld`, zip54) yalnız en yakın rakip 4 birimden uzaktayken çalışıyordu.
  - Yakın rakip varken düz "tut" eski taklitle (`respond`: iki rakip taşıyıcının önüne dolanır) ve artan şarjla tartılıyordu.
  - "Savunma yerleşir" düzeltmesi `o.to` isteyen bir süzgeçten geçtiği için "tut"a hiç uygulanmıyordu.
- **Kural:** Bekleme her durumda maçın kopyasında oynatılır. Baskı altında ikili ve gövde koruması da kopyada gerçek kuralla; Çekirdek kaybedilirse değer `−lossAt`.
- **Deney:**
  - `self.ALAN_BASKI_ESKI = true`: eski 4 birim kapısı.
  - `self.ALAN_BEKLE_TAKLIT`: eski taklit, ikisini birden açar.
  - `test-bekle.js { yeniTaklit: true }`: kaydı zip56 kuralıyla oynatır, o anda iki kuralı tartar.
- **Sınama:**
  - O an (tik 6962) yeniden kuruldu (Volkan 33,8 / 50,2), ama test aracındaki karar değerleri maçtaki kayda uymadı: "tut" 0,52 / 0,44 iki kuralda da. Araç maçtaki taktik bağlamını (`tacEff`, `macZ`) birebir kuramıyor olabilir. **Bu anın kanıtı değil; Claude Code ölçmeli.**
  - Hız: kısa sınama 11,8 ms/tik.
- **Ayrıca (aynı maç, 03:57):** Kartal (10, 1)'den güçlü gönderdi, sayı. Arka çizgiye 72°, ağza 30 birim.
  - Akıl 0,54 verdi; gönderme tablosu bu dilimde 0,215.
  - Direk bunu kesmez (yol ağzın içinden geçiyor). Bekçi yakın direkteydi.
  - **Claude Code:** zip50 ölçümündeki açı × mesafe tablosunu (60–75°, 25+) zip57'de yeniden bak. Bu hücrede sayılar hâlâ çıkıyorsa Bekçi'nin yakın direkteki uzanışını izle.
- **Claude Code ölçecek (16 maç):**
  - Topu kazandıktan sonra ilk 3 sn'de taşıyıcının tuttuğu süre (zip56 ile).
  - Baskı altında "tut" seçimi ve kayıp oranı.
  - Gol, pas, hız.

## 14 Ekim 2026 · zip56 · Plan duruma aittir: pası veren eski hedefine dönmez
- **Gözlem (kullanıcı, kayıt tohum 633201, 00:34):** Kontrada Tuna sağdan Cansu'ya pas verdi, sonra ileri değil kendi yarı sahasına koştu.
- **İz** (`test-gorev.js`, `iz: 'Tuna'`: aday tartımı kaydı):
  - Pastan sonra (tik 1032) Tuna'nın aday tartımı hiç çalışmamış. Hedef (58, 47), top ona gelmeden önceki eski planı (`p.otx`).
  - Plan, ritim tikine kadar ya da hedefe varana kadar korunuyor. Tuna geriye 40 tik koştu, varınca yeniden tarttı, sonra (102, 35)'e döndü.
  - Geçiş kuralı ve evre etkilemiyor (görev "boşluk", geçiş yok).
- **Kural:** Topu alan (taşıyıcı) ve topa koşan (alıcı, kovalayan) oyuncunun koşu planı silinir. Pası verince yeni durumda yeniden tartar.
- **Deney:**
  - `self.ALAN_PLAN_ESKI = true` (zip55).
  - `self.ALAN_IZ = 'Ad'`: o oyuncunun aday tartımı `self.__iz`'e yazılır (bileşenler: koşu, rol, takım değeri, bağ).
  - `test-gorev.js` `{ iz, planEski }` alır.
- **Sınama (aynı an, T1 = 1000, eski plan kuralıyla):** Tuna pas anında (1032) hedef (102, 35), ileri koşuyor; tik 1128'de (99, 37). Eskiden aynı tikte (75, 42) idi.
- Kısa sınama: 10.1 ms/tik.
- **Not:** Bir önceki denemede geçiş koşucularının kalıcılığı (`g.kosu`) eklenip ölçüldü. Bu anı değiştirmediği için geri alındı.
- **Claude Code ölçecek (16 maç):**
  - Pas verenin pastan sonraki 1 sn'de ileri giden mesafesi (ver-koş).
  - Pas sonrası geri koşan pasçı oranı.
  - Gol, pas, hız.

## 14 Ekim 2026 · zip55 · Presçi bıraktığı adamı tartar; arkada kalan da presçi adayı
- **Gözlem (kullanıcı, kayıt tohum 161648, 01:12–01:17):** `test-gorev.js` ile görev izi alındı.
  - **Volkan'a kimse basmıyor** (tik 2172–2196, Volkan 57,57 yan çizgide orta sahada). En yakın Mert (16 birim) "arkada" olduğu için aday değildi; diğerleri 30+ birim uzakta.
  - **Ece, Kartal'ı bırakıp Yıldırım'a ikinci presçi oluyor;** sonra Kartal'da Deniz ve Ece ikisi birden basıyor, Yıldırım uzaktan kapatılıyor. Presin bedeli (`prV cost`) yalnız presçinin **evinin** 9 birim yakınındaki rakiplere bakıyordu; tuttuğu adam evinden uzaksa bırakmanın bedeli 0'dı.
- **Kural 1:** Savunmacının o an tuttuğu rakip (`p.tutQ`: markaj, bölge, kapatma, arkada görevinin rakibi) presin bedeline girer. Basmaya çıkmak, bıraktığı adamın tehlikesi (hat açıklığıyla) kadar pahalı. Hücum eden takımda `tutQ` her tik silinir.
- **Kural 2:** "Arkada kalan" presçi adaylığından etiketle dışlanmaz. Arkasında bıraktığı rakibin bedeli karar verir.
- **Deney:** `self.ALAN_SAV_ESKI = true` (zip54 kuralı). `test-gorev.js` ile `{eski: true, eskiSonra: false, T1}`: T1'e kadar eski kural, sonra yeni.
- **Sınama (aynı an, T1 = 2160):**
  - Kartal topu aldığında Deniz ve Ece basıyor; Mert Volkan'ı, Kaan Efe'yi kapatıyor, Cansu ve Tuna Yıldırım tarafına geri koşuyor. Ece topu kazanıyor (eski kuralda Kartal pası Yıldırım'a veriyordu).
  - **Volkan'a yine basılmıyor:** orta sahada yan çizgide tehlikesi düşük, en yakın adaylar onun bir sonraki hamlesinden önce varamıyor. Orta blokta bu taktik bir sonuç; daha önde basmak için pres hattı ya da şiddeti.
  - Hız: kısa sınama 11,3 ms/tik.
- **Claude Code ölçecek (16 maç):**
  - İkinci presçinin bıraktığı adama giden pas ve o pastan şut.
  - Taşıyıcının 12 birim içinde presçi olmadan geçen süre.
  - Top kazanma, yenen gol, hız.

## 14 Ekim 2026 · zip54 · Beklemenin dünyası maçın kendi kuralıyla
- **Sorun (kullanıcı, kayıt tohum 720987, 3 kişilik kontra):** Cansu (84, 10) boşta, önünde Kaan ve Tuna. Durdu, savunma yerleşti; pozisyon sonunda sayı oldu ama çok daha zor yoldan.
  - Karar kaydı (tik 5528): "tut 0,5 sn bekle" 0,962 · "önüne → Kaan" 0,858. Bir saniye sonra aynı pas 0,623.
- **Sebep:** Beklemenin değeri, bekleme sonundaki dünyada yapılacak en iyi hamle. O dünya `predictWorld` ile kuruluyordu: maçtan ayrı, elle yazılmış bir taklit. Rakibin geri koşusunu küçük görüyordu (tepkiden sonra hedefe doğru sabit hızla; yakındaki iki kişi dışında hep "Çekirdekle Kuyu'nun %45'i"), arkadaşları ise hedeflerine tam hızla taşıyordu. Kontrada beklemek pastan değerli çıkıyordu. CLAUDE.md'deki "akıl kafada ayrı taklit yazmaz" ilkesine aykırıydı.
  - `lookAhead` (maçın kuralıyla kafada oynatma) `look: false` ile kapalı; açık olsaydı da bekleme bloğu onun değerini sonradan eziyordu (`o.look` varsa artık ezmiyor).
- **Kural:** `decide.js holdWorld`: taşıyıcı T tik yerinde tutar, maçın kopyası (`M.kopya`, `_lite`) `M.step` ile oynatılır. Herkes maçtaki gibi hareket eder: shape.position, ivme, geri koşu, pres.
  - Kopyada Çekirdek kaybedilirse beklemenin değeri `−lossAt`.
  - Rakip taktiği bilinmez, varsayılanla (lookAhead gibi).
  - Aynı karar içinde aynı T için bir kez oynanır.
- **Deney:** `self.ALAN_BEKLE_TAKLIT = true` eski taklidi açar.
  - `test-bekle.js`: kaydı eski kuralla T'ye kadar oynatır, o anda iki kuralı da tartar.
  - Tohum 720987, T 5528, Cansu: taklit "tut" 0,962 / "önüne → Kaan" 0,858 (eski kayıtla birebir). Maçın kuralı "önüne → Kaan" 0,858 / "tut" 0,178.
- **Hız:** kısa sınama 9,4 ms/tik (500 tikte 72 bekleme oynatması, ort. 30 tik). Kontrol paketiyle doğrulanmalı.
- **Claude Code ölçecek (16 maç):**
  - Topu kazandıktan sonra ilk 3 sn'de taşıyıcının tuttuğu süre ve geçişte şut.
  - Uzun tutuşta duran hücumcu.
  - Gol, pas, hız.
  - Aklın bekleme tahmini → gerçek (bekledikten sonraki en iyi seçeneğin değeri).

## 14 Ekim 2026 · zip53 · Devre devre hesap
- **Kullanıcı isteği:** Maç devre devre yüklenir. Taktik değişikliği içinde bulunduğu devreyi etkiler: ilk yarıdaki değişiklik ilk yarıyı hemen etkiler, devre arasındaki ikinci yarıyı.
- **İşçi (mac-isci):**
  - `yeni` mesajında `dur` (Canlı: `len / 2`). İşçi ilk yarının son tikinde durur ve `yari: true` ile haber verir (`son` değil; maç bitmiş sayılmaz).
  - `devam` mesajı `dur = len` yapar ve hesabı ikinci yarıya sürdürür.
  - Kopyadan yeniden oynatma (`degistir`) `dur`'a kadar gider.
  - Kopyası olmayan yeniden oynatmada `createMatch` artık `devre` bayrağını da alıyor (önce düşüyordu).
- **Canlı:**
  - Maç, ilk yarının %20'si hesaplanınca başlar (önce bütün maçın kalanının %20'si).
  - Devre tikinde izleme durur ve `devam` gönderilir: ikinci yarı devre arası ekranı açıkken arka planda hesaplanır.
  - "İkinci yarı" basıldığında hazır olan yetmezse yükleme ekranı, ikinci yarının kalanının %20'si hazır olunca açılır.
  - Devrede taktik değiştirmek kopyayı tam devre tikinden (3600) alır; yükleme biter, devre ekranı geri gelir, maç kendiliğinden başlamaz.
  - Yükleme çubuğu oynanan devreye göre dolar.
- Motor değişmedi: aynı tohum aynı maç.

## 14 Ekim 2026 · zip52 · Çekirdek sahanın dışında olamaz
- **Sorun (kullanıcı, kayıt tohum 812226, Kartal 0–1):** Kayıttan yeniden oynatıldı; olay sırası aynı.
  - Kartal arka çizgide (x 0,8, y 18,8), ağzın 6 birim yanında. Çekirdek gövdesinin duvar yanında, **çizginin arkasında** (x 0,42) yola çıktı. Gönderme duvara paralel.
  - Aras dokundu, Çekirdek sekti. Sekme Çekirdeği gövdenin öbür yanına, **duvarın arkasına** (x −0,68, y 23,2) koydu.
  - Kuyu denetimi yolun içeriden geldiğini varsayıyor (`tt` 1'e kırpılıyor, `yy = y + vy` = 25,1 ağız içinde): arkadan gelen Çekirdek sayı sayıldı.
  - Akıl da aynı fiziği gördüğü için çizgiden bu göndermeyi seçiyordu.
- **Kural:** Çekirdek hiçbir zaman sahanın dışında olamaz: `core.js sahaIci(b)`, Çekirdeği `[0,5, W − 0,5] × [0,5, H − 0,5]` içine alır. Çağrıldığı yerler:
  - `makeBall`: bütün vuruşlar; taşıyıcının gövdesinin duvar yanından çıkan Çekirdek çizgiye alınır.
  - `contact`: tutma yerleşimi ve gövdeden sekme yerleşimi.
  - `govde` ve `govdeIt`: gövde çarpışması ve itme.
- **Sınama** (aynı kayıt, 1500 tik): o sayı yok (Kartal o anda gönderemiyor; skor 1–0 Liman). Saha dışında Çekirdek: 0 tik.
- Maçlar bu andan sonra ayrışır. Gönderme tablosu etkilenmez (tablo vuruşları sahanın içinden).
- **Claude Code:** 16 maçta `b.x < 0,5` ya da `> W − 0,5` olan an sayısı (0 beklenir) ve sayıların gönderme noktasının arka çizgiye uzaklığı (1 birimden yakın gönderme sayıları kaldı mı?).

## 14 Ekim 2026 · zip51 · Maç 120 sn, devre arası, yükleme %20
- **Maç süresi (kullanıcı kararı):** 7200 tik = 120 sn (önce 5400 = 90 sn).
  - Tek kaynak `core.js` `self.ALAN_MAC_LEN`. Okuyanlar: `createMatch` varsayılanı, mac-isci, lab-isci, olcum-donma, `yzUyum`, Canlı.
  - Deney: core.js'ten önce `self.ALAN_MAC_LEN = 5400`.
  - Beklenen: sayı ~2,1 → ~2,8 / maç; oranlar aynı.
  - `MAC.oran` (tik başına sayı oranı) değişmedi; o bir oran, maç uzunluğu değil.
- **Devre arası:** `m.len / 2` tikinde, adımın başında `ev('devre')` ve 2. takımın santrası (ilk santra 1. takımın).
  - Kural adımın başında: devrede verilen taktik değişikliği (kopya tam 3600'de) santranın dizilişine de uyar.
  - Yan değişimi yok: saha simetrik, motor her takımı hep aynı yöne oynatıyor; yan değiştirmek oyunu değiştirmez.
  - `createMatch({ devre })`: varsayılan açık.
  - mac-isci `d.devre ?? false` (eski kayıtlar devresiz ve 90 sn, birebir oynar). Lab `devre: false` (yarım maç).
- **Canlı:**
  - İzleme devre tikinde durur. "DEVRE ARASI" ekranı: skor, dört istatistik, "İkinci yarı ▶" ve "Taktik". Taktik değişikliği ikinci yarı santrasından itibaren geçerli.
  - Zaman çubuğunda geri sarınca devre yeniden tetiklenir.
  - Maç kaydına `len` ve `devre: true` yazılıyor.
- **Yükleme:** Maç, kalan sürenin %20'si hesaplanınca başlar (önce %30); yükleme çubuğu da %20'ye göre dolar.
- **Claude Code ölçecek:**
  - 16 maç, 120 sn: sayı, gönderme, hız.
  - Devre sonrası ilk 10 sn'de sayı (santranın etkisi).
  - Eski kaydın birebir oynaması (len 5400, devresiz).

## 14 Ekim 2026 · zip50 · Bekçi rakipten önce varamayacağı topa çıkmaz
- **Sorun (Claude Code, 32 maç):**
  - Bekçi'nin yerinde olmadığı sayıların çoğunda (58'in 50'si) Bekçi rakip pasını kovalamış, yetişememiş; gönderme o dönerken gelmiş.
  - Maç başına ~17,6 kovalama var; bunların %65'inde top rakibe gidiyor.
  - Kural (`shape.js` ~112): Bekçi topa **kendi takımında** ilk o varacaksa çıkıyordu; rakibin varış süresine bakılmıyordu.
- **Kural:** Bekçi, kendi alanının (`KR`) dışındaki bir topa ancak rakibin en erken varışından belirgin önce varacaksa çıkar: `r.est + bekOnce + (20 − Yerleşim) × marginT < rakip est`.
  - `bekOnce` 2 tik. Yerleşim 20'de pay 2 tik, 10'da 8 tik: iyi Bekçi ne zaman çıkacağını daha iyi bilir.
  - Yetişemeyeceği topa çıkmazsa açıortayda kalır ve topun varış noktasına (`E`) göre yeniden yerleşir (`bekci()`, değişmedi).
  - Kendi alanındaki topa her zaman gider; oraya kimse giremez.
  - Arkadaşın alıcı olduğu paslarda kural yok; Bekçi zaten kendi takımının pasına koşmuyordu.
- **Kısa sınama** (tohum 3, 1500 tik): oyun akıyor, 12,4 ms/tik (sandbox oynaklığı içinde).
- **Claude Code ölçecek (16 maç):**
  - Bekçi kovalaması ve kovalamada topu kazanma oranı.
  - Sayılarda Bekçi'nin yerinde olma payı.
  - Açı × mesafe tablosu, toplam sayı.

## 14 Ekim 2026 · zip49 · Kuyu ağzında direk
- **Sorun (kullanıcı, Claude Code ölçümü):** Ağız arka çizgide düz bir boşluktu. Çizgiyi `|y − orta| ≤ MOUTH` aralığında geçen her yol sayıydı, açıdan bağımsız.
  - Cansu'nun sayısı (109, 1,5'ten, 69°) ağzın kenarından 0,15 payla girdi.
  - Sayıların üçte biri ağzın son 1 biriminden giriyor.
- **Kural:** Ağzın iki ucunda yuvarlak direk. Merkezi duvar çizgisinde (`x = 0,5 | W − 0,5`, `y = orta ± MOUTH`), yarıçap `DIREK = 0,3` (mutlak).
  - Çekirdeğin bu tikteki yolu direğe duvar çizgisinden önce değerse değdiği noktada durur ve duvar sekmesinin kuralıyla seker: hız yüzeye göre yansır, `rest` ile azalır, enerji `wallLoss` kadar düşer.
  - Yolun yaşı sıfırlanır (`walls++`).
  - Açı eşiği yok: ağza giren yolun direğe uzaklığı açıyla birlikte azalır; dar açıdan kenarı sıyıran yol direğe değer.
- **Tek fonksiyon:** `core.js direk()`; `stepBall`, `rollTo`, `predict` üçü de çağırıyor. Akıl ve gönderme tablosu aynı fiziği görüyor.
- **Olay:** `k: 'direk'` ("Direkten sekti"). Çekirdekte `b.direk` sayacı tutuluyor.
- **Deney:** `self.ALAN_DIREK = .6` (core.js'ten önce). `0` = direksiz: zip48'in tablosu ve sürümüyle birebir (zip49b'den itibaren; ilk zip49'da tablo sürümü uyuşmuyordu).
- **Gönderme tablosu yeniden üretildi:** `SHT_SURUM 'v5-15 · direk 0.3'`, tablo `direk` alanını da denetliyor. Açı dilimine göre ortalama (dilim 0,2 rad):
  - 0–69°: değişmedi (±0,01).
  - 80°+: 0,275 → 0,151.
  - Eski tabloda 80°+ dilimi 69°'den yüksekti (0,275 > 0,224); kenarı sıyıran sayıların izi. Artık açıyla düzenli azalıyor.
- **Kısa sınama:**
  - Cansu yolu (ağız kenarına 0,15) → direk, sayı yok. Ağzın 2,8 içi → sayı.
  - Dik, kenardan 0,15 → direk; kenardan 0,4 → sayı.
  - Çizgiyi sıyırarak gelen → direk.
  - Maç (120×60, tohum 3, 1500 tik) 11,4 ms/tik, direksizle aynı.
- **Canlı:** direkler ağzın uçlarında çiziliyor.
- **zip49b (Claude Code ölçümünden sonra):**
  - **Direk 0 birebir:** `SHT_SURUM` direk 0'da zip48'inki. `shot-table.js` iki tablo taşıyor: `ALAN_SHT` (direk 0,3) ve `ALAN_SHT_V14` (zip48, değiştirilmeden). Sınama: zip48 ile direk 0, tohum 3, 700 tik, her 25 tikte bütün konumlar eşit.
  - **Maç olayı:** `ev(m, 'direk', takım, '… · Direkten sekti')`, sayaç `m.st.direk[takım]` (gönderen takım). Canlı akışında da görünür.
- **Uyarı (Claude Code ölçümü için):** 0,3 yarıçap tabloda yalnız 80°+ dilimini belirgin değiştiriyor. "Uzak ve geniş (25+, 60–75°) %31" hücresi yalnız direkle düzelmeyebilir; o zaman Bekçi'nin yakın direkte uzanışına bakılmalı.

## 14 Ekim 2026 · zip47 · Saha 120 × 60
- **Karar (kullanıcı):** varsayılan saha 120 × 60 (`core.js` `ALAN_SAHA_BOY` varsayılanı).
  - Claude Code'un zip46 ölçümü (herkes 20, 16 maç): sayı 4,3 → 3,2, gönderme −%27, el değiştirme −%20, uzun tutuşta duran hücumcu 1,30 → 0,51, akıl aynı isabette, hesap 173 → 157 sn.
  - Gol düşüşü bilinçli kabul edildi. Maç süresi ve pas hızı ayrı düşünülecek.
- **Eski saha:** `ALAN_SAHA_BOY = [100, 50]` ile zip45–46 ile birebir aynı oynar.
- **Ekranlar:**
  - **Canlı:** Saha ölçüsünü motordan okuyor (çizim, alan ısı haritası, Kuyu'lar, orta çizgi, tünelden giriş, kenar sesi).
  - **Taktik:** Tahta 100 × 50 şematik kalıyor (oranlı ayarlar aynı görünür). Mutlak serbestlik halkası `÷ KX`.
  - **Lab:** Değişiklik gerekmedi.
- **Analiz işçileri:** saha ölçüsünü `self.ALAN_SAHA`'dan okuyor, `?saha=WxH` parametresini alıyor.
- **Maç kaydı:** `saha: [W, H]`. Eski kayıt 100 × 50 sayılır.
- **Tablolar:** Gönderme tablosu sahadan bağımsız (Kuyu'ya göre). Değer tablosu uzak noktaları son dilimden okuyor. `SHT_SURUM` değişmedi.
- **Kısa sınama** (120 × 60, tohum 3, 900 tik): 11,4 ms/tik, pas ve gönderme işliyor.
- **Ek (kontrol sonrası):**
  - Taktik'teki "hat aralığı", "genişlik", "ort. destek" yazıları gerçek saha biriminde (× KX / KY). 120×60'ta ~50 / ~38 br.
  - Canlı'daki Taktik iframe'i adresini ref ile alıyor; `{{ taktikSrc }}` 404'ü kalktı.
- **Açık:**
  - Alan gücünün değeri mekanikten ölçülmeli: rakip pasının alanlarda yavaşlaması ve sönmesi. Motorun söndüğü anı olay olarak yazması istendi; bir sonraki iş.
  - Geçişin seyrekliği kuraldan (sayıca üstünlük) geliyor, sahadan değil.

## 14 Ekim 2026 · zip46 · Saha sabitleri üç sınıfa ayrıldı
- **Tek kaynak:** `self.ALAN_SAHA` (`core.js`). Deney için `self.ALAN_SAHA_BOY = [W, H]`; işçide `mac-isci.js?saha=140x70`.
- **Sınıflar:** Geometri sahaya oranlı (`× GKX`, `× GKY`). Koşu süresine bağlı olanlar sabit hızla tanımlı. Beden ve fizik mutlak. Bütün liste: `SAHA-SABITLERI.md`.
- **Ne değişti:**
  - decide, match, shape ve core'da Kuyu yeri, orta çizgi, duvarlar, bölgeler, blok, genişlik, pres hattı, duruş eşikleri, şekil kuralı, santra, aday çizgileri ve alan ızgaraları artık sahadan hesaplanıyor.
  - Duruş tahtası 100 × 50 ölçeğinde kaydedilir, motor sahaya oranlar.
- **Birebir aynılık (100 × 50):** zip45 ile tohum 4 ve 9'da her tikte bütün konumlar, skor ve olaylar eşit.
  - Yöntem: Bütün çarpımlar 1 ile yapılır, işlem sırası değişmedi.
  - Kesirli sabitler (99,2, 49,2, 99,999) 100 × 50'de sayının kendisiyle yazılıyor.
- **140 × 70 kısa deneme:** oyun işliyor (santra, pas, gönderme, sayı), 10,4 ms/tik.
  - İlk denemede bulunan hata: `shape.js` hedef sınırı 98,5'te kalmıştı, herkes x 98'e yığılıyordu. Düzeltildi.
- **Henüz değil:**
  - Ekranlar 100 × 50 çiziyor.
  - Analiz işçileri `gx = 100` varsayıyor.
- **Claude Code:**
  - Birebir aynılık sınaması: 16 maç, her tik.
  - Üç saha boyu.
  - Her boyda Alan gücü 6 ve 14.

## 14 Ekim 2026 · zip45 · Elle verilen duruş ve serbestlik
- **Asıl sebep bulundu (bağ değildi):**
  - Son bölgede çizgiye konan kanatlar, Çekirdek bizdeyken zamanlarının yaklaşık yarısında **güvence**ydi; arkada bekleyen bir rakibi tutuyorlardı (Liman–Demir Blok, sd 1).
  - Güvence, arkada bekleyen her rakibe bir kişi gönderiyordu. Bu kişiler savunma slotuna ve anlık yere göre seçiliyordu. Demir Blok 4 kişiyi önde bırakınca 6 saha oyuncusundan 4'ü geri çekiliyordu; kanatlar da bunun içindeydi.
  - Duruş ayarı, aday tartımı ve bağ bu oyuncuya hiç ulaşmıyordu.
- **Güvence kuralı:**
  - Güvence yalnız hücum dizilişinin arka ve orta çizgisinden seçilir (`rslot.d < 0,6`). Öndeki üçlü rakip tutmak için geri çekilmez.
  - Seçim oyuncunun **evine** göre yapılır, anlık yerine göre değil.
  - Elle yeri verilen oyuncu, güvenceye ancak başka kimse yoksa çekilir.
- **El payı serbestliği ezmiyor:**
  - `elK = 1 + durusEl × el`, `durusEl` 0,5. Bağa yalnız küçük bir ek yapar.
  - zip44'teki "en az serbestlik 0" kuralı kalktı. Elle verilen yer evi belirler, ne kadar uzaklaşacağını serbestlik belirler.
- **Aday kümeleri serbestliğe uyuyor (elle konan oyuncu):**
  - Serbestlik 0,5'in altındaysa Kuyu önü halkası ve arkaya koşu adayları üretilmez. Oraya ancak bir pas çağırırsa gider (alıcı kovalaması ayrı kural).
  - Açılan boşluğun bağı gevşetmesi serbestlikle ölçekli: 0'da hiç, 0,5 ve üstünde tam.
- **Kısa sınama** (sd 1, 1100 tik). Ölçü: son bölgede kanatların ortadan uzaklığı. Ev 22, varsayılan 13,5.

  | | Seçilen hedef | Gerçek konum |
  |---|---|---|
  | zip44, serbestlik 0 | 13,0 (güvence %44) | 10,5 |
  | zip45, serbestlik 0 | 19,3 (güvence %0) | 13,0 |
  | zip45, serbestlik 1 | 6,4 (içeri giriyor) | 9,4 |

  - Gerçek konum hedefin gerisinde. Son bölge evresi kısa ve oyuncu yolda: ölçüt "kendi bölgesinde" süreye göre tartılmalı.
- **Ekran:** Taktik tahtasındaki kesik serbestlik halkası zaten duruştaki (elle konan) yerin çevresinde çiziliyor. Yarıçapı serbestlikten geliyor (`3 + serbestlik × 12`).
- **Claude Code ölçmeli:**
  - Kanatların serbestliği 0, 0,5 ve 1'de evden uzaklık; serbestlik 0'da yarı yarıya ölçütü.
  - Güvence değişikliğinin genel etkisi: kontra yenen gol, top kaybından sonra gol. Önde 3 kişi artık geri çekilmiyor.

## 14 Ekim 2026 · zip44
- **Karşı pres hatası (Claude Code buldu):**
  - `kpAkt` bitiş koşulunda çağıranın takımına bakıyordu; topu tutan takım için çağrılınca pres siliniyordu.
  - Artık basan takıma bakıyor (`bt === k.team`).
  - Kısa sınama ("her yerde, 3 sn", 1500 tik): 3 karşı pres, toplam 425 tik, ortalama ~140 tik. Önceden 2–14 tikti.
- **Duruş, ikinci kademe:**
  - Aday ızgarası zaten duruşla kaydırılmış evden üretiliyor (kontrol edildi). Eksik kalan, bağın gücüydü.
  - Ellenen oyuncu en az serbestlik 0 kadar evine bağlı. Üstüne el payı geliyor: `durusEl` 3 → 6.
  - Ölçüt aynı: ellenen oyuncu kendi bölgesinde evine en az yarı yarıya uymalı.

## 14 Ekim 2026 · zip43 · Claude Code'un özellik ve taktik ölçümüne cevap
- **Hata: göndermede maç Pas'ı kullanıyordu.** `match.js` `applyError`'a gönderme olduğunu söylemiyordu; Gönderme yalnız akılda görünüyordu ("Gönderme belirsiz" bulgusu buradan). Düzeltildi.
- **Uzanma'nın ters etkisi · aday sebep bulundu:** Göndermede Çekirdeğin alıcısı yok. Bu yüzden şut atanın arkadaşları da kendi göndermelerine dokunabiliyordu. Uzanma büyüdükçe öndeki arkadaşın şutu kesmesi artıyordu; akıl bunu görüp daha az gönderiyordu.
  - Yeni kural: Arkadaş, kendi takımının göndermesini başkasına giden pası bıraktığı gibi bırakır (sekme yoksa).
  - Kural hem maçta hem kafadaki maçta (`b.gon`) geçerli.
- **Güç bağlandı:**
  - Gövdeyle koruma: `shieldK × (1 − (Güç taşıyıcı − Güç presçi) × .04)`, 0,5 ile 1,5 arasında.
  - Çarpışma (`govdeAyir`): Güçlü olan daha az itilir. Eşit güçte eskisi gibi yarı yarıya.
- **Karşı pres geçişten ayrıldı (`m.kp`):**
  - Menajerin kuralı, kaybeden takım dağınık olmasa da geçerli.
  - Basanlar, kayıp anında topa en yakın 3 saha oyuncusu (16 birim içinde).
  - Top geri kazanılınca ya da süre dolunca biter.
- **Geçiş eşiği:** `tac.gecisEsik` ile ölçülebilir; varsayılan hâlâ 4.
  - Not: Eşiği 3'e indirmek geçişi daha seyrek yapar, çünkü kural "topun gerisinde eşikten az oyuncu" ise başlıyor. Daha sık geçiş için eşik 5 denenmeli.
- **Duruş sahada görünsün:**
  - `slotBase`, evin menajerin elinden çıkma payını taşıyor (`el`, 0–1).
  - Aday tartımında evden uzaklığın bedeli `1 + S.durusEl × el` katına çıkıyor (`durusEl` 3).
  - Ölçüt (Claude Code): Ellenen oyuncunun kendi bölgesindeki gerçek yeri evine en az yarı yarıya uymalı.
- **Sürme'nin ters etkisi · açık:** Kod okumasıyla sebep bulunamadı. Adaylar:
  - `keepP` Sürme'yle hızlı büyüyor ve akıl sürmeyi göndermeye ve pasa tercih ediyor.
  - Sürmenin değeri (`value-table`), şuta dönüşmeyen ilerlemeyi ödüllendiriyor.
  - Kopyalı ölçüm gerek: aynı an, Sürme 10 ve 14, seçeneklerin değeri.
- **Refleks üst tarafta etkisiz · açık:** Formül kontrol edilmeli (`bekR` tavanı).
- **Hız:** kısa sınamada 9,1 ms/tik.

## 14 Ekim 2026 · Oyuncu özellikleri: 9 → 15 + Bekçi + Taraf (zip42)
- **Tek erişim:** Bütün özellikler `self.ALAN_OZ(p, k)` ile okunuyor (`core.js`). Eski kayıtlar eşlemeyle okunur. Her özelliğin hangi mekaniği beslediği `OZELLIKLER.md`'de.
- **Okuma dörde ayrıldı:** Tepki, Görüş, Yer alma ve yeni Soğukkanlılık.
  - Eskiden Okuma'yı okuyan her yer (core 6, decide 23, match 9, shape 30 yer) tek tek içeriğine göre birine bağlandı.
  - `ALAN_OKC` kanalları: 'tepki' → Tepki, 'bekci' → Refleks, 'kontrol' → Görüş. Deney mekanizması (`ALAN_OKX`) aynen sürüyor.
- **Diğer ayrılanlar:**
  - Aktarım → Pas ve Gönderme. `applyError` hangisinin atıldığını biliyor: maçta `o.kind`, hayalde `D._gon`.
  - Tutuş → Kontrol ve Yükleme.
  - Yoğunluk → Güç (omuz ikilisi) ve Uzanma (menzil).
- **Yeniler:**
  - **Alan gücü:** `p.R` ve `p.D`, her tik.
  - **Yükleme'nin üç parçası:** dolum hızı, koşarken dolum (`chgMin` payı), pasla gelen topta kırılım (`recvCh` alıcıyı alıyor).
  - **Soğukkanlılık:** baskının dağıtma payı.
  - **Bekçi:** Refleks, Yerleşim, Alan hakimiyeti.
  - **Taraf:** tercih edilen yanda dönüş ve fiske ×1,12, öbür yanda ×0,9.
- **Kalkan:** Cesaret.
- **Davranış değişen yerler (bilerek):**
  - **Pas arası:** karşılama becerisi artık yalnız Kesme. Eskiden (Kesme + Tutuş) / 2'ydi.
  - **Presçinin fake'i:** aldatması Çeviklik'ten, fark edilmesi taşıyıcının Tepki'sinden. Eskiden iki taraf da Okuma'ydı.
  - **Risk:** Cesaret çarpanı kalktı. HEPSI_20'de bu çarpan 0,7'ydi; risk algısı değişir.
  - **Yükleme 20 (HEPSI_20):** koşarken dolum ve pasla kırılım artık etkili; şarj davranışı değişir.
  - **Taraf:** kadroda sol, sağ ve iki yanlı oyuncular var.
- **Bugünkü oyun:** HEPSI_20'de Alan gücü ve Soğukkanlılık 10'da bırakıldı (alanlar ve baskı payı bugünkü gibi).
- **Ekran (Taktik):** Oyuncu kartında özellikler dört grupta (Fiziksel, Topla, Zihinsel, Savunma); Bekçi'de kendi grubu. Taraf bir rozet olarak.
- **Hız:** 11,30 → 11,39 ms/tik (değişmedi).
- **Claude Code ölçmeli:** planın 5. bölümü.
  - Her özellik 6, 10 ve 14'te, takımın bütün saha oyuncularında: etkisi ve tek yönlülüğü.
  - Sahada görünürlük.
  - Taktiğe göre değer.
  - Ağırlık dengesi.
  - Hız.
  - Ölçüm aracı `HEPSI_20` yerine kendi değerlerini vermeli (`p.a` üzerine yazarak).

## 14 Ekim 2026 · Taktik ekranı taslağın 4. ve 5. panolarına göre (zip41, yalnız ekran)
- **Üst şerit:** "TAKTİK · LİMAN / Oyun planı", tek mod anahtarı ("Topa sahipken / Topa sahip değilken"), sağda "Oyuncular tablosu →".
- **Tahta:** Köşede "DURUŞ · TOP ORTADA" etiketi var; kesikli kutu kalktı. Serbestlik halkası yalnız seçili oyuncuda görünüyor.
- **Geçiş:** Tek satır: "Topu kazanınca: önde [sayıca üstünsek] git. Kaybedince: [rakip yarısında] [2 sn] karşı pres." Seçeneğe dokununca bir sonrakine geçiyor.
- **Oyuncu kartı:** Taslaktaki gibi satır listesi; satıra dokununca sıradaki kademe. Kart yalnız seçili modun talimatlarını gösteriyor.
  - Hücum: serbestlik, koşu, topu nerede ister, destek, pas riski, boşluk doldurur.
  - Savunma: alan/adam, kimi tutar (adam tutuyorsa), kişisel pres.
  - Pas riski ve kişisel preste orta kademe "takımın ayarı" yazıyor.
- **Oyuncular tablosu:** Tam sayfa, "← Tahtaya dön" ile dönülüyor. Sütunlar: Kendi kurduğun rol, Serbestlik, Koşu, Topu ister, Destek, Savunma, Kişisel pres.
  - Rol cümlesi dizilişteki yerden ve talimatlardan kendiliğinden yazılıyor ("Arkadan kuran savunmacı", "Kanattan genişlik veren", "Topu tutan pivot"…).
  - Tablonun altında üç not var. Satıra dokununca oyuncu tahtada açılıyor.
- Motor değişmedi.

## 14 Ekim 2026 · Taktik planı · Adım 2–4: geçiş durumdan, evreler kalktı, yeni Taktik ekranı (zip40)
Adım 1 (duruşlar) zip39'daydı. Bu zip üç adımı birlikte içeriyor. Kullanıcı "hepsini bitir" dedi; ölçüm ayrımı için adımlar aşağıda ayrı yazıldı.

**Adım 2 · Geçiş** (`match.js` gecisBasla, gecisGuncelle, kpAkt, inWin; `shape.js` güvence ve koşucular, karşı pres)
- **Başlangıç ve bitiş:** Geçiş top el değiştirince, kaybeden takım dağınıksa başlar (topun gerisinde Bekçi dışında 4'ten az oyuncu). Her tik bakılır; kaybeden takım yerleşince (4+) ya da top yeniden el değiştirince biter. Süresi sabit değil. `WIN_T` ve sabit 3 sn pencere kalktı.
- **Hücum geçişi** (`tac.gecisH`): 'ustun' | 'esit' | 'her' | 'hic'.
  - Sayı sahadan gelir: topun önündeki bizden (Bekçi ve taşıyıcı hariç) ile topun önündeki rakipten (Bekçi hariç) sayılır.
  - Kural "git" derse katılanlar topun önündekiler ve taşıyıcıdır. Top kendi bölgemizde (ilk 35) kazanıldıysa en yakın bir destek de katılır. Kalanlar evlerinde güvencedir (serbestlik bağı 2,5).
  - Katılanlar her tik yeniden belirlenir. `kontraN` kalktı.
  - Git sırasında risk +0,25 ve tempo en az 0,8 olur (eski Kontra kuralı). "Yerleş" artık ayrı bir ayar seti değil: geçiş oyunu oynanmaz, takım yerleşik planda kalır.
  - Eski kayıtlar: kazaninca Kontra → 'esit', Yerleş → 'hic', Dengeli → 'ustun'.
- **Karşı pres** (`tac.kpYer`: 'rakip' | 'her' | 'hic'; `tac.kpSure` 1–3 sn, varsayılan 2):
  - Basanlar kayıp anında topun 8 birim içindekilerdir (en çok 3); kimse yoksa basılmaz. Diğerleri savunma evlerine döner.
  - Süre dolunca ya da geçiş bitince karşı pres de biter. Basan seçimi pres listesini o süre boyunca geçersiz kılar (`shape.js`, kpAkt).
  - Eski `kayip` değeri: < 0,34 → 'hic', > 0,66 → 'her', arası → 'rakip'.
- İlk kontrol (tek maç; yön, kanıt değil): 'hic' kuralında geçişe 0 tik "git" düştü, 'her' kuralında 175 tik. Katılan sayısı geçiş içinde 0 ile 4 arasında değişti. Karşı presin bir örneğinde 2 kişi bastı.

**Adım 4 · Evreler ve arkada kalan** (`match.js` evreUygula, tl; `shape.js` nArk; TAC0)
- Evre başına ayar (`tac.evre`), evre başına diziliş ve evre başına talimat (`p.tl['huc.kurma']`) kalktı. Eski kayıtlarda bunlar yok sayılır.
  - Evre artık yalnız bilgi olarak hesaplanıyor (Canlı'daki "ŞU AN", maç sonu raporu). `EVKOD` ve `evreBul` bu yüzden duruyor.
  - Koşullar (skora göre) ve maç planı eskisi gibi çalışıyor.
- `arkada` kalktı: TAC0'dan ve rakiplerin taktiğinden çıkarıldı. Arkada kalan sayısını artık savunma dizilişinin en geri hattı veriyor (d ≤ 0,25).
  - **Dikkat:** Demir Blok'un 3-2-1 dizilişinde bu 3 kişi demek (eskiden 2'ydi). Rakibin davranışı değişti.

**Adım 3 · Taktik ekranı** (`ALAN v5 Taktik.dc.html`)
- **Mod tek:** "Topa sahipken / Topa sahip değilken". Tahta, duruşlar ve talimatlar hep aynı moddadır.
- **Tahta:** Seçili duruşu gösterir; sarı nokta Çekirdektir. Varsayılan duruşlar motorun şekil kuralının aynısıyla hesaplanır (`durSav`, `durHuc`).
  - Oyuncuyu sürüklemek yalnız o duruşu değiştirir; elle konan oyuncunun kenarı mavidir. "Bu duruşu dizilişe döndür" düğmesi var.
  - Hazır diziliş ve kademeler üç duruşu birden üretir; hazır diziliş, o tarafın elle konan duruşlarını sıfırlar.
- **Tahtanın altında:** Üç duruşun küçük resmi, ardından iki cümlelik **Geçiş kartı**.
- **Oyuncuya dokununca:** Sağda yalnız o modun talimatları açılır.
  - Hücum: koşu, topu nerede ister, destek, serbestlik, boşluk doldurur, pas riski.
  - Savunma: alan/adam, kimi tutar, kişisel pres.
  - Tahtada serbestlik kesikli bir halka, koşu sarı bir oktur.
- **Gizli ayar kalmadı:** Ekranda olmayan talimatlar (pas ritmi, kişisel tempo, gönderme, oyuncu başına pas↔sürme) motora varsayılan değerle gidiyor.
- **Oyuncular sekmesi:** Genel bakış tablosu (7 sütun; hücreye dokununca sıradaki kademe) ve her oyuncu için bir cümle.
- **Tekrarlar tek yere indi:**
  - Kenara sıkıştırma yalnız "Savunma tarzı"nda; seçilince hem sistem hem pres tuzağı açılır.
  - Takımın "Adam adama" düğmesi motorda hiçbir şeye bağlı değildi; kalktı. Yerine "Herkes alan / adam tutsun" kısayolu geldi, oyuncu talimatını herkese birden yazıyor.
- **Kalkanlar:** Evre şeridi, evre küçük resimleri, kontraya katılan sayısı, kazanınca ve kaybedince kaydırıcıları.
- Eski kayıt yeni biçime çevrilir: `gMig` geçiş kuralını üretir, E (evreler) atılır.

**Hız:** 2 maç × 600 tik: zip38 10,74 ms/tik → 11,30 (+%5).

**Claude Code ölçmeli** (planın 5. bölümü):
- Kabul 1: varsayılan plan. Değişen şeyler: geçiş kuralı, karşı presin varsayılanı ('rakip', 2 sn) ve arkada kalanın dizilişten gelmesi.
- Kabul 3–4: geçiş ve karşı pres.
- Kabul 5: hız.
- Kabul 6: tarayıcı testi.

## 14 Ekim 2026 · Taktik planı · Adım 1: ev = duruş karışımı (zip39)
Plan: "sekiz evre yerine iki mod". Bu zip yalnız 1. adımı içeriyor: geçiş, ekran ve evrelerin kaldırılması sonraki zip'lerde.
- **Ne:** Her takımın iki planı var: `tac.durus.huc` (topa sahipken) ve `tac.durus.sav` (topa sahip değilken). Her plan üç duruştan oluşur: top kendi bölgemizde (ileri 20), ortada (50), son bölgede (80). Bir duruş, oyuncu sırasına (i 1–6) göre 6 elemanlı bir dizidir: `{f, y}`.
  - f: takımın yönünde ileri (0–100).
  - y: yan (0–50). Dizilişle aynı biçimde tutulur, takıma göre aynalanmaz. Top yanda ortadayken (y 25) ölçülür.
  - `null` yazılan duruş, oyuncu ya da alan varsayılandan gelir. Yalnız `f` ya da yalnız `y` verilebilir.
- **Nasıl** (`shape.js slotBase`): Varsayılan duruş, bugünkü şekil kuralının (`slotBase0`, eski `slotBase`) top 20, 50 ve 80'deyken verdiği yerdir. Canlı hesaplanır, yani hücumda ön çizginin son savunmacıya göre kurulması gibi kurallar sürer.
  - Elle konan duruş, varsayılandan farkı (ofset) olarak uygulanır. Ofset komşu duruşlar arasında doğrusal karışır: ellenen duruşun kendi bölgesinde tam etkir, komşu duruşa doğru sıfıra iner.
  - Top 20'nin gerisindeyse ilk duruş, 80'in ilerisindeyse son duruş geçerlidir.
  - Yan kayma (top yana gidince blok onunla kayar) varsayılan kuraldan gelir, ofset buna dokunmaz.
  - Ev; aday tartımı, serbestlik, koşu penceresi, pas açısını kapatma ve markajın çıkış noktasıdır. Bunların hepsi olduğu gibi kaldı.
  - Bekçi kuralı değişmedi.
- **Neden ofset:** Duruş ellenmezse ev eski kuralla birebir aynıdır. Doğrudan duruşları karıştırmak, eski kuralın kıskaçlarını (blok sınırı, son savunmacı) düzleştirip ayar yapmayan takımı da değiştirirdi.
- **Blok ve genişlik:** Varsayılan duruşları üreten kısayol olarak çalışmaya devam ediyor; ayrı ayar olarak kaldırılmaları 4. adımda.
- **Akıl:** Ayrı taklit yok, kafadaki maç da `AlanShape.position` → `slotBase` kullanıyor (AKIL-FARKINDALIK'a satır eklendi).
- **Dışa açık** (Taktik ekranı için, 3. adım): `AlanShape.DUR_A`, `durusDef(m, p, k, att)` (varsayılan duruş), `durusW(f)` (karışım ağırlıkları), `slotBase`.
- **İlk kontrol** (tek maç; yön, kanıt değil):
  - Duruş yokken maç zip38 ile birebir aynı: tohum 4242, 450 tikte bütün konumlar ve skor eşit.
  - Hücum son duruşunda iki kanadı çizgiye çektim (y 3 / 47). Bu iki oyuncunun evinin ortadan ortalama yan açıklığı:
    - son bölgede 15,2 → 20,9
    - ortada 15,2 → 15,5
  - Oyuncuların gerçek yan açıklığı (aday tartımı evin üstüne çalışıyor):
    - son bölgede 16,5 → 17,1
    - ortada 15,5 → 14,9
  - Bir tikte evin en büyük sıçraması 0,33 birim, yani ani sıçrama yok.
- **Claude Code ölçmeli:**
  - Kabul 1: duruş yokken zip38 ile aynı.
  - Kabul 2: ellenen duruş kendi bölgesinde takım şeklini değiştiriyor mu, diğer bölgelerde az mı; ev hızında sıçrama var mı.
  - Hız.

## 14 Ekim 2026 · Herkesin durması (donma)
Sebep: topsuz hücumcu bir noktayı "takımın en iyi seçeneği" ile tartıyordu ve bu en iyi seçeneğe taşıyıcının kendi seçeneği (tutmak, sürmek) da giriyordu. Taşıyıcının seçeneği öndeyken her nokta aynı değeri alıyordu. Üstüne sabit bir "yerinde kal" primi (0,04) vardı. Bu yüzden herkes yerine çivileniyordu. Savunma da hiçbir şey değişmediği için oturuyordu.
- **Topsuz oyuncu kendi seçeneğini tartar** (`shape.js teamV`): değer, taşıyıcının ona atacağı pasın değeridir (laneOpen × V; taşıyıcının kullandığı hesap). Bir arkadaşın hedefi dibindeyse (alan paylaşımı mesafesi `ayr`), seçeneği onunla aynı sayılır.
- **Koşunun penceresi** (`teamV`): hücumcu c'ye `ta` tikte varır. Savunmacı o sürede ancak `ta − tepki − dönüş kaybı` kadar koşabilir; eskiden bu ulaşma mesafesi sabit 4'tü.
  - `tepki` = gecikmeli algı + okuma tepkisi; pası okumakla aynı kural.
  - `gec` = dönüp hızlanma kaybı. Formül değil: maçtaki `ivme` kuralı 80 tik oynatılarak ölçülür, sonuç çeviklik, hız oranı ve açı kovasına göre saklanır. Duran oyuncu için 4,6 tik, tam hızda ters dönen için 12,3 tik.
  - Yerinde duranın önünde pencere yoktur (savunma yerleşmiştir); hareket edenin önünde açılır.
- **Koşu bir karardır**: hücumcu hedefine varana ya da top el değiştirene kadar hedefini yeniden tartmaz. Varınca savunma yetişmiş olur, durduğu yer değer kaybeder ve sıradaki koşuyu seçer. Sabit "yerinde kal" primi kaldırıldı.
- **Pas açısını kapatma** (`kapatHat`): adamı olan savunmacı (bölge ya da markaj) adayları tartar: Kuyu tarafı (eski yer), pas hattının üstü ve ikisinin arası. Ölçü, kendi adamına giden pasın ve adamının 7 birim arkaya koşusunun değeridir (laneOpen × V). Okuma düşükse Kuyu tarafına yakın kalır. 4 tikte bir yeniden tartılır, arada adamına göre aynı yerde kalır.
- **Zaman bedeli** (`match.js macEff`): kalan süre 60 tiklik kovaya yuvarlanmıyor, iki kova arasında doğrusal hesaplanıyor. Bekleme bedeli artık süreyle artıyor; eskiden 0,5 sn ile 1 sn beklemek aynı bedeli tutuyordu.
- Ölçüm aracı: `alan-v5/olcum-donma.js`. Tanım Claude Code'unkiyle aynı: taşıyıcı 2 sn'den uzun tuttuğunda duran hücumcu ve savunmacı (hız < 0,03), açık pas yolu (laneOk > 0,5).
- İlk sonuç (her hücrede **tek maç: yön, kanıt değil**):
  - Liman–Rakip 2, tohum 77, ~2200 tik:
    - topu 2 sn'den uzun tutma oranı %37 → %5
    - duran hücumcu 4,40 → 0,05 /5
    - duran savunmacı 3,78 → 0,05 /6
  - Liman–Rakip 1, tohum 4242:
    - uzun tutma %25 → %14
    - duran hücumcu 2,41 → 0,80
    - 1 sn'den uzun duran 1,62 → 0,09
    - duran savunmacı 1,31 → 0,49
    - açık pas yolu 1,19 → 1,14
  - Hız: 9,7–11,2 → 10,9–12,6 ms/tik.
- Claude Code ölçmeli:
  - 16 maç önce/sonra donma
  - açık pas yolu
  - koşucuya giden pas
  - pas kalibrasyonu (hareket arttı, foto finişler değişir)
  - salınım (iki taraf sürekli yer değiştirip hiçbir yere varmıyor mu)
  - hız

## 13 Ekim 2026 · Maç kaydı: son 5 maç
- Canlı ekranı artık son 5 maçın kaydını `alan-v5-maclar` anahtarında (en yenisi sonda) tutuyor. `alan-v5-son-mac` yine en yeni maçı gösteriyor. Daha önce site açılınca başlayan yeni maç, bir önceki kaydın üstüne yazıyordu.

Bu dosya v5'e eklenen mekaniklerin listesidir. Motor bozulursa ya da bir dosya eski hâline dönerse, buradaki her maddenin hâlâ çalıştığı kontrol edilir. Her madde: **ne**, **nerede** (dosya ve aranacak kod), **nasıl test edilir**.

Motor dosyaları: `core.js` (fizik), `decide.js` (karar ve akıl), `match.js` (maç adımı), `shape.js` (yerleşim), `shot-table.js` (gönderme tablosu). Kopya: `alan-v5-iyi/` (ana klasörle birebir aynı tutulur).

---

## Sabitler
| | Değer | Yer |
|---|---|---|
| Kuyu ağzı | 10 birim (`MOUTH = 5`) | `core.js` |
| Bekçi alanı yarıçapı | 9 (`KR = 9`) | `core.js` |
| Gönderme tablosu sürümü | `v5-14 · Kuyu 10, alan 9` | `decide.js` `SHT_SURUM`, `shot-table.js` `surum` |

**Kural:** Kuyu ölçüsü, Bekçi kuralı, gönderme isabeti ya da Çekirdek fiziği değişirse `SHT_SURUM` değiştirilir ve tablo `shotCell` ile yeniden üretilir. Sürüm uyuşmazsa motor eski tabloyu kullanmaz.

---

## Bekçi
1. **Bekçi ikiliye girer.** İkili döngüsünde Bekçi istisnası kaldırıldı.
   *Yer:* `match.js` ikili döngüsü: `if (q.team === h.team || q.noTouch || q.cd > m.tick) continue;` (içinde `q.role === 'Bekçi'` olmamalı).
2. **Bekçi alanın dibinde taşıyıcının üstüne çıkar.** Taşıyıcı Kuyu'ya `KR + 6` birimden yakınsa Bekçi onun 1,2–2 birim önüne çıkar.
   *Yer:* `shape.js` `bekci()` içinde `L < KRx + 6 ? L - 1.2 - (20 - ok(p)) * .05`.
3. **Bekçi açıortayda durur.** Topla iki direk arasındaki açının ortası. `AlanShape.bekci` dışa açık.
   *Yer:* `shape.js` `bekci`, `AlanShape = { ..., bekci }`.
4. **Kuyu alanının derininde Bekçi'nin Çekirdeği güvende.**
   *Yer:* `decide.js` `keepP` başı: `if (h.role === 'Bekçi') { ... if (dc < C().KR - C().Q.reach - .4) return 1; }`.
5. **Alanın kenarında dokunma menzili.** Rakibin gövdesi alana giremez ama menzili girer. Çekirdek menzildeyse gövde koruması işe yaramaz; en yakın rakip "Çekirdeğe uzan" görevine geçer.
   *Yer:* `decide.js` `duelP` içinde `uzan = h.role === 'Bekçi' && ...`; `shape.js` `'pres · Çekirdeğe uzan'`.
   *Test:* Herkes 20; Bekçi'nin 4 saniyeden uzun sahipliği olmamalı (kilitlenme yok).
6. **Sürmede Bekçi riski.** `keepP` Bekçi'yi de rakip sayar.
   *Yer:* `decide.js` `keepP` döngüsü: `if (q.team === h.team || q.noTouch) continue;`.

## Gönderme
7. **İsabet duruma bağlı (`gonEk`).** Baskı, koşu ve güç dağıtır; Aktarım küçültür. Maç ve akıl aynı fonksiyonu kullanır.
   *Yer:* `decide.js` `GON = { taban, baski, hareket, guc, plase }`, `gonEk`; `match.js` `launch` içinde `D.gonEk(h, m.ps, o.shot)`.
8. **Plase.** Bekçi'nin uzak direğine (ağzın %90'ı), dağılma güçlüden yarı yarıya az (`GON.plase: .5`), hız `plaseV 1.36`.
   *Yer:* `decide.js` aday üretimi `/* v5 · plase Bekçi'nin uzak direğine */`, `aimPlase: .9`.
9. **Fırlatma (savurma).** Çekirdek gövdenin etrafında yarım ya da tam tur döner, sonra güçlü çıkar.
   *Yer:* `decide.js` `fırlat`, `slingT`; `match.js` `slingBasla`.
10. **Savurmanın riski.** Savurma bitmeden yaya yetişebilen rakip Çekirdeğe çarpar; akıl bunu tartar.
    *Yer:* `decide.js` `if (o.tur) { const Ts = slingT(...), sw = ...` (`savurma yayı` yorumu).
11. **Gönderme türleri ön elemede birbirini silmez** (`nS = inL ? 2 : 4`).
    *Yer:* `decide.js` ön eleme.

## Sürme ve top kontrolü
12. **Bekçi'yle karşılaşma (sürme değeri).** Sürme Kuyu yakınında bitecekse Bekçi'nin varış anındaki yeri maçın kuralıyla hesaplanır. İkili mesafesindeyse değer, göndermenin değil, Bekçi'yle ikilinin değeridir.
    *Yer:* `decide.js` sürme dalı `/* v5 · Bekçi'yle karşılaşma */`; `SB(bk, o.to, { holder: hE, ps: ... })` (üçüncü argüman olmazsa çalışmaz).
    *Test:* Fırlatma sahnesinde "Bekçi'nin üstüne sür" ~0,4 olmalı, 0,99 değil.
13. **Topu döndürme, fiske, gövdeyle koruma, Çeviklik, topla hız, katı gövde, gövdeden sekme, kontrol hatası.** (Daha önceki turlar; `AKIL-FARKINDALIK.md`'de.)

## İkili mücadele
14. **İkili fiziksel bir hamle.** Hazırlık (Çevikliğe bağlı), sonra vuruş. Türü geometri seçer: uzanma (Çekirdek açıktaysa) ya da omuz (Çekirdek gövdenin öbür yanındaysa; Yoğunluk farkı girer). Vuruş anında menzilde değilse boşa gider.
    *Yer:* `match.js` `q.hamle = { k: 'omuz' | 'uzan', ... }`, `omuz verdi`, `uzanıp dürttü`.
15. **Kendini adama.** Hamleye giren savunmacı hazırlık ve toparlanma süresince yön değiştiremez (`adandi`, `adT`). Taşıyıcı hamleyi Okuma farkıyla sezerse hemen karar verir.
    *Yer:* `match.js` `q.adandi = ...`; `decide.js` `adK`, `qP`.

## Fake
16. **Fake (gönderme, pas, gövde).** Gösterilen niyet; aldanma Okuma farkı ve inanılırlığa bağlı.
    *Yer:* `decide.js` `fakeSecenek`, `fakeP`, `fakeHedef`; `match.js` `fakeBasla`.
17. **Fake'in bedeli.**
    - Fake süresince taşıyıcı durur (hızını kaybeder): `h2.vx = 0; h2.vy = 0;`
    - Aldanmayan rakipler 22 birime kadar tam hızla yaklaşır: `if (L < 22 && L > 1.7)`.
    - Aldanmayan Bekçi açıyı düzeltir: `else { const SB = window.AlanShape && window.AlanShape.bekci ...`.
    - İnanılırlığı %6'nın altındaki fake aday değil: `if (pf < .06) continue;`.
    - Fake zaman yer: değer `Math.pow(D.disc, T / 6)` ile indirgenir.
    *Yer:* `decide.js` `dunya` ve fake aday döngüsü.
    *Test:* Fırlatma sahnesinde taşıyıcı topla durduğu zaman %5 civarı olmalı.

## Bekleme
18. **Bekleme planları.** "0,5 sn bekle" ve "1 sn bekle" seçenekleri; plan, rakip dibine (3,5 birim) girince bozulur.
    *Yer:* `decide.js` `{ kind: 'tut', hold: 30 }, { kind: 'tut', hold: 60 }`; `match.js` `h.holdD`.
19. **Bekleyip sonra oynamak.** Baskı yokken beklemenin değeri, bekleme sonunda arkadaşlar yerleşmişken (predictWorld) en iyi hamledir.
    *Yer:* `decide.js` `/* bekleyip sonra oynamak (v5) */`, `D._noHold`.

## Takım arkadaşları
20. **Ver-kaç koşusu.** Dibinde presçi varken kısa pas (≤16) veren oyuncu, presçinin arkasındaki boşluğa koşar. Koşu ancak boşluk boşsa ve pasör oraya presçiyle yarışabiliyorsa başlar.
    *Yer:* `match.js` `function verKac(`, `verKacYonet`, `launch` içinde `verKac(m, h, o, tp)`, step içinde `verKacYonet(m)`.
    *Test:* İkiye Bir Sahne, "Kanatta ikiye iki": 12 denemede ~13 ver-kaç, birkaçı geri alınmalı.
    *Eksik:* Alıcı, koşucunun varış noktasına pası henüz hedefle tartmıyor; paslar sık kesiliyor.
21. **Perde.** (Önceki tur; `AKIL-FARKINDALIK.md`.)

## Maç notu
22. **Maç notu.** Her tik durumun değeri (akıldaki V, Mavi gözünden) izlenir; değişimi o anki faile yazılır (taşıyıcı ya da son dokunan). Çekirdek takım değiştirirse değişimin yarısı kaybedene eksi, yarısı kazanana artı. Sayıda gönderene +(1 − önceki değer), yiyen Bekçi'ye yarısı eksi. Not = 6,3 + (puan − 0,13) × 3, 3–10 arası.
    *Yer:* `match.js` `notTik`, `notSayi`, `notlar` (`AlanMatch.notlar(m)` oyuncu başına puan ve not döndürür), step sarmalayıcısı.
    *Test:* 4 maçta not medyanı ~6,3, en iyi oyuncu ~8,5–9,5.
    *Eksik:* Puanlar hafif artıya kayık (herkesin medyanı +0,13); ekranlara henüz bağlı değil.

## Ekranlar
- **Canlı maç** (`ALAN v5 Canlı.dc.html`): `alan-v5/mac-isci.js` ile oynar. Maç kaydı tarayıcıya yazılır (`alan-v5-son-mac`): tohum, kurulum, taktik değişiklikleri.
- **İkili Sahne**, **İkiye Bir Sahne**: aynı motor dosyaları; sadece başlangıç yerleri senaryodan gelir. Kamera Kuyu'ya yakınken hedef Kuyu'yu kadraja alır.

---

## Bilinen açık konular
- Ver-kaçta alıcının koşucuya "önüne" pası koşu hedefine göre tartılmıyor.
- İkiye Bir Sahne'nin ölçüm kodu "İkiye bir" ve "Ver-kaç" senaryolarında sonucu hemen "pasla geçti" sayıyor (ölçüm hatası olabilir; sayfada gözle kontrol).
- Plase maçta nadir seçiliyor (Bekçi genelde ortada).
- Sürme değeri Kuyu'dan uzakta hâlâ tablonun fotoğrafına dayanıyor; dinamik riskler (kim yetişir) sadece Bekçi için eklendi.
- Herkes 20 denemesi: `kadro.js` `HEPSI_20` — açık mı kontrol et.


## Claude Code birleştirmesi (aklın pas hesabı maçla aynı kurala getirildi)
Ayrıntı: `CLAUDE-CODE-SONUC.md`. Alınan: `decide.js`, `match.js`, `shape.js`. Üstüne yeniden eklenenler (Claude Code'un sürümünde yoktu): sürmede Bekçi'yle karşılaşma (madde: Bekçi), dört gönderme türü ön elemede (`nS = inL ? 2 : 4`), fake bedeli (`h2.vx = 0`), maç notu (madde 22). Önceki sürüm: `alan-v5-onceki/`.
*Bedel:* hesap ~3,4 kat yavaş (194 → 57 tik/sn, tek iş parçacığı).


## Oynanabilir oyun (ALAN Oyun)
- `ALAN Oyun.dc.html`: kabuk. Sekmeler Kadro → Taktik → Maç → Sonuç; gece/gündüz düğmesi (`localStorage 'alan-oyun-tema'`).
- Taktik ve Canlı sayfaları kabuğun içinde iframe olarak açılır. Kabuk içindeyken: Taktik "Maça çık" → `postMessage({ tip: 'alan-git', ekran: 'mac' })`; Canlı "Rakip seç" → `ekran: 'taktik'`; maç bitince Canlı → `{ tip: 'alan-mac-son', ad, skor, st, goller }` (Sonuç ekranı bunu gösterir). Tek başına açılınca eskisi gibi birbirine yönlenirler.
- Taktik: kaydırıcılar yerine 5 kademe (`kademe()`), 700 pikselden dar ekranda dikey sürüklenebilir tahta (`dikey()`, `pmove` koordinatı çevirir).
- Tema: iki sayfada da `tema()` CSS değişkenleri (--bg, --panel, --ink, --ink-rgb, --gold) kurar; saha (yayın) iki temada da koyu.
- Motor değişmedi.


## Hızlandırma (Claude Code, sonucu bit bit koruyan)
- `match.js` `govdeAyir`: uzak çiftler için mesafe hesaplanmaz; bir turda iç içe gövde yoksa kalan turlar atlanır.
- `shape.js` alıcı: rakip tepki/hız bir kez hesaplanır, seçilemeyecek sonraki anlara bakılmaz.
- Ölçüm: 4 maç her tikte aynı, ~%24 hızlı. **Bu iki dosyayı yeniden yazarken bu iki hızlandırmayı koru.**

- **Bekçi süresi (kural):** `match.js` `BEK_SURE = 150` tik (1×'te 5 sn). Bekçi Çekirdeği kendi alanında (Kuyu + 1,5) bu kadar tutarsa Çekirdek alan sınırında en yakın rakibe geçer. Akıl bilir: `decide.js` süre dolacaksa "tut"/fake = kayıp.
- **Düz "tut":** durmak, durduktan sonra yapılabilecek olan kadar değerli (donmayı önler).
- **Bekçi derinliği:** ağız genişliğine göre (`shape.js` `bekci`).

- **Skora göre pres/blok yalnız yapay zekâ takımında:** `shape.js` `macPres` yalnız `tac.yz` olan takımda çalışır (Canlı rakibe `yz: true` verir). Ölçülü: az basan (0–1) yenik takım +1 kişi, 2+ basan sayıyı artırmaz, blok +1 kademe; önde ve çok basan −1. Kullanıcının takımında bu karar menajerin. Oyuncuların skor/süre farkındalığı (karar değerleri) iki takımda da sürer.

## Claude Code için not (12 Ekim 2026): YZ'ye özel kurallar
**Skora göre pres ve blok değişikliği SADECE yapay zekânın yönettiği takımda çalışır.** Kullanıcının yönettiği takım (Liman) taktikte ne seçildiyse onu oynar; geride ya da önde diye kendiliğinden daha çok basmaz, bloğu da değiştirmez.
- Bayrak: `tac.yz === true`. Canlı maç ekranı rakibin taktiğine `yz: true` ekler, Liman'a eklemez. Laboratuvar da "Liman vs rakip" kipinde rakibe ekler; "Tarz × tarz" kipinde iki tarafa da eklemez (şablon ne diyorsa o).
- Kod: `shape.js` `macPres(m, t)` (ilk satırı: `if (!ME || !(m.tac[t] && m.tac[t].yz)) return 0;`) ve aynı dosyada blok kaydırması (`dtac.yz &&`).
- Ölçülü: az basan (0–1) yenik takım +1, çok basan (2+) yenik takım +0; önde olan takım geri çekilmez.
- **Yeni sürümlerde bu bayrağı koruyun (kaldırmayın).** Skor/zaman farkındalığı (macEff) değer hesabında herkes için var; yalnız **taktiği kendiliğinden değiştiren** kısım YZ'ye özel.

- **Pres mekanik (blok mesafe sınırı kalktı):** Presçi, taşıyıcıya onun bir sonraki hamlesinden önce varabiliyorsa (tepki + koşu ≤ karar süresi + yerinde durduğu süre × 1,2) ve basmanın değeri (varma ihtimali × taşıyıcının tehlikesi, Presçi rolü ×1,3) arkasında bıraktığı en tehlikeli rakipten (×0,55) büyükse basar. Kaç kişi: taktik (+ YZ yenikken +1). Blok yalnız takımın durduğu yer. Uzun tutan taşıyıcıya (Bekçi dahil) basmak kendiliğinden kolaylaşır.
  *Yer:* `shape.js` `prV`, `Th`, ayarlar `prT0, prStill, prCost, prPmin`. Eski `pressR` artık kullanılmıyor.
- **Alan paylaşımı:** Tehlikeli boş rakip bölge dışından da savunmacı çeker (`zoneDng`); hedefleri 4,5'ten yakın iki savunmacıdan önceliksiz olan yana kayar (`ayr`). *Yer:* `shape.js` "alan paylaşımı".

- **Kuyu tarafı adama göre:** Çekirdek Kuyu'ya yaklaşınca adamı olan savunmacı kendi adamıyla Kuyu arasına çekilir (Çekirdekle Kuyu arasına değil). *Yer:* `shape.js` "Kuyu tarafı: adamı olan".
- **Geri koşu bölge hizasına:** geri koşan savunmacılar Çekirdeğin değil kendi bölgelerinin hizasına koşar (köşedeki Çekirdeğe 4-5 kişi toplanmıyordu). *Yer:* `shape.js` `p.job = 'geri koşu'` satırı, `y: cl(bs.y + (25 - bs.y) * .3 ...`.

- **Savunma takım işi (ortak atama):** Bütün görevlerin hedefleri seçildikten sonra tehlikeli rakipler (en tehlikelinin %30'u üstü) tek elden dağıtılır. Kimsenin kapatmadığı tehlikeli rakibe, işi gereksiz olan savunmacı (Çekirdeğin dibindeki fazlalık, aynı yeri kapatan ikinci kişi, geri koşan / kovalayan / sekme bekleyen, adamsız bölge) en çabuk varan önce gider: "kapatma · ad". İlk presçi ve Markajcı bırakılmaz.
  *Yer:* `shape.js` "savunma takım işi", ayarlar `ortakMin, ortakR`.

- **Markajcının adamını arkadaki tutmaz:** Süpürücünün tuttuğu derin rakip listesinden Markajcının adamı çıkarılır; Kuyu önünde iki savunmacı tek forvete kalmaz. *Yer:* `shape.js` `mkRef0`.

- **Pres şiddeti (taktik):** `tac.siddet` 0–1 (Taktik → Savunma → "Pres şiddeti"). Sert: presçi taşıyıcıya daha uzaktan çıkar (varış süresi payı ×0,6–1,4), arkasında bıraktığı rakibi daha az önemser (maliyet ×1,4–0,6) ve ikiliye daha kolay girer (eşik −0,1…+0,1). Kaç kişinin basacağı ayrı ayar.
  *Yer:* `shape.js` `sd = dtac.siddet`; `match.js` ikili döngüsü `sdq`.

- **Blok = presin başladığı hat (yeniden):** Taşıyıcı hattı geçince (kendi Kuyu'muza uzaklık < Düşük 42 · Orta 62 · Yüksek 100) mekanik pres tam çalışır. Hattın gerisinde yalnız (a) hatta 8 birimden yakınsa kısmen, (b) taşıyıcı topu uzun tutuyorsa (Bekçi 2 sn, saha oyuncusu 5 sn) basılır; hiç basmamak yok. Pres kilidi hattın gerisinde geçmez.
  *Yer:* `shape.js` `hat`, `hatK`, ayarlar `prHat, prDis, prHold`.

- **Beklemek savunmayı çekmezse bedeli var:** "Tut" ve "bekle" seçeneklerinde, bekleme süresince 18 birim içindeki rakiplerin taşıyıcıya ne kadar yaklaştığı ölçülür (çekilen mesafe). 4 birimden az çekiliyorsa beklemeye bedel eklenir (`bekBedel` 0,03 × süre/0,5 sn). Öndeyken zamanın değeri (macZ) bunu karşılar, o yüzden önde olan yine oyalayabilir.
  *Yer:* `decide.js` bekleme bloğu `cek`, ayarlar `bekCek, bekBedel`.

- **Tempo = beklemenin bedeli:** Tut/bekle seçeneğine tempo × `bekTempo` (0,05 / 0,5 sn) bedel; kısa (18 birimden kısa) yana/geri pasa (tempo − 0,5) × `cevTempo` (0,025) × tutma ihtimali ek değer. Sonuç: sabırlı + hızlı topu çabuk çevirir (90 sn'de ~74 pas, yerinde durma %1), sabırlı + yavaş saklar (~14 pas, en az kayıp). Sabır pasın yönünü, tempo hızını belirler.
  *Yer:* `decide.js` bekleme bloğu `bekTempo`, "tempo: kısa yana/geri pasla" satırı.

- **YZ uyumu (düşük bloğa karşı):** Yalnız `tac.yz` takımda. Rakip bekliyorsa (saha oyuncularından 5'i kendi yarısında, son 4 sn'de taşıyıcımıza 5 birimden yakın gelen yok) ve skor berabere/gerideyse tempo +0,4, sabır −0,2 (geride −0,35, son 30 sn'de −0,5). Öndeyken dokunmaz. *Yer:* `match.js` `yzUyum` (tacEff'in son halkası).

- **Basan oyuncu sayısı ayarı kapalı:** Ölçümde 1–3 sahada aynıydı (sınır nadiren doluyordu). Sayı motorda sabit 2; kimin basacağını mekanik seçer. Menajerde kalan pres ayarları: **blok** (nereden basılır) ve **şiddet** (ne kadar sert girilir, ikiliyi arar). YZ yenikken pres +1 yerine şiddet +0,25 (blok +1 sürer). Laboratuvar şablonlarında pres sayısı yerine şiddet. Pres kendi planlamasını sonra alacak.
  *Yer:* `shape.js` `presLim` (n = 2), `sd`; Taktik ekranı "NASIL SAVUNULUR"; Lab `SB`, `AY.siddet`.

- **YZ şiddet tepkisi ikiliye de:** yenik YZ takımında şiddet +0,25 hem pres kararında (`shape.js` `sd`) hem ikiliye girmede (`match.js` `sdq`).

- **Dürtülen topun yönü:** İkiliyi kazanıp topu dürten savunmacı, Çekirdeği 12 birim içindeki en boş arkadaşına doğru dürtmeye çalışır; isabet (Okuma + Kesme)/40 × 0,8. Eskiden yön ±46° rastgeleydi; sert preste boştaki topun yarısını rakip alıyordu.
  *Yer:* `match.js` ikili, "dürtme yönü".

## Aşama 1 · rolsüz oyuncu sistemi (13 Ekim 2026)
- **Davranış = talimat, başarı = özellik.** Oyuncu başına, evreye bağlı talimatlar: `p.tl = { huc: {...}, sav: {...} }`. Okuma: `AlanMatch.tl(p, evre, ad)` → oyuncu → takım evresi (`p.tlT`, Aşama 3 için) → varsayılan (`TLV`).
- **Top bizde (huc):** kosu (sabit ↔ koşu atan), istek (ayağına ↔ boşluğa), destek (taşıyıcıya yakın ↔ uzak), risk, ritim (tek dokunuş ↔ tutup bakan), tempo (takıma ±), gonder (gönderme yönelimi), serbest (diziliş yerinden uzaklaşma). **Top rakipte (sav):** adam (alan ↔ adam), kimi (belirli rakip no ya da otomatik), pres (kişisel şiddet).
- **Yer dizilişten:** genişlik (geniş yerleştirilen kenarda kalır), arkada/önde kalanlar (savunma dizilişinde en geride/önde duranlar; `tac.arkada` verilmezse d ≤ 0,25 sayısı). Paketin hücum yerini değiştiren `RSLOT` kullanılmıyor.
- **Paketler yalnız başlangıç önerisi** (`PAKET`): Kurucu {destek .2, serbest 0, kosu .3} · Koşucu {kosu 1, serbest .76} · Kanat/Pivot {serbest .21} · Markajcı {adam 1} · Presçi {pres 1}. Motor paket adına bakmaz (yalnız görev etiketinde görünür).
- **Gizli avantajlar kalktı:** Presçi ×1,3 → (1 + (pres − 0,5) × 0,6); Presçi önceliği → pres talimatı; Markajcı → adam ≥ 0,5; Pivot boşluk terimi kaldırıldı; uyum notları talimata göre.
  *Yer:* `match.js` TLV/PAKET/tl/tlKur, assignRoles, roleSample; `shape.js` sweep, cand, prV, roleLeash/roleY/roleRun, aday noktalar; `decide.js` risk, T (tempo), su (gönder), istek.

- **Baskı altında pas dağılır (zorlanan hata):** pasın nişan hatası × (1 + 1,5 × baskı), baskı = `pressAt` (gönderme isabetiyle aynı aile). Aktarım dağılmayı küçültür. Maç (`launch`) ve akıl (pas denemesi `trials`) aynı `pasEk` fonksiyonunu kullanır; oyuncu baskı altındaki pası daha riskli görür ve ya erken bırakır ya da başka yol arar.
  *Yer:* `decide.js` `PAS`, `pasEk`; `match.js` `launch`.

## Katman 2 · oyun evreleri (13 Ekim 2026)
- Her takım her an bir evrede: **top bizde** geçiş (kazandıktan sonra 3 sn) · kurma (kendi üçte birimiz) · ilerleme · son; **top rakipte** geçiş (kaybettikten sonra kpSure) · kurma (rakip kendi arkasında) · orta · son (bizim üçte birimiz). Sınır: kendi Kuyu'ya uzaklık 35 / 65. `AlanMatch.evreBul`, `EVKOD`.
- Evre ayarları: `tac.evre['huc.kurma'] = { tempo, blok, siddet, sabir, risk, genislik, diz }`; verilmeyen takım ayarından gelir. Evre dizilişi `diz` (6 slot, d/l) yalnız o evrenin tarafına (sav → slot, huc → rslot) uygulanır; evre bitince temel dizilişe dönülür. Yalnız hedefler değişir, oyuncular koşar.
- Oyuncu talimatı evreye göre: `p.tl['huc.kurma'].kosu` → `p.tl.huc.kosu` → takım → varsayılan.
- Uygulama: `step` başında `evreUygula`. Birleşik taktik `_ev` işaretli; temel taktik `m.tac0`. Maç içi taktik değişikliği yeni temel olur. Belirlenimci (aynı tohum aynı maç).
- Kare 40–41: iki takımın evre kodu (`EVKOD` sırası); `mac-isci.js` NF = 42.

## Katman 4 · ilişkiler
- **Boşluk doldurma:** talimat `huc.doldur` = arkadaş no. O arkadaş hücum yerinden 8+ birim ileri koşunca bu oyuncu onun yerine geçer ("boşluk doldurma · ad").
- **Pres tuzağı:** takım ayarı `tac.tuzak = 'kenar'`. Taşıyıcı ortadan 13+ birim kenara inince ilk presçi dışarıdan, ikinci savunmacı içeriden (taşıyıcı ile orta arası) kapatır ("pres tuzağı · içi kapat").
  *Yer:* `match.js` `iliski` (yerleşimden sonra hedefleri düzeltir); Taktik: savunma "Pres tuzağı", oyuncu kartı "Boşluk doldurur".

## Katman 5–6 · koşullar ve maç planı
- **Koşullar** (`tac.kosul = [{ ne: 'onde'|'geride'|'esit', son: sn|null, ayar: { blok, siddet, tempo, sabir } }]`): şart tutunca ayar evre ve takım ayarının üstüne yazılır (öncelik: koşul > evre > takım). `m.kosulAkt`.
- **Maç planı** (`tac.plan = { taraf: 'sol'|'sag', hedef: rakip no }`): tercih edilen kanada pas +0,015 (karşı kanat −0,0075); hedef rakip topu alınca pres kazancı ×1,4.
  *Yer:* `match.js` evreUygula; `shape.js` gainH; `decide.js` tercih (`planK`); Taktik "Maç planı" sekmesi (gelişmiş modda).

- **Geçiş yalnız rakip dağınıkken:** top kazanıldığı an kaybeden takımın topun gerisinde (kendi Kuyu tarafında) kaç saha oyuncusu olduğu sayılır; 4+ ise yerleşiktir → geçiş evresi, kontra koşuları ve karşı pres olmaz, oyun topun yerine göre normal evrede sürer (`m.gecis`).
- **Santra:** başlayan takım hücum dizilişiyle, savunan takım savunma dizilişiyle ve bloğuna göre derinlikte dizilir; santrada geçiş yok.
  *Yer:* `match.js` take (`m.gecis`), evreBul, inWin, kpEff, kickoff.

- **Taktik değişikliği hızlı:** maç her 300 tikte kopyalanır (`AlanMatch.kopya`; rng durumu `f.s`'de); değişiklikte baştan değil en yakın kopyadan yeniden oynanır, sonuç birebir aynı. *Yer:* `match.js` rng/kopya, `mac-isci.js` anlikK/yeniden.
- Hata: hücum dizilişi verilmeyen geniş oyuncuda `p.rslot.l` çöküyordu (`shape.js`). Taktik ekranı evre seçiliyken temel diziliş yerine evre şeklini gönderiyordu (düzeldi).

## Talimat düzeltmeleri (Claude Code etki testi sonrası)
- **Kişisel pres:** takımın şiddetinden sapma `sdp = sd + (pres − 0,5) × 0,8`; presçinin çıkış süresine (`ThP`) ve arkada bırakılanın bedeline girer (eski ×0,7–1,3 değer çarpanı kalktı).
- **Destek mesafesi:** taşıyıcıya hedef mesafe 6 + destek × 14 (yakın 6, uzak 20); sapmanın bedeli `destekK` 0,06 × ((d − hedef)/8)² (en çok 4), orta talimatta yok; asıl yer hesabının yanında tercih.
- **Adam adama:** adamı topu alınca o oyuncu ilk presçi (pres hattı içinde); geri koşu onu adamından koparmaz.
- **Koşu–serbestlik çelişkisi** oyuncu kartında uyarı.
  *Yer:* `shape.js` prV (sdp, ThP), cand sırası, press.push, geri koşu koşulu, roleY; Taktik oyuncu kartı.

## zip34 · Claude Code bulguları
- **Foto finiş belirsizliği:** pas denemesinin her tekrarında rakiplerin tepki anı ±3 tik farklı (`D._jitS`, `belirsiz` 6); yalnız kafadaki kovalamada (`chasePlan`). Okuma 20 bile rakibin bir sonraki hareketini kesin bilemez. Rahat yarış yine ~%99, 1 tiklik yarış ~%50.
- **Koşu talimatı alt yarısı:** 0,5 altı koşu noktalarına ceza (`roleRun` işaretli); 0,25 altında çizgi arkası adaylar dizilişten bağımsız eklenmez.
- Açık: koşucuya pas artmıyor (pasçı tarafı) — ölçüm sonrası bakılacak.
  *Yer:* `decide.js` simOne trials, chasePlan rc; `shape.js` roleRun, aday noktalar.

- **Düzeltme (zip38):** `decide.js` pas denemesinde arkadaş kopyalanırken `D._jitS` sıfırlanıyordu (yazım hatası) ve aynı satır rakiplerin Okuma'ya bağlı algı hatasını atlıyordu. Artık: arkadaş `{ ...p }`, rakip algı hatasıyla; belirsizlik her denemenin sonunda sıfırlanır.
- **olcum-donma.js:** sınırlar tik olarak düzeltildi (2 sn = 120, 1 sn = 60).

## zip39 · Düzeltme 1: donuk dünya (bayrak `ALAN_DONUK_ESKI`)
- `predictWorld`: hamle sonrası dünya artık hızları koruyarak, maçtaki ivme kuralıyla (`AlanMatch.ivme`: dönüş, fren, hızlanma) hamle süresi kadar (en çok 60 tik) ileri kaydırılır. Rakip tepki süresine kadar mevcut hızıyla sürer, sonra hedefine döner (hedefler eskisiyle aynı). Hızlar sıfırlanmaz. `self.ALAN_DONUK_ESKI = true` eski donuk hâli geri getirir.
- Kısa deneme (3 maç, ~45 sn): önü açık koşan taşıyıcıda "sür" %55 → %36, tut/fake %0 → %19 (ölçütün tersi yönde, ama ≤ %30 sınırında); gönderme ↑, pas tutma %90 → %93, hesap ~%20 hızlı. Örnek küçük; Ölçüm 1 tekrarı karar versin. Tutmazsa yama yok, önce neden.
  *Yer:* `decide.js` `predictWorld`.

## zip40 · Gönderme kalibrasyonu, 1. adım: hayaldeki Bekçi teması = maçtaki `contact` (bayrak `ALAN_BEK_HAYAL_ESKI`)
- `simOne`: kafadaki denemede Çekirdek rakip Bekçi'ye değince artık ayrı tahmin (tut / delip geçer / sekip kime düşer) yok; maçın `core.contact`'ı denemenin kendi zarıyla çağrılır (uzanma, tutma zarı, delme, çelme yönü); seken top aynı denemede oynanmaya devam eder.
- **Gönderme tablosu yeniden üretilmedi** (sürüm aynı; tablo hücreleri aynı ızgarada yeniden üretilmeli).
- **Henüz ölçülmedi.** İlk bakış (birkaç tablo hücresi): neredeyse aynı (15 birim ortadan hâlâ ~%100). Bu, iyimserliğin büyük kısmının temasın kendisinden değil, hayaldeki Bekçi'nin **yerinden ve uzanma zamanlamasından** geldiğini düşündürüyor. Örnek: tablodaki Bekçi kendi formülüyle yerleşiyor (`r = min(6, L × 0,22)`), maçtaki `AlanShape.bekci` değil. Kafadaki denemelerin çoğunda Bekçi Çekirdeğe hiç değmiyor (bir hücrede 26 denemenin 4'ü).
- Karar kaydına sür / fake / tut değer bileşenleri: **yapılmadı**, sırada.
  *Yer:* `decide.js` `simOne` "Bekçi teması maçtaki kuralın kendisiyle".

## zip40 · Gönderme kalibrasyonu, 1. adım: hayaldeki Bekçi teması = maçtaki `contact` (bayrak `ALAN_BEK_HAYAL_ESKI`)
- `simOne`: kafadaki denemede Çekirdek rakip Bekçi'ye değince ayrı tahmin yerine maçın `core.contact`'ı denemenin kendi zarıyla çağrılır (uzanma, tutma zarı, delme, çelme yönü); seken top aynı denemede oynanmaya devam eder.
- Gönderme tablosu yeniden üretilmedi (sürüm aynı).
- **Henüz ölçülmedi.** İlk bakış: birkaç tablo hücresi neredeyse aynı (15 birim ortadan ~%100). Kafadaki denemelerin çoğunda Bekçi Çekirdeğe hiç değmiyor (bir hücrede 26 denemenin 4'ü). İyimserliğin büyük kısmı büyük ihtimalle hayaldeki Bekçi'nin yerinden / uzanma zamanlamasından (tablo Bekçi'si `r = min(6, L × 0,22)` ile yerleşiyor, maçtaki `AlanShape.bekci` ile değil).
- Karar kaydına sür / fake / tut bileşenleri: yapılmadı, sırada.
  *Yer:* `decide.js` `simOne`.
