# ALAN v3g · İhtimal zinciri — tasarım notu

Durum: **plan kesinleşti**. Kod: adım 0a bitti (maç durumu kopyalanabilir, testler geçti); 0b (önde koşan oynatma sayfası) sırada. `alan-v3g/` şu an v3f'in birebir kopyası; v3f'e dokunulmaz.
Her değişiklik `ALAN v3 Tasarım.md` ilkelerine karşı denetlenir. Bu not onaylanmadan kod yazılmaz.

---

## 1. Neyi çözüyoruz

v3f'te her taşıyıcı her kararda geleceği kafasında birkaç zarla oynatıyor (imagine + lookAhead). Bunun üç bedeli var:

| Sorun | Ölçü | Kaynağı |
|---|---|---|
| Yavaşlık | ~9 sn / 1000 tik; karar başı ~90–100 ms | Her seçenek için 2–6 tam simülasyon |
| Gürültü | Kusursuz iki oyuncu aynı anda kararların ~%17'sinde ayrışıyor | Az zarla Monte Carlo |
| Kalibrasyon kayması | Her yeni mekanikte tahmin/gerçek yeniden kayıyor | Kafadaki dünya ile maç dünyası ayrı ayrı bakım istiyor |

Hedef: **aynı fizik, aynı ilkeler; hesap önceden ve bir kez.** Oyuncu maçta tabloya bakar, tabloyu Okuma'sı kadar bozuk görür.

## 2. Üç katman

### a) Önceden ölçülmüş olasılık tabloları (çevrimdışı)
Motor binlerce sahne/maç oynar ve şu soruların cevabını hücrelere yazar:

1. **Aktarım tablosu** — bir pas/gönderiş/kenardan pasın sonucu.
   - Girdi (durum özeti): mesafe, açı, hız sınıfı, şarj, hat üstündeki en yakın rakibin yanal mesafesi ve tepki payı, alıcının boşluğu, kenardan mı.
   - Çıktı: P(arkadaşa ulaşır), P(rakibe geçer), P(boşta kalır), P(sayı) + varış durumunun özeti.
2. **Ellenme tablosu** — taşıyıcı tutar/sürer/döner/fiske/ikili.
   - Girdi: baskı (en yakın 2 rakibin mesafe+açısı), Çekirdeğin gövde tarafı, hız, Sürme/Kesme farkı sınıfı.
   - Çıktı: T tik sonra P(hâlâ bizde) ve kaybın yeri.
3. **Durum değeri V(s)** — bir durumdan hücumun sayıyla bitme olasılığı eksi rakibe sayı verdirme olasılığı.
   - Girdi: Çekirdeğin bölgesi, boşluk (decide.space), taraflar arası sayısal üstünlük (Çekirdek ile Kuyu arasında kaç rakip/kaç arkadaş), geçiş anı mı.

Bağlantı: **Markov zinciri.** V(s) = Σ seçenekler · P(sonuç) · V(sonuç durumu). Değer yinelemesiyle çözülür; `value-table.js` ve `shot-table.js` bunun ilk iki parçası.

Hücre sayısı bütçesi: her tablo ≤ ~5.000 hücre; hücre başına ≥ 30 örnek (proje kuralı). Az örnekli hücreler komşulardan pürüzsüzleştirilir, bu not tutulur.

