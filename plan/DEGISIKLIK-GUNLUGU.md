# ALAN v5 · Motor değişiklik günlüğü

> **15 Ekim 2026'dan itibaren motoru Claude Code yönetir** (bkz. `CALISMA-DUZENI.md`). Aşağıdaki girdiler zip71'den sonrası; ayrıntı ve ölçümler `DESIGN-NOT-zip78-sitede.md`'de.

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
