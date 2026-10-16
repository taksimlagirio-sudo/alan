# An kataloğu

Kullanıcının maçta gördüğü sorunlu anlar. Amaç tek tek yama değil: anlar biriktikçe ortak ilkeleri bulup toplu çözüm üretmek.
Her an: kimlik (Anlar ekranı `a-…` ya da tohum + saat), kullanıcının sözü, hangi ilkeye bağlandığı. Motor sürümü yazılır; o sürümden eski anlar yeni motorda başka oynar.

## Plan · A, C, D, E birlikte (16 Ekim, kullanıcıyla)
Ortak çekirdek: "top sahibinin her seçeneği ne kadar değerli, ne kadar olası". Hücum bununla seçer, savunma aynı listeye olasılık × zarar ile bakar. Sıra:
1. **C** · değerler: sürme neden pastan değerli çıkıyor (sürme-sonrası hesabı savunmanın kapanmasını sayıyor mu); boş arkadaşa pas, alıcının alacağı zamanı ve açıyı sayıyor mu. Önce ölç, göster; değişiklik onayla.
2. **D** · önüne pas adayları: koşan herkese (hızından, görev yazısından değil) koşusunun önüne, kaleye doğru. Buluşma ve kale ağzı yamaları bunun içinde erir.
3. **E** · topsuz destek: alıcının yeri C'deki ölçüyle (savunmacının varma süresi, kaleye açı, geri pas yolu).
4. **A** · savunma: presçi en zararlı seçenekleri kapatarak yaklaşır, ikinci adam kalan en zararlı yola, son adam tercih yapar. Kapatma, karşılama, buluşma presi gibi parça kurallar bunun içinde erir.
Değilen yerlerde görev yazısı koşulları temizlenir. Her adım bayraklı.
**Sınama:** tarama yok. Her adımdan sonra sınama anı `a-52-0-6951-kontra` (ve kataloğun öteki anları) için seçenek dökümü önce/sonra; kullanıcı sitede anlarla kontrol eder.

## Genel kural · Kararlar görev yazısını koşul olarak okumaz
Görev ataması kalır (savunmada kim kime basar, kim markaj, kim arkada: bunu atmak savunmayı dağıtır) ve görev kartındaki yazı kalır. Ama hiçbir karar görev **yazısına** bakmaz ("görevi 'koşu' içeriyorsa…"); sahada görünen duruma bakar (hız, yön, konum, kimi tuttuğu). Örnek: Şimşek kaleye koşarken görevi "boşluk" yazdığı için buluşma pası üretilmedi (D1).
Bugün motorda görev yazısını okuyan ~17 yer var (match.js 8, shape.js 8, decide.js 1). İlkeler çözülürken bu kurala göre temizlenir. Kullanıcıyla 16 Ekim'de kararlaştırıldı.

## İlke A · Baskının yönü: presçi, top sahibinin en tehlikeli seçeneklerini gövdesiyle kapatarak yaklaşır
Topa düz koşmak yerine, top sahibiyle onun en tehlikeli seçenekleri (kale, en boş arkadaş) arasına doğru yaklaşmak. İkinci presçi topa değil, kalan en tehlikeli pas yoluna.
Tartı olasılık değil, **olasılık × bize zararı** (varış yerinin hücum değeri − şimdiki değer; negatifse 0). Kullanıcı: "taşıyıcı arkaya pası da ihtimal olarak görüyorsa sorun yaratmaz mı" — geri pas olası ama zararsız; presçi onu bilerek açık bırakır, top sahibini geriye zorlar. Bu yüzden A, C'ye (değerlerin doğruluğu) bağlı.

- **A1 · Deniz–Ece (Liman–Demir Blok, 01:40, motor 6ad507e, eski an a-52-0-2845-kontra; yeni motorda yok, ekran görüntüsünden).** Şimşek ve Gökay ikisi de Deniz'in üstüne düz koşuyor. Kullanıcı: "biri baskı yaparken diğeri sağ köşede durup pas açısını kapatarak baskı yapmalı." Ayrıca Deniz'in kararı: sür 0.198 / pas → Ece 0.192, oysa Ece çok boş; pas belirgin daha değerli olmalı (bkz. İlke C).
- **A2 · Yelkovan–Tuna (Liman–Saat Kulesi, 01:55, motor 82a0ff0, `a-129-2-3473-iki-topta`).** Yelkovan Tuna'ya arkasından baskıya gidiyor. Kullanıcı: "prese gitmesi sorun değil ancak arkasındaki Kaan'ı da düşünerek, ayrıca Tuna'nın kaleye bakış açısını da sınırlamaya çalışarak durmalı."