### b) Karar modeli: değer kusursuz, oyuncu kestirir (kuruldu · sadece paslar)
- **Değer** motorda kusursuz hesaplanır (bugün: oynatma kapalı v3g değerlendirmesi; ileride durum değeri tablosu). Pas için algı hatası yok.
- **Kestirim yarışın üzerindedir, değerin üzerinde değil.** Oyuncu, en tehlikeli rakibin Çekirdeğin yoluna yetişme payını (tik) Okuma'ya bağlı hatayla kestirir: Okuma 6 ±16, 10 ±8, 14 ±3, 18 ±0,3 tik (`dmK × ((20 − Okuma)/10)^dmP`, dmK 8, dmP 2). Bu, ulaşma ihtimalini kaydırır; değer sadece o kaymanın kazanç/bedel karşılığı kadar değişir. Bariz yarışta ihtimal doymuştur, kayma kendiliğinden ~0; kıl payında en büyüktür. Sınırsız değer gürültüsü yok.
- **Hata rakip başına:** oyuncu bir rakibi yanlış kestirirse o rakibin yarıştığı bütün seçenekler aynı yönde kayar. Hata top elindeyken kalıcıdır (titreme yok).
- **Kayma (karakter):** Cesaret ve taktik riski kaybın tartısında (risk çarpanı).
- **Hedef eğri bırakıldı:** "Okuma 6 %45 eşdeğer" bu motorun değer dünyasına uymuyor (en iyi–ikinci farkı medyan 0,001; ±48 tik hata bile %69 eşdeğer verir). Okuma'nın hatası fiziğe göre seçilir; etkisi maç sonucuyla ölçülür (Okuma 18 takımı – Okuma 6 takımı hücresi). Eşdeğerlik ölçüsü tablo gelince yeniden kullanılır.
- Eski yol `dm: false` ile açılır.