- **A3 · Deniz, Rüzgâr–Şimşek'e karşı (D1/E1 ile aynı an, `a-52-0-6951-kontra`).** Deniz son adam; karşısında top sürülen Rüzgâr ve yanında koşan Şimşek (2'ye 1'e yakın). Kullanıcı: "Deniz de tabii ki bu duruma göre pozisyon almalı, gerekirse tercih yapmalı; hem Şimşek'i düşünmeli hem Rüzgâr'ın kaleye doğru sürme ve kaleyi görme–şut atma açısını."
  A'nın genel hâli: savunmacının yeri tek bir tehdide göre değil, birden fazla seçeneğin (sürüp kaleye gitmek, gönderme açısı, Şimşek'e pas) olasılık × zararına göre seçilir; hepsini kapatamıyorsa en zararlısını kapatıp ötekini geciktirir (tercih). Önceki Volkan–Deniz anı ("önü boşken yavaşlat") da bunun tek tehditli hâli.

### A 1. adım (main, `ALAN_TEHLIKE_YONU_ESKI`)
- match.js: top sahibinin kararındaki seçenekler (yer + değer) `m._sec`'e. shape.js `tehlike()`: her seçenek seçilme olasılığı (değerden softmax, τ 0.04) × zarar (değer − beklemenin değeri) ile tartılır, yakın yönler (< 0.45 rad) birleşir.
- İlk presçi kale tarafı yerine en tehlikeli yönün tarafına geçer (görev "pres · kale tarafına" ya da pas yoluysa "pres · yolu kapat"). İkinci presçi topun dibine değil, kapanmamış en tehlikeli yolun üçte birine (2.5–8 br; "pres · ikinci yolu kes"). Jokey ("geri geri", "ikisini kapat") kendi kalesi yönünde değil tehlikelerin tartılı yönünde durur.
- Kontrol (a-129-2-3473, 260 tik): top sahipliği tiklerinin 94/114'ünde tehlike okunuyor; yeni görevler görülüyor (kale tarafına 48, ikinci yolu kes 20, yolu kapat 9). Anlar motor değişikliklerinden sonra erken ayrılıyor (Rüzgâr–Deniz ve Yelkovan–Tuna pozisyonları bu sürümde oluşmuyor); kontrol kullanıcıda.

## İlke C · Değer: çok boş bir arkadaşa pas, sürmekten belirgin değerli
- **C1 · Deniz (A1 ile aynı an).** Sürme değeri, "sürdükten sonraki en iyi hamle"den geliyor (`ALAN_SUR_SONRA_ESKI`); sürünce pasın hâlâ açık kalacağı varsayılıyor olabilir. Doğrulanmadı.