### b-eski) Okuma tabloya uygulanır, tablonun yerine geçmez (önceki karar; yerine yukarıdaki geçti)
- Kusursuz bilgi tabloda. Oyuncu tabloyu **kendi gördüğü durumla** sorgular: rakiplerin yeri ve hızı Okuma'ya bağlı sapmayla görülür (v3f'teki algı hatasının aynısı).
- **Ayrı bir değer gürültüsü yok.** Dağınıklık sadece algı hatasından gelir (düşük Okuma'lı oyuncu kapalı hattı açık sanır). Algı hatası yansız olduğu sürece oyuncu ortalamada doğru okur. Ek bir gürültü düğmesi "sayıyla gizleme" olurdu.
- **Algı hatasının büyüklüğü bir hedef eğriyle seçilir**, sonra sahnede gözle onaylanır:

| Okuma | En iyi hamleyi seçme oranı | Büyük hata (değer kaybı > 0,10) |
|---|---|---|
| 18 | ≥ %85 | ≤ %3 |
| 10 | ~%65 | ≤ %10 |
| 6 | ~%45 | ≤ %20 |

  Ölçü: aynı karar anlarında oyuncunun seçtiği hamle ile kusursuz algılı seçim karşılaştırılır (Claude Code'daki `tools/zeka.js`). Algı hatasının Okuma'ya bağlı ölçeği bu eğriye oturtulur. "Okuma 6 ne kadar saçmalar?" bir ayar değil, bu tablodaki tasarım kararıdır.
- Aktarım/Tutuş/Sürme tabloya girdi olarak girer (hız sınıfı, nişan hatası, kapasite) — fizik aynı kaldığı için tablo onları zaten ölçer.

### c) Takım planı (sonraya)
Ayrı iş parçacığı yarım saniyede bir takım için kısa bir plan önerir: koşular, pas zinciri, pres tuzağı.
**Sınır (ilke 1, kararlar yereldir):** plan sadece oyuncuların görebildiğine dayanır; her oyuncu planı Okuma'sı kadar geç ve eksik algılar; uygulamak ona kalır. a+b oturmadan başlamaz.

## 3. Ne değişir, ne değişmez

| Değişmez | Değişir |
|---|---|
| core.js fiziği (Çekirdek, karşılama, dönüş, kenar, alanlar, şarj) | decide.js: `imagine`/`lookAhead` → tablo sorgusu |
| Yerleşim (shape.js) ve taktik ayarları | Seçenek değeri: aynı birim (V), tablo kaynaklı |
| Maç döngüsü, olaylar, laboratuvar arayüzü | Tek dokunuş/kontrol kararı: tablo + kontrol süresi |
| Okuma/Aktarım/… özelliklerinin anlamı | Okuma'nın etkisi: sadece algı hatası, büyüklüğü hedef eğriden |

`imagine` silinmez: **tabloyu üreten ve denetleyen** araç olarak kalır.

## 4. Doğrulama (her adımda)
0. **Taban:** önce 60 zarlı kâhinin kendi kendisiyle uyumu ölçülür (bugün iki kusursuz oyuncu aynı hamlede sadece ~%48 buluşuyor; seçenekler çoğu zaman neredeyse eşit). Bütün hedefler bu tabana göre konur.
1. **Kâhin testi:** aynı durumlarda tablo kararı ile Okuma 20 + 60 zarlı imagine kararı karşılaştırılır. Ölçü "aynı hamle" değil, **eşdeğer hamle** (en iyiden değer farkı < 0,02) ve **değer kaybı** (medyan + büyük kayıp oranı). Ayrıca hızlı geçişlerde ve savunmanın kaymasını gerektiren kararlarda ayrıca ölçülür: ileriye bakışı kaldırmanın oyunu düzleştirip düzleştirmediğini ("salağlaşmasınlar" şartı) bu söyler.
2. **Dilim kalibrasyonu:** tür başına ortalama değil, her dilimde (yakın/uzak × baskılı/baskısız × pas türü) tahmin/gerçek farkı ≤ 5 puan. Ortalama ters yönlü hataları gizler (stillerde tahmin %84 / gerçek %67 böyle saklanmıştı).
3. **V(s) kalibrasyonu:** "bu durumdan sayı %30" diyen hücreler maçta ~%30 sayıyla bitiyor mu.
4. **Özellik farklarıyla:** testler sadece herkes 10'ken değil; Hız 6'ya karşı 14, Tutuş 6'ya karşı 14, Kesme/Sürme farkları da.
5. **Sahneler:** K3 karar sahneleri + pres kırma + kontra + Kuyu önü kartları v3f'teki gibi geçer.
6. **Hız:** 1000 tik ≤ 1,5 sn (hedef ~6×).
7. **Denge:** laboratuvarda v3f ile aynı hücreler, hücre başına 30 maç; sonuçlar şans payı içinde ya da gerekçeli farklı.

**Tablo üretimi kuralları:**
- Varış durumunun özeti, **savunma kaydıktan sonraki** boşluğu içerir (`space`, `front`), pas anındakini değil.
- Örnekler düzgün bir ızgaradan değil, **maçlardan kaydedilen gerçek durumlardan** toplanır.
- Paralel üretim **belirlenimci:** her hücre kendi tohumuyla; sonuç iş parçacığı sayısına ve sırasına bağlı olmaz.

## 5. Sıra

0. **Maç durumunu kopyalanabilir yap + önde koşan oynatma** (tablolardan bağımsız, önce yapılır): zar üreteci, önbellekler, tik tik kopya testi; maç sayfası tampondan oynatır, taktik değişince geri yükleyip yeniden hesaplar. Bugünkü motorla bile takılmayı bitirir.
1. **Durum özetini tanımla** (aktarım için): hangi değişkenler, kaç dilim. Kâhin testiyle "bu özet kararı ne kadar belirliyor" ölçülür; yetersizse değişken eklenir.
2. **Aktarım tablosunu üret** (paralel iş parçacıklarıyla, imagine ile). Kalibrasyon testi.
3. **decide.js'te pas/gönderiş değerini tablodan al**; imagine yedekte, karşılaştırma için açık.
4. **Ellenme tablosu** (tut/sür/dön/fiske/ikili).
5. **V(s) zinciri** — öne alındı, 1'den önce yapılır (Bölüm 9).
6. **Algı hatasını Okuma hedef eğrisine oturt** + sahne ve laboratuvar denetimi.
7. (Onayla) Takım planı.

Her adım gözle ve ölçüyle onaylanmadan bir sonrakine geçilmez.

## 6. Tablolar nereden gelir (karar verildi · karma)

| Tablo | Nasıl üretilir | Özellikler nasıl girer | Ne zaman |
|---|---|---|---|
| Aktarım (pas, gönderiş, kenardan) | **genel**, bir kez çevrimdışı | sorgu anında fizik üzerinden: rakibin yetişme payı kendi Hız'ı ve tepkisiyle, alıcının payı Tutuş'uyla, pasçının nişan hatası Aktarım'ıyla | fizik değişince elle |
| Ellenme (tut, sür, dön, fiske, ikili) | **genel**, bir kez çevrimdışı | sorgu anında: Sürme/Tutuş/Kesme/Hız farkı | fizik değişince elle |
| Durum değeri V(s) | **taktikçe**: savunan takımın taktik birleşimlerine göre çevrimdışı üretilmiş tablolardan biri seçilir | — | fizik değişince elle |

- Önce V'nin taktikle gerçekten farklılaşıp farklılaşmadığı ölçülür; farklılaşmıyorsa tek tablo yeter.
- Maç öncesi ve taktik değişiminde **tablo hesabı yok**, sadece seçim. İstenirse sonra maç öncesi kısa deneme maçlarıyla küçük bir düzeltme eklenir.
- Her tabloya **fizik damgası** yazılır; motorun fiziğiyle uyuşmazsa laboratuvar durur ve uyarır.
- **Yükleme ekranı** yine var, ama tablolar için değil: maç başlarken önde koşan oynatmanın tamponu dolarken ve taktik değişince kalan maç yeniden oynanırken (Bölüm 7). Taktik paneli "Uygula" ile onaylanır; art arda değişiklik tek yeniden oynatma yapar. İptal edilirse eski taktikle devam.
- **Taktik değişince rakibin bilgisi (karar verildi):** kendi takımın yeni talimata hemen uyar. Rakip oyuncular yeni durumu ancak değişikliği "okuyunca" hesaba katar; her rakip oyuncu için gecikme Okuma'ya bağlı. O süre boyunca rakip eski taktiğe göre seçilmiş V ile düşünür. Böylece sürpriz taktik değişikliği gerçek bir hamledir (ilke 4).

## 7. Maç nasıl izlenir: önce oynanır, sonra izlenir (karar verildi · B)

- **"Maçı başlat":** yükleme ekranı → (1) taktiklere uyan tablolar seçilir, (2) maç arka planda **izlemenin önünde koşturularak** oynanır ve kaydedilir (her tik: oyuncular, Çekirdek, olaylar, karar paneli). İzleme, hesap yeterince öne geçince başlar (tampon: ~15–20 sn önde; tablolarla maçın tamamı zaten ~10 sn'de biter). Hesap ortalamada gerçek zamana yetmesi yeter; anık ağır kararlar sadece tamponu kısaltır.
- **İzleme:** kusursuz akıcı; 1×/2×/4×, duraklat, ileri-geri sarma, olay listesinden atlama. Takılma imkânsız çünkü izlerken hesap yok.
- **Taktik değişikliği:** izleme duraklar → yükleme ekranı → (1) yeni taktiğe uyan V tablosu seçilir (rakip için Okuma gecikmeli, Bölüm 6), (2) maç **o tikten itibaren** yeniden oynanır (o anki durum aynen alınır; kaydın o andan sonrası atılıp yenisi yazılır). İzleme kaldığı yerden devam eder.
- **Kural:** menajer sadece **şu ana kadar izlediği** kısmı bilir; kaydın izlenmemiş kısmı gösterilmez (ileri sarma sadece izlenmiş kısımda ya da "hızlı izle" olarak, sonuç ekranda açılmadan). Yoksa sonucu görüp taktik değiştirmek olur.
- **Rastgelelik ve tam kopya:** maç tohumla oynanır. Yeniden oynatma için maçın **tam durumu** kopyalanabilir olmalı: oyuncular, Çekirdek, olaylar, **zar üretecinin içi** (bugün `m.r` bir kapanış fonksiyonu, kopyalanamıyor → tek sayı tutan bir yapıya çevrilir) ve motordaki önbellekler (`m.sh`, `_kc`, oyuncularda `_lc`, `_plan`, `_dz`, `manRef`…). Test: her tikte kopyala→geri yükle→devam et ile kesintisiz devam aynı sonucu vermeli; "aynı tohum + aynı taktik değişiklikleri = aynı maç".
- **Sınır (ilke 4):** önceden oynanan maç sadece **gösterim** içindir; aynı maçtaki kararlar için bilgi kaynağı olamaz. Tablolar her zaman maçtan bağımsız simülasyonlardan gelir (V(s) için kullanılan kısa deneme maçları da ayrı tohumla, ayrı maçlardır).
- **Laboratuvar** aynı yolu kullanır (oynat, kaydetme); iki ekran aynı motoru aynı şekilde çalıştırır.
- **Bütçe:** tablo hesabı yok; maç ~10 sn (90 sn'lik maç = 5.400 tik, hedef ≤ 1,5 sn / 1000 tik). İzleme tampon dolunca başlar. Taktik değişince: kalan maç.
- **Kayıt boyutu:** 5.400 tik × 14 oyuncu × (x, y, iş) ≈ birkaç MB; iş etiketleri ve karar paneli sadece değiştiğinde yazılır.

## 8. Kararlar

- **A (karma tablo):** evet → Bölüm 6.
- **B (rakip taktik değişikliğini Okuma'sı kadar geç fark eder):** evet → Bölüm 6.
- Tablo ne zaman yeniden üretilir → fizik damgası, elle.
- Özellikler tabloda boyut mu → hayır, sorgu anında fizik üzerinden.
- Okuma gürültüsü → ayrı gürültü yok; algı hatası hedef eğriye oturtulur (Bölüm 2b).

Açık soru yok.

## 9. Durum değeri V(s) · nasıl öğrenilir (karar verildi)

**Neden önce bu.** v3f'te kazanç ile kayıp farklı terazilerle tartılıyor: kazanç elle yazılmış `build = 0,06 + 0,28 × threat` eğrisinden, kayıp ise rakibin o noktadaki ölçülü değerinden (`lossAt = V(rakip, …)`) geliyor. Kuyu'dan uzakta eğri düz, kayıp ağır; oyuncu "kaybetmeme" oynuyor (tüm tempo/risk ayarlarında Çekirdekle zamanın %73–98'i kendi yarısında, ~1 maçlık ölçüm). Bu bir katsayı değil, mekanik hatası. V(s) aktarım tablosundan önce yapılır (Bölüm 5'te 5 → 1'in önüne).

**Tek ölçü:** V(s) = P(sıradaki sayıyı biz atarız) − P(rakip atar). Sıfır toplamlı: kazanç da kayıp da bu farktır. Elle yazılmış eğri kalmaz.

**Durum girdileri:** Çekirdeğin bölgesi · boşluk (savunma kaydıktan sonra) · Çekirdek–Kuyu arasındaki sayısal üstünlük · geçiş anı mı · **savunmanın yerleşikliği** (bloğun dizilişine ne kadar oturduğu). Yerleşiklik beklemenin bedelini kural yazmadan getirir: yerleşmiş bloğa karşı Çekirdeğe sahip olmak daha az değerli çıkar; sabır ancak bloğu oynatırsa değer kazandırır.

**Öğrenme:**
- **TD (zamansal fark):** her durumun değeri bir sonraki durumun değerinden güncellenir; sayı olduğu yerde gerçek sonuç girer. Maç başına 3–4 sayıyla doğrudan gollerden doldurmak binlerce maç isterdi.
- **Turlar:** ölç → oyuncular yeni tabloyla oynar → yeniden ölç; değerler oturana kadar. Bugünkü pasif motorun maçlarından tek seferde ölçülen tablo o pasifliği öğrenirdi.
- **Sönüm:** her tur eski ve yeni tablonun karışımı; iki uç arasında salınmayı önler.
- **Kusursuz algı:** turlar kusursuz algılı oyuncularla oynanır; tablo "kusursuz değer"dir. Maçta her oyuncu onu kendi Okuma'sı kadar bozarak görür (Bölüm 2b). Okuma'nın etkisi tek yerden gelir.
- **Özellik çeşitliliği:** turlarda oyuncu özellikleri rastgele 6–16 dağıtılır; hızlı savunmacının, Tutuş'u düşük alıcının yarattığı durumlar da tabloya girer.

**Taktik karışımı:**
- **%80 tutarlı çekirdek:** aşağıdaki altı tarz, her biri kendi içinde oynanmış hâlleriyle (pres ±1, blok ±1 kademe, örneklerin üçte birinde savunma sistemi komşu sisteme).
- **%20 rastgele:** menüden tamamen rastgele kombinasyonlar. Bunların **büyük kısmı tutarlı çekirdeğe karşı** oynanır (iki tarafta da); ceza, mantıklı takımın saçma takımın açtığı boşluğu bulduğu anda oluşur. Saçma taktikler dışarıda bırakılmaz: tablo onların durumlarını görmezse cezaları da tabloda kaybolur.
- **Kapsama denetimi:** taktiklerin değil durumların kapsanması önemli. Her turda: hangi hücreler sadece rastgele maçlardan doluyor, oralarda ≥ 30 örnek var mı. Yoksa o maçların sayısı artırılır.
- **Her turda denge denetimi:** ilke 5 (hiçbir tarz hep kaybetmez) laboratuvarla kontrol edilir; turlar tek tarzı "tek doğru" yapmaya başlarsa durulur.

**Altı tarz (tek kaynak):**

| Tarz | sistem | pres | arkada | blok | genişlik | tempo | risk | kazanınca |
|---|---|---|---|---|---|---|---|---|
| Dengeli | Alan | 1 | 1 | Orta | Normal | 0,5 | 0,5 | Dengeli |
| Sabırlı | Alan | 1 | 1 | Orta | Geniş | 0,2 | 0,2 | Yerleş |
| Dikine | Alan | 1 | 1 | Orta | Normal | 0,8 | 0,8 | Kontra |
| Kontra | Alan | 0 | 1 | Düşük | Dar | 0,8 | 0,8 | Kontra |
| Ön alan | Adam adama | 3 | 1 | Yüksek | Geniş | 0,5 | 0,5 | Dengeli |
| Kuyu önü | Alan | 0 | 2 | Düşük | Dar | 0,2 | 0,2 | Yerleş |

Komşu sistem: Alan ↔ Kenara sıkıştır, Adam adama ↔ Kenara sıkıştır. Böylece Kenara sıkıştır çekirdekte de yer alır.

**Önce teşhis (V'den önce, Claude Code):**
1. Aynı 5 tempo/risk ayarı, ayar başına 6–10 maç; ölçü: Çekirdekle kendi yarısında kalma oranı, ileri/geri pas dengesi.
   - (a) mevcut ölçülü tablo açık (`useValTab: true`);
   - (b) kaybın bedeli kazançla aynı eğriden — **sadece teşhis**, kalıcı değil. Tek amacı "pasifliğin sebebi terazinin dengesizliği mi" sorusuna evet/hayır.
2. **Tut değişikliği:** `tut = P × max(build, en iyi gönderiş)` f7'ye göre pasifliği artırıyor mu. Artırıyorsa V gelene kadar geri alınır (yeni sayı değil, ölçülmüş bir sebebin geri alınması).
3. Sonuç evetse V(s) bu bölümdeki gibi kurulur; hayırsa sebep başka yerde (örn. kendi yarıda ileriye bakışın kapalı olması) aranır.

Sıradaki: Bölüm 9, durum değeri tablosu · tur 0 (veri ve TD hesabı Claude Code'da, bulutta; bağlama, maç ve sahne kontrolleri Design'da).

**Bağlantı noktası (kuruldu):** `window.AlanState.idx(src, takım, x, y, geçiş)` → hücre (0…539). Tabloyu üreten ve kullanan aynı fonksiyonu çağırır. Tablo `window.ALAN_VS = { v: [540 değer, −1…+1], fiz: '…' }` olarak yüklenir, `D.useVS = true` ile `build()` yerine geçer. Kaybın bedeli (`lossAt`) zaten `V(rakip, aynı nokta)`; tablo açıkken aynı ölçüden gelir. Geçiş: Çekirdek kazanılalı 180 tikten az (`m.winT`).
**Okuma kanalları (ölçüm için):** `self.ALAN_OKX = { eq: [...], only: '...', mid: 12 }`; kanallar: karar, kararHizi, rakipModel, gonder, kontrol, kenar, algi, tepki, ikili, yerlesim, bekci.