### C ölçümü 1 · Rüzgâr 6903 (`a-52-0-6951-kontra`, motor 82a0ff0; araç: tools/anlar/c-olc.js)
- Pas → Şimşek ayağına: tutma %96 ama değer 0.010. Konum değeri düşük değil (V 0.124; Rüzgâr'ın kendi yeri 0.137), düşüren şu: Mert 5 br yanında, Şimşek pası alınca topu tutma ihtimali 0.64. Motorun mantığı tutarlı; çözüm C değil **E** (Şimşek açılsa bu ihtimal ve değer artar).
- Koşusunun önüne pas (31.9, 46): kafada %99,6 kayıp. Pas hızı 0.31 br/tik (oyuncu koşusu 0.29; `D.arrive` 0.3 ayağa pasın varış hızı) → 9 br'yi ~45 tikte gidiyor. Düz yetişme hesabında Şimşek top yoluna 18. tikte, Deniz 32., Mert 36. tikte yetişiyor; yani Şimşek topa koşsa alır. Kayıp büyük ihtimalle: pas çok yavaş + alıcı topa değil hedef noktaya koşuyor (doğrulanmadı). Çözüm **D**: önüne pasın sert türü (bugün sert tür yalnız ayağa pasta) ve alıcının topla buluşması.
- Sürme 0.284: "3–4 br sür, sonra önüne → Şimşek" planının değeri; sürme-sonrası hesabı yalnız yukarı düzeltiyor (`v2 > o.v` ise). Çözüm **C**: plan değeri bekleme/sürme riskiyle birlikte, iki yönlü.

### D ölçümü ve 1. adım (main, `ALAN_SERT_ONUNE_ESKI`)
- Rüzgâr 6903, önüne → Şimşek (31.9, 46) kafada: hız 0.31 → %0; 0.46 → %98; 0.62 → %99. Rakip çıkarma: araya giren Deniz (Deniz yokken yavaş pas %87). Sebep: buluşma noktası topun hızlı (1,1 br/tik) gideceği varsayımıyla bulunuyor, pas ise ayağa pasın varış hızıyla (0.3) yavaş atılıyordu.
- Değişiklik: her önüne pasın sert türü (varış ≥ `arriveFirm` 0.8) de aday; buluşma pası adayı görev yazısına değil koşu yönüne/hedefine bakar.
- Sonuç (aynı an, yeni motor, 6869): önüne → Şimşek (36.5, 43.1) yavaş %0,4 tutuyor, sert %99,4 tutuyor. Ama değeri 0.046, sür 0.330 ("sonra: fake" planından). Pas artık mümkün; seçilmemesi C (sürmenin iyimserliği) ve alınan yerin değeri meselesi.

### C ölçümü 2 · Rüzgâr 6869 (main 2fc7a9f; araç: tools/anlar/c-olc2.js, sonra hesabını açan hata ayıklama kancasıyla)
- Seçilen sür 0.330. "Sonra" hesabı olmadan: tut 0.231 · sür 0.194 · … · önüne → Şimşek (sert, %99 tutuyor) 0.085.
- 0.330'un kaynağı: 40 tik sürüp kopyada (31.2, 28.7)'ye varınca oradaki en iyi hamle **fake 0.363**; oradaki sür 0.187. İnanılırlığı %0'a yakın bir fake'in, fake'siz en iyi hamleden yüksek çıkması iyimserlik.
- Bulgular: **C-a** asimetri: sürme önündeki boşluğu (`runOn`) sayıyor, koşarak önüne pas alan alıcı saymıyor (`recvVal` yalnız V × tutma). **C-b** ikinci dünya iyimserliği: "sür sonra" ve "fake" başka bir dünyada yeniden verilen kararın en iyisini alıyor, sür-sonra yalnız yukarı düzeltiyor; birçok gürültülü tahminin en büyüğü sistematik olarak yüksek. **C-c** kontrada tut 0.231 sürmeden yüksek (kullanıcı: "topu tut asla yüksek değerli çıkmamalı").

### C 1. adım (main, `ALAN_ALICI_BOSLUK_ESKI`, `ALAN_SONRA_YARI_ESKI`) ve sonucu
- Alıcı da önündeki boşlukla (`runOn`) tartılır; sür-sonra değeri yarı ağırlıkla ve iki yönlü (kopyada kayıpsa düşer); fake'in aldanılmayan dünyası şimdiki en iyiyi geçemez.
- Aynı an (yeni motor): Rüzgâr 6883 sür 0.430 (sonra: sür), 40 tikte kaleye yaklaşıp 6928'de güçlü gönderiyor (değer 0.826), Aras tutuyor. Sürmenin yüksek değeri artık gerçek: Deniz onu kapatmadığı için gerçekten gönderme yerine varıyor → **A3**. Şimşek'e pas yine düşük (P 0.99'da değer −0.03…0.09): Mert dibinde, alınca tutma ihtimali düşük → **E**. Bu anın istenen hâli A ve E'ye bağlı.

### C-c (main, `ALAN_BEKLE_YARI_ESKI`) · katalog anı k-ruzgar-deniz, 6903
- Bekleme (0,5 sn) değeri = bekledikten sonraki en iyi hamle, tam ağırlıkla alınıyordu → yarı ağırlık, iki yönlü (sürme-sonrası ile aynı kural). 0.508 → 0.309; sür 0.204, yine bekleme seçiliyor.
- Kaynak: 30 tik bekleyince Mert dibine geliyor; o dünyada en iyi hamle **gövde fake'i 0.534** (sür 0.006). Maçta da tam bu oluyor: 6926 Rüzgâr yan gidiş gösteriyor, Mert ve Ece aldanıyor, Deniz'i geçiyor, Şimşek'e önüne pas. Yani "bekle, rakibi üstüne çek, çalımla geç" planı bu anda gerçekten işliyor. Açık soru (kullanıcıya): kontrada bu plan kabul mü, yoksa fake'in tutma ihtimali (herkes Okuma 20 iken %84 inanılırlık) mı fazla?

## İlke D · Koşucunun önüne ara pas: koşunun yönünde, kaleye doğru boşluğa
Pas hedefleri bugün üç kaynaktan: arkadaşın ayağı, "boşluğa pas" taraması (arkadaş savunmadan önce varmalı), buluşma noktası (yalnız görevi "koşu" olana). Koşan arkadaşın **önüne, kaleye doğru** atılan ara pas sistematik olarak ya üretilmiyor ya çok düşük değerleniyor.

- **D1 · Rüzgâr–Şimşek (Liman–Demir Blok, 03:50, motor 82a0ff0, `a-52-0-6951-kontra`).** Kullanıcı: "Şimşek gayet müsait pozisyondayken önüne ara pası hiç düşünmüyor bile, en sonunda atağın tehlikesini yiyor." Döküm (tik 6903, 19 seçenek): Şimşek (37.9, 46.0) kaleye doğru koşuyor (hız −0.32, −0.17). Ona pas adayları var ama hedefleri koşunun **yanında ya da gerisinde**: önüne → (36.1, 51.7) 0.089, pas → ayağa 0.010; koşunun önünde, kaleye doğru (≈ 28–30, 42) hiç aday yok. Seçilen sür 0.284. Görevi o anda "boşluk" (6916'da "kontra koşusu") olduğu için buluşma pası da üretilmiyor.
- Bağlı eski şikâyetler: Yıldırım'ın arkasına düşen pas; Volkan–Kartal kale ağzı pası (`ALAN_AGIZ_PAS_ESKI` yamasıyla kapatılmıştı); "Koşucunun arkasına pas" Anlar türü (9 maçta 23 kez).

## İlke E · Topsuz destek: top sahibinin düellosundan uzaklaşıp açı ve zaman yarat
Topsuz arkadaş, top sahibiyle savunmacının mücadelesine fazla yakın durmamalı: pası aldığı an savunmacı ona hemen gelebiliyor. Biraz (çok değil) açılırsa: pası aldığında savunmacının üstüne gelmesi zaman alır; o sürede ya top sahibine geri verip onu boşta bırakır (ver-kaç), ya da kaleyi daha iyi açıdan görüp gönderir.
Ölçü: alıcının yeri, "pası aldığında en yakın savunmacının ona varma süresi" ve "oradan kaleye açı / geri pas yolu" ile seçilmeli; topa yakınlık değil. (Motorda: shape.js "3 · Hücum: boşluk arama".)

### E 1. adım (main, `ALAN_DESTEK_TUT_ESKI`, `ALAN_IKILEM_ESKI`) ve sonucu
- Yer tartısında (shape.js teamV) alıcı seçeneği artık pası orada alınca tutma ihtimali × max(yer değeri, önündeki boşluk); savunmanın cevabı takıma karşı tartılır (beni kapatan savunmacı taşıyıcıyı bırakırsa taşıyıcı serbestler; değerim = takımın en iyisinin, beni saymayan savunmaya göre artışı).
- Bulgu: Şimşek'in yer seçiminde takım değeri bütün adaylarda 0 çıkıyordu (her noktayı yetişebilen bir savunmacı kapatıyor sayılıyordu); yeri yalnız "evden uzaklaşma" bağı belirliyordu.
- Sonuç bu anda: değişmedi. Mert Şimşek'i 3 br'den izliyor; Şimşek nereye giderse Mert onu kapatıp Rüzgâr'ı Deniz'e bırakabiliyor, yani Şimşek'in hiçbir yeri takıma bir şey katmıyor. Bu anın sonucu artık Deniz'e (A3) bağlı. Değişikliğin öteki anlarda etkisi kullanıcının kontrolünde.

- **E1 · Şimşek (D1 ile aynı an, `a-52-0-6951-kontra`).** Kullanıcı: "Şimşek de biraz genişleyebilir, çünkü savunmacı–Rüzgâr arasındaki mücadeleye fazla yakın; Rüzgâr ona pas attığı an savunmacı hemen ona kolayca gelebilir. Biraz genişlese, pas aldığı an savunmacı üzerine geldiğinde vakti olur ve tekrar Rüzgâr'a pas çıkarıp onu uygun pozisyonda bırakabilir, ya da kaleyi görme açısı çok daha iyi olur ve şut çekebilir."
- D ile bağlantı: D pası atanın tarafı (önüne pas hedefi), E alacak olanın tarafı (nerede durduğu). Aynı anda ikisi de eksik.

## Daha önce konuşulup aynı ilkelere bağlananlar (16 Ekim, ekran görüntüleri)
- Volkan önü boş sürerken Deniz yavaşlatmalıydı (sahte pres, yolu kapatma) → A'nın "yardım yoksa uzaktan yavaşlat" hâli.
- Kaan ve Mert ikisi de Volkan'a, Yıldırım boş → A (ikinci presçi pas yoluna).
- Savunma hattı baskısız topta çok yüksek; hücumcular arkadaki boşluğu kullanmıyor → (ayrı ilke, B: hattın yüksekliği topa baskıya göre).
